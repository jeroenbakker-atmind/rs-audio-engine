use crate::node::Node;

#[derive(Debug, Default, Copy, Clone)]
pub enum LinkPath {
    #[default]
    Grid,
    Straight,
}

impl LinkPath {
    pub fn distance(&self, from_node: &Node, to_node: &Node) -> f32 {
        let dx = from_node.grid_location.0 - to_node.grid_location.0;
        let dy = from_node.grid_location.1 - to_node.grid_location.1;
        match self {
            LinkPath::Grid => dx + dy,
            LinkPath::Straight => (dx * dx + dy * dy).sqrt(),
        }
    }
}
