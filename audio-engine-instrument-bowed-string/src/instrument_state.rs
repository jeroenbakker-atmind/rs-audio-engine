use audio_engine_common::{digital_sound::sound_state::SoundState, note_time::NoteTime};

use crate::processor::StringProcessor;

#[derive(Debug, Clone)]
pub struct BowedStringInstrumentState<P>
where
    P: StringProcessor + Sized + Clone,
{
    pub string_processors: Vec<P>,
    pub last_hand_position: f64,
    pub last_string_index: usize,
    pub last_note_time: NoteTime,
}

impl<P> Default for BowedStringInstrumentState<P>
where
    P: StringProcessor + Sized + Clone,
{
    fn default() -> Self {
        Self {
            string_processors: Vec::default(),
            last_hand_position: 1.0,
            last_string_index: 100,
            last_note_time: NoteTime::default(),
        }
    }
}
impl<P> SoundState for BowedStringInstrumentState<P> where P: StringProcessor + Sized + Clone {}
