#[derive(Debug, Copy, Clone, PartialEq)]
pub struct NodeIndex {
    index: usize,
}

impl Into<NodeIndex> for usize {
    fn into(self) -> NodeIndex {
        NodeIndex { index: self }
    }
}

