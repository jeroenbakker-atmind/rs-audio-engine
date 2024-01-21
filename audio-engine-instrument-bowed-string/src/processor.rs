use crate::string::String;

pub trait StringProcessor {
    fn new(sample_rate: f64, string: &String) -> Self;
    fn reset_string_states(&mut self);
    fn set_input_position(&mut self, input_position: f64);
    fn set_read_position(&mut self, input_position: f64);
    fn compute_state(&mut self);
    fn read_output(&mut self) -> f64;
}
