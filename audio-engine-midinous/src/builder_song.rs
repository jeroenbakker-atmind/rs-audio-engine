use audio_engine_sequencer::instrument::Instrument;

use crate::{builder::Builder, builder_link::LinkBuilder, link::Link, node::Node, node_index::NodeIndex, song::Song};

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
                self.link(
                    *LinkBuilder::new()
                        .from_node(last_node_index)
                        .to_node(new_node_index),
                );
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
