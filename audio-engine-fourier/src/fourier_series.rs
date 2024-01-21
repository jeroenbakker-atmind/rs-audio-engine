use std::f32::consts::TAU;

use crate::parameters::Parameters;

/// ComplexNumber can hold the real and imaginary part of a complex number.
///
/// Doing audio processing it isn't really necessary to know how complex
/// number work. See the `real` part as being associated with cosine and
/// the imaginary part with sine. The terms complex number, real and
/// imaginary are kept for alignment with other materials about fourier
/// transforms.
pub type ComplexNumber = (f32, f32);
pub trait ComplexNumberMethods {
    fn amplitude(&self) -> f32;
}
impl ComplexNumberMethods for ComplexNumber {
    fn amplitude(&self) -> f32 {
        (self.0 * self.0 + self.1 * self.1).sqrt()
    }
}

/// RadianSpeed of a frequency
pub type RadianSpeed = f32;

#[derive(Debug, Clone)]
pub struct FourierSeries {
    /// Parameters of this series.
    pub parameters: Parameters,

    pub amplitudes: Vec<ComplexNumber>,
}

impl FourierSeries {
    pub fn collect_radian_speed(parameters: &Parameters) -> Vec<RadianSpeed> {
        (0..parameters.steps)
            .map(|integer_step| TAU * (integer_step as f32))
            .collect::<Vec<f32>>()
    }

    pub fn collect_radian_speed_with_amplitude(&self) -> Vec<(RadianSpeed, ComplexNumber)> {
        FourierSeries::collect_radian_speed(&self.parameters)
            .iter()
            .zip(self.amplitudes.iter())
            .map(|(a, b)| (*a, *b))
            .collect::<Vec<(RadianSpeed, ComplexNumber)>>()
    }

    pub fn frequency(&self, step: usize) -> f32 {
        1.0 / (step + 1) as f32
    }
}
