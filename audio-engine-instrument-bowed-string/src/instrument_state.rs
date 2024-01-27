use audio_engine_common::{digital_sound::sound_state::SoundState, note_time::NoteTime};

use crate::modal_processor::ModalProcessor;

#[derive(Debug, Clone)]
pub struct BowedStringInstrumentState {
    pub string_processors: Vec<ModalProcessor>,
    pub last_hand_position: f64,
    pub last_string_index: usize,
    pub last_note_time: NoteTime,
}

impl Default for BowedStringInstrumentState {
    fn default() -> Self {
        Self {
            string_processors: Vec::default(),
            last_hand_position: 1.0,
            last_string_index: 100,
            last_note_time: NoteTime::default(),
        }
    }
}
impl SoundState for BowedStringInstrumentState {}
