use crate::{
    digital_sound::{sound::Sound, sound_state::SoundState},
    phase_time::PhaseTime,
};

#[derive(Debug, Default, Copy, Clone)]
pub enum Waveform {
    #[default]
    Sine,
    Triangle,
    Square,
    /// A saw waveform
    ///
    /// Saw(false): ramp up/increasing slope
    /// Saw(true): ramp down/decreasing slope
    Saw(bool),
    Pulse(f32),
}

pub type WaveformState = PhaseTime;
impl SoundState for WaveformState {}

impl Sound for Waveform {
    type SoundState = WaveformState;
    fn init_sound_state(&self) -> Self::SoundState {
        PhaseTime::default()
    }

    fn sample(
        &self,
        _note_time: crate::note_time::NoteTime,
        _note_off: Option<crate::note_time::NoteTime>,
        note_pitch: f32,
        sample_rate: f32,
        state: &mut Self::SoundState,
    ) -> f32 {
        let result = match self {
            Waveform::Sine => (state.time * std::f32::consts::TAU).sin(),
            Waveform::Triangle => {
                let shifted = *state + PhaseTime { time: 0.25 };
                if shifted.time < 0.5 {
                    -1.0 + shifted.time * 4.0
                } else {
                    1.0 - (shifted.time - 0.5) * 4.0
                }
            }
            Waveform::Square => {
                if state.time < 0.5 {
                    -1.0
                } else {
                    1.0
                }
            }
            Waveform::Pulse(factor) => {
                if state.time < *factor {
                    -1.0
                } else {
                    1.0
                }
            }
            Waveform::Saw(inverse) => {
                let sample = state.time * 2.0 - 1.0;

                if *inverse {
                    1.0 - sample
                } else {
                    sample
                }
            }
        };
        *state += PhaseTime::delta_phase_time(note_pitch, sample_rate);
        result
    }
}
