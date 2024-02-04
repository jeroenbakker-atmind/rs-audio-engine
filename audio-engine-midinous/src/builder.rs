use audio_engine_common::{level::Level, note_duration::NoteDuration};
use audio_engine_notes::Pitch;
use audio_engine_sequencer::{instrument::Instrument, instrument_index::InstrumentIndex};

use crate::{
    link::Link, link_selection::LinkSelection, node::Node, node_index::NodeIndex, song::Song,
};

pub trait Builder {
    type Inner;
    fn new() -> Self;
    fn build(&self) -> Self::Inner;
}

#[derive(Default)]
pub struct SongBuilder {
    song: Song,
    last_added_node_index: Option<NodeIndex>,
}

impl Builder for SongBuilder {
    type Inner = Song;
    fn new() -> Self {
        Self::default()
    }

    fn build(&self) -> Song {
        self.song.clone()
    }
}

impl SongBuilder {
    pub fn instrument<I>(&mut self, instrument: I) -> &mut Self
    where
        I: Into<Instrument>,
    {
        self.song.instruments.push(instrument.into());
        self
    }
    /*
    pub fn instruments<I>(&mut self, instruments: &[I]) -> &mut Self
    where
    I: Into<Instrument> + Sized,
    {
        for instrument in instruments {
            self.instrument(instrument);
        }
        self
    }
    */
}

impl SongBuilder {
    /// Add a new node to the song. The new node will be an entry point for the song.
    pub fn entry_point<N>(&mut self, node: N) -> &mut Self
    where
        N: Into<Node>,
    {
        let index = self.song.nodes.len().into();
        self.song.start_nodes.push(index);
        self.node(node);
        self
    }

    /// Add a new node to the song.
    pub fn node<N>(&mut self, node: N) -> &mut Self
    where
        N: Into<Node>,
    {
        let new_node_index = self.next_node_index();
        self.song.nodes.push(node.into());
        self.last_added_node_index = Some(new_node_index);
        self
    }

    /// Add a new node to the song. Add a link to the last added node.
    pub fn connect_to_last<N>(&mut self, node: N) -> &mut Self
    where
        N: Into<Node>,
    {
        let new_node_index = self.next_node_index();
        match self.last_added_node_index.clone() {
            None => {}
            Some(last_node_index) => {
                self.link(LinkBuilder::between(last_node_index, new_node_index).build());
            }
        }
        self.node(node);
        self
    }

    fn next_node_index(&self) -> NodeIndex {
        self.song.nodes.len().into()
    }
}

impl SongBuilder {
    pub fn link<L>(&mut self, link: L) -> &mut Self
    where
        L: Into<Link>,
    {
        self.song.links.push(link.into());
        self
    }
}

pub struct NodeBuilder {
    node: Node,
}
impl NodeBuilder {
    pub fn new() -> NodeBuilder {
        NodeBuilder {
            node: Node::default(),
        }
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

pub struct LinkBuilder {
    link: Link,
}

impl Builder for LinkBuilder {
    type Inner = Link;
    fn new() -> Self {
        LinkBuilder {
            link: Link {
                from_node: 0.into(),
                to_node: 0.into(),
                weight: 1.0,
            },
        }
    }

    fn build(&self) -> Self::Inner {
        self.link
    }
}

impl LinkBuilder {
    pub fn between<From, To>(from: From, to: To) -> LinkBuilder
    where
        From: Into<NodeIndex> + Sized,
        To: Into<NodeIndex> + Sized,
    {
        LinkBuilder {
            link: Link {
                from_node: from.into(),
                to_node: to.into(),
                weight: 1.0,
            },
        }
    }

    pub fn from_node<N>(&mut self, node_index: N) -> &mut Self
    where
        N: Into<NodeIndex>,
    {
        self.link.from_node = node_index.into();
        self
    }
    pub fn to_node<N>(&mut self, node_index: N) -> &mut Self
    where
        N: Into<NodeIndex>,
    {
        self.link.to_node = node_index.into();
        self
    }

    pub fn build(self) -> Link {
        self.link
    }
}

impl Into<Link> for LinkBuilder {
    fn into(self) -> Link {
        self.build()
    }
}

#[cfg(test)]
mod test {

    use super::{Builder, LinkBuilder, NodeBuilder, SongBuilder};

    #[test]
    fn empty_song() {
        let song = SongBuilder::new().build();
    }

    #[test]
    fn two_nodes() {
        let mut builder = SongBuilder::new();
        let song = builder
            .entry_point(NodeBuilder::new())
            .connect_to_last(NodeBuilder::new())
            .build();
    }

    #[test]
    fn loop_four_nodes() {
        let mut builder = SongBuilder::new();
        let song = builder
            .entry_point(NodeBuilder::new())
            .connect_to_last(NodeBuilder::new())
            .connect_to_last(NodeBuilder::new())
            .connect_to_last(NodeBuilder::new())
            .link(LinkBuilder::between(3, 0).build())
            .build();
    }
}
