use audio_engine_common::{
    digital_sound::{parameters::NoteParameters, sound::Sound},
    song_time::SongTime,
};

use crate::{
    link_path::LinkPath, link_selection::LinkSelection, link_state::LinkState,
    node_index::NodeIndex, node_state::NodeState, song::Song, traveler::Traveler,
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
                self.update_travelers();
            }
            buffer[sample_index] = self.read_sample();
        }
    }

    fn first_sample(&mut self) {
        let trigger = Traveler::default();
        let mut new_travelers = Vec::new();
        for node_index in self.song.start_nodes.clone() {
            self.trigger_node(node_index, &trigger, &mut new_travelers);
        }
        self.travelers.append(&mut new_travelers);
    }

    fn update_travelers(&mut self) {
        self.move_travelers();
        let triggers = self.extract_triggers();
        self.trigger_nodes(&triggers);
    }
    fn move_travelers(&mut self) {
        let beats_per_second = self.song.beats_per_minute / 60.0;
        let beats_per_sample = beats_per_second / self.sample_rate;
        self.travelers
            .iter_mut()
            .for_each(|traveler| traveler.distance_traveled += beats_per_sample);
    }
    fn extract_triggers(&mut self) -> Vec<Traveler> {
        let triggering_indices = self
            .travelers
            .iter()
            .enumerate()
            .filter(|(_index, traveler)| {
                let link_distance = self.song.link(traveler.link).weight
                    * self.link_states[traveler.link.as_usize()].length;
                traveler.distance_traveled > link_distance
            })
            .map(|(index, _traveler)| index)
            .collect::<Vec<usize>>();
        let triggers = triggering_indices
            .iter()
            .map(|index| self.travelers[*index])
            .collect::<Vec<Traveler>>();
        for index in (0..triggering_indices.len()).rev() {
            self.travelers.remove(triggering_indices[index]);
        }
        triggers
    }

    fn trigger_nodes(&mut self, triggers: &[Traveler]) {
        let mut new_travelers = Vec::new();
        for trigger in triggers {
            let node_index = self.song.link(trigger.link).to_node;
            self.trigger_node(node_index, trigger, &mut new_travelers);
        }
        self.travelers.append(&mut new_travelers);
    }

    fn trigger_node(
        &mut self,
        node_index: NodeIndex,
        trigger: &Traveler,
        new_travelers: &mut Vec<Traveler>,
    ) {
        let node_state = &mut self.node_states[node_index.as_usize()];
        let node = self.song.node(node_index);
        // init node for playing
        node_state.node_time = 0.0;
        node_state.is_active = true;
        let instrument = self.song.instrument(node.instrument);
        node_state.note_state = instrument.init_sound_state();
        node_state.note_pitch = node.note_pitch;
        // TODO copy carrier from trigger.
        let mut cascade_travelers = Vec::new();
        // create new travelers
        let outgoing_links = match node.link_selection {
            LinkSelection::Sequential => {
                let next_link = node_state.next_sequential_link;
                node_state.next_sequential_link += 1;
                node_state.next_sequential_link %= node_state.outgoing_links.len();
                let link_index = node_state.outgoing_links[next_link];
                vec![link_index]
            }
            LinkSelection::All => node_state.outgoing_links.clone(),
            LinkSelection::Random => {
                todo!()
            }
        };

        for link_index in outgoing_links {
            let link_weight =
                self.link_states[link_index.as_usize()].length * self.song.link(link_index).weight;
            let traveler = Traveler {
                link: link_index,
                distance_traveled: 0.0,
            };
            if link_weight == 0.0 {
                cascade_travelers.push(traveler);
            } else {
                new_travelers.push(traveler);
            }
        }

        for trigger in cascade_travelers {
            let link = self.song.link(trigger.link);
            self.trigger_node(link.to_node, &trigger, new_travelers)
        }
    }

    fn read_sample(&mut self) -> f32 {
        let mut result = 0.0;
        // TODO: split in two loops?
        for (node, node_state) in self.song.nodes.iter().zip(&mut self.node_states) {
            node_state.node_time += 1.0 / self.sample_rate;
            // TODO: check if node is finished.

            let instrument = self.song.instrument(node.instrument);
            let parameters = NoteParameters {
                note_time: node_state.node_time,
                note_off: None,
                note_pitch: node_state.note_pitch.frequency as f32,
                gain: node.note_level,
                sample_rate: self.sample_rate,
            };
            let sample = instrument.sample(&parameters, &mut node_state.note_state);
            result += sample;
        }
        result
    }
}

impl Player {
    pub fn render(&mut self, time: SongTime) -> Vec<f32> {
        let num_samples = (time * self.sample_rate) as usize;
        let mut buffer = vec![0.0; num_samples];
        self.sample(&mut buffer);
        buffer
    }
}
