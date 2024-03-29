use audio_engine_common::digital_sound::sound_state::SoundState;

use crate::piano2::Piano;

#[derive(Debug, Default, Clone)]
pub struct PianoNoteState2 {
    pub piano: Option<Piano>,
}

impl SoundState for PianoNoteState2 {}
