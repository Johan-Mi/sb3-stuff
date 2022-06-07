#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Index {
    Nth(usize),
    Last,
}
