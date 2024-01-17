use std::f32::consts::PI;

use crate::string::String;

pub trait DampingCoeffcient {
    fn damping_coeffcient(&self, eigen_frequency: f32) -> f32;
    fn damping_coeffcients(&self, eigen_frequencies: &[f32]) -> Vec<f32> {
        eigen_frequencies
            .iter()
            .map(|eigen_frequency| self.damping_coeffcient(*eigen_frequency))
            .collect::<Vec<f32>>()
    }
}

impl DampingCoeffcient for String {
    /// Desvages damping coeffcients
    fn damping_coeffcient(&self, eigen_frequency: f32) -> f32 {
        const RHO_AIR: f32 = 1.225;
        const MU_AIR: f32 = 1.619e-5;

        let d0 = -2.0 * RHO_AIR * MU_AIR / (self.density * self.radius * self.radius);
        let d1 = -2.0 * RHO_AIR * (2.0 * MU_AIR).sqrt() / (self.density * self.radius);
        let d2 = -1.0 / 18000.0;
        let d3 = (-0.003
            * self.young_mod
            * self.density
            * PI
            * PI
            * self.radius
            * self.radius
            * self.radius
            * self.radius
            * self.radius
            * self.radius)
            / (4.0 * self.tension * self.tension);
        d0 + d1 * eigen_frequency.sqrt()
            + d2 * eigen_frequency
            + d3 * eigen_frequency * eigen_frequency * eigen_frequency
    }
}
