use audio_engine_common::buffer::ring_buffer::RingBuffer;
use audio_engine_effect::effect_state::EffectState;

#[derive(Default, Debug, Clone)]
pub struct DistortionState {}
impl EffectState for DistortionState {}
