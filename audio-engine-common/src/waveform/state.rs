use crate::{digital_sound::sound_state::SoundState, phase_time::PhaseTime};

#[derive(Debug, Copy, Clone, Default)]
pub struct WaveformState {
    pub phase_time: PhaseTime,
}
impl SoundState for WaveformState {}
