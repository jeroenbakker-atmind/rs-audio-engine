use audio_engine_sequencer::instrument_note_state::InstrumentNoteState;

#[derive(Clone)]
pub struct NodeState {
    instrument: InstrumentNoteState,
    pub node_time: f32,
    pub is_active: bool,
}

impl Default for NodeState {
    fn default() -> Self {
        Self {
            instrument: InstrumentNoteState::default(),
            node_time: 0.0,
            is_active: false,
        }
    }
}
