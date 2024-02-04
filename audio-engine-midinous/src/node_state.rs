use audio_engine_sequencer::instrument_note_state::InstrumentNoteState;

#[derive(Clone)]
pub struct NodeState {
    instrument: InstrumentNoteState,
}

impl Default for NodeState {
    fn default() -> Self {
        Self {
            instrument: InstrumentNoteState::default(),
        }
    }
}
