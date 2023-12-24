use std::f32::consts::PI;

use super::HarmonicShape;

#[derive(Copy, Clone, Debug, Default)]
pub struct SawShape {}
impl HarmonicShape for SawShape {
    fn get_harmonic_amplitude(&self, harmonic: u8) -> f32 {
        PI / 4.0 / harmonic as f32
    }
}

#[cfg(test)]
mod test {
    use crate::waveform::shape::test::test_harmonic_shape;

    use super::SawShape;

    #[test]
    fn test_saw_harmonics() {
        let shape = SawShape {};
        test_harmonic_shape(&shape);
    }
}
