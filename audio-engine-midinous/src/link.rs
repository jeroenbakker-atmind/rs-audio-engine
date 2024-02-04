use crate::node_index::NodeIndex;

#[derive(Debug, Copy, Clone)]
pub struct Link {
    pub from_node: NodeIndex,
    pub to_node: NodeIndex,
    pub weight: f64,
}
