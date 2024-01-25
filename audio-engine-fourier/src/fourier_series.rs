use std::{cmp::Ordering, f32::consts::TAU};

use crate::{
    complex_number::{ComplexNumber, ComplexNumberMethods},
    parameters::Parameters,
};

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
            .map(|integer_step| {
                (parameters.steps as f32 / parameters.period_duration(integer_step)) * TAU
            })
            .collect::<Vec<f32>>()
    }

    pub fn collect_radian_speed_with_amplitude(&self) -> Vec<(RadianSpeed, ComplexNumber)> {
        FourierSeries::collect_radian_speed(&self.parameters)
            .iter()
            .zip(self.amplitudes.iter())
            .map(|(a, b)| (*a, *b))
            .collect::<Vec<(RadianSpeed, ComplexNumber)>>()
    }

    pub fn semitone(&self, step: usize) -> f32 {
        (step + 1) as f32
    }

    pub fn amplitude(&self, step: usize) -> f32 {
        self.amplitudes[step].amplitude()
    }

    pub fn find_largest_amplitude(&self) -> usize {
        self.amplitudes
            .iter()
            .map(|c| c.amplitude())
            .enumerate()
            .max_by(|a, b| {
                if a.1 > b.1 {
                    Ordering::Greater
                } else if a.1 < b.1 {
                    Ordering::Less
                } else {
                    Ordering::Equal
                }
            })
            .map(|(i, _)| i)
            .unwrap()
    }
}
