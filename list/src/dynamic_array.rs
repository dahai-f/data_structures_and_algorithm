use std::cmp;

type Node = Option<u64>;

const MIN_SIZE: usize = 10;

pub struct TimestampSaver {
    buf: Box<[Node]>,
    pub length: usize,
}

impl TimestampSaver {
    pub fn new_empty() -> TimestampSaver {
        TimestampSaver {
            buf: vec![None; MIN_SIZE].into_boxed_slice(),
            length: 0,
        }
    }

    pub fn cap(&self) -> usize {
        self.buf.len()
    }

    fn grow(&mut self, min_cap: usize) {
        let old_cap = self.buf.len();
        let new_cap = old_cap + (old_cap >> 1);
        let new_cap = cmp::max(min_cap, new_cap);
        let new_cap = cmp::min(new_cap, usize::MAX);
        let mut new = vec![None; new_cap].into_boxed_slice();
        new[..self.length].clone_from_slice(&self.buf[..self.length]);
        self.buf = new;
    }

    pub fn append(&mut self, value: u64) {
        if self.length == self.cap() {
            self.grow(self.length + 1);
        }
        self.buf[self.length] = Some(value);
        self.length += 1;
    }

    pub fn at(&self, index: usize) -> Node {
        if self.length > index {
            self.buf[index]
        } else {
            None
        }
    }
}

impl IntoIterator for TimestampSaver {
    type Item = u64;
    type IntoIter = ListIterator;

    fn into_iter(self) -> Self::IntoIter {
        ListIterator {
            current: 0,
            list: self,
        }
    }
}

pub struct ListIterator {
    current: usize,
    list: TimestampSaver,
}

impl Iterator for ListIterator {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current < self.list.length {
            let current = self.current;
            self.current += 1;
            self.list.buf[current]
        } else {
            None
        }
    }
}

impl DoubleEndedIterator for ListIterator {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.current < self.list.length {
            let current = self.current;
            self.current -= 1;
            self.list.buf[current]
        } else {
            None
        }
    }
}
