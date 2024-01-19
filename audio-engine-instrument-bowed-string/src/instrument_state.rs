use audio_engine_common::digital_sound::sound_state::SoundState;

use crate::sherman_morrison_processor::ShermanMorrison;

#[derive(Debug, Default, Clone)]
pub struct BowedStringInstrumentState {
    pub string_processors: Vec<ShermanMorrison>,

}

impl SoundState for BowedStringInstrumentState {}
