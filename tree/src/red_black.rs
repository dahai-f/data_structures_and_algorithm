use std::borrow::Borrow;
use std::cell::RefCell;
use std::cmp;
use std::cmp::Ordering;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
use std::option::Option::Some;
use std::rc::{Rc, Weak};

use crate::red_black::Color::{Black, Red};

type NodeRef<K, V> = RefCell<Node<K, V>>;
type RcNodeRef<K, V> = Rc<NodeRef<K, V>>;
type WeakNodeRef<K, V> = Weak<NodeRef<K, V>>;

#[derive(Copy, Clone, Eq, PartialEq)]
enum Color {
    Red,
    Black,
}

pub struct Node<K: Ord, V> {
    key: K,
    value: V,
    color: Color,
    parent: Option<WeakNodeRef<K, V>>,
    left: Option<RcNodeRef<K, V>>,
    right: Option<RcNodeRef<K, V>>,
}

#[derive(Default)]
pub struct RedBlackTree<K: Ord, V> {
    root: Option<RcNodeRef<K, V>>,
    length: usize,
}

struct ValidationResult {
    red_red_count: usize,
    black_height_min: usize,
    black_height_max: usize,
}

impl ValidationResult {
    fn is_valid(&self) -> bool {
        self.red_red_count == 0 && self.black_height_min == self.black_height_max
    }
}

impl<K: Ord, V> RedBlackTree<K, V> {
    pub fn length(&self) -> usize {
        self.length
    }

    pub fn is_valid(&self) -> bool {
        Self::validate(&self.root, Color::Red).is_valid()
    }
    fn validate(tree: &Option<RcNodeRef<K, V>>, parent_color: Color) -> ValidationResult {
        match tree {
            None => ValidationResult {
                red_red_count: 0,
                black_height_min: 1,
                black_height_max: 1,
            },
            Some(tree) => {
                let tree = tree.deref().borrow();
                let left_result = Self::validate(&tree.left, tree.color);
                let right_result = Self::validate(&tree.right, tree.color);
                let (self_red_red_count, self_black_count) = match tree.color {
                    Red => match parent_color {
                        Red => (1, 0),
                        Black => (0, 0),
                    },
                    Black => (0, 1),
                };

                ValidationResult {
                    red_red_count: left_result.red_red_count
                        + right_result.red_red_count
                        + self_red_red_count,
                    black_height_min: self_black_count
                        + cmp::min(left_result.black_height_min, right_result.black_height_min),
                    black_height_max: self_black_count
                        + cmp::min(left_result.black_height_max, right_result.black_height_max),
                }
            }
        }
    }

    fn add_rec(
        tree: Option<Rc<NodeRef<K, V>>>,
        parent: Option<Weak<NodeRef<K, V>>>,
        key: K,
        value: V,
    ) -> (RcNodeRef<K, V>, Result<RcNodeRef<K, V>, V>) {
        match tree {
            None => {
                let new_node = Rc::new(RefCell::new(Node {
                    key,
                    value,
                    color: Red,
                    parent,
                    left: None,
                    right: None,
                }));
                (new_node.clone(), Ok(new_node))
            }
            Some(tree) => {
                let weak = Rc::downgrade(&tree);
                let mut tree_node = RefCell::borrow_mut(&tree);
                let new_node;
                match key.cmp(&tree_node.key) {
                    Ordering::Less => {
                        let (left_root, new_node_) =
                            Self::add_rec(tree_node.left.take(), Some(weak), key, value);
                        tree_node.left = Some(left_root);
                        new_node = new_node_;
                    }
                    Ordering::Equal => {
                        let mut old_vale = value;
                        std::mem::swap(&mut tree_node.value, &mut old_vale);
                        new_node = Err(old_vale);
                    }
                    Ordering::Greater => {
                        let (right_root, new_node_) =
                            Self::add_rec(tree_node.right.take(), Some(weak), key, value);
                        tree_node.right = Some(right_root);
                        new_node = new_node_;
                    }
                }
                drop(tree_node);
                (tree, new_node)
            }
        }
    }
    fn is_left(child: &Node<K, V>, parent: &Node<K, V>) -> bool {
        if let Some(left) = &parent.left {
            std::ptr::eq(left.as_ptr(), child)
        } else {
            false
        }
    }
    fn replace_child(
        parent: &Option<WeakNodeRef<K, V>>,
        child: &Node<K, V>,
        new_child: RcNodeRef<K, V>,
    ) {
        new_child.borrow_mut().parent = match parent {
            Some(parent_weak) => {
                let parent_rc = parent_weak.upgrade().unwrap();
                let mut parent = parent_rc.borrow_mut();
                if Self::is_left(child, &parent) {
                    parent.left = Some(new_child.clone());
                } else {
                    parent.right = Some(new_child.clone());
                }
                Some(parent_weak.clone())
            }
            None => None,
        }
    }
    fn rotate(&mut self, node_rc: RcNodeRef<K, V>, is_right: bool) {
        let mut node = node_rc.borrow_mut();
        let node_child = if is_right {
            node.left.clone()
        } else {
            node.right.clone()
        };
        match node_child {
            None => {}
            Some(node_child_rc) => {
                Self::replace_child(&node.parent, &node, node_child_rc.clone());
                let mut node_child = node_child_rc.borrow_mut();
                if is_right {
                    node.left = node_child.right.clone();
                    if let Some(node_child_right) = &node_child.right {
                        node_child_right.borrow_mut().parent = Some(Rc::downgrade(&node_rc))
                    }
                    node_child.right = Some(node_rc.clone());
                } else {
                    node.right = node_child.left.clone();
                    if let Some(node_child_left) = &node_child.left {
                        node_child_left.borrow_mut().parent = Some(Rc::downgrade(&node_rc))
                    }
                    node_child.left = Some(node_rc.clone());
                }
                node.parent = Some(Rc::downgrade(&node_child_rc));

                if self.root.clone().unwrap().as_ptr() == (node.deref_mut()) as *mut _ {
                    drop(node_child);
                    self.root = Some(node_child_rc)
                }
            }
        }
    }
    fn fix_on_add(&mut self, mut new_node_rc: RcNodeRef<K, V>) {
        loop {
            let new_node = new_node_rc.deref().borrow();
            let parent_rc = match &new_node.parent {
                None => {
                    break;
                }
                Some(parent_weak) => parent_weak.upgrade().unwrap(),
            };
            let mut parent = parent_rc.borrow_mut();
            if parent.color == Black {
                break;
            }
            let grand_parent_rc = match &parent.parent {
                None => {
                    break;
                }
                Some(grand_parent) => grand_parent.upgrade().unwrap(),
            };
            let mut grand_parent = grand_parent_rc.borrow_mut();
            let parent_is_left = Self::is_left(&parent, &grand_parent);
            let uncle = if parent_is_left {
                &grand_parent.right
            } else {
                &grand_parent.left
            };
            if let Some(uncle) = uncle {
                let mut uncle = uncle.borrow_mut();
                if uncle.color == Red {
                    uncle.color = Black;
                    parent.color = Black;
                    drop(uncle);
                    grand_parent.color = Red;
                    drop(new_node);
                    new_node_rc = grand_parent_rc.clone();
                    continue;
                }
            }

            let self_is_left = Self::is_left(&new_node_rc.deref().borrow(), &parent);
            if parent_is_left == self_is_left {
                grand_parent.color = Red;
                parent.color = Black;
                drop(grand_parent);
                drop(parent);
                self.rotate(grand_parent_rc, parent_is_left);
                return;
            } else {
                drop(parent);
                drop(new_node);
                drop(grand_parent);
                self.rotate(parent_rc.clone(), self_is_left);
                new_node_rc = parent_rc;
            }
        }

        self.root.as_ref().unwrap().borrow_mut().color = Black;
    }
    pub fn add(&mut self, key: K, value: V) -> Option<V> {
        let (root, new_node) = Self::add_rec(self.root.take(), None, key, value);
        self.root = Some(root);
        match new_node {
            Ok(new_node) => {
                self.length += 1;
                self.fix_on_add(new_node);
                None
            }
            Err(old_value) => Some(old_value),
        }
    }

    fn left_most(mut node_rc: RcNodeRef<K, V>) -> RcNodeRef<K, V> {
        loop {
            let node = node_rc.deref().borrow();
            if let Some(left) = node.deref().left.clone() {
                drop(node);
                node_rc = left;
            } else {
                break;
            }
        }
        node_rc
    }
    fn right_most(mut node_rc: RcNodeRef<K, V>) -> RcNodeRef<K, V> {
        loop {
            let node = node_rc.deref().borrow();
            if let Some(right) = node.deref().right.clone() {
                drop(node);
                node_rc = right;
            } else {
                break;
            }
        }
        node_rc
    }
    fn first_right_parent(mut node_rc: RcNodeRef<K, V>) -> Option<RcNodeRef<K, V>> {
        loop {
            let node = node_rc.deref().borrow();
            if let Some(parent) = node.parent.clone() {
                let parent = parent.upgrade().unwrap();
                if Self::is_left(&node.borrow(), &parent.deref().borrow()) {
                    return Some(parent);
                }
                drop(node);
                node_rc = parent
            } else {
                break;
            }
        }
        None
    }
    fn next(node_rc: RcNodeRef<K, V>) -> Option<RcNodeRef<K, V>> {
        let node = node_rc.deref().borrow();
        let right = &node.right;
        match right {
            None => {
                drop(node);
                Self::first_right_parent(node_rc)
            }
            Some(right) => Some(Self::left_most(right.clone())),
        }
    }
    pub fn iter(&self) -> Iter<K, V> {
        Iter {
            cur: self.root.clone().map(Self::left_most),
            _phantom: Default::default(),
        }
    }
    fn swap_key_and_value(a: &mut Node<K, V>, b: &mut Node<K, V>) {
        std::mem::swap(&mut a.key, &mut b.key);
        std::mem::swap(&mut a.value, &mut b.value);
    }
    fn fix_on_remove(&mut self, mut node_rc: RcNodeRef<K, V>) {
        loop {
            let node = node_rc.deref().borrow();
            if node.color == Red {
                break;
            }
            let parent_rc = match &node.parent {
                None => {
                    break;
                }
                Some(parent) => parent.upgrade().unwrap(),
            };
            let mut parent = parent_rc.deref().borrow_mut();
            let is_left = Self::is_left(&node, &parent);
            let brother_rc = if is_left {
                parent.right.clone().unwrap()
            } else {
                parent.left.clone().unwrap()
            };
            let mut brother = brother_rc.deref().borrow_mut();
            if brother.color == Red {
                brother.color = Black;
                parent.color = Red;
                drop(parent);
                drop(brother);
                self.rotate(parent_rc, !is_left);
            } else {
                if let Some(brother_far) = if is_left {
                    brother.right.clone()
                } else {
                    brother.left.clone()
                } {
                    let mut brother_far = brother_far.borrow_mut();
                    if brother_far.color == Red {
                        brother.color = parent.color;
                        parent.color = Black;
                        brother_far.color = Black;
                        drop(parent);
                        drop(brother);
                        self.rotate(parent_rc, !is_left);
                        break;
                    }
                }

                if let Some(brother_close) = if is_left {
                    brother.left.clone()
                } else {
                    brother.right.clone()
                } {
                    let mut brother_close = brother_close.borrow_mut();
                    if brother_close.color == Red {
                        brother.color = Red;
                        brother_close.color = Black;
                        drop(brother);
                        drop(brother_close);
                        self.rotate(brother_rc, is_left);
                        continue;
                    }
                }

                brother.color = Red;
                if parent.color == Red {
                    parent.color = Black;
                    break;
                }

                drop(node);
                drop(parent);
                node_rc = parent_rc;
            }
        }
    }
    fn remove_node(&mut self, node_rc: RcNodeRef<K, V>) -> RcNodeRef<K, V> {
        let mut node = node_rc.deref().borrow_mut();
        if let Some(left) = &node.left {
            let left = Self::right_most(left.clone());
            Self::swap_key_and_value(&mut node, &mut left.borrow_mut());
            self.remove_node(left)
        } else if let Some(right) = &node.right {
            let right = Self::left_most(right.clone());
            Self::swap_key_and_value(&mut node, &mut right.borrow_mut());
            self.remove_node(right)
        } else {
            self.fix_on_remove(node_rc.clone());
            if let Some(parent) = &node.parent {
                let parent = parent.upgrade().unwrap();
                let mut parent = parent.deref().borrow_mut();
                if Self::is_left(&node, &parent) {
                    parent.left = None;
                } else {
                    parent.right = None;
                }
                node.parent = None;
            }
            drop(node);
            node_rc
        }
    }
    pub fn remove(&mut self, key: &K) -> Option<V> {
        let mut node_opt = self.root.clone();
        loop {
            let node_rc = match &node_opt {
                None => {
                    return None;
                }
                Some(node) => node.clone(),
            };

            let node = node_rc.deref().borrow();
            let next = match key.cmp(&node.key) {
                Ordering::Less => node.left.clone(),
                Ordering::Equal => {
                    drop(node);
                    let removed_node = self.remove_node(node_rc);
                    if removed_node.as_ptr() == self.root.as_ref().unwrap().as_ptr() {
                        self.root = None
                    }
                    let removed_node = Rc::try_unwrap(removed_node).ok().unwrap();
                    return Some(removed_node.into_inner().value);
                }
                Ordering::Greater => node.right.clone(),
            };

            drop(node);
            node_opt = next;
        }
    }
}

pub struct Iter<'n, K: Ord, V> {
    cur: Option<RcNodeRef<K, V>>,
    _phantom: PhantomData<&'n ()>,
}

impl<'n, K: Ord, V> Iterator for Iter<'n, K, V> {
    type Item = RcNodeRef<K, V>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.cur.take() {
            None => None,
            Some(cur) => {
                self.cur = RedBlackTree::next(cur.clone());
                Some(cur)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::ops::Deref;

    use rand::prelude::SliceRandom;

    use crate::red_black::*;

    macro_rules! new_nodes {
        ($num:expr) => {{
            let mut nodes = vec![];
            nodes.reserve($num);
            for i in 0..$num {
                nodes.push(Rc::new(RefCell::new(Node {
                    key: i,
                    value: (),
                    color: Color::Red,
                    parent: None,
                    left: None,
                    right: None,
                })));
            }
            nodes
        }};
    }

    #[test]
    fn add() {
        let mut tree = RedBlackTree::default();
        const NUM: i32 = 100;
        let mut elements: Vec<i32> = (0..NUM).collect();
        elements.shuffle(&mut rand::thread_rng());
        elements.iter().for_each(|element| {
            let element = *element;
            assert_eq!(tree.add(element, element), None);
            assert!(tree.is_valid());
        });
        assert_eq!(tree.add(99, 99), Some(99));
        assert_eq!(
            tree.iter()
                .map(|node| node.deref().borrow().key)
                .collect::<Vec<i32>>(),
            (0..NUM).collect::<Vec<i32>>()
        );
    }

    #[test]
    fn rotate_left() {
        let mut tree = RedBlackTree::default();
        let nodes = new_nodes!(10);
        tree.root = Some(nodes[0].clone());
        tree.root.as_ref().unwrap().borrow_mut().right = Some(nodes[1].clone());
        nodes[1].borrow_mut().parent = Some(Rc::downgrade(&nodes[0]));
        tree.rotate(tree.root.clone().unwrap(), false);
        //   1
        // 0
        assert_eq!(tree.root.clone().unwrap().borrow_mut().key, 1);
        assert_eq!(
            tree.root
                .clone()
                .unwrap()
                .borrow_mut()
                .left
                .clone()
                .unwrap()
                .borrow_mut()
                .key,
            0
        );
        assert!(tree.root.clone().unwrap().borrow_mut().right.is_none());
        assert_eq!(
            tree.root
                .clone()
                .unwrap()
                .deref()
                .borrow()
                .left
                .clone()
                .unwrap()
                .borrow_mut()
                .parent
                .clone()
                .unwrap()
                .upgrade()
                .unwrap()
                .deref()
                .borrow()
                .key,
            1
        );
        assert!(nodes[1].borrow_mut().parent.is_none());

        tree.root.clone().unwrap().borrow_mut().right = Some(nodes[2].clone());
        tree.rotate(tree.root.clone().unwrap(), false);
        //    2
        //  1
        // 0
        assert_eq!(tree.root.clone().unwrap().borrow_mut().key, 2);
        assert_eq!(
            tree.root
                .clone()
                .unwrap()
                .borrow_mut()
                .left
                .clone()
                .unwrap()
                .borrow_mut()
                .key,
            1
        );
        assert!(tree.root.clone().unwrap().borrow_mut().right.is_none());
        assert_eq!(
            tree.root
                .clone()
                .unwrap()
                .borrow_mut()
                .left
                .clone()
                .unwrap()
                .borrow_mut()
                .left
                .clone()
                .unwrap()
                .borrow_mut()
                .key,
            0
        );
        fn get_left_left_parent<K: Ord + Copy, V>(root: &Option<RcNodeRef<K, V>>) -> K {
            root.clone()
                .unwrap()
                .borrow_mut()
                .left
                .clone()
                .unwrap()
                .deref()
                .borrow()
                .left
                .clone()
                .unwrap()
                .borrow_mut()
                .parent
                .clone()
                .unwrap()
                .upgrade()
                .unwrap()
                .deref()
                .borrow()
                .key
        }
        assert_eq!(get_left_left_parent(&tree.root), 1);
        fn get_left_parent<K: Ord + Copy, V>(root: &Option<RcNodeRef<K, V>>) -> K {
            root.clone()
                .unwrap()
                .deref()
                .borrow()
                .left
                .clone()
                .unwrap()
                .borrow_mut()
                .parent
                .clone()
                .unwrap()
                .upgrade()
                .unwrap()
                .deref()
                .borrow()
                .key
        }
        assert_eq!(get_left_parent(&tree.root), 2);

        tree.root
            .clone()
            .unwrap()
            .borrow_mut()
            .left
            .clone()
            .unwrap()
            .borrow_mut()
            .right = Some(nodes[3].clone());
        tree.root
            .clone()
            .unwrap()
            .borrow_mut()
            .left
            .clone()
            .unwrap()
            .borrow_mut()
            .right
            .clone()
            .unwrap()
            .borrow_mut()
            .right = Some(nodes[4].clone());

        tree.root
            .clone()
            .unwrap()
            .borrow_mut()
            .left
            .clone()
            .unwrap()
            .borrow_mut()
            .right
            .clone()
            .unwrap()
            .borrow_mut()
            .right
            .clone()
            .unwrap()
            .borrow_mut()
            .parent = Some(Rc::downgrade(&nodes[3]));
        let root_left = tree.root.clone().unwrap().deref().borrow().left.clone();
        tree.rotate(root_left.unwrap(), false);
        let root_left = tree.root.clone().unwrap().deref().borrow().left.clone();
        //    2
        //   3
        //  1 4
        // 0
        assert_eq!(tree.root.clone().unwrap().borrow_mut().key, 2);
        assert_eq!(root_left.clone().unwrap().borrow_mut().key, 3);
        assert_eq!(
            root_left
                .clone()
                .unwrap()
                .borrow_mut()
                .left
                .clone()
                .unwrap()
                .borrow_mut()
                .key,
            1
        );
        assert_eq!(
            root_left
                .clone()
                .unwrap()
                .borrow_mut()
                .right
                .clone()
                .unwrap()
                .borrow_mut()
                .key,
            4
        );
        assert_eq!(
            root_left
                .unwrap()
                .borrow_mut()
                .left
                .clone()
                .unwrap()
                .borrow_mut()
                .left
                .clone()
                .unwrap()
                .borrow_mut()
                .key,
            0
        );

        assert_eq!(
            tree.root
                .clone()
                .unwrap()
                .borrow_mut()
                .left
                .clone()
                .unwrap()
                .borrow_mut()
                .left
                .clone()
                .unwrap()
                .deref()
                .borrow()
                .left
                .clone()
                .unwrap()
                .borrow_mut()
                .parent
                .clone()
                .unwrap()
                .upgrade()
                .unwrap()
                .deref()
                .borrow()
                .key,
            1
        );
        assert_eq!(get_left_left_parent(&tree.root), 3);
        assert_eq!(
            tree.root
                .clone()
                .unwrap()
                .borrow_mut()
                .left
                .clone()
                .unwrap()
                .deref()
                .borrow()
                .right
                .clone()
                .unwrap()
                .borrow_mut()
                .parent
                .clone()
                .unwrap()
                .upgrade()
                .unwrap()
                .deref()
                .borrow()
                .key,
            3
        );
        assert_eq!(get_left_parent(&tree.root), 2);

        //    2             3
        //   3            1   2
        //  1 4     ->   0   4
        // 0
        tree.rotate(tree.root.clone().unwrap(), true);
        assert_eq!(tree.root.clone().unwrap().as_ptr(), nodes[3].as_ptr());
        assert!(tree.root.unwrap().deref().borrow().parent.is_none());
    }

    #[test]
    fn rc_eq() {
        let a = Rc::new(RefCell::new(1));
        let b = Rc::new(RefCell::new(1));
        assert!(!a.as_ptr().eq(&b.as_ptr()));
        assert_eq!(a, b);
        assert_ne!(a.as_ptr(), b.as_ptr());
    }
}
