use std::cell::RefCell;
use std::rc::Rc;

type Link = Option<Rc<RefCell<Node>>>;

struct Node {
    value: String,
    next: Link,
}

impl Node {
    fn new(value: String) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node {
            value,
            next: None,
        }))
    }
}

pub struct TransactionLog {
    head: Link,
    tail: Link,
    length: u64,
}

impl TransactionLog {
    pub fn new_empty() -> TransactionLog {
        TransactionLog {
            head: None,
            tail: None,
            length: 0,
        }
    }

    pub fn length(&self) -> u64 {
        self.length
    }

    pub fn append(&mut self, value: String) {
        let new_node = Node::new(value);
        match self.tail.replace(new_node.clone()) {
            Some(last_tail) => {
                last_tail.borrow_mut().next = Some(new_node);
            }
            None => {
                self.head = Some(new_node);
            }
        }
        self.length += 1;
    }

    pub fn pop(&mut self) -> Option<String> {
        self.head.take().map(|head| {
            match head.borrow_mut().next.take() {
                Some(next) => {
                    self.head = Some(next);
                }
                None => {
                    self.tail = None;
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