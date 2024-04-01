//! Port of the Piano model implemented by Lorenzoncina

use std::f64::consts::TAU;

use audio_engine_common::convolve::Convolution;
use audio_engine_discrete_time::transfer_function::TransferFunction;

use crate::piano2::piano_string::PianoString;

use self::{
    piano_ir::PIANO_IR_SAMPLES, piano_note::PianoNote,
    string_group_configuration::StringGroupConfigurations,
};
mod piano_ir;
mod piano_note;
mod piano_string;
mod string_group_configuration;

/// Input length of the precalculated forces, before switching to wave guiding.
const INPUT_LENGTH: usize = 150;

#[derive(Debug, Default, Clone)]
pub struct Piano {
    /// Samplerate
    pub sample_rate: f64,
    /// Number of spatial grid points
    pub num_spatial_grid_points: usize,
    /// Length of the piano wire
    pub string_length: f64,
    /// Mass of the piano wire in Kg
    pub string_mass: f64,
    ///Tension of the wire
    pub string_tension: f64,
    /// Stiffness non-linear coefficient
    pub p: f64,
    /// Mass of the hammer in Kg
    pub hammer_mass: f64,
    /// Hammer stiffness coefficient
    pub hammer_stiffness_coefficient: f64,
    /// Relativa striking position
    pub relative_striking_position: f64,
    /// Damping coefficient
    pub damping_coefficient_a: f64,
    /// Damping coefficient
    pub damping_coefficient_b: f64,
    /// String stiffness parameter
    pub epsilon: f64,

    pub string_configurations: StringGroupConfigurations,
    pub note: PianoNote,
}

pub const AL: f64 = -0.001;
pub const AD: f64 = -0.30;

impl Piano {
    // #region Initialization
    pub fn new(sample_rate: f64) -> Piano {
        let mut result = Piano {
            sample_rate,
            num_spatial_grid_points: 65,
            string_length: 0.62,
            string_mass: 3.93 / 1000.0,
            hammer_mass: 2.97 / 1000.0,
            hammer_stiffness_coefficient: 4.5e9,
            string_tension: 670.0,
            p: 2.5,
            relative_striking_position: 0.12,
            damping_coefficient_a: 0.5,
            damping_coefficient_b: 6.25e-9,
            epsilon: 3.82e-5,
            ..Piano::default()
        };

        result
    }

    /// Initialize the first steps of the simulation.
    fn init_force_out(&mut self, hammer_velocity: f64) -> Vec<f64> {
        let mut result = Vec::new();
        result.resize(INPUT_LENGTH, 0.0);

        // Impact location of the hammer on the string.
        let i0 = (self.relative_striking_position * self.num_spatial_grid_points as f64).round()
            as usize
            - 1;
        let d = 1.0
            + self.damping_coefficient_a / self.sample_rate
            + 2.0 * self.damping_coefficient_b * self.sample_rate;
        let c = (self.string_tension / (self.string_mass / self.string_length)).sqrt();
        let r = c * self.num_spatial_grid_points as f64 / (self.sample_rate * self.string_length);
        let a1 = (2.0 - 2.0 * r * r + self.damping_coefficient_b * self.sample_rate
            - 6.0
                * self.epsilon
                * self.num_spatial_grid_points as f64
                * self.num_spatial_grid_points as f64
                * r
                * r)
            / d;
        let a2 = (-1.0
            + self.damping_coefficient_a / self.sample_rate
            + 2.0 * self.damping_coefficient_b * self.sample_rate)
            / d;
        let a3 = (r
            * r
            * (1.0
                + 4.0
                    * self.epsilon
                    * self.num_spatial_grid_points as f64
                    * self.num_spatial_grid_points as f64))
            / d;
        let a4 = (self.damping_coefficient_b * self.sample_rate
            - self.epsilon
                * self.num_spatial_grid_points as f64
                * self.num_spatial_grid_points as f64
                * r
                * r)
            / d;
        let a5 = (-self.damping_coefficient_b * self.sample_rate) / d;

        // Displacement of the string
        let mut ys = vec![vec![0.0; INPUT_LENGTH]; self.num_spatial_grid_points];
        // Displacement of the hammer
        let mut yh = vec![0.0; INPUT_LENGTH];

        // Step 0 Initial state (all zeros)
        // Step 1
        yh[1] = hammer_velocity / self.sample_rate;
        ys[0][1] = 0.0;
        ys[self.num_spatial_grid_points - 1][1] = 0.0;
        result[1] = self.hammer_stiffness_coefficient * (yh[1] - ys[i0][1]).abs().powf(self.p);

        // Step 2
        ys[0][2] = 0.0;
        ys[self.num_spatial_grid_points - 1][2] = 0.0;
        // Apply hammer. first part of this equation is always 0.0
        ys[i0][2] = ys[i0 + 1][1] + ys[i0 - 1][1] - ys[i0][0]
            + ((1.0 / self.sample_rate).powi(2) * self.num_spatial_grid_points as f64 * result[1])
                / self.string_mass;
        yh[2] =
            2.0 * yh[1] - yh[0] - ((1.0 / self.sample_rate).powi(2) * result[1]) / self.hammer_mass;
        result[2] = self.hammer_stiffness_coefficient * (yh[2] - ys[i0][2]).abs().powf(self.p);

        // All subsequent steps are calculated with previous steps as input.
        for n in 3..INPUT_LENGTH {
            // update string
            ys[0][n] = 0.0;
            ys[self.num_spatial_grid_points - 1][n] = 0.0;
            ys[1][n] = (a1 * ys[1][n - 1])
                + (a2 * ys[1][n - 2])
                + (a3 * (ys[2][n - 1] + ys[0][n - 1]))
                + (a4 * (ys[3][n - 1] - ys[1][n - 1]))
                + (a5 * (ys[2][n - 2] + ys[0][n - 2] + ys[1][n - 3]));
            ys[self.num_spatial_grid_points - 2][n] = (a1
                * ys[self.num_spatial_grid_points - 2][n - 1])
                + (a2 * ys[self.num_spatial_grid_points - 2][n - 2])
                + (a3
                    * (ys[self.num_spatial_grid_points - 1][n - 1]
                        + ys[self.num_spatial_grid_points - 3][n - 1]))
                + (a4
                    * (ys[self.num_spatial_grid_points - 4][n - 1]
                        - ys[self.num_spatial_grid_points - 2][n - 1]))
                + (a5
                    * (ys[self.num_spatial_grid_points - 1][n - 2]
                        + ys[self.num_spatial_grid_points - 3][n - 2]
                        + ys[self.num_spatial_grid_points - 2][n - 3]));

            for m in 2..self.num_spatial_grid_points - 2 {
                ys[m][n] = (a1 * ys[m][n - 1])
                    + (a2 * ys[m][n - 2])
                    + (a3 * (ys[m + 1][n - 1] + ys[m - 1][n - 1]))
                    + (a4 * (ys[m + 2][n - 1] + ys[m - 2][n - 1]))
                    + (a5 * (ys[m + 1][n - 2] + ys[m - 1][n - 2] + ys[m][n - 3]));
            }
            // update string displacement
            // TODO: Use += and only applty the string mass
            ys[i0][n] = (a1 * ys[i0][n - 1])
                + (a2 * ys[i0][n - 2])
                + (a3 * (ys[i0 + 1][n - 1] + ys[i0 - 1][n - 1]))
                + (a4 * (ys[i0 + 2][n - 1] + ys[i0 - 2][n - 1]))
                + (a5 * (ys[i0 + 1][n - 2] + ys[i0 - 1][n - 2] + ys[i0][n - 3]))
                + ((1.0 / self.sample_rate).powi(2)
                    * self.num_spatial_grid_points as f64
                    * result[n - 1])
                    / self.string_mass;

            // update hammer displacement
            yh[n] = 2.0 * yh[n - 1]
                - yh[n - 2]
                - ((1.0 / self.sample_rate).powi(2) * result[n - 1]) / self.hammer_mass;

            // check hammer still touches string
            if (yh[n] - ys[i0][n]) > 0.0 {
                result[n] =
                    self.hammer_stiffness_coefficient * (yh[n] - ys[i0][n]).abs().powf(self.p);
            } else {
                result[n] = 0.0;
            }
        }
        result
    }

    fn init_velocity(&mut self, forces: &[f64]) -> Vec<f64> {
        let r0 = (self.string_tension * self.string_mass / self.string_length).sqrt();
        let velocities = forces
            .iter()
            .map(|force| force / (2.0 * r0))
            .collect::<Vec<f64>>();
        PIANO_IR_SAMPLES.convolve(&velocities)
    }
    // #endregion

    // #region Start Note
    pub fn init_note(&mut self, frequency: f64, hammer_velocity: f64) {
        let config = self.string_configurations.get_configuration(frequency);

        let exact_t = TAU * frequency / self.sample_rate;
        let exact_a = (AD.powi(2) - 1.0) * exact_t.sin();
        let exact_b = 2.0 * AD + (AD.powi(2) + 1.0) * exact_t.cos();
        let n_exact = (TAU + config.ap_num as f64 * (exact_a / exact_b).atan()) / exact_t;
        let m = (n_exact / 2.0).floor() as i32;
        let p = n_exact - 2.0 * m as f64;
        let c = (1.0 - p) / (1.0 + p);
        let i0 = (self.relative_striking_position * m as f64).round() as i32;

        // Init discrete time data. This data is per string and constant for the duration of the note.
        // actual playing will perform a 'filter' for the next sample

        let z = TransferFunction::new(1.0 / self.sample_rate);
        let dl1 = z.pow(-(m - i0));
        let dl2 = z.pow(-i0);

        let hl = config.gl * (1.0 + AL) / &(1.0 + AL * z.pow(-1));
        let hd = (AD + z.pow(-1)) / (1.0 + AD * z.pow(-1));

        let hfd1 = (c + z.pow(-1)) / (1.0 + c * z.pow(-1));
        let hfd2 = (c * (1.0 + config.offtune) + z.pow(-1))
            / (1.0 + c * (1.0 + config.offtune) * z.pow(-1));
        let hfd3 = (c * (1.0 - config.offtune) + z.pow(-1))
            / (1.0 + c * (1.0 - config.offtune) * z.pow(-1));
        let hlhd = &hl * &hd.pow(config.ap_num);
        let h1 = &hlhd * &hfd1;
        let h2 = &hlhd * &hfd2;
        let h3 = &hlhd * &hfd3;

        let dl1dl22 = &dl1 * &dl2 * &dl2;
        let dl12dl22 = &dl1dl22 * &dl1;

        let dw1 = &dl1 / (1.0 + &h1 * &dl12dl22) + -1.0 * &dl1dl22 / (1.0 + &h1 * &dl12dl22);
        let dw2 = &dl1 / (1.0 + &h2 * &dl12dl22) + -1.0 * &dl1dl22 / (1.0 + &h2 * &dl12dl22);
        let dw3 = &dl1 / (1.0 + &h3 * &dl12dl22) + -1.0 * &dl1dl22 / (1.0 + &h3 * &dl12dl22);

        self.note.strings.clear();
        self.note.strings.push(PianoString::from(dw1));
        self.note.strings.push(PianoString::from(dw2));
        self.note.strings.push(PianoString::from(dw3));
        let forces = self.init_force_out(hammer_velocity);
        self.note.input_velocities = self.init_velocity(&forces);
        self.note.sample_index = 0;
    }
    // #endregion

    // #region sample
    pub fn sample(&mut self) -> f64 {
        self.note.sample()
    }
    // #endregion
}

#[test]
fn piano() {
    let mut piano = Piano::new(44100.0);
    piano.init_note(440.0, 4.0);
    let mut max: f64 = 0.0;
    for i in 0..10000 {
        let sample = piano.sample();
        //println!("{i}: {sample}");
        max = max.max(sample.abs());
    }
    println!("max: {max}");
}
