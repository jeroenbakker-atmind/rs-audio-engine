//! Shape waveforms.
//!
//! A shape waveform is a morph between multiple basic waveforms.
//! This is done by interpolating between the basic waveforms in
//! the frequency domain.

use std::f32::consts::TAU;

use crate::phase_time::PhaseTime;

pub mod morph;
pub mod saw;
pub mod sine;
pub mod square;
pub mod triangle;

pub trait HarmonicShape {
    fn get_harmonic_amplitude(&self, harmonic: u8) -> f32;
}

pub fn shape_sample(shape: &dyn HarmonicShape, phase_time: PhaseTime, num_harmonics: u8) -> f32 {
    let mut result = 0.0;
    for harmonic in 1..=num_harmonics {
        let amplitude = shape.get_harmonic_amplitude(harmonic);
        let scale = TAU * harmonic as f32;
        result += amplitude * (phase_time.time * scale).sin()
    }
    result
}

#[cfg(test)]
mod test {
    use crate::{
        phase_time::PhaseTime,
        waveform::shape::{shape_sample, HarmonicShape},
    };

    fn test_harmonic(shape: &dyn HarmonicShape, num_harmonics: u8) {
        for phase in 0..16 {
            let phase_time = PhaseTime {
                time: phase as f32 / 16.0,
            };
            let sam = shape_sample(shape, phase_time, num_harmonics);
            print!("{sam:.1}, ");
        }
        println!();
    }

    pub fn test_harmonic_shape(shape: &dyn HarmonicShape) {
        for num_harmonics in 1..=16 {
            test_harmonic(shape, num_harmonics);
        }
    }
}
