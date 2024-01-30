use std::time::Instant;

use crate::duration::Duration;

use super::{
    parameters::{NoteParameters, SoundParameters},
    sound::Sound,
};

pub fn bench_realtime_factor_single<T>(
    sound: &T,
    parameters: &mut T::Parameters,
    time: Duration,
) -> f32
where
    T: Sound + Sized,
    T::Parameters: BenchmarkParameters,
{
    let mut sound_state = sound.init_sound_state();
    let start_time = Instant::now();
    for _ in 0..(parameters.get_sample_rate() * time) as usize {
        parameters.init_next_sample();
        let _ = sound.sample(parameters, &mut sound_state);
    }
    let end_time = Instant::now();

    let duration = end_time - start_time;
    let realtime_factor = time / duration.as_secs_f32();

    realtime_factor
}

pub fn bench_realtime_factor<T>(sound: &T, parameters: &mut T::Parameters, time: Duration) -> f32
where
    T: Sound + Sized,
    T::Parameters: BenchmarkParameters,
{
    for _ in 0..(time * 5.0) as usize {
        bench_realtime_factor_single(sound, parameters, time);
        parameters.reset();
    }
    bench_realtime_factor_single(sound, parameters, time)
}

pub trait BenchmarkParameters: SoundParameters {
    fn reset(&mut self);
    fn init_next_sample(&mut self);
}

impl BenchmarkParameters for NoteParameters {
    fn reset(&mut self) {
        self.note_time = 0.0;
    }
    fn init_next_sample(&mut self) {
        self.note_time += 1.0 / self.get_sample_rate();
    }
}

#[test]
fn test_benchmark() {
    use crate::waveform::Waveform;

    let sound = Waveform::Sine;
    let mut parameters = NoteParameters {
        note_time: 0.0,
        note_off: None,
        note_pitch: 440.0,
        gain: 1.0,
        sample_rate: 44100.0,
    };

    let realtime_factor = bench_realtime_factor(&sound, &mut parameters, 5.0);
    println!("Benchmark Sound: realtime_factor={realtime_factor}");
}
