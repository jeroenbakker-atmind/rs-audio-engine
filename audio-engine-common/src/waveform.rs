use crate::phase_time::PhaseTime;

pub enum Waveform {
    Sine,
    Triangle,
    Square,
    /// A saw waveform
    ///
    /// Saw(false): increasing slope
    /// Saw(true): decreasing slope
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
                if *inverse {
                    1.0 - (phase_time.time) * 2.0 - 1.0
                } else {
                    phase_time.time * 2.0 - 1.0
                }
            }
        }
    }

    pub fn advance(&self, phase_time: &mut PhaseTime, frequency: f32, sample_rate: f32) {
        *phase_time += PhaseTime::delta_phase_time(frequency, sample_rate);
    }
}
