use crate::{string::String, string_processor::StringProcessor};

#[test]
fn string_processor() {
    let string = String {
        radius: 3.75e-04,
        density: 3.7575e3,
        tension: 153.0,
        young_mod: 25e9,
        length: 0.69,
    };
    let mut string_processor = StringProcessor::new(44100.0, &string);
    string_processor.vb = 0.2;
    string_processor.fb = 10.0;
    string_processor.set_input_position(0.5);
    string_processor.set_read_position(0.1);
    string_processor.is_being_played = true;
    string_processor.gain = 10.0;
    for _ in 0..4096 {
        string_processor.compute_state();
        let sample = string_processor.read_output();
        println!("{sample:?}");
    }
}
