use audio_engine_common::digital_sound::sound_state::SoundState;

use crate::piano::Piano;

#[derive(Debug, Default, Clone)]
pub struct PianoNoteState {
    pub piano: Option<Piano>,
}

impl SoundState for PianoNoteState {}
