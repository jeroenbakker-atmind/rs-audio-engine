//! Fourier Transformation for audio processing.
//!
//! In audio processing fourier transformation is used in many use-cases.
//!
//! - Frequency filtering
//! - Providing warmer sounds by emulating electronics
//! - Time stretching without loosing pitch
//!
//! Commonly a library is used to transform from time to frequency domain and back.
//! As the goal is to explain many details of constructing a audio engine a basic
//! fourier transform library will be created.
//!
//! NOTE: This library is limited to what is needed. Many expected features for a
//! full fourier transfer library are missing and aren't planned.
//!
//! Thanks to 1 blue 3 brown for their excellent explanation videos which where a
//! base of this implementation.
//!
//! # Transform from time to frequency domain
//!
//! ```rust
//!
//!
//! ```
//!
//! # Transform from frequency to time domain
//!
//! This example will generate the values of a single sine wave.
//!
//! ```rust
//! use audio_engine_fourier::fourier_series::FourierSeries;
//! use audio_engine_fourier::parameters::Parameters;
//! use audio_engine_fourier::to_time_domain::ToTimeDomain;
//!
//! let series = FourierSeries {
//!     parameters: Parameters {
//!         data_len: 256,
//!         steps: 2,
//!         sub_steps: 0,
//!     },
//!     amplitudes: vec![0.0, 1.0],
//! };
//!
//! let time_domain = series.to_time_domain();
//! println!("{time_domain:#?}");
//! ```

use std::f32::consts::TAU;

use crate::to_time_domain::ToTimeDomain;

pub mod fourier_series;
pub mod parameters;
pub mod to_frequency_domain;
pub mod to_time_domain;

#[test]
fn frequency_to_time() {
    use fourier_series::FourierSeries;
    use parameters::Parameters;
    use to_time_domain::ToTimeDomain;

    let series = FourierSeries {
        parameters: Parameters {
            data_len: 256,
            steps: 2,
            sub_steps: 0,
        },
        amplitudes: vec![0.0, 1.0],
    };

    let time_domain = series.to_time_domain();
    println!("{time_domain:#?}");
}

#[test]
fn time_to_frequency() {
    use to_frequency_domain::ToFrequencyDomain;
    let time_domain = (0..16)
        .map(|e| e as f32 / 16.0 * TAU)
        .map(|radian| radian.sin())
        .collect::<Vec<f32>>();
    let fourier_series = time_domain.as_slice().to_frequency_domain(4, 0);
    println!("{:#?}", fourier_series);
}

#[test]
fn time_to_frequency_to_time() {
    use to_frequency_domain::ToFrequencyDomain;
    let input = (0..1024)
        .map(|e| e as f32 / 1024.0 * TAU)
        .map(|radian| radian.sin())
        .collect::<Vec<f32>>();
    println!("input={:#?}", input);
    let fourier_series = input.as_slice().to_frequency_domain(1024, 0);
    println!("series={:#?}", fourier_series.amplitudes);
    let output = fourier_series.to_time_domain();
    println!("output={:#?}", output);
    for (a, b) in input.iter().zip(output.iter()) {
        println!("{a} -> {b}");
    }
}
