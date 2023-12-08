use audio_engine_fm::instrument::FMInstrumentNoteState;
use audio_engine_sample::sample_note_state::SampleNoteState;

#[derive(Default, Copy, Clone)]
pub struct InstrumentNoteState {
    pub fm_note_state: FMInstrumentNoteState,
    pub sample_note_state: SampleNoteState,
}

impl InstrumentNoteState {
    pub fn reset(&mut self) {
        self.fm_note_state.reset();
        self.sample_note_state.reset();
    }
}
