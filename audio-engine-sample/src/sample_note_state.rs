use audio_engine_common::digital_sound::sound_state::SoundState;

#[derive(Debug, Default, Copy, Clone)]
pub struct SampleNoteState {
    pub is_finished: bool,
    pub sample_offset: f32,
}

impl SoundState for SampleNoteState {}
