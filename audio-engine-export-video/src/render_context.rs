use audio_engine_fourier::{fourier_series::FourierSeries, to_frequency_domain::ToFrequencyDomain};

#[derive(Default)]
pub struct RenderContext {
    pub fourier_series: Vec<FourierSeries>,
}

impl RenderContext {
    pub fn init_fourier_series(&mut self, samples: &[f32], samples_per_frame: usize) {
        for i in 0..=samples.len() / samples_per_frame {
            let frame_samples =
                &samples[i * samples_per_frame..((i + 1) * samples_per_frame).min(samples.len())];
            self.fourier_series
                .push(frame_samples.to_frequency_domain(samples_per_frame, 0));
        }
    }
}
