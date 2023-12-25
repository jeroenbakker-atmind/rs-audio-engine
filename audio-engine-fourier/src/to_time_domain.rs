use crate::fourier_series::FourierSeries;

pub trait ToTimeDomain {
    /// Perform a transformation from frequency domain to time domain.
    fn to_time_domain(&self) -> Vec<f32>;
}

impl ToTimeDomain for FourierSeries {
    fn to_time_domain(&self) -> Vec<f32> {
        let radian_speed_with_amplitudes = self.collect_radian_speed_with_amplitude();

        (0..self.parameters.data_len)
            .map(|elem| {
                let r = elem as f32 / (self.parameters.data_len) as f32;
                let sample = radian_speed_with_amplitudes
                    .iter()
                    .map(|(radian_speed, amplitude)| (radian_speed * r).sin() * amplitude)
                    .sum::<f32>();
                sample
            })
            .collect::<Vec<f32>>()
    }
}
