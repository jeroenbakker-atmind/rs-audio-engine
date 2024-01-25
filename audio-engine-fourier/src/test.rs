use std::f32::consts::TAU;

use crate::{
    fourier_series::FourierSeries,
    parameters::{FrequencyRange, Parameters, StepType},
    to_frequency_domain::ToFrequencyDomain,
    to_time_domain::ToTimeDomain,
};

#[test]
fn frequency_to_time() {
    let series = FourierSeries {
        parameters: Parameters {
            data_len: 256,
            steps: 2,
            step_type: StepType::Semitones,
        },
        amplitudes: vec![(0.0, 0.0), (0.0, 1.0)],
    };

    let time_domain = series.to_time_domain();
    println!("{time_domain:#?}");
}

#[test]
fn time_to_frequency() {
    let time_domain = (0..16)
        .map(|e| e as f32 / 16.0 * TAU)
        .map(|radian| radian.sin())
        .collect::<Vec<f32>>();
    let fourier_series = time_domain.as_slice().to_frequency_domain(4);
    println!("{:#?}", fourier_series);
}

#[test]
fn time_to_frequency_to_time() {
    let input = (0..1024)
        .map(|e| e as f32 / 1024.0 * TAU)
        .map(|radian| radian.sin())
        .collect::<Vec<f32>>();
    println!("input={:#?}", input);
    let fourier_series = input.as_slice().to_frequency_domain(1024);
    println!("series={:#?}", fourier_series.amplitudes);
    let output = fourier_series.to_time_domain();
    println!("output={:#?}", output);
    for (a, b) in input.iter().zip(output.iter()) {
        println!("{a} -> {b}");
    }
}

#[test]
fn frequency_range() {
    let buffer_size = 4096;
    let sample_rate = 44100.0;
    let note_pitch = 467.0;

    let parameters = Parameters {
        data_len: buffer_size,
        steps: 4096,
        step_type: StepType::FrequencyRange(FrequencyRange {
            start_frequency: 400.0,
            end_frequency: 600.0,
            sample_rate: sample_rate,
        }),
    };

    let input_data = (0..4096)
        .map(|step| (step as f32 / (sample_rate / note_pitch)) * TAU)
        .map(|radian| radian.sin())
        .collect::<Vec<f32>>();
    println!("{input_data:#?}");

    let fourier_series = input_data
        .as_slice()
        .to_frequency_domain_with_parameters(parameters);

    (0..fourier_series.parameters.steps).for_each(|step| {
        let frequency = fourier_series.parameters.frequency(step);
        let amplitude = fourier_series.amplitude(step);
        println!("{step}({frequency}): {amplitude}");
    });

    let step = fourier_series.find_largest_amplitude();
    let frequency = fourier_series.parameters.frequency(step);
    let amplitude = fourier_series.amplitude(step);
    println!("{step}({frequency}Hz): {amplitude}");
}
