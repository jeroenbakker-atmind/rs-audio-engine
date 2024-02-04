#[derive(Debug, Copy, Clone, PartialEq)]
pub struct InstrumentIndex {
    index: usize,
}

impl Into<InstrumentIndex> for usize {
    fn into(self) -> InstrumentIndex {
        InstrumentIndex { index: self }
    }
}
