use crate::{
    link_index::LinkIndex, link_state::LinkState, node_index::NodeIndex, node_state::NodeState,
    traveler::Traveler,
};

pub struct SongState {
    pub node_states: Vec<NodeState>,
    pub link_states: Vec<LinkState>,
    pub travelers: Vec<Traveler>,
}

impl SongState {
    pub fn node<N>(&self, index: N) -> &NodeState
    where
        N: Into<NodeIndex> + Sized,
    {
        &self.node_states[index.into().as_usize()]
    }

    pub fn link<L>(&self, index: L) -> &LinkState
    where
        L: Into<LinkIndex> + Sized,
    {
        &self.link_states[index.into().as_usize()]
    }

    pub fn node_mut<N>(&mut self, index: N) -> &mut NodeState
    where
        N: Into<NodeIndex> + Sized,
    {
        &mut self.node_states[index.into().as_usize()]
    }

    pub fn link_mut<L>(&mut self, index: L) -> &mut LinkState
    where
        L: Into<LinkIndex> + Sized,
    {
        &mut self.link_states[index.into().as_usize()]
    }
}
