use std::f32::consts::PI;

use super::HarmonicShape;

#[derive(Copy, Clone, Debug, Default)]
pub struct SquareShape {}
impl HarmonicShape for SquareShape {
    fn get_harmonic_amplitude(&self, harmonic: u8) -> f32 {
        let is_odd = (harmonic & 1) != 0;
        if is_odd {
            (4.0 / PI) * (1.0 / harmonic as f32)
        } else {
            0.0
        }
    }
}

#[cfg(test)]
mod test {
    use crate::waveform::shape::test::test_harmonic_shape;

    use super::SquareShape;

    #[test]
    fn test_square_harmonics() {
        let shape = SquareShape {};
        test_harmonic_shape(&shape);
    }
}
