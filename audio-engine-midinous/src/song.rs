use audio_engine_common::{beats_per_minute::BeatsPerMinute, note_time::NoteTime};
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
    pub fn node<N>(&self, index: N) -> &Node
    where
        N: Into<NodeIndex> + Sized,
    {
        &self.nodes[index.into().as_usize()]
    }

    pub fn link<L>(&self, index: L) -> &Link
    where
        L: Into<LinkIndex> + Sized,
    {
        &self.links[index.into().as_usize()]
    }

    pub fn instrument<I>(&self, index: I) -> &Instrument
    where
        I: Into<InstrumentIndex>,
    {
        &self.instruments[index.into().as_usize()]
    }
}

impl Song {
    pub fn node_duration(&self, node: &Node) -> NoteTime {
        // TODO: add measure to song.
        10.0
    }
}
