use crate::phase_time::PhaseTime;

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

impl Waveform {
    pub fn sample(&self, phase_time: &PhaseTime) -> f32 {
        match self {
            Waveform::Sine => (phase_time.time * std::f32::consts::TAU).sin(),
            Waveform::Triangle => {
                let shifted = *phase_time + PhaseTime { time: 0.25 };
                if shifted.time < 0.5 {
                    -1.0 + shifted.time * 4.0
                } else {
                    1.0 - (shifted.time - 0.5) * 4.0
                }
            }
            Waveform::Square => {
                if phase_time.time < 0.5 {
                    -1.0
                } else {
                    1.0
                }
            }
            Waveform::Pulse(factor) => {
                if phase_time.time < *factor {
                    -1.0
                } else {
                    1.0
                }
            }
            Waveform::Saw(inverse) => {
                let sample = phase_time.time * 2.0 - 1.0;

                if *inverse {
                    1.0 - sample
                } else {
                    sample
                }
            }
        }
    }

    pub fn advance(&self, phase_time: &mut PhaseTime, note_pitch: f32, sample_rate: f32) {
        *phase_time += PhaseTime::delta_phase_time(note_pitch, sample_rate);
    }
}
