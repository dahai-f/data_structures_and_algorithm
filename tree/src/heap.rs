use std::cmp::Ordering;

pub struct Heap<T: Ord> {
    heap: Vec<Box<T>>,
}

impl<T: Ord> Default for Heap<T> {
    fn default() -> Self {
        Self { heap: vec![] }
    }
}

impl<T: Ord> Heap<T> {
    pub fn push(&mut self, val: T) {
        let cur_index = self.heap.len();
        self.heap.push(Box::new(val));
        self.fix_up(cur_index);
    }

    pub fn pop(&mut self) -> Option<T> {
        let last_index = match self.heap.len().checked_sub(1) {
            None => {
                return None;
            }
            Some(last_index) => last_index,
        };
        let mut cur_index = 0;
        while cur_index < last_index {
            let left_index = (cur_index + 1) * 2 - 1;
            let (swap_index, need_fix_up) = match left_index.cmp(&last_index) {
                Ordering::Less => {
                    let right_index = left_index + 1;
                    if self.heap[left_index] < self.heap[right_index] {
                        (left_index, false)
                    } else {
                        (right_index, false)
                    }
                }
                Ordering::Equal => (last_index, false),
                Ordering::Greater => (last_index, true),
            };
            self.heap.swap(cur_index, swap_index);
            if need_fix_up {
                self.fix_up(cur_index);
            }
            cur_index = swap_index;
        }
        self.heap.pop().map(|result| *result)
    }
    fn fix_up(&mut self, mut index: usize) {
        while index > 0 {
            let parent_index = (index + 1) / 2 - 1;
            if self.heap[parent_index] <= self.heap[index] {
                break;
            }

            self.heap.swap(parent_index, index);
            index = parent_index;
        }
    }

    pub fn drain(&mut self) -> DrainIter<T> {
        DrainIter { heap: self }
    }
}

pub struct DrainIter<'h, T: Ord> {
    heap: &'h mut Heap<T>,
}

impl<'h, T: Ord> Iterator for DrainIter<'h, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.heap.pop()
    }
}

#[cfg(test)]
mod tests {
    use rand::prelude::SliceRandom;

    use crate::heap::Heap;

    #[test]
    fn it_works() {
        let mut rng = rand::thread_rng();
        let mut values: Vec<i32> = (0..100).collect();
        values.shuffle(&mut rng);

        let mut heap = Heap::default();
        values.into_iter().for_each(|value| heap.push(value));

        assert_eq!(
            heap.drain().collect::<Vec<i32>>(),
            (0..100).collect::<Vec<i32>>()
        )
    }
}
