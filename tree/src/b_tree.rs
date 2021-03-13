use std::cmp::Ordering;
use std::option::Option::None;

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
        Self::new(256)
    }
}

impl<K: Ord, V> BTree<K, V> {
    pub fn new(max_children_length: usize) -> Self {
        assert!(max_children_length >= 4);
        Self {
            root: None,
            max_children_length,
            length: 0,
        }
    }
    pub fn len(&self) -> usize {
        self.length
    }
    pub fn is_empty(&self) -> bool {
        self.length == 0
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
                        assert!(root.children.len() <= 1);
                        (root.children.pop(), removed_value)
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
                        self.fix_on_child_removed(node, found);
                        Some(pre.1)
                    }
                }
            }
            Err(to_insert) => match node.children.get_mut(to_insert) {
                None => None,
                Some(child) => {
                    let removed = self.remove_r(child, key);
                    self.fix_on_child_removed(node, to_insert);
                    removed
                }
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
            Some(last_child) => {
                let removed = self.remove_right_most_r(last_child);
                self.fix_on_child_removed(node, node.children.len() - 1);
                removed
            }
        }
    }
    fn fix_on_child_removed(&mut self, node: &mut Tree<K, V>, child_index: usize) {
        let min_pairs_len = self.max_children_length / 2 - 1;
        if node.children[child_index].pairs.len() >= min_pairs_len {
            return;
        }

        let left_child_index = match child_index.checked_sub(1) {
            None => None,
            Some(left_child_index) => {
                if node.children[left_child_index].pairs.len() > min_pairs_len {
                    let mut temp = node.children[left_child_index].pairs.pop().unwrap();
                    std::mem::swap(&mut temp, &mut node.pairs[left_child_index]);
                    node.children[child_index].pairs.insert(0, temp);
                    if let Some(left_child_right) = node.children[left_child_index].children.pop() {
                        node.children[child_index]
                            .children
                            .insert(0, left_child_right);
                    }
                    return;
                }
                Some(left_child_index)
            }
        };
        let right_child_index = child_index + 1;
        let right_child_index = match right_child_index >= node.children.len() {
            true => None,
            false => {
                if node.children[right_child_index].pairs.len() > min_pairs_len {
                    let mut temp = node.children[right_child_index].pairs.remove(0);
                    std::mem::swap(&mut temp, &mut node.pairs[child_index]);
                    node.children[child_index].pairs.push(temp);
                    if !node.children[right_child_index].children.is_empty() {
                        let right_child_left = node.children[right_child_index].children.remove(0);
                        node.children[child_index].children.push(right_child_left);
                    }
                    return;
                }
                Some(right_child_index)
            }
        };
        let (merge_left, to_merge_child_index) = match (left_child_index, right_child_index) {
            (Some(left_child_index), Some(right_child_index)) => {
                if node.children[left_child_index].pairs.len()
                    < node.children[right_child_index].pairs.len()
                {
                    (true, left_child_index)
                } else {
                    (false, right_child_index)
                }
            }
            (None, Some(right_child_index)) => (false, right_child_index),
            (Some(left_child_index), None) => (true, left_child_index),
            (None, None) => {
                unreachable!()
            }
        };

        let (merge_left, pair_in_node, mut merge_right) = if merge_left {
            let pair_in_node = node.pairs.remove(to_merge_child_index);
            let child = node.children.remove(child_index);
            let left_child = &mut node.children[to_merge_child_index];
            (left_child, pair_in_node, child)
        } else {
            let pair_in_node = node.pairs.remove(child_index);
            let merge_right = node.children.remove(to_merge_child_index);
            let merge_left = &mut node.children[child_index];
            (merge_left, pair_in_node, merge_right)
        };
        merge_left.pairs.push(pair_in_node);
        merge_left.pairs.append(&mut merge_right.pairs);
        merge_left.children.append(&mut merge_right.children);
    }

    pub fn iter(&self) -> Iter<K, V> {
        Iter {
            b_tree: self,
            remain_len: self.len(),
            stack: vec![],
        }
    }
}

pub struct Iter<'t, K: Ord, V> {
    b_tree: &'t BTree<K, V>,
    remain_len: usize,
    stack: Vec<(&'t Tree<K, V>, usize)>,
}

impl<'t, K: Ord, V> Iter<'t, K, V> {
    fn push_node(&mut self, node: &'t Tree<K, V>) {
        self.stack.push((node, 0));
        let mut left = node;
        while let Some(left_left) = left.children.first() {
            left = left_left;
            self.stack.push((left, 0));
        }
    }
    fn pop_node(&mut self) {
        self.stack.pop();
        while let Some((top_node, pair_index_in_top_node)) = self.stack.last() {
            if *pair_index_in_top_node < top_node.pairs.len() {
                return;
            } else {
                self.stack.pop();
            }
        }
    }
}

impl<'t, K: Ord, V> Iterator for Iter<'t, K, V> {
    type Item = &'t Pair<K, V>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remain_len == 0 {
            return None;
        }

        if self.stack.is_empty() {
            let root = self.b_tree.root.as_ref().unwrap();
            self.push_node(root);
        }

        let (top_node, pair_index_in_top_node) = self.stack.last_mut().unwrap();
        let res = &top_node.pairs[*pair_index_in_top_node];
        *pair_index_in_top_node += 1;
        if let Some(next_child) = top_node.children.get(*pair_index_in_top_node) {
            self.push_node(next_child)
        } else if *pair_index_in_top_node >= top_node.pairs.len() {
            self.pop_node();
        }

        self.remain_len -= 1;
        Some(res)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.remain_len, Some(self.remain_len))
    }
}

#[cfg(test)]
mod tests {
    use rand::prelude::SliceRandom;

    use crate::b_tree::BTree;

    #[test]
    fn add_and_remove() {
        let test_nums = 1000usize;
        let mut b_tree = BTree::default();
        let mut nums: Vec<usize> = (0..test_nums).collect();
        let mut rng = rand::thread_rng();
        nums.shuffle(&mut rng);
        let mut len = 0usize;
        nums.iter().for_each(|&i| {
            assert_eq!(b_tree.add(i, i), None);
            assert!(b_tree.is_valid());
            len += 1;
            assert_eq!(b_tree.len(), len);
        });

        assert_eq!(b_tree.add(100, 100), Some(100));
        assert_eq!(b_tree.len(), len);
        assert_eq!(len, test_nums);

        nums.shuffle(&mut rng);
        nums.iter().for_each(|&i| {
            assert_eq!(b_tree.remove(&i), Some(i));
            assert!(b_tree.is_valid());
            len -= 1;
            assert_eq!(b_tree.len(), len);
        });

        assert_eq!(len, 0);
        assert_eq!(b_tree.remove(&10), None);
        assert_eq!(b_tree.len(), 0);
    }

    #[test]
    fn iter() {
        let test_nums = 100_000usize;
        let mut nums: Vec<usize> = (0..test_nums).collect();
        let mut rng = rand::thread_rng();
        nums.shuffle(&mut rng);
        let mut b_tree = BTree::new(256);
        nums.iter().for_each(|&i| {
            b_tree.add(i, i);
        });

        let mut num = 0usize;
        b_tree.iter().for_each(|&(key, value)| {
            assert_eq!(key, num);
            assert_eq!(value, num);
            num += 1;
        });
        assert_eq!(num, b_tree.len());
    }

    #[test]
    fn iter_std_b_tree() {
        let test_nums = 100_000usize;
        let mut nums: Vec<usize> = (0..test_nums).collect();
        let mut rng = rand::thread_rng();
        nums.shuffle(&mut rng);
        let mut b_tree = std::collections::btree_map::BTreeMap::default();
        nums.iter().for_each(|&i| {
            b_tree.insert(i, i);
        });

        let mut num = 0usize;
        b_tree.iter().for_each(|(&key, &value)| {
            assert_eq!(key, num);
            assert_eq!(value, num);
            num += 1;
        });
        assert_eq!(num, b_tree.len());
    }
}
