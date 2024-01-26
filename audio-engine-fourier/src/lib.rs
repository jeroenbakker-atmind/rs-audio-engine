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
//! The implementation of fourier transform in this library is known to be slow.
//! It uses a `o^2` complexity. This can be solved by using a one of the many FFT
//! algorithms. The idea would be to add a feature set to wrap the API into an
//! existing library providing the FFT algorithms.
//!
//! This makes the library not useful for real-time applications (yet). Currently
//! would recommend it only for off-line audio rendering/processing.
//!
//! Thanks to 1 blue 3 brown for their excellent explanation videos which where a
//! base of this implementation.
//!
//! # Transform from time to frequency domain
//!
//! ```rust
//! use std::f32::consts::TAU;
//! use audio_engine_fourier::to_frequency_domain::ToFrequencyDomain;
//!
//! let time_domain = (0..16)
//!     .map(|e| e as f32 / 16.0 * TAU)
//!     .map(|radian| radian.sin())
//!     .collect::<Vec<f32>>();
//! let fourier_series = time_domain.as_slice().to_frequency_domain(4);
//! println!("{:#?}", fourier_series);
//! ```
//!
//! # Transform from frequency to time domain
//!
//! This example will generate the values of a single sine wave.
//!
//! ```rust
//! use audio_engine_fourier::fourier_series::FourierSeries;
//! use audio_engine_fourier::parameters::{Parameters, StepType};
//! use audio_engine_fourier::to_time_domain::ToTimeDomain;
//!
//! let series = FourierSeries {
//!     parameters: Parameters {
//!         data_len: 256,
//!         steps: 2,
//!         step_type: StepType::Semitones,
//!     },
//!     amplitudes: vec![(0.0, 0.0), (0.0, 1.0)],
//! };
//!
//! let time_domain = series.to_time_domain();
//! println!("{time_domain:#?}");
//! ```

pub mod complex_number;
pub mod fourier_series;
pub mod parameters;
pub mod to_frequency_domain;
pub mod to_time_domain;

#[cfg(test)]
mod test;
