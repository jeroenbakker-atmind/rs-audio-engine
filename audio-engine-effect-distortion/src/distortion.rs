use audio_engine_common::{buffer::ring_buffer::PushOperation, duration::Duration, level::Level};
use audio_engine_effect::effect::Effect;

use crate::distortion_state::DistortionState;

/// Delay effect is a feedback based on a delay time and the level from the feedback is
/// changed before mixing.
#[derive(Debug, Copy, Clone)]
pub struct Distortion {
    pub is_enabled: bool,
    pub level: Level,
}

impl Default for Distortion {
    fn default() -> Self {
        Distortion {
            is_enabled: false,
            level: 0.2,
        }
    }
}

impl Effect for Distortion {
    type EffectState = DistortionState;

    fn effect_create_state(&self) -> Self::EffectState {
        DistortionState::default()
    }

    fn effect_apply(
        &self,
        audio_buffer: &mut [f32],
        _sample_rate: f32,
        _effect_state: &mut Self::EffectState,
    ) {
        if !self.is_enabled || self.level == 0.0 {
            return;
        }

        let level_inv = 1.0 - self.level;
        let level_mult = 1.0 / level_inv;

        audio_buffer.iter_mut().for_each(|out_sample| {
            let new_sample = out_sample.clamp(-level_inv, level_inv) * level_mult;
            *out_sample = new_sample;
        })
    }
}
