use std::f64::consts::{PI, TAU};

use crate::{
    bow::{Bow, BOW_FREE_PARAMETER},
    damping::DampingCoefficient,
    eigen_frequencies::EigenFrequency,
    processor::StringProcessor,
    string_and_hand::StringAndHand,
};

const DEBUG_VALUES: bool = false;
// friction can become a trait.

#[derive(Default, Debug, Clone)]
struct Modes {
    modes: Vec<f64>,
}

#[derive(Default, Debug, Clone)]
pub struct ModalVar1Processor {
    string_and_hand: StringAndHand,
    pub bow: Bow,
    pub gain: f64,
    sample_rate: f64,
    oversampling: u32,

    eigen_frequencies: Vec<f64>,
    damping_profile: Vec<f64>,

    excit_position: f64,

    modes_in: Modes,
    outputs: Vec<Modes>,

    states: Vec<f64>,

    a21: Vec<f64>,
    a22: Vec<f64>,

    inv_shur_comp: Vec<f64>,

    b21: Vec<f64>,
    b22: Vec<f64>,

    previous_sample: f64,

    // scratch space
    inv_av2: Vec<f64>,
    inv_ab1: Vec<f64>,
    inv_ab2: Vec<f64>,
}

impl StringProcessor for ModalVar1Processor {
    fn new(sample_rate: f64, string: &crate::string::String) -> Self {
        let mut processor = Self::default();
        processor.string_and_hand.string = string.clone();
        // TODO: This should be string specific. Not sure why....
        processor.excit_position = processor.string_and_hand.excit_position();
        processor.sample_rate = sample_rate;
        processor.gain = 1.0;
        processor.oversampling = 1;
        processor.initialize();

        processor
    }

    fn set_gain(&mut self, value: f64) {
        self.gain = value;
    }
    fn update_bow(&mut self, bow: Bow) {
        self.bow = bow;
    }
    fn set_hand_position_multiplier(&mut self, hand_position_multiplier: f64) {
        let previous_length = self.string_and_hand.length();
        self.string_and_hand.hand.fretting_position = hand_position_multiplier;
        if previous_length != self.string_and_hand.length() {
            // Find out if new modes are needed and try to migrate the current state to the new state.
            self.initialize();
        }
    }

    fn reset_string_states(&mut self) {
        self.states.fill(0.0);
        self.previous_sample = 0.0;
    }

    fn read_output(&mut self) -> f64 {
        let mut result = 0.0;
        for _ in 0..self.oversampling {
            result += self.sample_next_state();
        }

        (result / self.oversampling as f64) * self.gain
    }
}

impl ModalVar1Processor {
    fn sample_next_state(&mut self) -> f64 {
        let mode_len = self.mode_len();
        let zeta1 = (0..mode_len)
            .map(|mode| self.modes_in.modes[mode] * self.states[mode + mode_len])
            .sum::<f64>();

        // Bilbao friction
        let eta = zeta1 - self.bow.velocity;
        let d = (2.0 * BOW_FREE_PARAMETER).sqrt() * (-BOW_FREE_PARAMETER * eta * eta + 0.5).exp();
        let lambda = d * (1.0 - 2.0 * BOW_FREE_PARAMETER * eta.powi(2));

        let mut v1 = 0.0;
        let mut v2 = 0.0;

        let sample_duration = self.sample_duration();
        let b12 = self.b12();
        let a12 = self.a12();

        for mode in 0..mode_len {
            let state_0 = self.states[mode];
            let state_1 = self.states[mode + mode_len];
            let mode_in = self.modes_in.modes[mode];
            let zeta2 = mode_in * zeta1;
            let b1 = self.b11() * state_0 + b12 * state_1;
            let b2 = self.b21[mode] * state_0
                + self.b22[mode] * state_1
                + zeta2 * 0.5 * sample_duration * self.bow.pressure * (lambda - 2.0 * d)
                + sample_duration * self.bow.pressure * d * mode_in * self.bow.velocity;

            let v = 0.5 * sample_duration * self.bow.pressure * lambda * mode_in;
            let inv_shur = self.inv_shur_comp[mode];
            self.inv_av2[mode] = inv_shur * v;

            let y2 = self.a11() * b1;
            let z2 = b2 - self.a21[mode] * y2;
            self.inv_ab2[mode] = inv_shur * z2;
            self.inv_ab1[mode] = y2 - self.a11() * a12 * self.inv_ab2[mode];

            v1 += mode_in * self.inv_av2[mode];
            v2 += mode_in * self.inv_ab2[mode];
        }
        let coeff = 1.0 / (1.0 + v1);

        for mode in 0..mode_len {
            let inv_av1 = -self.a11() * a12 * self.inv_av2[mode];
            self.states[mode] = self.inv_ab1[mode] - coeff * inv_av1 * v2;
            self.states[mode + mode_len] = self.inv_ab2[mode] - coeff * self.inv_av2[mode] * v2;
        }

        if DEBUG_VALUES {
            for output in &self.outputs {
                let value = output
                    .modes
                    .iter()
                    .zip(&self.states)
                    .map(|(m, s)| m * s)
                    .sum::<f64>();
                print!("{value:?}, ");
            }
            println!();
        }

        let result = self
            .outputs
            .iter()
            .map(|output| {
                // TODO use zip
                (0..mode_len)
                    .map(|mode| output.modes[mode] * self.states[mode])
                    .sum::<f64>()
            })
            .sum::<f64>();
        result
    }
}

impl ModalVar1Processor {
    fn initialize(&mut self) {
        self.initialize_eigen_frequencies();
        self.initialize_modes();
        self.initialize_damping();
        self.initialize_matrices();
        self.initialize_states();
    }

    fn initialize_eigen_frequencies(&mut self) {
        const EIGEN_THRESHOLD: f64 = 20e3 * TAU;
        let mut mode = 1;
        let mut eigen_frequencies = Vec::with_capacity(200);

        while *eigen_frequencies.last().unwrap_or(&0.0) < EIGEN_THRESHOLD {
            eigen_frequencies.push(self.string_and_hand.calc_eigen_frequency(mode));
            mode += 1;
        }
        // Last calculated frequency would be above the threshold of 20Khz.
        eigen_frequencies.pop().unwrap();

        eigen_frequencies.shrink_to_fit();
        self.eigen_frequencies = eigen_frequencies;
    }

    fn initialize_modes(&mut self) {
        let mode_len = self.mode_len();
        fn calc_mode(string_and_hand: &StringAndHand, position: f64, mode: usize) -> f64 {
            (2.0 / string_and_hand.length()).sqrt()
                * (mode as f64 * PI * position / string_and_hand.length()).sin()
        }
        self.modes_in.modes = (1..=mode_len)
            .map(|mode| calc_mode(&self.string_and_hand, self.excit_position, mode))
            .collect::<Vec<f64>>();

        let num_outputs = 2;
        self.outputs.resize(num_outputs, Modes::default());
        for (index, output) in self.outputs.iter_mut().enumerate() {
            let position = self.string_and_hand.output_position(index);
            // TODO: could this be optimized by resizing and mutating inline?
            output.modes = (1..=mode_len)
                .map(|mode| calc_mode(&self.string_and_hand, position, mode))
                .collect::<Vec<f64>>();
        }
    }

    fn a11(&self) -> f64 {
        1.0
    }
    fn a12(&self) -> f64 {
        -0.5 * self.sample_duration()
    }
    fn b11(&self) -> f64 {
        1.0
    }
    fn b12(&self) -> f64 {
        0.5 * self.sample_duration()
    }

    fn initialize_matrices(&mut self) {
        let mode_len = self.mode_len();

        self.a21 = self
            .eigen_frequencies
            .iter()
            .map(|eigen_frequency| {
                -0.5 * self.sample_duration() * (-eigen_frequency * eigen_frequency)
            })
            .collect::<Vec<f64>>();
        self.a22 = self
            .damping_profile
            .iter()
            .map(|damping_coefficient| 1.0 + 0.5 * self.sample_duration() * damping_coefficient)
            .collect::<Vec<f64>>();

        self.inv_shur_comp = (0..mode_len)
            .map(|mode| self.a22[mode] - self.a21[mode] * self.a11() * self.a12())
            .map(|shur_comp| 1.0 / shur_comp)
            .collect::<Vec<f64>>();

        // Should still be checked with reference implementation.
        self.b21 = self
            .eigen_frequencies
            .iter()
            .map(|eigen_frequency| {
                0.5 * self.sample_duration() * (-eigen_frequency * eigen_frequency)
            })
            .collect::<Vec<f64>>();
        self.b22 = self
            .damping_profile
            .iter()
            .map(|damping_coefficient| 1.0 - 0.5 * self.sample_duration() * damping_coefficient)
            .collect::<Vec<f64>>();
    }

    fn initialize_states(&mut self) {
        let mode_len = self.mode_len();
        self.states = vec![0.0; mode_len * 2];
        self.inv_ab1 = vec![0.0; mode_len];
        self.inv_ab2 = vec![0.0; mode_len];
        self.inv_av2 = vec![0.0; mode_len];
    }

    fn initialize_damping(&mut self) {
        self.damping_profile = self
            .string_and_hand
            .string
            .damping_coefficients(&self.eigen_frequencies);
    }

    /// Get the number of modes in play.
    ///
    /// Only valid after the eigen frequencies have been initialized.
    fn mode_len(&self) -> usize {
        self.eigen_frequencies.len()
    }

    fn sample_duration(&self) -> f64 {
        1.0 / (self.sample_rate * self.oversampling as f64)
    }
}
