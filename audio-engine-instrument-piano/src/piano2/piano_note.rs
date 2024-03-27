use super::PianoString;

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
