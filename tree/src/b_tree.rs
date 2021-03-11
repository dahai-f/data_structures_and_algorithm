use std::cmp::Ordering;

type Pair<K, V> = (K, V);
type Tree<K, V> = Box<Node<K, V>>;

struct SplitInfo<K: Ord, V> {
    mid_pair: Pair<K, V>,
    right_child: Tree<K, V>,
}

impl<K: Ord, V> SplitInfo<K, V> {
    fn new(mid_pair: Pair<K, V>, right_child: Tree<K, V>) -> Self {
        Self {
            mid_pair,
            right_child,
        }
    }
}

struct Node<K: Ord, V> {
    pairs: Vec<Pair<K, V>>,
    children: Vec<Tree<K, V>>,
}

impl<K: Ord, V> Node<K, V> {
    fn new_leaf() -> Tree<K, V> {
        Box::new(Self {
            pairs: vec![],
            children: vec![],
        })
    }

    fn new_with_data(pairs: Vec<Pair<K, V>>, children: Option<Vec<Tree<K, V>>>) -> Tree<K, V> {
        Box::new(Self {
            pairs,
            children: children.unwrap_or_else(Vec::default),
        })
    }

    fn new_root(pair: Pair<K, V>, left_child: Tree<K, V>, right_child: Tree<K, V>) -> Tree<K, V> {
        Box::new(Self {
            pairs: vec![pair],
            children: vec![left_child, right_child],
        })
    }

    fn add_pair(
        &mut self,
        max_children_length: usize,
        to_insert: usize,
        pair: Pair<K, V>,
        right_child: Option<Tree<K, V>>,
    ) -> Option<SplitInfo<K, V>> {
        if self.pairs.len() + 1 >= max_children_length {
            let new_pair_index = (self.pairs.len() + 1) / 2;
            let (mid_pair, right_pairs, right_children) = match to_insert.cmp(&new_pair_index) {
                Ordering::Less => {
                    let right_pairs = self.pairs.split_off(new_pair_index);
                    let mid_pair = self.pairs.pop().unwrap();
                    self.pairs.insert(to_insert, pair);
                    let right_children = right_child.map(|right_child| {
                        let right_children = self.children.split_off(new_pair_index);
                        self.children.insert(to_insert + 1, right_child);
                        right_children
                    });
                    (mid_pair, right_pairs, right_children)
                }
                Ordering::Equal => {
                    let right_pairs = self.pairs.split_off(new_pair_index);
                    let right_children = right_child.map(|right_child| {
                        let mut right_children = self.children.split_off(new_pair_index + 1);
                        right_children.insert(0, right_child);
                        right_children
                    });
                    (pair, right_pairs, right_children)
                }
                Ordering::Greater => {
                    let mut right_pairs = self.pairs.split_off(new_pair_index + 1);
                    let mid_pair = self.pairs.pop().unwrap();
                    let to_insert = to_insert - (new_pair_index + 1);
                    right_pairs.insert(to_insert, pair);
                    let right_children = right_child.map(|right_child| {
                        let mut right_children = self.children.split_off(new_pair_index + 1);
                        right_children.insert(to_insert + 1, right_child);
                        right_children
                    });
                    (mid_pair, right_pairs, right_children)
                }
            };
            Some(SplitInfo::new(
                mid_pair,
                Node::new_with_data(right_pairs, right_children),
            ))
        } else {
            self.pairs.insert(to_insert, pair);
            if let Some(right_child) = right_child {
                self.children.insert(to_insert + 1, right_child);
            }
            None
        }
    }
}

pub struct BTree<K: Ord, V> {
    root: Option<Tree<K, V>>,
    max_children_length: usize,
    length: usize,
}

impl<K: Ord, V> Default for BTree<K, V> {
    fn default() -> Self {
        Self::new(128)
    }
}

impl<K: Ord, V> BTree<K, V> {
    pub fn new(max_children_length: usize) -> Self {
        Self {
            root: None,
            max_children_length,
            length: 0,
        }
    }

    pub fn is_valid(&self) -> bool {
        match self.root.as_ref() {
            None => true,
            Some(root) => self.validate(root, 0).is_ok(),
        }
    }
    fn validate(&self, node: &Tree<K, V>, node_level: usize) -> Result<usize, ()> {
        let mut children = node.children.iter();
        if let Some(child) = children.next() {
            if node.children.len() > self.max_children_length {
                return Err(());
            }
            let min_children_len = if node_level > 0 {
                self.max_children_length / 2
            } else {
                2
            };
            if node.children.len() < min_children_len {
                return Err(());
            };

            let next_level = node_level + 1;
            let child_level = self.validate(child, next_level)?;
            for child in children {
                if self.validate(child, next_level)? != child_level {
                    return Err(());
                }
            }
            Ok(child_level)
        } else {
            // 无子节点说明自己是叶子节点
            Ok(node_level)
        }
    }

    pub fn add(&mut self, key: K, value: V) -> Option<V> {
        let mut node = match self.root.take() {
            Some(root) => root,
            None => Node::new_leaf(),
        };
        let (split, old_value) = self.add_r(&mut node, key, value);
        self.root = match split {
            None => Some(node),
            Some(split) => Some(Node::new_root(split.mid_pair, node, split.right_child)),
        };
        old_value
    }
    fn add_r(
        &mut self,
        node: &mut Tree<K, V>,
        key: K,
        value: V,
    ) -> (Option<SplitInfo<K, V>>, Option<V>) {
        let mut value = value;
        match node.pairs.binary_search_by_key(&&key, |(key, _value)| &key) {
            Ok(found) => {
                std::mem::swap(&mut value, &mut node.pairs[found].1);
                (None, Some(value))
            }
            Err(to_insert) => {
                match node.children.get_mut(to_insert) {
                    None => {
                        // 找不到child，说明node为叶子节点，则执行插入
                        self.length += 1;
                        let split =
                            node.add_pair(self.max_children_length, to_insert, (key, value), None);
                        (split, None)
                    }
                    Some(child) => {
                        let (split, old_value) = self.add_r(child, key, value);
                        let split = split.and_then(|split| {
                            node.add_pair(
                                self.max_children_length,
                                to_insert,
                                split.mid_pair,
                                Some(split.right_child),
                            )
                        });
                        (split, old_value)
                    }
                }
            }
        }
    }

    pub fn remove(&mut self, key: &K) -> Option<V> {
        let (root, removed_value) = match self.root.take() {
            None => (None, None),
            Some(mut root) => {
                let removed_value = self.remove_r(&mut root, key);
                if removed_value.is_some() {
                    if root.pairs.is_empty() {
                        (None, removed_value)
                    } else {
                        (Some(root), removed_value)
                    }
                } else {
                    (Some(root), None)
                }
            }
        };

        self.root = root;
        removed_value
    }
    fn remove_r(&mut self, node: &mut Tree<K, V>, key: &K) -> Option<V> {
        match node.pairs.binary_search_by_key(&key, |(key, _value)| &key) {
            Ok(found) => {
                match node.children.get_mut(found) {
                    None => {
                        // 未获取到child，则说明node为叶子节点，则执行删除
                        self.length -= 1;
                        Some(node.pairs.remove(found).1)
                    }
                    Some(child) => {
                        let mut pre = self.remove_right_most_r(child);
                        std::mem::swap(&mut pre, &mut node.pairs[found]);
                        Some(pre.1)
                    }
                }
            }
            Err(to_insert) => match node.children.get_mut(to_insert) {
                None => None,
                Some(child) => self.remove_r(child, key),
            },
        }
    }
    fn remove_right_most_r(&mut self, node: &mut Tree<K, V>) -> Pair<K, V> {
        match node.children.last_mut() {
            None => {
                // 未获取到child，则说明node为叶子节点，则执行删除
                self.length -= 1;
                node.pairs.pop().unwrap()
            }
            Some(last_child) => self.remove_right_most_r(last_child),
        }
    }
}
