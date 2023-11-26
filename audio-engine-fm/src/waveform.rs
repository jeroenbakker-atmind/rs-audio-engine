use audio_engine_common::phase_time::PhaseTime;

pub enum Waveform {
    Sine,
    Block,
    Saw,
}

impl Waveform {
    pub fn sample(&self, phase_time: &PhaseTime) -> f32 {
        match self {
            Waveform::Sine => (phase_time.time * std::f32::consts::TAU).sin(),
            Waveform::Block => {
                if phase_time.time < 0.5 {
                    -1.0
                } else {
                    1.0
                }
            }
            Waveform::Saw => phase_time.time * 2.0 - 1.0,
        }
    }

    pub fn sample_and_advance(
        &self,
        phase_time: &mut PhaseTime,
        frequency: f32,
        sample_rate: f32,
    ) -> f32 {
        let result = self.sample(phase_time);
        *phase_time += PhaseTime::delta_phase_time(frequency, sample_rate);
        result
    }
}
