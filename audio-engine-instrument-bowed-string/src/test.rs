use crate::{
    processor::StringProcessor, sherman_morrison_processor::ShermanMorrison, string::{String, CELLO_STRING_C2, CELLO_STRING_G2},
};

#[test]
fn string_processor() {
    let string = CELLO_STRING_C2;
    let mut string_processor = ShermanMorrison::new(44100.0, &string);
    string_processor.bow.velocity = 0.2;
    string_processor.bow.pressure = 10.0;
    for _ in 0..4096 {
        string_processor.compute_state();
        let sample = string_processor.read_output();
        println!("{sample:?}");
    }
}
