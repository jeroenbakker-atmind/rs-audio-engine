use crate::{hand::Hand, string::String};

#[derive(Default, Debug)]
pub struct StringAndHand {
    pub string: String,
    pub hand: Hand,
}

impl StringAndHand {
    /// Get the actual length of the string that isn't blocked by the hand position.
    pub fn length(&self) -> f32 {
        self.string.length * self.hand.fretting_position
    }

    /// Length of the string with the hand blockage applied.
    // TODO: should be replaced by length/unblocked length
    pub fn upper_l(&self) -> f32 {
        self.length()
    }

    pub fn upper_k(&self) -> f32 {
        self.string.inertia() * self.string.young_mod
            / (self.string.lin_density() * self.length().powi(4)).sqrt()
    }
}
