use std::f32::consts::{PI, TAU};

use crate::{
    bow::{Bow, BOW_FREE_PARAMETER},
    damping::DampingCoeffcient,
    eigen_frequencies::EigenFrequency,
    processor::StringProcessor,
    string_and_hand::StringAndHand,
};
// friction can become a trait.

#[derive(Default, Debug)]
pub struct ShermanMorrison {
    string_and_hand: StringAndHand,
    pub bow: Bow,
    pub gain: f32,
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

    shur_comp: Vec<f32>,

    b11: f32,
    b12: f32,
    b21: Vec<f32>,
    b22: f32,

    // scratch space
    inv_av1: Vec<f32>,
    inv_av2: Vec<f32>,
    inv_ab1: Vec<f32>,
    inv_ab2: Vec<f32>,
}

impl StringProcessor for ShermanMorrison {
    fn new(sample_rate: f32, string: &crate::string::String) -> Self {
        let mut processor = ShermanMorrison::default();
        processor.string_and_hand.string = string.clone();
        processor.excit_position = processor.string_and_hand.upper_l() * 0.733;
        processor.output_position_left = processor.string_and_hand.upper_l() * 0.53;
        processor.output_position_right = processor.string_and_hand.upper_l() * 0.77;
        processor.sample_rate = sample_rate;
        processor.gain = 1000.0;
        processor.initialize();

        processor
    }

    fn reset_string_states(&mut self) {}
    fn set_input_position(&mut self, input_position: f32) {}
    fn set_read_position(&mut self, read_position: f32) {}
    fn compute_state(&mut self) {
        // TODO: check current state with previous state
        // reset matrices and frequencies based on the actual change.
        // This will allow physical accurate reverbs by fast moving your finger.
    }
    fn read_output(&mut self) -> f32 {
        let mode_len = self.mode_len();
        let zeta1 = (0..mode_len)
            .map(|mode| self.modes_in[mode] * self.states[mode + mode_len])
            .sum::<f32>();

        // Bilbao friction
        let eta = zeta1 - self.bow.velocity;
        let d = (2.0 * BOW_FREE_PARAMETER).sqrt() * (-BOW_FREE_PARAMETER * eta * eta + 0.5).exp();
        let lambda = d * (1.0 - 2.0 * BOW_FREE_PARAMETER * eta.powi(2));

        let mut v1 = 0.0;
        let mut v2 = 0.0;

        for mode in 0..mode_len {
            let zeta2 = self.modes_in[mode] * zeta1;
            let b1 = self.b11 * self.states[mode] + self.b12 * self.states[mode + mode_len];
            let b2 = self.b21[mode] * self.states[mode]
                + self.b22 * self.states[mode + mode_len]
                + zeta2 * 0.5 * self.k() * self.bow.pressure * (lambda - 2.0 * d)
                + self.k() * self.bow.pressure * d * self.modes_in[mode] * self.bow.velocity;

            // Sherman Morrison Solver
            let v = 0.5 * self.k() * self.bow.pressure * lambda * self.modes_in[mode];
            self.inv_av2[mode] = (1.0 / self.shur_comp[mode]) * v;
            self.inv_av1[mode] = self.a11 * self.a12 * self.inv_av2[mode];
            let y2 = self.a11 * b1;
            let z2 = b2 - self.a21[mode] * y2;
            self.inv_ab2[mode] = (1.0 / self.shur_comp[mode]) * z2;
            self.inv_ab1[mode] = y2 - self.a11 * self.a12 * self.inv_ab2[mode];

            v1 += self.modes_in[mode] * self.inv_av2[mode];
            v2 += self.modes_in[mode] * self.inv_ab2[mode];
        }

        for mode in 0..mode_len {
            self.states[mode] = self.inv_ab1[mode] - (1.0 / (1.0 + v1)) * self.inv_av1[mode] * v2;
            self.states[mode + mode_len] =
                self.inv_ab2[mode] - (1.0 / (1.0 + v1)) * self.inv_av2[mode] * v2;
        }

        // TODO: We should only output mono or support any amount of channels.
        let result_left = (0..mode_len)
            .map(|mode| self.modes_out_left[mode] * self.states[mode])
            .sum::<f32>();
        let result_right = (0..mode_len)
            .map(|mode| self.modes_out_right[mode] * self.states[mode])
            .sum::<f32>();

        (result_left + result_right) * 0.5 * self.gain
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
        self.shur_comp = (0..mode_len)
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
        self.b22 = 1.0;
    }

    fn initialize_states(&mut self) {
        let mode_len = self.mode_len();
        self.states = vec![0.0; mode_len * 2];
        self.inv_ab1 = vec![0.0; mode_len];
        self.inv_ab2 = vec![0.0; mode_len];
        self.inv_av1 = vec![0.0; mode_len];
        self.inv_av2 = vec![0.0; mode_len];
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

    fn sample_duration(&self) -> f32 {
        1.0 / self.sample_rate
    }

    // TODO rename to sample_duration
    fn k(&self) -> f32 {
        self.sample_duration()
    }
}
