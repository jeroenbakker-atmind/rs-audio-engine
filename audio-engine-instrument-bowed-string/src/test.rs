use crate::{
    nemus_processor::NemusProcessor, processor::StringProcessor,
    sherman_morrison_processor::ShermanMorrison, string::String,
};

#[test]
fn string_processor() {
    let string = String {
        radius: 7.2e-4,
        density: 1.3017e4,
        tension: 172.74,
        young_mod: 22.4e9,
        length: 0.69,
    };
    let mut string_processor = ShermanMorrison::new(44100.0, &string);
    string_processor.bow.velocity = 0.2;
    string_processor.bow.pressure = 10.0;
    for _ in 0..4096 {
        string_processor.compute_state();
        let sample = string_processor.read_output();
        println!("{sample:?}");
    }
}
