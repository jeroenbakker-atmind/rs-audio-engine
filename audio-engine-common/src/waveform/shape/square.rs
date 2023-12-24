use std::f32::consts::PI;

use super::Shape;

pub struct SquareShape {}
impl Shape for SquareShape {
    fn get_shape_amplitude(&self, harmonic: u8) -> f32 {
        let is_odd = (harmonic & 1) != 0;
        if is_odd {
            (4.0 / PI) * (1.0 / harmonic as f32)
        } else {
            0.0
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{
        phase_time::PhaseTime,
        waveform::shape::{shape_sample, Shape},
    };

    use super::SquareShape;

    fn test_harmonic(shape: &dyn Shape, num_harmonics: u8) {
        for phase in 0..16 {
            let phase_time = PhaseTime {
                time: phase as f32 / 16.0,
            };
            let sam = shape_sample(shape, phase_time, num_harmonics);
            print!("{sam:.1}, ");
        }
        println!("");
    }

    #[test]
    fn test_harmonics() {
        let shape = SquareShape {};
        for num_harmonics in 0..16 {
            test_harmonic(&shape, num_harmonics);
        }
    }
    #[test]
    fn test_harmonic_3() {
        let shape = SquareShape {};
        test_harmonic(&shape, 3);
    }
}
