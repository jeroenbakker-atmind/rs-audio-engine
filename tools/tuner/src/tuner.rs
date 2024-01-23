use std::{cmp::Ordering, thread::sleep, time::Duration};

use audio_engine_fourier::{
    fourier_series::{ComplexNumberMethods, FourierSeries},
    to_frequency_domain::ToFrequencyDomain,
};
use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    Device, Host, SupportedStreamConfig,
};

use crate::arguments::Arguments;

pub struct Tuner {
    arguments: Arguments,

    host: Host,
    device: Device,
    config: SupportedStreamConfig,

    recording_buffer: Vec<f32>,
}

impl Tuner {
    pub fn new(arguments: Arguments) -> Tuner {
        let host = cpal::default_host();
        let device = host.default_input_device().unwrap();
        let config = device.default_input_config().unwrap();
        let recording_buffer = vec![0.0; arguments.buffer_size];

        Tuner {
            arguments,
            host,
            device,
            config,
            recording_buffer,
        }
    }

    fn fill_recording_buffer(&mut self) {
        let mut index = 0;
        let input_stream = self.device.build_input_stream(
            &self.config.config(),
            move |data:&[f32],_: &_| {
                data.chunks(self.config.channels() as usize).map(|a|a.iter().sum::<f32>() / a.len() as f32).for_each(|sample|
                    if index < self.recording_buffer.len() {
                        self.recording_buffer[index] = sample;
                        index += 1;
                    }
                );
            },
            move |error| {},
            None,
        ).unwrap();
        input_stream.play().unwrap();
        while index < self.recording_buffer.len() {
            sleep(Duration::new(0, 10000));
        }
    }

    pub fn sample_frequency(&mut self) -> f32 {
        self.fill_recording_buffer();
        let frequency_domain = self
            .recording_buffer
            .as_slice()
            .to_frequency_domain(self.recording_buffer.len());
        let step = find_step_with_highest_amplitude(&frequency_domain);
        let buffer_frequency = frequency_domain.frequency(step);
        let audio_frequency =
            buffer_frequency * (self.arguments.sample_rate / self.arguments.buffer_size as f32);

        audio_frequency
    }
}

fn find_step_with_highest_amplitude(series: &FourierSeries) -> usize {
    series
        .amplitudes
        .iter()
        .enumerate()
        .max_by(|(_, a), (_, b)| {
            let a_amplitude = a.amplitude();
            let b_amplitude = b.amplitude();
            if a_amplitude > b_amplitude {
                Ordering::Greater
            } else if a_amplitude < b_amplitude {
                Ordering::Less
            } else {
                Ordering::Equal
            }
        })
        .map(|(index, complex)| index)
        .unwrap_or_default()
}
