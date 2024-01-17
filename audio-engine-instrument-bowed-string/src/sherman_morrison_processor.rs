use std::f32::consts::{PI, TAU};

use crate::{
    bow::Bow,
    damping::DampingCoeffcient,
    eigen_frequencies::{self, EigenFrequency},
    hand::Hand,
    processor::StringProcessor,
    string::String,
    string_and_hand::StringAndHand,
};
#[derive(Default, Debug)]
pub struct ShermanMorrison {
    string_and_hand: StringAndHand,
    pub bow: Bow,
    sample_rate: f32,

    eigen_frequencies: Vec<f32>,
    damping_profile: Vec<f32>,

    excit_position: f32,
    output_position_left: f32,
    output_position_right: f32,

    modes_in: Vec<f32>,
    modes_out_left: Vec<f32>,
    modes_out_right: Vec<f32>,

    states: Vec<f32>,

    a11: f32,
    a12: f32,
    a21: Vec<f32>,
    a22: f32,

    schur_comp: Vec<f32>,

    b11: f32,
    b12: f32,
    b21: Vec<f32>,
    b22: Vec<f32>,
}

impl StringProcessor for ShermanMorrison {
    fn new(sample_rate: f32, string: &crate::string::String) -> Self {
        let mut processor = ShermanMorrison::default();
        processor.string_and_hand.string = string.clone();
        processor.excit_position = processor.string_and_hand.upper_l() * 0.733;
        processor.output_position_left = processor.string_and_hand.upper_l() * 0.53;
        processor.output_position_right = processor.string_and_hand.upper_l() * 0.77;
        processor.sample_rate = sample_rate;
        processor.initialize();

        processor
    }

    fn reset_string_states(&mut self) {}
    fn set_input_position(&mut self, input_position: f32) {}
    fn set_read_position(&mut self, read_position: f32) {}
    fn compute_state(&mut self) {}
    fn read_output(&mut self) -> f32 {
        0.0
    }
}

impl ShermanMorrison {
    fn initialize(&mut self) {
        self.initialize_eigen_frequencies();
        self.initialize_modes();
        self.initialize_damping();
        self.initialize_matrices();
        self.initialize_states();
    }
    fn initialize_eigen_frequencies(&mut self) {
        let mut mode = 1;
        let mut eigen_frequencies = Vec::with_capacity(200);

        while *eigen_frequencies.last().unwrap_or(&0.0) < 20e3 * TAU {
            eigen_frequencies.push(self.string_and_hand.calc_eigen_frequency(mode));
            mode += 1;
        }

        eigen_frequencies.shrink_to_fit();
        self.eigen_frequencies = eigen_frequencies;
    }

    fn initialize_modes(&mut self) {
        let mode_len = self.mode_len();
        fn calc_mode(string_and_hand: &StringAndHand, position: f32, mode: usize) -> f32 {
            (2.0 / string_and_hand.length()).sqrt()
                * (mode as f32 * PI * position / string_and_hand.length()).sin()
        }
        self.modes_in = (1..=mode_len)
            .map(|mode| calc_mode(&self.string_and_hand, self.excit_position, mode))
            .collect::<Vec<f32>>();
        self.modes_out_left = (1..=mode_len)
            .map(|mode| calc_mode(&self.string_and_hand, self.output_position_left, mode))
            .collect::<Vec<f32>>();
        self.modes_out_right = (1..=mode_len)
            .map(|mode| calc_mode(&self.string_and_hand, self.output_position_right, mode))
            .collect::<Vec<f32>>();
    }

    fn initialize_matrices(&mut self) {
        let mode_len = self.mode_len();

        self.a11 = 1.0;
        self.a12 = -0.5 * self.k();
        self.a21 = self
            .eigen_frequencies
            .iter()
            .map(|eigen_frequency| -0.5 * self.k() * (-eigen_frequency * eigen_frequency))
            .collect::<Vec<f32>>();
        self.a22 = 1.0;
        /*  self
        .damping_profile
        .iter()
        .map(|damping_coeefcient| 1.0 + 0.5 * self.k() * damping_coeefcient)
        .collect::<Vec<f32>>();*/
        // TODO a22 should be 1.0 ?

        // Optimize 1 - a21 *-0.5k
        self.schur_comp = (0..mode_len)
            .map(|mode| self.a22 - self.a21[mode] * self.a11 * self.a12)
            .collect::<Vec<f32>>();

        // Should still be checked with reference implementation.
        self.b11 = 1.0;
        self.b12 = 0.5 * self.k();
        self.b21 = self
            .eigen_frequencies
            .iter()
            .map(|eigen_frequency| 0.5 * self.k() * (-eigen_frequency * eigen_frequency))
            .collect::<Vec<f32>>();
        self.b22 = self
            .damping_profile
            .iter()
            .map(|damping_coeefcient| 1.0 - 0.5 * self.k() * damping_coeefcient)
            .collect::<Vec<f32>>();
    }
    fn initialize_states(&mut self) {
        self.states = vec![0.0; self.mode_len() * 2];
    }

    fn initialize_damping(&mut self) {
        self.damping_profile = self
            .string_and_hand
            .string
            .damping_coeffcients(&self.eigen_frequencies);
    }

    /// Get the number of modes in play.
    ///
    /// Only valid after the eigen frequencies have been initialized.
    fn mode_len(&self) -> usize {
        self.eigen_frequencies.len()
    }

    // TODO rename to sample_duration
    fn k(&self) -> f32 {
        1.0 / self.sample_rate
    }
}
