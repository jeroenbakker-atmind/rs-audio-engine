use std::{f64::consts::PI, mem::swap};

use crate::{processor::StringProcessor, string::String};

#[derive(Default, Debug)]
pub struct NemusProcessor {
    pub string: String,
    pub is_being_played: bool,
    pub gain: f64,

    // TODO: Don't store a copy of parameters if they don't change.
    radius: f64,
    density: f64,
    tension: f64,
    area: f64,
    lin_density: f64,
    c: f64,
    young_mod: f64,
    inertia: f64,
    k: f64,
    length: f64,
    excit_position: f64,
    read_position: f64,
    damping_coeffs: Vec<f64>,

    // String states
    states: Vec<Vec<f64>>,
    state_current: usize,
    state_new: usize,

    // Bow parameters
    // TODO: extract to own struct.
    /// Bow Pressure
    pub fb: f64,
    /// Bow Speed
    pub vb: f64,
    a: f64,

    // FDS modal parameters
    // TODO: Remove
    oversampling_factor: i32,
    pub timestep: f64,
    modes_number: i32,
    eigen_frequencies: Vec<f64>,
    // TODO: Extract to own struct
    modes_in: Vec<Vec<f64>>,
    modes_in_current: usize,
    modes_in_new: usize,

    modes_out: Vec<Vec<f64>>,
    modes_out_current: usize,
    modes_out_new: usize,

    t11: Vec<f64>,
    t12: Vec<f64>,
    t21: Vec<f64>,
    t22: Vec<f64>,

    schur_comp: Vec<f64>,

    b11: Vec<f64>,
    b12: Vec<f64>,
    b21: Vec<f64>,
    b22: Vec<f64>,

    zeta2: Vec<f64>,
    b1: Vec<f64>,
    b2: Vec<f64>,

    z1: Vec<f64>,
    inv_av1: Vec<f64>,
    inv_av2: Vec<f64>,

    y2: Vec<f64>,
    z2: Vec<f64>,
    inv_ab1: Vec<f64>,
    inv_ab2: Vec<f64>,

    previous_sample: f64,
}

impl StringProcessor for NemusProcessor {
    fn new(sample_rate: f64, string: &String) -> NemusProcessor {
        let mut processor = NemusProcessor::default();
        processor.oversampling_factor = 1;
        processor.timestep = 1.0 / (sample_rate * processor.oversampling_factor as f64);

        processor.string = string.clone();
        processor.radius = string.radius;
        processor.density = string.density;
        processor.tension = string.tension;
        processor.length = string.length;
        processor.young_mod = string.young_mod;

        processor.area = PI * processor.radius * processor.radius;
        processor.lin_density = processor.density * processor.area;
        processor.inertia =
            PI * processor.radius * processor.radius * processor.radius * processor.radius / 4.0;
        processor.k = ((processor.young_mod * processor.inertia)
            / (processor.lin_density
                * processor.length
                * processor.length
                * processor.length
                * processor.length))
            .sqrt();
        processor.c = (processor.tension / processor.lin_density).sqrt();

        processor.a = 100.0;

        processor.recompute_modes_numbers();
        processor.recompute_eigen_frequencies();
        processor.initialize_in_modes();
        processor.initialize_out_modes();
        processor.recompute_damping_profile();

        processor.initialize_states();
        processor.reset_matrices();

        processor
    }

    fn reset_string_states(&mut self) {
        self.is_being_played = false;
        self.states[0].fill(0.0);
        self.states[1].fill(0.0);
        self.previous_sample = 0.0;
    }

    fn set_input_position(&mut self, input_position: f64) {
        assert!(input_position >= 0.0);
        assert!(input_position <= 1.0);
        self.excit_position = self.length * input_position;
        self.recompute_in_modes();
    }

    fn set_read_position(&mut self, read_position: f64) {
        assert!(read_position >= 0.0);
        assert!(read_position <= 1.0);
        self.read_position = self.length * read_position;
        self.recompute_out_modes();
    }

    fn compute_state(&mut self) {
        if !self.is_being_played {
            return;
        }

        let zeta_1 = (0..self.modes_number as usize)
            .map(|mode_number| {
                self.modes_in[self.modes_in_current][mode_number]
                    * self.states[self.state_current][mode_number + self.modes_number as usize]
            })
            .sum::<f64>();
        let eta = zeta_1 - self.vb;
        let d = (2.0 * self.a).sqrt() * (-self.a * eta * eta + 0.5).exp();
        let lambda = d * (1.0 - 2.0 * self.a * eta * eta);

        let mut vt1 = 0.0;
        let mut vt2 = 0.0;

        (0..self.modes_number as usize).for_each(|mode_number| {
            let zeta_2 = self.modes_in[self.modes_in_current][mode_number] * zeta_1;
            let b1 = self.b11[mode_number] * self.states[self.state_current][mode_number]
                + self.b12[mode_number]
                    * self.states[self.state_current][mode_number + self.modes_number as usize];
            let b2 = self.b21[mode_number] * self.states[self.state_current][mode_number]
                + self.b22[mode_number]
                    * self.states[self.state_current][mode_number + self.modes_number as usize]
                + zeta_2 * 0.5 * self.timestep * self.fb * (lambda - 2.0 * d)
                + self.timestep
                    * self.fb
                    * d
                    * self.modes_in[self.modes_in_current][mode_number]
                    * self.vb;
            let z1 = 0.5
                * self.timestep
                * self.fb
                * lambda
                * self.modes_in[self.modes_in_current][mode_number];
            self.inv_av2[mode_number] = 1.0 / self.schur_comp[mode_number] * z1;
            self.inv_av1[mode_number] =
                -self.t11[mode_number] * self.t12[mode_number] * self.inv_av2[mode_number];

            let y2 = self.t11[mode_number] * b1;
            let z2 = b2 - self.t21[mode_number] * y2;
            self.inv_ab2[mode_number] = 1.0 / self.schur_comp[mode_number] * z2;
            self.inv_ab1[mode_number] =
                y2 - self.t11[mode_number] * self.t12[mode_number] * self.inv_ab2[mode_number];

            vt1 += self.modes_in[self.modes_in_current][mode_number] * self.inv_av2[mode_number];
            vt2 += self.modes_in[self.modes_in_current][mode_number] * self.inv_ab2[mode_number];
        });

        let coeff = 1.0 / (1.0 + vt1);

        (0..self.modes_number as usize).for_each(|mode_number| {
            self.states[self.state_new][mode_number] =
                self.inv_ab1[mode_number] - coeff * self.inv_av1[mode_number] * vt2;
            self.states[self.state_new][mode_number + self.modes_number as usize] =
                self.inv_ab2[mode_number] - coeff * self.inv_av2[mode_number] * vt2;
        });

        swap(&mut self.state_current, &mut self.state_new);
    }
    fn read_output(&mut self) -> f64 {
        let result = if self.is_being_played {
            (0..self.modes_number as usize)
                .map(|mode_number| {
                    self.modes_in[self.modes_in_current][mode_number]
                        * self.states[self.state_current][mode_number]
                })
                .sum::<f64>()
        } else {
            0.0
        };

        let diff = self.gain * (result - self.previous_sample) / self.timestep;
        self.previous_sample = result;
        diff
    }
}

impl NemusProcessor {
    fn compute_eigen_frequency(&self, mode_number: i32) -> f64 {
        let n = mode_number as f64 * PI / self.length;
        ((self.tension / self.lin_density) * n * n
            + (self.young_mod * self.inertia / self.lin_density) * n * n * n * n)
            .sqrt()
    }

    fn recompute_modes_numbers(&mut self) {
        let mut result = 1;
        const LIMIT_FREQUENCY: f64 = 20e3 * 2.0 * PI;
        loop {
            let frequency = self.compute_eigen_frequency(result);
            if frequency > LIMIT_FREQUENCY {
                result -= 1;
                break;
            }
            result += 1;
        }

        self.modes_number = result;
    }

    fn recompute_eigen_frequencies(&mut self) {
        self.eigen_frequencies = (0..self.modes_number)
            .map(|mode_number| self.compute_eigen_frequency(mode_number + 1))
            .collect::<Vec<f64>>();
    }

    fn compute_damping_coefficients(&self, frequency: f64) -> f64 {
        let rho_air = 1.225;
        let mu_air = 1.619e-5;
        let d0 = -2.0 * rho_air * mu_air / (self.density * self.radius * self.radius);
        let d1 = -2.0 * rho_air * (2.0 * mu_air).sqrt() / (self.density * self.radius);
        let d2 = -1.0 / 18000.0;
        let d3 = (-0.003
            * self.young_mod
            * self.density
            * PI
            * PI
            * self.radius
            * self.radius
            * self.radius
            * self.radius
            * self.radius
            * self.radius)
            / (4.0 * self.tension * self.tension);
        d0 + d1 * frequency.sqrt() + d2 * frequency + d3 * frequency * frequency * frequency
    }

    fn initialize_in_modes(&mut self) {
        self.modes_in = vec![vec![0.0; self.modes_number as usize]; 2];
        self.modes_in_current = 0;
        self.modes_in_new = 1;
        self.recompute_in_modes();
    }
    fn recompute_in_modes(&mut self) {
        (0..self.modes_number as usize).for_each(|mode_number| {
            self.modes_in[self.modes_in_new][mode_number] =
                self.compute_mode(self.excit_position, mode_number + 1);
        });

        swap(&mut self.modes_in_current, &mut self.modes_in_new);
    }
    fn compute_mode(&self, position: f64, mode_number: usize) -> f64 {
        (2.0 / self.length).sqrt() * (mode_number as f64 * PI * position / self.length).sin()
    }
    fn initialize_out_modes(&mut self) {
        self.modes_out = vec![vec![0.0; self.modes_number as usize]; 2];
        self.modes_out_current = 0;
        self.modes_out_new = 1;
        self.recompute_out_modes();
    }
    fn recompute_out_modes(&mut self) {
        (0..self.modes_number as usize).for_each(|mode_number| {
            self.modes_out[self.modes_out_new][mode_number] =
                self.compute_mode(self.read_position, mode_number + 1);
        });

        swap(&mut self.modes_out_current, &mut self.modes_out_new);
    }
    fn recompute_damping_profile(&mut self) {
        self.damping_coeffs = (0..self.modes_number)
            .map(|mode_number| {
                let frequency = self.eigen_frequencies[mode_number as usize];
                -self.compute_damping_coefficients(frequency)
            })
            .collect::<Vec<f64>>();
    }
    fn initialize_states(&mut self) {
        self.states
            .resize(2, vec![0.0; self.modes_number as usize * 2]);
        self.state_current = 0;
        self.state_new = 1;
    }
    fn reset_matrices(&mut self) {
        self.t11.resize(self.modes_number as usize, 0.0);
        self.t12.resize(self.modes_number as usize, 0.0);
        self.t21.resize(self.modes_number as usize, 0.0);
        self.t22.resize(self.modes_number as usize, 0.0);
        self.schur_comp.resize(self.modes_number as usize, 0.0);

        self.b11.resize(self.modes_number as usize, 0.0);
        self.b12.resize(self.modes_number as usize, 0.0);
        self.b21.resize(self.modes_number as usize, 0.0);
        self.b22.resize(self.modes_number as usize, 0.0);

        self.zeta2.resize(self.modes_number as usize, 0.0);
        self.b1.resize(self.modes_number as usize, 0.0);
        self.b2.resize(self.modes_number as usize, 0.0);

        self.z1.resize(self.modes_number as usize, 0.0);
        self.inv_av1.resize(self.modes_number as usize, 0.0);
        self.inv_av2.resize(self.modes_number as usize, 0.0);

        self.y2.resize(self.modes_number as usize, 0.0);
        self.z2.resize(self.modes_number as usize, 0.0);
        self.inv_ab1.resize(self.modes_number as usize, 0.0);
        self.inv_ab2.resize(self.modes_number as usize, 0.0);

        (0..self.modes_number as usize).for_each(|mode_number| {
            self.t11[mode_number] = 1.0;
            self.t12[mode_number] = 0.5 * self.timestep;
            self.t21[mode_number] = 0.5
                * self.timestep
                * (-self.eigen_frequencies[mode_number] * self.eigen_frequencies[mode_number]);
            self.t22[mode_number] = 1.0 + 0.5 * self.timestep * self.damping_coeffs[mode_number];

            self.schur_comp[mode_number] = self.t22[mode_number]
                - self.t21[mode_number] * (self.t11[mode_number] * self.t12[mode_number]);

            self.b11[mode_number] = 1.0;
            self.b12[mode_number] = 0.5 * self.timestep;
            self.b21[mode_number] = 0.5
                * self.timestep
                * (-self.eigen_frequencies[mode_number] * self.eigen_frequencies[mode_number]);
            self.b22[mode_number] = 1.0 - 0.5 * self.timestep * self.damping_coeffs[mode_number];
        });
    }
}
