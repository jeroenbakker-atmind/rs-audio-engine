use audio_engine_common::buffer::ring_buffer::RingBuffer;
use audio_engine_effect::effect_state::EffectState;

#[derive(Default, Debug, Clone)]
pub struct DelayState {
    pub buffer: RingBuffer<f32>,
}
impl EffectState for DelayState {}
