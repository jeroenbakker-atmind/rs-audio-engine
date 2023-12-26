use crate::fourier_series::FourierSeries;

pub trait ToTimeDomain {
    /// Perform a transformation from frequency domain to time domain.
    fn to_time_domain(&self) -> Vec<f32>;
}

impl ToTimeDomain for FourierSeries {
    fn to_time_domain(&self) -> Vec<f32> {
        let radian_speed_with_amplitudes = self.collect_radian_speed_with_amplitude();
        // TODO: Should still find proof how to deal with sub steps and to many integer steps.
        // Seems to be related to sub steps, num sames and steps. */
        let global_multiplier = 1.0 / (self.parameters.sub_steps + 1) as f32 / 2.0;
        (0..self.parameters.data_len)
            .map(|elem| {
                let r = elem as f32 / (self.parameters.data_len) as f32;
                let sample = radian_speed_with_amplitudes
                    .iter()
                    .map(|(radian_speed, amplitude)| {
                        (radian_speed * r).sin() * amplitude * global_multiplier
                    })
                    .sum::<f32>();
                sample
            })
            .collect::<Vec<f32>>()
    }
}
