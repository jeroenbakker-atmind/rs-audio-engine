
use super::Piano;

#[test]
fn generate_c4() {
    let mut piano = Piano::default();
    let s = 5;
    let samples = s * 44100;
    piano.init(437.0, 44100.0, 5.0, samples);
    let mut result = Vec::<f32>::new();
    result.resize(samples, 0.0);
    piano.go(&mut result);
    println!("{result:#?}");
}