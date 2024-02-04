use crate::{
    link_index::LinkIndex, node::Node, node_state::NodeState, song::Song, traveler::Traveler,
};

pub struct Player {
    song: Song,
    outgoing_link_lookup: Vec<Vec<LinkIndex>>,
    node_states: Vec<NodeState>,
    travelers: Vec<Traveler>,
}

impl Player {
    pub fn new(song: Song) -> Player {
        let num_nodes = song.nodes.len();
        let outgoing_link_lookup = Player::create_link_lookup(&song);
        Player {
            song,
            outgoing_link_lookup,
            node_states: vec![NodeState::default(); num_nodes],
            travelers: Vec::with_capacity(32),
        }
    }

    fn create_link_lookup(song: &Song) -> Vec<Vec<LinkIndex>> {
        let mut result = Vec::new();
        result.resize(song.nodes.len(), Vec::new());
        for (link_index, link) in song.links.iter().enumerate() {
            result[link.from_node.as_usize()].push(link_index.into());
        }
        result
    }
}

impl Player {
    pub fn sample(&mut self, buffer: &mut [f32]) {
        if self.travelers.is_empty() {
            self.first_sample();
        }
        for sample_index in 0..buffer.len() {
            buffer[sample_index] = self.read_sample();
        }
    }

    fn first_sample(&mut self) {
        let trigger = Traveler::default();
        let mut new_travelers = Vec::new();
        for entry_point in &self.song.start_nodes {
            self.travelers.push(Traveler::default());
            trigger_node(
                &self.song.nodes[entry_point.as_usize()],
                &mut self.node_states[entry_point.as_usize()],
                &trigger,
                &mut new_travelers,
            );
        }
    }

    fn read_sample(&mut self) -> f32 {
        let mut result = 0.0;
        for (node, node_state) in self.song.nodes.iter().zip(&mut self.node_states) {
            result += read_node_sample(node, node_state);
        }
        result
    }
}

fn trigger_node(
    node: &Node,
    node_state: &mut NodeState,
    trigger: &Traveler,
    new_travelers: &mut Vec<Traveler>,
) {
    // init node for playing
    // create new travelers
}

fn read_node_sample(node: &Node, node_state: &mut NodeState) -> f32 {
    // check if node_state is playing if not return 0.0
    // if is update the note state
    // read the sample.
    0.0
}
