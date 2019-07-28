use std::cell::RefCell;
use std::rc::Rc;
use std::io::IntoInnerError;
use std::ops::Deref;

type Link = Option<Rc<RefCell<Node>>>;

struct Node {
    value: String,
    next: Link,
    prev: Link,
}

impl Node {
    pub fn new(value: String) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node {
            value,
            next: None,
            prev: None,
        }))
    }
}

pub struct BetterTransactionLog {
    head: Link,
    tail: Link,
    length: usize,
}

impl BetterTransactionLog {
    pub fn new_empty() -> BetterTransactionLog {
        BetterTransactionLog {
            head: None,
            tail: None,
            length: 0,
        }
    }

    pub fn length(&self) -> usize {
        self.length
    }
}

impl IntoIterator for BetterTransactionLog {
    type Item = String;
    type IntoIter = ListIterator;

    fn into_iter(self) -> Self::IntoIter {
        unimplemented!()
    }
}

pub struct ListIterator {
    current: Link
}

impl ListIterator {
    fn new(start_at: Link) -> ListIterator {
        ListIterator {
            current: start_at
        }
    }
}

impl Iterator for ListIterator {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let result: Option<Self::Item>;
        self.current = match &self.current {
            Some(current) => {
                let current = current.borrow();
                result = Some((*current).value.clone());
                current.next.clone()
            }
            None => {
                result = None;
                None
            }
        };
        result
    }
}

