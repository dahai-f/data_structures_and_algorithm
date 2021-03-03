use std::borrow::Borrow;
use std::cell::RefCell;
use std::cmp;
use std::cmp::Ordering;
use std::marker::PhantomData;
use std::ops::Deref;
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
            left.as_ptr() as usize == child as *const _ as usize
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
    fn rotate_left(node_rc: RcNodeRef<K, V>) -> RcNodeRef<K, V> {
        let mut node = node_rc.borrow_mut();
        let node_right = node.right.clone();
        match node_right {
            None => {
                drop(node);
                node_rc
            }
            Some(node_right_rc) => {
                Self::replace_child(&node.parent, &node, node_right_rc.clone());
                let mut node_right = node_right_rc.borrow_mut();
                node.right = node_right.left.clone();
                if let Some(node_right_left) = &node_right.left {
                    node_right_left.borrow_mut().parent = Some(Rc::downgrade(&node_rc))
                }
                node_right.left = Some(node_rc.clone());
                node.parent = Some(Rc::downgrade(&node_right_rc));
                drop(node_right);
                node_right_rc
            }
        }
    }
    fn rotate_right(node_rc: RcNodeRef<K, V>) -> RcNodeRef<K, V> {
        let mut node = node_rc.borrow_mut();
        let node_left = node.left.clone();
        match node_left {
            None => {
                drop(node);
                node_rc
            }
            Some(node_left_rc) => {
                Self::replace_child(&node.parent, &node, node_left_rc.clone());
                let mut node_left = node_left_rc.borrow_mut();
                node.left = node_left.right.clone();
                if let Some(node_left_right) = &node_left.right {
                    node_left_right.borrow_mut().parent = Some(Rc::downgrade(&node_rc))
                }
                node_left.right = Some(node_rc.clone());
                node.parent = Some(Rc::downgrade(&node_left_rc));
                drop(node_left);
                node_left_rc
            }
        }
    }
    fn fix(root: RcNodeRef<K, V>, mut cur_node_rc: RcNodeRef<K, V>) -> RcNodeRef<K, V> {
        loop {
            let cur_node = cur_node_rc.deref().borrow();
            let parent_rc = match &cur_node.parent {
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
                    drop(cur_node);
                    cur_node_rc = grand_parent_rc.clone();
                    continue;
                }
            }

            let self_is_left = Self::is_left(&cur_node_rc.deref().borrow(), &parent);
            match parent_is_left {
                true => {
                    if self_is_left {
                        grand_parent.color = Red;
                        parent.color = Black;
                        drop(grand_parent);
                        drop(parent);
                        let new_grand_parent = Self::rotate_right(grand_parent_rc);
                        return if new_grand_parent.deref().borrow().parent.is_none() {
                            new_grand_parent
                        } else {
                            root
                        };
                    } else {
                        drop(parent);
                        drop(cur_node);
                        drop(grand_parent);
                        Self::rotate_left(parent_rc.clone());
                        cur_node_rc = parent_rc;
                    }
                }
                false => {
                    if self_is_left {
                        drop(parent);
                        drop(cur_node);
                        drop(grand_parent);
                        Self::rotate_right(parent_rc.clone());
                        cur_node_rc = parent_rc;
                    } else {
                        grand_parent.color = Red;
                        parent.color = Black;
                        drop(grand_parent);
                        drop(parent);
                        let new_grand_parent = Self::rotate_left(grand_parent_rc);
                        return if new_grand_parent.deref().borrow().parent.is_none() {
                            new_grand_parent
                        } else {
                            root
                        };
                    }
                }
            }
        }

        root.borrow_mut().color = Black;
        root
    }
    pub fn add(&mut self, key: K, value: V) -> Option<V> {
        let (root, new_node) = Self::add_rec(self.root.take(), None, key, value);
        let new_node = match new_node {
            Ok(new_node) => {
                self.length += 1;
                new_node
            }
            Err(old_value) => {
                self.root = Some(root);
                return Some(old_value);
            }
        };
        self.root = Some(Self::fix(root, new_node));
        None
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
        tree.root = Some(RedBlackTree::rotate_left(tree.root.unwrap()));
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
        tree.root = Some(RedBlackTree::rotate_left(tree.root.unwrap()));
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
        let root = tree.root.clone().unwrap();
        let root_left = root.borrow_mut().left.clone();
        root.borrow_mut().left = Some(RedBlackTree::rotate_left(root_left.unwrap()));
        let root_left = root.borrow_mut().left.clone();
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
    }
}
