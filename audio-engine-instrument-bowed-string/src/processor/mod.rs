use crate::{bow::Bow, friction::desvages::Desvages, string::String};
pub mod modal;
pub mod modal_optimized;
#[cfg(test)]
mod test;

pub trait StringProcessor {
    fn new(sample_rate: f64, string: &String) -> Self;
    fn set_gain(&mut self, value: f64);
    fn update_bow(&mut self, bow: Bow);
    fn set_hand_position_multiplier(&mut self, value: f64);
    fn reset_string_states(&mut self);
    fn read_output(&mut self) -> f64;
}

pub type DefaultStringProcessor = modal_optimized::OptimizedModalProcessor<Desvages>;
