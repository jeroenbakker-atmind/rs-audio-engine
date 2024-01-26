#[derive(Debug, Default, Copy, Clone)]
pub struct Pitch {
    pub frequency: f64,
}

impl From<f32> for Pitch {
    fn from(value: f32) -> Self {
        Pitch {
            frequency: value as f64,
        }
    }
}

impl From<f64> for Pitch {
    fn from(value: f64) -> Self {
        Pitch { frequency: value }
    }
}
