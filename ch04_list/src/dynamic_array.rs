type Node = u64;

pub struct TimestampSaver {
    buf: Option<[Node]>,
    cap: usize,
    pub length: usize,
}
