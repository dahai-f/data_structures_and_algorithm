use std::cmp::Ordering;

type Tree<K, V> = Option<Box<Node<K, V>>>;

struct Node<K: Ord, V> {
    key: K,
    value: V,
    left: Tree<K, V>,
    right: Tree<K, V>,
}

pub struct BinarySearchTree<K: Ord, V> {
    root: Tree<K, V>,
    length: usize,
}

impl<K: Ord, V> Default for BinarySearchTree<K, V> {
    fn default() -> Self {
        Self {
            root: None,
            length: 0,
        }
    }
}

impl<K: Ord, V> BinarySearchTree<K, V> {
    fn add_rec(root: Tree<K, V>, (key, value): (K, V)) -> Box<Node<K, V>> {
        match root {
            None => Box::new(Node {
                key,
                value,
                left: None,
                right: None,
            }),
            Some(mut root) => {
                match key.cmp(&root.key) {
                    Ordering::Less => {
                        root.left = Some(Self::add_rec(root.left, (key, value)));
                    }
                    Ordering::Equal => root.value = value,
                    Ordering::Greater => {
                        root.right = Some(Self::add_rec(root.right, (key, value)));
                    }
                }
                root
            }
        }
    }

    pub fn add(&mut self, key: K, value: V) {
        self.length += 1;
        let root = self.root.take();
        self.root = Some(Self::add_rec(root, (key, value)));
    }

    fn find_rec<'b>(root: &'b Tree<K, V>, target: &K) -> Option<&'b V> {
        match root {
            None => None,
            Some(root) => match target.cmp(&root.key) {
                Ordering::Less => Self::find_rec(&root.left, target),
                Ordering::Equal => Some(&root.value),
                Ordering::Greater => Self::find_rec(&root.right, target),
            },
        }
    }

    pub fn find(&self, target: &K) -> Option<&V> {
        Self::find_rec(&self.root, target)
    }

    fn walk_rec(root: &Tree<K, V>, callback: &mut impl FnMut((&K, &V))) {
        if let Some(root) = root {
            Self::walk_rec(&root.left, callback);
            callback((&root.key, &root.value));
            Self::walk_rec(&root.right, callback);
        }
    }

    pub fn walk(&self, mut callback: impl FnMut((&K, &V))) {
        Self::walk_rec(&self.root, &mut callback)
    }
}
