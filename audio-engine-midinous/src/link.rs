use crate::{link_path::LinkPath, node_index::NodeIndex};

#[derive(Debug, Copy, Clone)]
pub struct Link {
    pub from_node: NodeIndex,
    pub to_node: NodeIndex,
    pub weight: f32,
    pub path: LinkPath,
}
