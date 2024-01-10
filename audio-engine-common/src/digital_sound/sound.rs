use super::{parameters::SoundParameters, sound_state::SoundState};

/// Generic trait for anything that can produce a sound signal
pub trait Sound {
    type SoundState: SoundState + Sized;
    type Parameters: SoundParameters;

    fn init_sound_state(&self) -> Self::SoundState;
    fn sample(&self, parameters: &Self::Parameters, state: &mut Self::SoundState) -> f32;
}
