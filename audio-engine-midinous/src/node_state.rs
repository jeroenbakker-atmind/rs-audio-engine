use audio_engine_notes::Pitch;
use audio_engine_sequencer::instrument_note_state::InstrumentNoteState;

use crate::link_index::LinkIndex;

#[derive(Clone)]
pub struct NodeState {
    pub note_state: InstrumentNoteState,
    pub node_time: f32,
    pub note_pitch: Pitch,
    pub is_active: bool,
    pub outgoing_links: Vec<LinkIndex>,
    pub next_sequential_link: usize,
}

impl Default for NodeState {
    fn default() -> Self {
        Self {
            note_state: InstrumentNoteState::default(),
            node_time: 0.0,
            note_pitch: 440.0.into(),
            is_active: false,
            outgoing_links: Vec::default(),
            next_sequential_link: 0,
        }
    }
}
