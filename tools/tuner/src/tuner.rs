use std::{thread::sleep, time::Duration};

use audio_engine_fourier::{
    parameters::{FrequencyRange, Parameters, StepType},
    to_frequency_domain::ToFrequencyDomain,
};
use audio_engine_notes::ChromaticNote;
use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    BufferSize, Device, StreamConfig, SupportedStreamConfig,
};

use crate::arguments::Arguments;

pub struct Tuner {
    arguments: Arguments,
    device: Device,
    config: SupportedStreamConfig,
}

impl Tuner {
    pub fn new(arguments: Arguments) -> Tuner {
        let host = cpal::default_host();
        let device = host.default_input_device().unwrap();
        let config = device.default_input_config().unwrap();

        Tuner {
            arguments,
            device,
            config,
        }
    }

    pub fn start(&mut self) {
        let config = StreamConfig {
            buffer_size: BufferSize::Fixed(4096 * self.config.channels() as u32),
            ..self.config.config()
        };
        let sample_rate = config.sample_rate.0 as f32;
        let num_channels = self.config.channels() as usize;
        let args = self.arguments.clone();
        let input_stream = self
            .device
            .build_input_stream(
                &config,
                move |data: &[f32], _: &_| {
                    process_samples(data, sample_rate, num_channels, args);
                },
                move |_error| {},
                None,
            )
            .unwrap();
        input_stream.play().unwrap();
        loop {
            sleep(Duration::new(0, 10000));
        }
    }
}

fn process_samples(input_samples: &[f32], sample_rate: f32, num_channels: usize, args: Arguments) {
    let mono_samples = input_samples
        .chunks(num_channels)
        .map(|samples| samples[0])
        .collect::<Vec<f32>>();
    let pitch = args.note.pitch();
    let start_frequency = pitch / 2.0;
    let end_frequency = pitch * 2.0;
    let parameters = Parameters {
        data_len: mono_samples.len(),
        steps: args.steps,
        step_type: StepType::FrequencyRange(FrequencyRange {
            sample_rate,
            start_frequency,
            end_frequency,
        }),
    };
    let frequency_domain = mono_samples
        .as_slice()
        .to_frequency_domain_with_parameters(parameters);
    let step = frequency_domain.find_largest_amplitude();
    let frequency = frequency_domain.parameters.frequency(step);
    let amplitude = frequency_domain.amplitude(step);
    if amplitude > args.threshold {
        let closest_note = ChromaticNote::from(frequency);
        println!(
            "current={closest_note:?}({frequency}Hz),wanted={:?}({pitch}Hz)",
            args.note
        );
    }
}
