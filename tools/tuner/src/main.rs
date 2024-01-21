use core::time;
use std::cmp::Ordering;

use arguments::Arguments;
use audio_engine_fourier::{
    fourier_series::{ComplexNumberMethods, FourierSeries},
    to_frequency_domain::ToFrequencyDomain,
};
use audio_engine_notes::{ChromaticNote, ChromaticTone};
mod arguments;

fn main() {
    let arguments = Arguments {
        chromatic_note: ChromaticNote::new(ChromaticTone::C, 4),
        buffer_size: 4096,
        sample_rate: 44100.0,
    };

    init_recording_device();
    let mut record_buffer = Vec::<f32>::new();
    record_buffer.resize(arguments.buffer_size, 0.0);
    loop {
        fill_recording_buffer(&mut record_buffer);
        let frequency_domain = to_frequency_domain(&record_buffer);
        let step = find_step_with_highest_amplitude(&frequency_domain);
        let buffer_frequency = frequency_domain.frequency(step);
        let audio_frequency =
            buffer_frequency * (arguments.sample_rate / arguments.buffer_size as f32);
        println!(
            "input: {audio_frequency}Hz, target: {}Hz",
            arguments.chromatic_note.pitch()
        );
    }
}

fn init_recording_device() {}

fn fill_recording_buffer(buffer: &mut [f32]) {
    buffer.fill(0.0);
}

fn to_frequency_domain(time_domain: &[f32]) -> FourierSeries {
    time_domain.to_frequency_domain(time_domain.len())
}

fn find_step_with_highest_amplitude(series: &FourierSeries) -> usize {
    series
        .amplitudes
        .iter()
        .enumerate()
        .max_by(|(_, a), (_, b)| {
            let a_amplitude = a.amplitude();
            let b_amplitude = b.amplitude();
            if a_amplitude > b_amplitude {
                Ordering::Greater
            } else if a_amplitude < b_amplitude {
                Ordering::Less
            } else {
                Ordering::Equal
            }
        })
        .map(|(index, complex)| index)
        .unwrap_or_default()
}
