use std::f64::consts::{PI, TAU};

use crate::{
    bow::{Bow, BOW_FREE_PARAMETER},
    damping::DampingCoeffcient,
    eigen_frequencies::EigenFrequency,
    processor::StringProcessor,
    string_and_hand::StringAndHand,
};
// friction can become a trait.

#[derive(Default, Debug, Clone)]
pub struct ShermanMorrison {
    string_and_hand: StringAndHand,
    pub bow: Bow,
    pub gain: f64,
    sample_rate: f64,

    eigen_frequencies: Vec<f64>,
    damping_profile: Vec<f64>,

    excit_position: f64,
    output_position_left: f64,
    output_position_right: f64,

    modes_in: Vec<f64>,
    modes_out_left: Vec<f64>,
    modes_out_right: Vec<f64>,

    states: Vec<f64>,

    a11: f64,
    a12: f64,
    a21: Vec<f64>,
    a22: f64,

    shur_comp: Vec<f64>,

    b11: f64,
    b12: f64,
    b21: Vec<f64>,
    b22: f64,

    // scratch space
    inv_av1: Vec<f64>,
    inv_av2: Vec<f64>,
    inv_ab1: Vec<f64>,
    inv_ab2: Vec<f64>,
}

impl StringProcessor for ShermanMorrison {
    fn new(sample_rate: f64, string: &crate::string::String) -> Self {
        let mut processor = ShermanMorrison::default();
        processor.string_and_hand.string = string.clone();
        // TODO: This should be string specific. Not sure why....
        processor.excit_position = processor.string_and_hand.length() * 0.733;
        processor.output_position_left = processor.string_and_hand.length() * 0.53;
        processor.output_position_right = processor.string_and_hand.length() * 0.77;
        processor.sample_rate = sample_rate;
        processor.gain = 500.0;
        processor.initialize();

        processor
    }

    fn reset_string_states(&mut self) {
        self.states.fill(0.0);
    }

    fn set_input_position(&mut self, input_position: f64) {}
    fn set_read_position(&mut self, read_position: f64) {}
    fn compute_state(&mut self) {
        // TODO: check current state with previous state
        // reset matrices and frequencies based on the actual change.
        // This will allow physical accurate reverbs by fast moving your finger.
    }
    fn read_output(&mut self) -> f64 {
        let mode_len = self.mode_len();
        let zeta1 = (0..mode_len)
            .map(|mode| self.modes_in[mode] * self.states[mode + mode_len])
            .sum::<f64>();

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
                + zeta2 * 0.5 * self.sample_duration() * self.bow.pressure * (lambda - 2.0 * d)
                + self.sample_duration()
                    * self.bow.pressure
                    * d
                    * self.modes_in[mode]
                    * self.bow.velocity;

            // Sherman Morrison Solver
            let v = 0.5 * self.sample_duration() * self.bow.pressure * lambda * self.modes_in[mode];
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

        let result_left = (0..mode_len)
            .map(|mode| self.modes_out_left[mode] * self.states[mode])
            .sum::<f64>();
        let result_right = (0..mode_len)
            .map(|mode| self.modes_out_right[mode] * self.states[mode])
            .sum::<f64>();

        (result_left + result_right) * self.gain
    }
}
impl ShermanMorrison {
    pub fn set_hand_position_multiplier(&mut self, hand_position_multiplier: f64) {
        let previous_length = self.string_and_hand.length();
        self.string_and_hand.hand.fretting_position = hand_position_multiplier;
        if previous_length != self.string_and_hand.length() {
            self.initialize();
        }
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
        fn calc_mode(string_and_hand: &StringAndHand, position: f64, mode: usize) -> f64 {
            (2.0 / string_and_hand.length()).sqrt()
                * (mode as f64 * PI * position / string_and_hand.length()).sin()
        }
        self.modes_in = (1..=mode_len)
            .map(|mode| calc_mode(&self.string_and_hand, self.excit_position, mode))
            .collect::<Vec<f64>>();
        self.modes_out_left = (1..=mode_len)
            .map(|mode| calc_mode(&self.string_and_hand, self.output_position_left, mode))
            .collect::<Vec<f64>>();
        self.modes_out_right = (1..=mode_len)
            .map(|mode| calc_mode(&self.string_and_hand, self.output_position_right, mode))
            .collect::<Vec<f64>>();
    }

    fn initialize_matrices(&mut self) {
        let mode_len = self.mode_len();

        self.a11 = 1.0;
        self.a12 = -0.5 * self.sample_duration();
        self.a21 = self
            .eigen_frequencies
            .iter()
            .map(|eigen_frequency| {
                -0.5 * self.sample_duration() * (-eigen_frequency * eigen_frequency)
            })
            .collect::<Vec<f64>>();
        self.a22 = 1.0;
        /*  self
        .damping_profile
            .iter()
            .map(|damping_coeefcient| 1.0 + 0.5 * self.k() * damping_coeefcient)
            .collect::<Vec<f64>>();*/
        // TODO a22 should be 1.0 ?

        // Optimize 1 - a21 *-0.5k
        self.shur_comp = (0..mode_len)
            .map(|mode| self.a22 - self.a21[mode] * self.a11 * self.a12)
            .collect::<Vec<f64>>();

        // Should still be checked with reference implementation.
        self.b11 = 1.0;
        self.b12 = 0.5 * self.sample_duration();
        self.b21 = self
            .eigen_frequencies
            .iter()
            .map(|eigen_frequency| {
                0.5 * self.sample_duration() * (-eigen_frequency * eigen_frequency)
            })
            .collect::<Vec<f64>>();
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

    fn sample_duration(&self) -> f64 {
        1.0 / self.sample_rate
    }
}
