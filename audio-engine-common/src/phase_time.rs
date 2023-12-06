use std::ops::{Add, AddAssign};

#[derive(Debug, Default, Copy, Clone)]
pub struct PhaseTime {
    pub time: f32,
}

impl PhaseTime {
    pub fn delta_phase_time(frequency: f32, sample_rate: f32) -> PhaseTime {
        PhaseTime {
            time: (frequency / sample_rate).fract(),
        }
    }
}

impl AddAssign for PhaseTime {
    fn add_assign(&mut self, other: Self) {
        self.time = (self.time + 1.0 + other.time).fract()
    }
}

impl Add for PhaseTime {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        PhaseTime {
            time: (self.time + 1.0 + other.time).fract(),
        }
    }
}
