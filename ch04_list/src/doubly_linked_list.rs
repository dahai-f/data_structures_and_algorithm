use std::cell::RefCell;
use std::io::IntoInnerError;
use std::ops::Deref;
use std::panic::resume_unwind;
use std::rc::Rc;

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

    pub fn append(&mut self, value: String) {
        let node = Node::new(value);
        match self.tail.take() {
            None => {
                self.head = Some(node.clone());
            }
            Some(old) => {
                old.borrow_mut().next = Some(node.clone());
                node.borrow_mut().prev = Some(old);
            }
        };
        self.tail = Some(node);
        self.length += 1;
    }

    pub fn pop(&mut self) -> Option<String> {
        self.head.take().map(|head| {
            match head.borrow_mut().next.take() {
                None => {
                    self.tail = None;
                }
                Some(next) => {
                    next.borrow_mut().prev = None;
                    self.head = Some(next);
                }
            }
            self.length -= 1;
            Rc::try_unwrap(head)
                .ok()
                .expect("Something is terribly wrong")
                .into_inner()
                .value
        })
    }
}

impl IntoIterator for BetterTransactionLog {
    type Item = String;
    type IntoIter = ListIterator;

    fn into_iter(self) -> Self::IntoIter {
        ListIterator::new(self.head)
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

impl DoubleEndedIterator for ListIterator {
    fn next_back(&mut self) -> Option<Self::Item> {
        let result: Option<Self::Item>;
        self.current = match &self.current {
            Some(current) => {
                let current = current.borrow();
                result = Some((*current).value.clone());
                current.prev.clone()
            }
            None => {
                result = None;
                None
            }
        };
        result
    }
}
