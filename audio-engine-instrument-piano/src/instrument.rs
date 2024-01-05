use audio_engine_common::{digital_sound::sound::Sound, note_time::NoteTime};

use crate::{note_state::PianoNoteState, piano::Piano};

#[derive(Debug, Default, Copy, Clone)]
pub struct PianoInstrument {}

impl Sound for PianoInstrument {
    type SoundState = PianoNoteState;
    fn init_sound_state(&self) -> Self::SoundState {
        PianoNoteState {
            ..PianoNoteState::default()
        }
    }

    fn sample(
        &self,
        _note_time: NoteTime,
        _note_off: Option<NoteTime>,
        note_pitch: f32,
        sample_rate: f32,
        state: &mut PianoNoteState,
    ) -> f32 {
        if state.piano.is_none() {
            let mut piano = Piano::default();
            piano.init(note_pitch, sample_rate, 5.0, 1);
            state.piano = Some(piano);
        }
        let mut result = [0.0; 1];
        if let Some(piano) = &mut state.piano {
            piano.go(&mut result);
        }
        result[0]
    }
}
