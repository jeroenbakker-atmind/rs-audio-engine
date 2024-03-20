//! Port of the Piano model implemented by Lorenzoncina

use std::{f64::consts::TAU, ops::Range};

use audio_engine_common::convolve::Convolution;

use self::piano_ir::PIANO_IR_SAMPLES;
mod piano_ir;

/// Input length of the precalculated forces, before switching to wave guiding.
const INPUT_LENGTH: usize = 150;

#[derive(Debug, Default)]
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
    /// Initial hammer velocity
    pub hammer_velocity_in: f64,

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

    /// Force signal output Contains INPUT_LENGTH number of items.
    pub f_out: Vec<f64>,
    /// Input velocities
    /// NOTE: v only contains valid samples, in original paper it would be extended with 0 to fit the number of samples that will be generated.
    pub v: Vec<f64>,

    pub string_configurations: StringGroupConfigurations,
    pub note: PianoNote,
}

#[derive(Debug)]
pub struct StringGroupConfigurations {
    pub configurations: Vec<(FrequencyRange, StringGroupConfiguration)>,
}

pub type FrequencyRange = Range<f64>;

#[derive(Debug, Default, Copy, Clone)]
pub struct StringGroupConfiguration {
    pub detune: f64,
    pub ap_num: f64,
    pub gl: f64,
}

impl Default for StringGroupConfigurations {
    fn default() -> Self {
        Self {
            configurations: vec![
                (
                    0.0..120.0,
                    StringGroupConfiguration {
                        detune: 0.25,
                        ap_num: 20.0,
                        gl: -0.96,
                    },
                ),
                (
                    120.0..150.0,
                    StringGroupConfiguration {
                        detune: 0.18,
                        ap_num: 20.0,
                        gl: -0.968,
                    },
                ),
                (
                    150.0..200.0,
                    StringGroupConfiguration {
                        detune: 0.13,
                        ap_num: 18.0,
                        gl: -0.975,
                    },
                ),
                (
                    200.0..261.626,
                    StringGroupConfiguration {
                        detune: 0.09,
                        ap_num: 16.0,
                        gl: -0.98,
                    },
                ),
                (
                    261.626..390.0,
                    StringGroupConfiguration {
                        detune: 0.06,
                        ap_num: 14.0,
                        gl: -0.985,
                    },
                ),
                (
                    390.0..750.0,
                    StringGroupConfiguration {
                        detune: 0.04,
                        ap_num: 12.0,
                        gl: -0.99,
                    },
                ),
                (
                    750.0..980.0,
                    StringGroupConfiguration {
                        detune: 0.03,
                        ap_num: 8.0,
                        gl: -0.993,
                    },
                ),
                (
                    980.0..1500.0,
                    StringGroupConfiguration {
                        detune: 0.02,
                        ap_num: 6.0,
                        gl: -0.995,
                    },
                ),
                (
                    1500.0..1800.0,
                    StringGroupConfiguration {
                        detune: 0.01,
                        ap_num: 4.0,
                        gl: -0.995,
                    },
                ),
                (
                    1800.0..1900.0,
                    StringGroupConfiguration {
                        detune: 0.005,
                        ap_num: 3.0,
                        gl: -0.977,
                    },
                ),
                (
                    1900.0..3000.0,
                    StringGroupConfiguration {
                        detune: 0.005,
                        ap_num: 2.0,
                        gl: -0.977,
                    },
                ),
                (
                    3000.0..f64::MAX,
                    StringGroupConfiguration {
                        detune: 0.01,
                        ap_num: 0.0,
                        gl: -0.977,
                    },
                ),
            ],
        }
    }
}

impl StringGroupConfigurations {
    pub fn get_configuration(&self, frequency: f64) -> &'_ StringGroupConfiguration {
        for (range, config) in &self.configurations {
            if range.contains(&frequency) {
                return config;
            }
        }
        unreachable!()
    }
}

pub const AL: f64 = -0.001;
pub const AD: f64 = -0.30;

#[derive(Debug, Default)]
pub struct PianoNote {
    /// Frequency
    pub f0: f64,
    pub config: StringGroupConfiguration,
    pub c: f64,
    pub i0: f64,
    pub m: f64,
}

impl Piano {
    // #region Initialization
    pub fn new() -> Piano {
        let mut result = Piano {
            fs: 44100.0,
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
            hammer_velocity_in: 4.0,
            ..Piano::default()
        };
        result.init();
        result.init_wave_equation();
        result.init_force_out();
        result.init_velocity();

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
    fn init_force_out(&mut self) {
        // Displacement of the string
        let mut ys = vec![vec![0.0; INPUT_LENGTH]; self.n];
        // Displacement of the hammer
        let mut yh = vec![0.0; INPUT_LENGTH];
        self.f_out.resize(INPUT_LENGTH, 0.0);

        // Step 0 Initial state (all zeros)
        // Step 1
        yh[1] = self.hammer_velocity_in / self.fs;
        ys[0][1] = 0.0;
        ys[self.n - 1][1] = 0.0;
        self.f_out[1] = self.k * (yh[1] - ys[self.i0][1]).abs().powf(self.p);

        // Step 2
        ys[0][2] = 0.0;
        ys[self.n - 1][2] = 0.0;
        // Apply hammer. first part of this equation is always 0.0
        ys[self.i0][2] = ys[self.i0 + 1][1] + ys[self.i0 - 1][1] - ys[self.i0][0]
            + ((1.0 / self.fs).powi(2) * self.n as f64 * self.f_out[1]) / self.mass_string;
        yh[2] = 2.0 * yh[1] - yh[0] - ((1.0 / self.fs).powi(2) * self.f_out[1]) / self.mass_hammer;
        self.f_out[2] = self.k * (yh[2] - ys[self.i0][2]).abs().powf(self.p);

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
                + ((1.0 / self.fs).powi(2) * self.n as f64 * self.f_out[n - 1]) / self.mass_string;

            // update hammer displacement
            yh[n] = 2.0 * yh[n - 1]
                - yh[n - 2]
                - ((1.0 / self.fs).powi(2) * self.f_out[n - 1]) / self.mass_hammer;

            // check hammer still touches string
            if (yh[n] - ys[self.i0][n]) > 0.0 {
                self.f_out[n] = self.k * (yh[n] - ys[self.i0][n]).abs().powf(self.p);
            } else {
                self.f_out[n] = 0.0;
            }
        }
    }
    fn init_velocity(&mut self) {
        let velocities = self
            .f_out
            .iter()
            .map(|force| force / (2.0 * self.r0))
            .collect::<Vec<f64>>();
        self.v = PIANO_IR_SAMPLES.convolve(&velocities);
    }
    // #endregion

    // #region Start Note
    pub fn init_note(&mut self, frequency: f64) {
        let note = &mut self.note;
        note.f0 = frequency;
        note.config = *self.string_configurations.get_configuration(frequency);

        let exact_t = TAU * frequency / self.fs;
        let exact_a = (AD.powi(2) - 1.0) * exact_t.sin();
        let exact_b = 2.0 * AD + (AD.powi(2) + 1.0) * exact_t.cos();
        let n_exact = (TAU + note.config.ap_num * (exact_a / exact_b).atan()) / exact_t;
        let m = (n_exact / 2.0).floor();
        let p = n_exact - 2.0 * m;
        let c = (1.0 - p) / (1.0 + p);
        let i0 = (self.alpha * m).round();
        note.c = c;
        note.i0 = i0;
        note.m = m;
        println!("{:#?}", note);
        // Init discrete time data. This data is per string and constant for the duration of the note.
        // actual playing will perform a 'filter' for the next sample
        // Will require some discrete time data structures to add, multiply, set the denomater/number of the equation.
        // We can 'optimize' the data structure so all zero data elements are removed.
        // We should implement this in its own library. (audio-engine-discrete-time)
    }
    // #endregion
}

#[test]
fn piano() {
    let mut piano = Piano::new();
    piano.init_note(440.0);
}
