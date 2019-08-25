use std::cell::RefCell;
use std::fmt::{Error, Formatter};
use std::rc::Rc;

type Link = Option<Rc<RefCell<Node>>>;

pub struct Node {
    next: Vec<Link>,
    offset: u64,
    command: String,
}

impl Node {
    pub fn new(num_next: usize, offset: u64, command: String) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node {
            next: vec![None; num_next],
            offset,
            command,
        }))
    }
}

pub struct BestTransactionLog {
    head: Link,
    tails: Vec<Link>,
    max_level: usize,
    length: usize,
}

impl BestTransactionLog {
    pub fn new_empty(max_level: usize) -> BestTransactionLog {
        BestTransactionLog {
            head: None,
            tails: vec![None; max_level + 1],
            max_level,
            length: 0,
        }
    }

    pub fn length(&self) -> usize {
        self.length
    }

    fn random_level(&self) -> usize {
        let mut l = 0;
        while l < self.max_level && rand::random::<bool>() {
            l += 1;
        }
        l
    }

    pub fn append(&mut self, offset: u64, command: String) {
        let level = 1 + if self.head.is_none() {
            self.max_level
        } else {
            self.random_level()
        };

        let new = Node::new(level, offset, command);

        for i in 0..level {
            if let Some(tail) = self.tails[i].take() {
                tail.borrow_mut().next[i] = Some(new.clone());
            }
            self.tails[i] = Some(new.clone());
        }

        if self.head.is_none() {
            self.head = Some(new);
        }

        self.length += 1;
    }

    pub fn find(&self, offset: u64) -> Option<String> {
        match self.head.as_ref() {
            Some(head) => {
                let node = head.borrow();
                if node.offset == offset {
                    return Some(node.command.clone());
                }

                let mut max_level = self.max_level;
                loop {
                    if node.next[max_level].is_some() {
                        break;
                    }
                    max_level -= 1;
                    if max_level >= node.next.len() {
                        return None;
                    }
                }

                let mut node = head.clone();
                for cur_level in (0..=max_level).rev() {
                    loop {
                        let next = node.borrow().next[cur_level].clone();
                        match next {
                            Some(next) => {
                                if next.borrow().offset <= offset {
                                    node = next;
                                } else {
                                    break;
                                }
                            }
                            _ => break
                        };
                    }

                    let node = node.borrow();
                    if node.offset == offset {
                        return Some(node.command.clone());
                    }
                }

                None
            }
            None => None
        }
    }

    pub fn iter(&self, level: usize) -> ListIterator {
        ListIterator::new(self.head.clone(), level)
    }
}

pub struct ListIterator {
    current: Link,
    level: usize,
}

impl ListIterator {
    pub fn new(start_at: Link, level: usize) -> ListIterator {
        ListIterator {
            current: start_at,
            level,
        }
    }
}

impl Iterator for ListIterator {
    type Item = (u64, String);

    fn next(&mut self) -> Option<Self::Item> {
        self.current.take().map(|current| {
            let current = current.borrow();
            self.current = current.next[self.level].clone();
            (current.offset, current.command.clone())
        })
    }
}

impl IntoIterator for BestTransactionLog {
    type Item = (u64, String);
    type IntoIter = ListIterator;

    fn into_iter(self) -> Self::IntoIter {
        ListIterator::new(self.head, 0)
    }
}

impl std::fmt::Debug for BestTransactionLog {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self.head.as_ref() {
            None => { write!(f, "The list is empty: []")?; }
            Some(_) => {
                for level in (0..=self.max_level).rev() {
                    write!(f, "{}: ", level)?;
                    for log in self.iter(level) {
                        write!(f, "[{}] ", log.0)?;
                    }
                    writeln!(f, "")?;
                }
            }
        };
        Ok(())
    }
}
