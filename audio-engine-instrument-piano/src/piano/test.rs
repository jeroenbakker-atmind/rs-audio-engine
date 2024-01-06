use super::Piano;

#[test]
fn generate_c4() {
    let mut piano = Piano::default();
    let s = 5;
    let samples = s * 44100;
    piano.init(261.63, 44100.0, 5.0);
    let mut result = Vec::<f32>::new();
    result.resize(samples, 0.0);
    piano.go(&mut result);
    println!("{result:#?}");
}
