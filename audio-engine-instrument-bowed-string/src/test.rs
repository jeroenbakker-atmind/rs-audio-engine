use crate::{modal_processor::ModalProcessor, processor::StringProcessor, string::CELLO_STRING_G2};

#[test]
fn string_processor() {
    let string = CELLO_STRING_G2;
    let mut string_processor = ModalProcessor::new(44100.0, &string);
    string_processor.bow.velocity = 0.2;
    string_processor.bow.pressure = 10.0;
    string_processor.gain = 1000.0;
    for _ in 0..4096 {
        let sample = string_processor.read_output();
        println!("{sample:?}");
    }
}
