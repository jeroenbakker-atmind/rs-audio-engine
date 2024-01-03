use audio_engine_common::{digital_sound::sound::Sound, note_time::NoteTime};

use crate::note_state::NoteState;

#[derive(Debug, Copy, Clone)]
pub struct PianoInstrument {}

impl Sound for PianoInstrument {
    type SoundState = NoteState;
    fn init_sound_state(&self) -> Self::SoundState {
        NoteState {
            ..NoteState::default()
        }
    }

    fn sample(
        &self,
        note_time: NoteTime,
        note_off: Option<NoteTime>,
        note_pitch: f32,
        sample_rate: f32,
        state: &mut NoteState,
    ) -> f32 {
        0.0
    }
}
