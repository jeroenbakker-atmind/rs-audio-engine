use std::f32::consts::PI;

use super::HarmonicShape;

#[derive(Copy, Clone, Debug)]
pub struct TriangleShape {
    mode: i32,
}

impl Default for TriangleShape {
    fn default() -> Self {
        TriangleShape { mode: 16 }
    }
}

impl HarmonicShape for TriangleShape {
    fn get_harmonic_amplitude(&self, harmonic: u8) -> f32 {
        let is_odd = (harmonic & 1) != 0;
        let is_odd_odd = (harmonic & 3) != 0;
        let sign = if is_odd_odd { 1.0 } else { -1.0 };
        if is_odd {
            sign * (8.0 / (PI * PI)) * (1.0 / ((harmonic as f32).powi(self.mode)))
        } else {
            0.0
        }
    }
}

#[cfg(test)]
mod test {
    use crate::waveform::shape::test::test_harmonic_shape;

    use super::TriangleShape;

    #[test]
    fn test_triangle_harmonics() {
        let shape = TriangleShape { mode: 10 };
        test_harmonic_shape(&shape);
    }
}
