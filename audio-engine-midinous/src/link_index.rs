#[derive(Debug, Copy, Clone, PartialEq)]
pub struct LinkIndex {
    index: usize,
}

impl Into<LinkIndex> for usize {
    fn into(self) -> LinkIndex {
        LinkIndex { index: self }
    }
}
