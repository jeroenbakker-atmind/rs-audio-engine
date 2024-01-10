use audio_engine_common::digital_sound::{parameters::NoteParameters, sound::Sound};

use crate::{note_state::PianoNoteState, piano::Piano};

#[derive(Debug, Default, Copy, Clone)]
pub struct PianoInstrument {}

impl Sound for PianoInstrument {
    type SoundState = PianoNoteState;
    type Parameters = NoteParameters;

    fn init_sound_state(&self) -> Self::SoundState {
        PianoNoteState {
            ..PianoNoteState::default()
        }
    }

    fn sample(&self, parameters: &Self::Parameters, state: &mut PianoNoteState) -> f32 {
        if state.piano.is_none() {
            let mut piano = Piano::default();
            piano.init(
                parameters.note_pitch,
                parameters.sample_rate,
                // TODO: use a curve to transforma a gain to hammer velocity
                parameters.gain * 10.0,
            );
            state.piano = Some(piano);
        }
        let mut result = [0.0; 1];
        if let Some(piano) = &mut state.piano {
            piano.go(&mut result);
        }
        result[0]
    }
}
