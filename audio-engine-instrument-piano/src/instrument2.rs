use audio_engine_common::digital_sound::{parameters::NoteParameters, sound::Sound};

use crate::{note_state2::PianoNoteState2, piano2::Piano};

#[derive(Debug, Default, Copy, Clone)]
pub struct PianoInstrument2 {}

impl Sound for PianoInstrument2 {
    type SoundState = PianoNoteState2;
    type Parameters = NoteParameters;

    fn init_sound_state(&self) -> Self::SoundState {
        PianoNoteState2 {
            ..PianoNoteState2::default()
        }
    }

    fn sample(&self, parameters: &Self::Parameters, state: &mut PianoNoteState2) -> f32 {
        if state.piano.is_none() {
            let hammer_velocity = parameters.gain as f64 * 10.0;
            let mut piano = Piano::new(parameters.sample_rate as f64);
            piano.init_note(parameters.note_pitch as f64, hammer_velocity);
            state.piano = Some(piano);
        }

        if let Some(piano) = &mut state.piano {
            piano.sample() as f32
        } else {
            0.0
        }
    }
}
