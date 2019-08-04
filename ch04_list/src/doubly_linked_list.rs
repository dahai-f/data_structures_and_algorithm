use std::borrow::Borrow;
use std::cell::RefCell;
use std::ops::Deref;
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
        match self.tail.replace(node.clone()) {
            None => {
                self.head = Some(node);
            }
            Some(old) => {
                old.borrow_mut().next = Some(node.clone());
                node.borrow_mut().prev = Some(old);
            }
        };
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
                .map_or_else(
                    |head| RefCell::borrow(&head).deref().value.clone(),
                    |head| head.into_inner().value)
        })
    }

    pub fn iter(&self) -> ListIterator {
        ListIterator::new(self.head.as_ref().map(|head| head.borrow()))
    }

    pub fn back_iter(&self) -> ListIterator {
        ListIterator::new(self.tail.as_ref().map(|tail| tail.borrow()))
    }
}

type IterLink<'a> = Option<&'a RefCell<Node>>;

pub struct ListIterator<'a> {
    current: IterLink<'a>
}

impl<'a> ListIterator<'a> {
    fn new(start_at: IterLink<'a>) -> ListIterator<'a> {
        ListIterator {
            current: start_at
        }
    }
}

impl<'a> Iterator for ListIterator<'a> {
    type Item = &'a String;

    fn next(&mut self) -> Option<Self::Item> {
        match self.current.take() {
            None => None,
            Some(current) => unsafe {
                self.current = match (*current.as_ptr()).next {
                    None => None,
                    Some(ref next) => Some(Rc::borrow(&next)),
                };
                Some(&(*current.as_ptr()).value)
            }
        }
    }
}

impl<'a> DoubleEndedIterator for ListIterator<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.current.take().map(|current| unsafe {
            let current = &*current.as_ptr();
            self.current = current.prev.as_ref().map(|prev| Rc::borrow(&prev));
            &current.value
        })
    }
}
