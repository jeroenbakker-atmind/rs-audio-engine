use std::f32::consts::TAU;

use crate::parameters::Parameters;

pub type ComplexNumber = (f32, f32);
pub type RadianSpeed = f32;

#[derive(Debug, Clone)]
pub struct FourierSeries {
    /// Parameters of this series.
    pub parameters: Parameters,

    pub amplitudes: Vec<ComplexNumber>,
}

impl FourierSeries {
    pub fn collect_radian_speed(parameters: &Parameters) -> Vec<RadianSpeed> {
        let mut result = Vec::with_capacity(parameters.steps * (parameters.sub_steps + 1));

        (0..parameters.steps).for_each(|integer_step| {
            (0..=parameters.sub_steps).for_each(|sub_step| {
                let radian_speed = TAU
                    * (integer_step as f32 + sub_step as f32 / (parameters.sub_steps + 1) as f32);
                result.push(radian_speed);
            })
        });

        result
    }

    pub fn collect_radian_speed_with_amplitude(&self) -> Vec<(RadianSpeed, ComplexNumber)> {
        FourierSeries::collect_radian_speed(&self.parameters)
            .iter()
            .zip(self.amplitudes.iter())
            .map(|(a, b)| (*a, *b))
            .collect::<Vec<(RadianSpeed, ComplexNumber)>>()
    }
}
