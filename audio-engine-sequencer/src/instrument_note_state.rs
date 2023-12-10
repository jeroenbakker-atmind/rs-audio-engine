use audio_engine_fm::instrument::FMInstrumentNoteState;
use audio_engine_sample::sample_note_state::SampleNoteState;

use crate::instrument::Instrument;

#[derive(Default, Copy, Clone)]
pub enum InstrumentNoteState {
    #[default]
    None,
    FM(FMInstrumentNoteState),
    Sample(SampleNoteState),
}

impl InstrumentNoteState {
    pub fn reset(&mut self, instrument: Option<&Instrument>) {
        match instrument {
            None | Some(Instrument::None) => *self = Self::None,
            Some(Instrument::FM(_instrument)) => *self = Self::FM(FMInstrumentNoteState::default()),
            Some(Instrument::Sample(_instrument)) => {
                *self = Self::Sample(SampleNoteState::default())
            }
        }
    }
}
