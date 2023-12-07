pub const RATED_1: OperatorFrequency = OperatorFrequency::Rated(1.0);
pub const RATED_2: OperatorFrequency = OperatorFrequency::Rated(2.0);
pub const RATED_4: OperatorFrequency = OperatorFrequency::Rated(4.0);
pub const RATED_8: OperatorFrequency = OperatorFrequency::Rated(8.0);
pub const RATED_12: OperatorFrequency = OperatorFrequency::Rated(12.0);
pub const RATED_16: OperatorFrequency = OperatorFrequency::Rated(16.0);

#[derive(Debug, Copy, Clone)]
pub enum OperatorFrequency {
    Rated(f32),
    Fixed(f32),
}

impl OperatorFrequency {
    pub fn apply(&self, input: f32) -> f32 {
        match self {
            OperatorFrequency::Fixed(fixed) => *fixed,
            OperatorFrequency::Rated(rate) => input * rate,
        }
    }
}

impl Default for OperatorFrequency {
    fn default() -> Self {
        RATED_1
    }
}
