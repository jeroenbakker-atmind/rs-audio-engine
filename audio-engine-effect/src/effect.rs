use crate::effect_state::EffectState;

pub trait Effect {
    type EffectState: Sized + EffectState;

    fn effect_create_state(&self) -> Self::EffectState;

    /// Apply the effect to the given buffer
    fn effect_apply(
        &self,
        audio_buffer: &mut [f32],
        sample_rate: f32,
        effect_state: &mut Self::EffectState,
    );
}
