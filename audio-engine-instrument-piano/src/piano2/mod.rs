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
    pub fs: f64,
    /// Number of spatial grid points
    /// TODO: Make a constant
    pub n: usize,
    /// Length of the piano wire
    pub length: f64,
    /// Mass of the piano wire in Kg
    pub mass_string: f64,
    /// Mass of the hammer in Kg
    pub mass_hammer: f64,
    /// Hammer stiffness coefficient
    pub k: f64,
    ///Tension of the wire
    pub tension: f64,
    /// Stiffness non-linear coefficient
    pub p: f64,
    /// Relativa striking position
    pub alpha: f64,
    /// Damping coefficient
    pub b1: f64,
    /// Damping coefficient
    pub b3: f64,
    /// String stiffness parameter
    pub epsilon: f64,

    // cached values could be extracted into functions.
    pub r0: f64,
    pub i0: usize,
    pub c: f64,

    // Coefficient of the wave equation
    pub d: f64,
    pub r: f64,
    pub a1: f64,
    pub a2: f64,
    pub a3: f64,
    pub a4: f64,
    pub a5: f64,

    /// Input velocities
    /// NOTE: v only contains valid samples, in original paper it would be extended with 0 to fit the number of samples that will be generated.
    pub v: Vec<f64>,

    pub string_configurations: StringGroupConfigurations,
    pub note: PianoNote,
}

pub const AL: f64 = -0.001;
pub const AD: f64 = -0.30;

impl Piano {
    // #region Initialization
    pub fn new(sample_rate: f64) -> Piano {
        let mut result = Piano {
            fs: sample_rate,
            n: 65,
            length: 0.62,
            mass_string: 3.93 / 1000.0,
            mass_hammer: 2.97 / 1000.0,
            k: 4.5e9,
            tension: 670.0,
            p: 2.5,
            alpha: 0.12,
            b1: 0.5,
            b3: 6.25e-9,
            epsilon: 3.82e-5,
            ..Piano::default()
        };
        result.init();
        result.init_wave_equation();

        result
    }

    fn init(&mut self) {
        self.r0 = (self.tension * self.mass_string / self.length).sqrt();
        self.i0 = (self.alpha * self.n as f64).round() as usize - 1;
        self.c = (self.tension / (self.mass_string / self.length)).sqrt();
    }

    fn init_wave_equation(&mut self) {
        self.d = 1.0 + self.b1 / self.fs + 2.0 * self.b3 * self.fs;
        self.r = self.c * self.n as f64 / (self.fs * self.length);
        self.a1 = (2.0 - 2.0 * self.r * self.r + self.b3 * self.fs
            - 6.0 * self.epsilon * self.n as f64 * self.n as f64 * self.r * self.r)
            / self.d;
        self.a2 = (-1.0 + self.b1 / self.fs + 2.0 * self.b3 * self.fs) / self.d;
        self.a3 =
            (self.r * self.r * (1.0 + 4.0 * self.epsilon * self.n as f64 * self.n as f64)) / self.d;
        self.a4 = (self.b3 * self.fs
            - self.epsilon * self.n as f64 * self.n as f64 * self.r * self.r)
            / self.d;
        self.a5 = (-self.b3 * self.fs) / self.d;
    }

    /// Initialize the first steps of the simulation.
    fn init_force_out(&mut self, hammer_velocity: f64) -> Vec<f64> {
        let mut result = Vec::new();
        result.resize(INPUT_LENGTH, 0.0);

        // Displacement of the string
        let mut ys = vec![vec![0.0; INPUT_LENGTH]; self.n];
        // Displacement of the hammer
        let mut yh = vec![0.0; INPUT_LENGTH];

        // Step 0 Initial state (all zeros)
        // Step 1
        yh[1] = hammer_velocity / self.fs;
        ys[0][1] = 0.0;
        ys[self.n - 1][1] = 0.0;
        result[1] = self.k * (yh[1] - ys[self.i0][1]).abs().powf(self.p);

        // Step 2
        ys[0][2] = 0.0;
        ys[self.n - 1][2] = 0.0;
        // Apply hammer. first part of this equation is always 0.0
        ys[self.i0][2] = ys[self.i0 + 1][1] + ys[self.i0 - 1][1] - ys[self.i0][0]
            + ((1.0 / self.fs).powi(2) * self.n as f64 * result[1]) / self.mass_string;
        yh[2] = 2.0 * yh[1] - yh[0] - ((1.0 / self.fs).powi(2) * result[1]) / self.mass_hammer;
        result[2] = self.k * (yh[2] - ys[self.i0][2]).abs().powf(self.p);

        for n in 3..INPUT_LENGTH {
            // update string
            ys[0][n] = 0.0;
            ys[self.n - 1][n] = 0.0;
            ys[1][n] = (self.a1 * ys[1][n - 1])
                + (self.a2 * ys[1][n - 2])
                + (self.a3 * (ys[2][n - 1] + ys[0][n - 1]))
                + (self.a4 * (ys[3][n - 1] - ys[1][n - 1]))
                + (self.a5 * (ys[2][n - 2] + ys[0][n - 2] + ys[1][n - 3]));
            ys[self.n - 2][n] = (self.a1 * ys[self.n - 2][n - 1])
                + (self.a2 * ys[self.n - 2][n - 2])
                + (self.a3 * (ys[self.n - 1][n - 1] + ys[self.n - 3][n - 1]))
                + (self.a4 * (ys[self.n - 4][n - 1] - ys[self.n - 2][n - 1]))
                + (self.a5
                    * (ys[self.n - 1][n - 2] + ys[self.n - 3][n - 2] + ys[self.n - 2][n - 3]));

            for m in 2..self.n - 2 {
                ys[m][n] = (self.a1 * ys[m][n - 1])
                    + (self.a2 * ys[m][n - 2])
                    + (self.a3 * (ys[m + 1][n - 1] + ys[m - 1][n - 1]))
                    + (self.a4 * (ys[m + 2][n - 1] + ys[m - 2][n - 1]))
                    + (self.a5 * (ys[m + 1][n - 2] + ys[m - 1][n - 2] + ys[m][n - 3]));
            }
            // update string displacement
            // TODO: Use += and only applty the string mass
            ys[self.i0][n] = (self.a1 * ys[self.i0][n - 1])
                + (self.a2 * ys[self.i0][n - 2])
                + (self.a3 * (ys[self.i0 + 1][n - 1] + ys[self.i0 - 1][n - 1]))
                + (self.a4 * (ys[self.i0 + 2][n - 1] + ys[self.i0 - 2][n - 1]))
                + (self.a5
                    * (ys[self.i0 + 1][n - 2] + ys[self.i0 - 1][n - 2] + ys[self.i0][n - 3]))
                + ((1.0 / self.fs).powi(2) * self.n as f64 * result[n - 1]) / self.mass_string;

            // update hammer displacement
            yh[n] = 2.0 * yh[n - 1]
                - yh[n - 2]
                - ((1.0 / self.fs).powi(2) * result[n - 1]) / self.mass_hammer;

            // check hammer still touches string
            if (yh[n] - ys[self.i0][n]) > 0.0 {
                result[n] = self.k * (yh[n] - ys[self.i0][n]).abs().powf(self.p);
            } else {
                result[n] = 0.0;
            }
        }
        result
    }

    fn init_velocity(&mut self, forces: &[f64]) {
        let velocities = forces
            .iter()
            .map(|force| force / (2.0 * self.r0))
            .collect::<Vec<f64>>();
        self.v = PIANO_IR_SAMPLES.convolve(&velocities)
    }
    // #endregion

    // #region Start Note
    pub fn init_note(&mut self, frequency: f64, hammer_velocity: f64) {
        let forces = self.init_force_out(hammer_velocity);
        self.init_velocity(&forces);

        let config = self.string_configurations.get_configuration(frequency);

        let exact_t = TAU * frequency / self.fs;
        let exact_a = (AD.powi(2) - 1.0) * exact_t.sin();
        let exact_b = 2.0 * AD + (AD.powi(2) + 1.0) * exact_t.cos();
        let n_exact = (TAU + config.ap_num as f64 * (exact_a / exact_b).atan()) / exact_t;
        let m = (n_exact / 2.0).floor() as i32;
        let p = n_exact - 2.0 * m as f64;
        let c = (1.0 - p) / (1.0 + p);
        let i0 = (self.alpha * m as f64).round() as i32;

        // Init discrete time data. This data is per string and constant for the duration of the note.
        // actual playing will perform a 'filter' for the next sample
        // Will require some discrete time data structures to add, multiply, set the denomater/number of the equation.
        // We can 'optimize' the data structure so all zero data elements are removed.
        // We should implement this in its own library. (audio-engine-discrete-time)

        let z = TransferFunction::new(1.0 / self.fs);
        let dl1 = z.pow(-(m - i0));
        let dl2 = z.pow(-i0);

        let hl = config.gl * (1.0 + AL) / &(1.0 + AL * z.pow(-1));
        let hd = (AD + z.pow(-1)) / (1.0 + AD * z.pow(-1));

        let hfd1 = (c + z.pow(-1)) / (1.0 + c * z.pow(-1));
        let hfd2 =
            (c * (1.0 + config.detune) + z.pow(-1)) / (1.0 + c * (1.0 + config.detune) * z.pow(-1));
        let hfd3 =
            (c * (1.0 - config.detune) + z.pow(-1)) / (1.0 + c * (1.0 - config.detune) * z.pow(-1));
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
        self.note.sample_index = 0;
    }
    // #endregion

    // #region sample
    pub fn sample(&mut self) -> f64 {
        let sample_in = if self.note.sample_index < self.v.len() {
            self.v[self.note.sample_index]
        } else {
            0.0
        };
        let sample_out = self.note.filter(sample_in);
        self.note.sample_index += 1;
        // TODO: 2000 is based on experiments...
        sample_out / 2000.0
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
