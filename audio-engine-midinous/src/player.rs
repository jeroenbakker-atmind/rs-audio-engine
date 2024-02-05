use crate::{
    link_index::LinkIndex, link_path::LinkPath, link_selection::LinkSelection,
    link_state::LinkState, node::Node, node_state::NodeState, song::Song, traveler::Traveler,
};

pub struct Player {
    song: Song,
    node_states: Vec<NodeState>,
    link_states: Vec<LinkState>,
    travelers: Vec<Traveler>,
    sample_rate: f32,
}

impl Player {
    pub fn new(song: Song, sample_rate: f32) -> Player {
        let node_states = Player::create_node_states(&song);
        let link_states = Player::create_link_states(&song);
        Player {
            song,
            node_states,
            link_states,
            travelers: Vec::with_capacity(32),
            sample_rate,
        }
    }

    fn create_node_states(song: &Song) -> Vec<NodeState> {
        let mut result = Vec::new();
        result.resize(song.nodes.len(), NodeState::default());
        for (link_index, link) in song.links.iter().enumerate() {
            result[link.from_node.as_usize()]
                .outgoing_links
                .push(link_index.into());
        }
        result
    }

    fn create_link_states(song: &Song) -> Vec<LinkState> {
        let mut result = Vec::new();
        for link in &song.links {
            let from_node = song.node(link.from_node);
            let to_node = song.node(link.to_node);
            let length = match link.path {
                LinkPath::Grid => {
                    let dx = from_node.grid_location.0 - to_node.grid_location.0;
                    let dy = from_node.grid_location.1 - to_node.grid_location.1;
                    dx.abs() + dy.abs()
                }
                LinkPath::Straight => {
                    let dx = from_node.grid_location.0 - to_node.grid_location.0;
                    let dy = from_node.grid_location.1 - to_node.grid_location.1;
                    (dx * dx + dy * dy).sqrt()
                }
            };
            result.push(LinkState { length: length })
        }
        result
    }
}

impl Player {
    pub fn sample(&mut self, buffer: &mut [f32]) {
        let mut first_sample = self.travelers.is_empty();
        for sample_index in 0..buffer.len() {
            if first_sample {
                self.first_sample();
                first_sample = false;
            } else {
                self.move_travelers();
            }
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
            // Check for cascading travelers (link weight*distance == 0.0)
        }
    }

    fn move_travelers(&mut self) {
        // for traveler.
        // move traveler among links.
        // trigger nodes (and remove traveler)
        // add new traveler to player.
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
    node_state.node_time = 0.0;
    node_state.is_active = true;
    // create new travelers
    match node.link_selection {
        LinkSelection::Sequential => {
            todo!()
        }
        LinkSelection::All => {
            for link in &node_state.outgoing_links {
                // check weight * distance and put it in cascade or new_travelers.
                new_travelers.push(Traveler {
                    link: *link,
                    distance_traveled: 0.0,
                })
            }
        }
        LinkSelection::Random => {
            todo!()
        }
    }
}

fn read_node_sample(node: &Node, node_state: &mut NodeState) -> f32 {
    // check if node_state is playing if not return 0.0
    // check if playing is finished if so finish and return 0.0
    // if is update the note state
    // read the sample.
    0.0
}
