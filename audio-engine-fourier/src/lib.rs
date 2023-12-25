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
//! # Transform from frequency to time domain
//!
//! This example will generate the values of a single sine wave.
//! ```
//! use audio_engine_fourier::fourier_series::FourierSeries;
//! use audio_engine_fourier::parameters::Parameters;
//! use audio_engine_fourier::to_time_domain::ToTimeDomain;
//!
//! let series = FourierSeries {
//!     parameters: Parameters {
//!         data_len: 256,
//!         steps: 2,
//!         sub_step: 0,
//!     },
//!     amplitudes: vec![0.0, 1.0],
//! };
//!
//! let time_domain = series.to_time_domain();
//! println!("{time_domain:#?}");
//! ```

pub mod fourier_series;
pub mod parameters;
pub mod to_time_domain;
