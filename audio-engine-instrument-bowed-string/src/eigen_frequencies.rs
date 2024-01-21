use std::f64::consts::PI;

use crate::string_and_hand::StringAndHand;

pub trait EigenFrequency {
    fn calc_eigen_frequency(&self, mode_number: usize) -> f64;
    fn eigen_frequencies(&self, length: usize) -> Vec<f64> {
        (1..=length)
            .map(|mode| self.calc_eigen_frequency(mode))
            .collect::<Vec<f64>>()
    }
}

impl EigenFrequency for StringAndHand {
    fn calc_eigen_frequency(&self, mode_number: usize) -> f64 {
        let n = mode_number as f64 * PI / self.length();
        let lin_density = self.string.lin_density();
        ((self.string.tension / lin_density) * n * n
            + (self.string.young_mod * self.string.inertia() / lin_density) * n * n * n * n)
            .sqrt()
    }
}
