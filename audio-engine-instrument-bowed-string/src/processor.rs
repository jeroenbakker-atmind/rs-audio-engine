use crate::string::String;

pub trait StringProcessor {
    fn new(sample_rate: f64, string: &String) -> Self;
    fn reset_string_states(&mut self);
    fn read_output(&mut self) -> f64;
}
