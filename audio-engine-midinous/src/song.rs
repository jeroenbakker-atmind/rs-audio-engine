use audio_engine_common::beats_per_minute::BeatsPerMinute;
use audio_engine_sequencer::{instrument::Instrument, instrument_index::InstrumentIndex};

use crate::{link::Link, link_index::LinkIndex, node::Node, node_index::NodeIndex};

#[derive(Default, Debug, Clone)]
pub struct Song {
    pub nodes: Vec<Node>,
    pub links: Vec<Link>,
    pub start_nodes: Vec<NodeIndex>,
    pub instruments: Vec<Instrument>,
    pub beats_per_minute: BeatsPerMinute,
}

impl Song {
    pub fn node(&self, index: NodeIndex) -> &Node {
        &self.nodes[index.as_usize()]
    }

    pub fn link(&self, index: LinkIndex) -> &Link {
        &self.links[index.as_usize()]
    }

    pub fn instrument(&self, index: InstrumentIndex) -> &Instrument {
        &self.instruments[index.as_usize()]
    }
}
