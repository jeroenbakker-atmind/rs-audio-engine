#[derive(Debug, Copy, Clone, PartialEq)]
pub struct LinkIndex {
    index: usize,
}

impl LinkIndex {
    pub fn as_usize(&self) -> usize {
        self.index
    }
}

impl Into<LinkIndex> for usize {
    fn into(self) -> LinkIndex {
        LinkIndex { index: self }
    }
}
