//! Benchmark performance of Sounds
//!
//! Not all sound processing can be parallized and optimizing them to run
//! fast on a single thread is often preferred. Very often sound sampling depends
//! on the state of the previous sample.
//!
//! This module contains tools that help benchmark the performance of
//! sound sampling.
//! 
//! Some areas in audio engine will have examples projects that prints out the
//! real time factor. This is used to optimize parts of the project.
use std::time::Instant;

use crate::duration::Duration;

use super::{
    parameters::{NoteParameters, SoundParameters},
    sound::Sound,
};

/// Benchmark a sound for the given duration.
/// 
/// Returns a real-time factor. The factor is how many times faster it evaluates than real-time.
/// eg when 1 second of sound is evaluated in 0.2 seconds, it will return 5.0. It indicates
/// that the sound can be evaluated 5 times simultaniously and still be real-time.s
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

/// Run the benchmark multiple times, return the timing of the last benchmark only.
///
/// #warmup_times is the number of times the sound is benchmarked, before running
/// the benchmark for real. Each time the #time is sampled.
pub fn bench_realtime_factor<T>(
    sound: &T,
    parameters: &mut T::Parameters,
    time: Duration,
    warmup_times: usize,
) -> f32
where
    T: Sound + Sized,
    T::Parameters: BenchmarkParameters,
{
    for _ in 0..warmup_times as usize {
        bench_realtime_factor_single(sound, parameters, time);
        parameters.reset();
    }
    bench_realtime_factor_single(sound, parameters, time)
}

/// Trait for reading parameters needed during benchmarking.
///
/// When a custom SoundParameters is used this trait
/// needs to be implemented in order to benchmark the performance
pub trait BenchmarkParameters: SoundParameters {
    /// Reset the sound parameters to its initial state.
    ///
    /// When benchmarking the benchmark can be run multiple times. In stead of
    /// recreating the parameters, the instance is reset.
    fn reset(&mut self);

    /// Sound parameters often contain the time of the note to sample. The time
    /// needs to be advanced before each sample.
    fn init_next_sample(&mut self);
}

/// Implement the BenchmarkParameter for NoteParameters.
///
/// NoteParameters is mostly used when sampling digital sounds.
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

    let realtime_factor = bench_realtime_factor(&sound, &mut parameters, 5.0, 3);
    println!("Benchmark Sound: realtime_factor={realtime_factor}");
}
