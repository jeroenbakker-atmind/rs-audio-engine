use super::PianoString;

#[derive(Debug, Default, Clone)]
pub struct PianoNote {
    pub strings: Vec<PianoString>,
    pub sample_index: usize,
    /// Hammer input velocity over time.
    pub input_velocities: Vec<f64>,
}

impl PianoNote {
    pub fn sample(&mut self) -> f64 {
        let sample_in = if self.sample_index < self.input_velocities.len() {
            self.input_velocities[self.sample_index]
        } else {
            0.0
        };
        let sample_out = self.filter(sample_in);
        self.sample_index += 1;
        sample_out / 1000.0
    }

    pub fn filter(&mut self, value_in: f64) -> f64 {
        let mut result = 0.0;
        for string in &mut self.strings {
            result += string.filter(value_in);
        }
        result
    }
}
