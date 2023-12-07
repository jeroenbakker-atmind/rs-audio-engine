use audio_engine_common::note_time::NoteTime;

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

impl Sample {
    pub fn sample(
        &self,
        note_time: NoteTime,
        note_off: Option<NoteTime>,
        note_pitch: f32,
        sample_rate: f32,
        state: &mut SampleNoteState,
    ) -> f32 {
        0.0
    }
}
