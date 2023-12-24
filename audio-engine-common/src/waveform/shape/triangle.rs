use super::HarmonicShape;

#[derive(Copy, Clone, Debug, Default)]
pub struct TriangleShape {}

impl HarmonicShape for TriangleShape {
    fn get_harmonic_amplitude(&self, harmonic: u8) -> f32 {
        if harmonic & 1 == 0 {
            0.0
        } else {
            ((-1.0_f32).powf((harmonic as f32 - 1.0) / 2.0)) / ((harmonic as f32).powi(2))
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
