use audio_engine_common::{digital_sound::sound::Sound, note_time::NoteTime};

use crate::sample_note_state::SampleNoteState;

#[derive(Debug, Copy, Clone)]
pub struct Sample {
    pub start: usize,
    pub end: usize,

    pub is_looped: bool,
    pub loop_start: usize,
    pub loop_end: usize,

    pub sample_rate_c4: f32,

    pub data: &'static [f32],
}

impl Sound for Sample {
    type SoundState = SampleNoteState;
    fn init_sound_state(&self) -> Self::SoundState {
        SampleNoteState {
            sample_offset: self.start as f32,
            ..SampleNoteState::default()
        }
    }

    fn sample(
        &self,
        note_time: NoteTime,
        note_off: Option<NoteTime>,
        note_pitch: f32,
        sample_rate: f32,
        state: &mut SampleNoteState,
    ) -> f32 {
        let sample_offset = state.sample_offset as usize;
        if state.is_finished {
            return 0.0;
        }

        let result = self.data[sample_offset];

        let is_note_released = match note_off {
            Some(note_off) => note_time > note_off,
            None => false,
        };
        let do_loop_evaluation = self.is_looped && !is_note_released;

        // TODO: delta should depend on note_pitch and self.sample_rate_c4 and sample_rate
        let mut new_sample_offset = state.sample_offset + 1.0;

        if new_sample_offset >= self.loop_end as f32 && do_loop_evaluation {
            new_sample_offset -= (self.loop_end - self.loop_start) as f32;
        }
        if new_sample_offset >= self.end as f32 {
            state.is_finished = true;
        }

        state.sample_offset = new_sample_offset;

        result
    }
}
