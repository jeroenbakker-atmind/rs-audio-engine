#[derive(Debug, Copy, Clone, PartialEq)]
pub struct NodeIndex {
    index: usize,
}
impl NodeIndex {
    pub fn as_usize(&self)->usize {
self.index
    }
}

impl Into<NodeIndex> for usize {
    fn into(self) -> NodeIndex {
        NodeIndex { index: self }
    }
}

