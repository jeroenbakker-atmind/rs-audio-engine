use audio_engine_sequencer::instrument::Instrument;

use crate::{link::Link, node::Node, node_index::NodeIndex};

#[derive(Default, Debug, Clone)]
pub struct Song {
    pub nodes: Vec<Node>,
    pub links: Vec<Link>,
    pub start_nodes: Vec<NodeIndex>,
    pub instruments: Vec<Instrument>,
}
