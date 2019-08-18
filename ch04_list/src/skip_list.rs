use std::cell::RefCell;
use std::rc::Rc;

type Link = Option<Rc<RefCell<Node>>>;

struct Node {
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
}