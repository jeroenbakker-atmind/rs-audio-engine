use crate::{hand::Hand, string::String};

#[derive(Default, Debug, Copy, Clone)]
pub struct StringAndHand {
    pub string: String,
    pub hand: Hand,
}

impl StringAndHand {
    /// Get the actual length of the string that isn't blocked by the hand position.
    /// Length of the string with the hand blockage applied.
    pub fn length(&self) -> f64 {
        self.string.length * self.hand.fretting_position
    }

    pub fn upper_k(&self) -> f64 {
        self.string.inertia() * self.string.young_mod
            / (self.string.lin_density() * self.length().powi(4)).sqrt()
    }

    pub fn excit_position(&self) -> f64 {
        self.string.excit_position.get_value(self.length())
    }
    pub fn output_position_left(&self) -> f64 {
        self.string.output_position_left.get_value(self.length())
    }
    pub fn output_position_right(&self) -> f64 {
        self.string.output_position_right.get_value(self.length())
    }
}
