use audio_engine_common::digital_sound::{parameters::NoteParameters, sound::Sound};

use crate::instrument_state::BowedStringInstrumentState;


#[derive(Debug, Default, Copy, Clone)]
pub struct BowedStringInstrument {}

impl Sound for BowedStringInstrument {
    type SoundState = BowedStringInstrumentState;
    type Parameters = NoteParameters;

    fn init_sound_state(&self) -> Self::SoundState {
        BowedStringInstrumentState::default()
    }

    fn sample(&self, parameters: &Self::Parameters, state: &mut BowedStringInstrumentState) -> f32 {
        0.0
    }
}
