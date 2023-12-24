//! Shape waveforms.
//!
//! A shape waveform is a morph between multiple basic waveforms.
//! This is done by interpolating between the basic waveforms in
//! the frequency domain.

use std::f32::consts::TAU;

use crate::{
    digital_sound::sound::Sound,
    phase_time::{self, PhaseTime},
};

use self::{sine::SineShape, square::SquareShape};
pub mod sine;
pub mod square;

pub trait Shape {
    fn get_shape_amplitude(&self, harmonic: u8) -> f32;
}

pub fn shape_sample(shape: &dyn Shape, phase_time: PhaseTime, num_harmonics: u8) -> f32 {
    let mut result = 0.0;
    for harmonic in 1..=num_harmonics {
        let amplitude = shape.get_shape_amplitude(harmonic);
        let scale = TAU * harmonic as f32;
        result += amplitude * (phase_time.time * scale).sin()
    }
    result
}
