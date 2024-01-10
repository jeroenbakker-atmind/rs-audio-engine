use super::{parameters::SoundParameters, sound_state::SoundState};

/// Generic trait for anything that can produce a sound signal
pub trait Sound {
    type SoundState: SoundState + Sized;
    /// Type to used to store parameters.
    ///
    /// It limits the amount of data that is pushed onto the stack and makes the Sound
    /// trait to be used in more cases where additional parameters are needed for example
    /// when sampling FM synthesis.
    type Parameters: SoundParameters;

    fn init_sound_state(&self) -> Self::SoundState;
    fn sample(&self, parameters: &Self::Parameters, state: &mut Self::SoundState) -> f32;
}
