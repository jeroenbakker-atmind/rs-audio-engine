use crate::{hand::Hand, string::String};

#[derive(Default, Debug, Clone)]
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

    pub fn excit_position(&self) -> f64 {
        self.string.excit_position.get_value(self.length())
    }
    pub fn output_position(&self, index: usize) -> f64 {
        let position = if index == 0 {
            self.string.output_position_left
        } else {
            self.string.output_position_right
        };
        position.get_value(self.length())
    }
}
