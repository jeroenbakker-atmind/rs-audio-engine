#[derive(Debug, Default, Copy, Clone)]
pub struct Pitch {
    pub frequency: f64,
}

impl From<f64> for Pitch {
    fn from(frequency: f64) -> Self {
        Pitch { frequency }
    }
}
