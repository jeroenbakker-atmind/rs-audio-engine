use crate::note_time::NoteTime;

use super::sound_state::SoundState;

/// Generic trait for anything that can produce a sound signal
pub trait Sound {
    type SoundState: SoundState + Sized;

    fn init_sound_state(&self) -> Self::SoundState;
    fn sample(
        &self,
        note_time: NoteTime,
        note_off: Option<NoteTime>,
        note_pitch: f32,
        sample_rate: f32,
        state: &mut Self::SoundState,
    ) -> f32;
}
