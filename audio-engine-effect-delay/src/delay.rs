use audio_engine_common::{buffer::ring_buffer::PushOperation, duration::Duration, level::Level};
use audio_engine_effect::effect::Effect;

use crate::delay_state::DelayState;

/// Delay effect is a feedback based on a delay time and the level from the feedback is
/// changed before mixing.
#[derive(Debug, Copy, Clone)]
pub struct Delay {
    pub is_enabled: bool,
    pub delay_time: Duration,
    pub level: Level,
}

impl Default for Delay {
    fn default() -> Self {
        Delay {
            is_enabled: false,
            delay_time: 1.0,
            level: 0.2,
        }
    }
}

impl Effect for Delay {
    type EffectState = DelayState;

    fn effect_create_state(&self) -> Self::EffectState {
        DelayState::default()
    }

    fn effect_apply(
        &self,
        audio_buffer: &mut [f32],
        sample_rate: f32,
        effect_state: &mut Self::EffectState,
    ) {
        if !self.is_enabled {
            return;
        }
        // -1 is needed as we first pop before push.
        let offset = (sample_rate * self.delay_time) as usize - 1;
        let max_offset = offset + audio_buffer.len() + 4;
        effect_state.buffer.ensure_size(max_offset);

        audio_buffer.iter_mut().for_each(|out_sample| {
            let new_sample = *out_sample + effect_state.buffer.pop_or_default() * self.level;
            effect_state
                .buffer
                .push(offset, new_sample, PushOperation::Add);

            *out_sample = new_sample;
        })
    }
}

#[cfg(test)]
mod test {
    use audio_engine_effect::effect::Effect;

    use super::Delay;

    #[test]
    fn delay_small_input() {
        let delay = Delay {
            is_enabled: true,
            delay_time: 1.0,
            level: 0.5,
        };

        let in_samples = [1.0];
        let sample_rate = 4.0;

        let mut delay_state = delay.effect_create_state();

        for out_value in [1.0, 1.5, 1.75, 1.875] {
            for _ in 0..4 {
                let mut samples = in_samples;
                delay.effect_apply(&mut samples, sample_rate, &mut delay_state);
                assert_eq!(samples, [out_value]);
            }
        }
    }

    #[test]
    fn delay_large_input() {
        let delay = Delay {
            is_enabled: true,
            delay_time: 1.0,
            level: 0.5,
        };

        let in_samples = [1.0; 16];
        let sample_rate = 4.0;

        let mut delay_state = delay.effect_create_state();

        let mut samples = in_samples;
        delay.effect_apply(&mut samples, sample_rate, &mut delay_state);
        assert_eq!(
            samples,
            [
                1.0, 1.0, 1.0, 1.0, 1.5, 1.5, 1.5, 1.5, 1.75, 1.75, 1.75, 1.75, 1.875, 1.875,
                1.875, 1.875
            ]
        );
    }
}
