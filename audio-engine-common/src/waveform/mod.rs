use crate::{
    digital_sound::{parameters::NoteParameters, sound::Sound},
    phase_time::PhaseTime,
};

use self::{
    shape::{morph::MorphShape, shape_sample},
    state::WaveformState,
};
pub mod shape;
pub mod state;

#[derive(Debug, Default, Copy, Clone)]
pub enum Waveform {
    #[default]
    Sine,
    Triangle,
    Square,
    /// A saw waveform
    /// bool parameter is used to inverse the waveform.
    ///
    /// Saw(false): ramp up/increasing slope
    /// Saw(true): ramp down/decreasing slope
    Saw(bool),
    Pulse(f32),
    Morph(f32, f32, u8),
}

impl Sound for Waveform {
    type SoundState = WaveformState;
    type Parameters = NoteParameters;

    fn init_sound_state(&self) -> Self::SoundState {
        WaveformState::default()
    }

    fn sample(&self, parameters: &Self::Parameters, state: &mut Self::SoundState) -> f32 {
        let result = match self {
            Waveform::Sine => (state.phase_time.time * std::f32::consts::TAU).sin(),
            Waveform::Triangle => {
                let shifted = state.phase_time + PhaseTime { time: 0.25 };
                if shifted.time < 0.5 {
                    -1.0 + shifted.time * 4.0
                } else {
                    1.0 - (shifted.time - 0.5) * 4.0
                }
            }
            Waveform::Square => {
                if state.phase_time.time < 0.5 {
                    -1.0
                } else {
                    1.0
                }
            }
            Waveform::Pulse(factor) => {
                if state.phase_time.time < *factor {
                    -1.0
                } else {
                    1.0
                }
            }
            Waveform::Saw(inverse) => {
                let sample = state.phase_time.time * 2.0 - 1.0;

                if *inverse {
                    1.0 - sample
                } else {
                    sample
                }
            }
            Waveform::Morph(x, y, num_harmonics) => {
                let shape = MorphShape::new(*x, *y);
                shape_sample(&shape, state.phase_time, *num_harmonics)
            }
        } * parameters.gain;
        state.phase_time +=
            PhaseTime::delta_phase_time(parameters.note_pitch, parameters.sample_rate);
        result
    }
}
