use super::HarmonicShape;

#[derive(Copy, Clone, Debug, Default)]
pub struct SineShape {}
impl HarmonicShape for SineShape {
    fn get_harmonic_amplitude(&self, harmonic: u8) -> f32 {
        if harmonic == 1 {
            1.0
        } else {
            0.0
        }
    }
}
#[cfg(test)]
mod test {
    use super::SineShape;
    use crate::waveform::shape::test::test_harmonic_shape;

    #[test]
    fn test_sine_harmonics() {
        let shape = SineShape {};
        test_harmonic_shape(&shape);
    }
}
