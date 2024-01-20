use audio_engine_common::digital_sound::sound_state::SoundState;

use crate::sherman_morrison_processor::ShermanMorrison;

#[derive(Debug, Clone)]
pub struct BowedStringInstrumentState {
    pub string_processors: Vec<ShermanMorrison>,
    pub last_hand_position: f32,
    pub last_string_index: usize,
}
impl Default for BowedStringInstrumentState {
    fn default() -> Self {
        Self {
            string_processors: Vec::default(),
            last_hand_position: 1.0,
            last_string_index: 100,
        }
    }
}
impl SoundState for BowedStringInstrumentState {}
