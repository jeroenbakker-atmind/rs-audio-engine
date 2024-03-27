use super::PianoString;

// TODO: PianoNote is a set of strings, wiht a f0 and hammer velocity, input velocities
// Per string a single filter.
#[derive(Debug, Default, Clone)]
pub struct PianoNote {
    pub strings: Vec<PianoString>,
    pub sample_index: usize,
}

impl PianoNote {
    pub fn filter(&mut self, value_in: f64) -> f64 {
        let mut result = 0.0;
        for string in &mut self.strings {
            result += string.filter(value_in);
        }
        result
    }
}
