use std::cell::RefCell;
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

struct BestTransactionLog {
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
                    if max_level < 0 {
                        return None;
                    }
                }

                let mut node = head.clone();
                for cur_level in (0..=max_level).rev() {
                    loop {
                        let next = node.borrow().next[cur_level].clone();
                        match next {
                            Some(next) => {
                                if next.borrow().offset <= offset
                                {
                                    break;
                                }
                                node = next;
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
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        unimplemented!()
    }
}

impl IntoIterator for BestTransactionLog {
    type Item = String;
    type IntoIter = ListIterator;

    fn into_iter(self) -> Self::IntoIter {
        ListIterator::new(self.head, 0)
    }
}
