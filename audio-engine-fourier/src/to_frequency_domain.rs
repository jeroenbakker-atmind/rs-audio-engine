use crate::{
    complex_number::ComplexNumber,
    fourier_series::{FourierSeries, RadianSpeed},
    parameters::Parameters,
};

// Similar to https://gist.github.com/anonymous/129d477ddb1c8025c9ac
// https://betterexplained.com/articles/an-interactive-guide-to-the-fourier-transform/

pub trait ToFrequencyDomain {
    /// Perform a transformation from time domain to frequency domain.
    fn to_frequency_domain(&self, steps: usize) -> FourierSeries;
    fn to_frequency_domain_with_parameters(&self, parameters: Parameters) -> FourierSeries;
}

impl ToFrequencyDomain for &[f32] {
    fn to_frequency_domain(&self, steps: usize) -> FourierSeries {
        self.to_frequency_domain_with_parameters(Parameters {
            data_len: self.len(),
            steps,
            step_type: crate::parameters::StepType::Semitones,
        })
    }

    fn to_frequency_domain_with_parameters(&self, parameters: Parameters) -> FourierSeries {
        let radian_speeds = FourierSeries::collect_radian_speed(&parameters);
        let amplitudes = radian_speeds
            .iter()
            .map(|radian_speed| calc_frequency_amplitude(self, *radian_speed))
            .collect::<Vec<ComplexNumber>>();

        FourierSeries {
            parameters,
            amplitudes,
        }
    }
}

fn calc_frequency_amplitude(time_domain: &[f32], radian_speed: RadianSpeed) -> ComplexNumber {
    let mut amplitude = ComplexNumber::default();
    for (index, sample) in time_domain.iter().enumerate() {
        let radian = radian_speed * (index as f32 / time_domain.len() as f32);
        amplitude.0 += radian.cos() * sample;
        amplitude.1 += radian.sin() * sample;
    }
    amplitude.0 /= time_domain.len().max(1) as f32;
    amplitude.1 /= time_domain.len().max(1) as f32;

    amplitude
}
