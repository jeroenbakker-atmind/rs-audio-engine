use std::f32::consts::PI;

use crate::string::String;

#[derive(Default, Debug)]
pub struct StringProcessor<'a> {
    pub string: String,
    pub is_being_played: bool,
    pub gain: f32,

    radius: f32,
    density: f32,
    tension: f32,
    area: f32,
    lin_density: f32,
    c: f32,
    young_mod: f32,
    inertia: f32,
    k: f32,
    length: f32,
    excit_position: f32,
    read_position: f32,
    dampning_coeffs: Vec<f32>,

    // String states
    states: Vec<Vec<f32>>,
    state_pointers: Vec<&'a f32>,

    // Bow parameters
    /// Bow Pressure
    pub fb: f32,
    /// Bow Speed
    pub vb: f32,
    a: f32,

    // FDS modal parameters
    oversampling_factor: i32,
    pub timestep: f32,
    modes_number: i32,
    eigen_frequencies: Vec<f32>,
    modes_in: Vec<Vec<f32>>,
    modes_in_current: Vec<&'a f32>,
    modes_in_new: Vec<&'a f32>,

    modes_out: Vec<Vec<f32>>,
    modes_out_current: Vec<&'a f32>,
    modes_out_new: Vec<&'a f32>,

    t11: Vec<i32>,
    t12: Vec<f32>,
    t21: Vec<f32>,
    t22: Vec<i32>,

    schur_comp: Vec<f32>,

    b11: Vec<i32>,
    b12: Vec<f32>,
    b21: Vec<f32>,
    b22: Vec<f32>,

    zeta2: Vec<f32>,
    b1: Vec<f32>,
    b2: Vec<f32>,

    z1: Vec<f32>,
    inv_av1: Vec<f32>,
    inv_av2: Vec<f32>,

    y2: Vec<f32>,
    z2: Vec<f32>,
    inv_ab1: Vec<f32>,
    inv_ab2: Vec<f32>,

    previous_sample: f32,
}

impl<'a> StringProcessor<'a> {
    pub fn new(sample_rate: f32, string: &String) -> StringProcessor<'a> {
        let mut processor = StringProcessor::default();
        processor.oversampling_factor = 1;
        processor.timestep = 1.0 / (sample_rate * processor.oversampling_factor as f32);

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

    pub fn reset_string_states(&mut self) {
        self.is_being_played = false;
        self.states[0].fill(0.0);
        self.states[1].fill(0.0);
        self.previous_sample = 0.0;
    }

    pub fn set_input_position(&mut self, input_position: f32) {
        assert!(input_position >= 0.0);
        assert!(input_position <= 1.0);
        self.excit_position = self.length * input_position;
        self.recompute_in_modes();
    }

    pub fn set_read_position(&mut self, read_position: f32) {
        assert!(read_position >= 0.0);
        assert!(read_position <= 1.0);
        self.read_position = self.length * read_position;
        self.recompute_out_modes();
    }
}

impl<'a> StringProcessor<'a> {
    fn compute_eigen_frequency(&self, mode_number: i32) -> f32 {
        let n = mode_number as f32 * PI / self.length;
        ((self.tension / self.lin_density) * n * n
            + (self.young_mod * self.inertia / self.lin_density) * n * n * n * n)
            .sqrt()
    }

    fn recompute_modes_numbers(&mut self) {
        let mut result = 1;
        const limit_frequency: f32 = 20e3 * 2.0 * PI;
        loop {
            let frequency = self.compute_eigen_frequency(result);
            if frequency > limit_frequency {
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
            .collect::<Vec<f32>>();
    }
}
