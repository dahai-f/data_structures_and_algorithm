pub struct DynamicArray {
    buf: Box<Option<u64>>,
    cap: usize,
    pub length: usize,
}

