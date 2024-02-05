use audio_engine_common::{level::Level, note_duration::NoteDuration};
use audio_engine_notes::Pitch;
use audio_engine_sequencer::instrument_index::InstrumentIndex;

use crate::{builder::Builder, link_selection::LinkSelection, node::Node};

#[derive(Copy, Clone)]
pub struct NodeBuilder {
    node: Node,
}
impl NodeBuilder {
    pub fn new() -> NodeBuilder {
        NodeBuilder {
            node: Node::default(),
        }
    }

    pub fn location(&mut self, x: f32, y: f32) -> &mut Self {
        self.node.grid_location = (x, y);
        self
    }

    pub fn instrument<I>(&mut self, instrument_index: I) -> &mut Self
    where
        I: Into<InstrumentIndex> + Sized,
    {
        self.node.instrument = instrument_index.into();
        self
    }

    pub fn note_level(&mut self, note_level: Level) -> &mut Self {
        self.node.note_level = note_level;
        self
    }

    pub fn note_pitch<P>(&mut self, note_pitch: P) -> &mut Self
    where
        P: Into<Pitch>,
    {
        self.node.note_pitch = note_pitch.into();
        self
    }

    pub fn repeat(&mut self, repeat: usize) -> &mut Self {
        self.node.repeat = repeat;
        self
    }

    pub fn repeat_delay<D>(&mut self, repeat_delay: D) -> &mut Self
    where
        D: Into<NoteDuration>,
    {
        self.node.repeat_delay = repeat_delay.into();
        self
    }

    pub fn link_selection<S>(&mut self, link_selection: S) -> &mut Self
    where
        S: Into<LinkSelection>,
    {
        self.node.link_selection = link_selection.into();
        self
    }

    pub fn build(self) -> Node {
        self.node
    }
}

impl Into<Node> for NodeBuilder {
    fn into(self) -> Node {
        self.build()
    }
}

impl Builder for NodeBuilder {
    type Inner = Node;
    fn new() -> Self {
        NodeBuilder {
            node: Node::default(),
        }
    }

    fn build(&self) -> Self::Inner {
        self.node.clone()
    }
}
