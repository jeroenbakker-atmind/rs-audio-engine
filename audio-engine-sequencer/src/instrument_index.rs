#[derive(Debug, Copy, Clone, PartialEq)]
pub struct InstrumentIndex {
    index: usize,
}

impl InstrumentIndex {
    pub fn as_usize(&self) -> usize {
        self.index
    }
}

impl Into<InstrumentIndex> for usize {
    fn into(self) -> InstrumentIndex {
        InstrumentIndex { index: self }
    }
}
