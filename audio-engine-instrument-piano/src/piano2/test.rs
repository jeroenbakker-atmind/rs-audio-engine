use super::Piano;

#[test]
fn test_52326hz_10ms() {
    let frequency = 523.26; //0009765625;
    let hammer_velocity = 10.0;
    let mut piano = Piano::new(44100.0);
    piano.start_note(frequency, hammer_velocity);
    let mut m = 0.0;
    for _ in 0..100000 {
        let sample = piano.sample();
        assert!(sample.abs() < 10.0);
        m = sample.abs().max(m);
    }
    println!("{m}");
}

#[test]
fn test_52325hz_10ms() {
    let frequency = 523.25;
    let hammer_velocity = 10.0;
    let mut piano = Piano::new(44100.0);
    piano.start_note(frequency, hammer_velocity);
    let mut m = 0.0;
    for _ in 0..100000 {
        let sample = piano.sample();
        assert!(sample.abs() < 10.0);
        m = sample.abs().max(m);
    }
    println!("{m}");
}
