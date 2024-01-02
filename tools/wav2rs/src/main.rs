use std::{fs::File, path::Path};

use clap::Parser;
use convert_f32s::Convert2F32;
use wav::Header;

use crate::argument::Arguments;

mod argument;
mod convert_f32s;

fn main() {
    let args = Arguments::parse();

    let mut wav_file = File::open(Path::new(&args.input_filename)).unwrap();
    let (header, data) = wav::read(&mut wav_file).unwrap();
    let mut floats = data.to_f32s();
    floats = join_channels(header, &floats);
    if args.trim_start {
        floats = trim_start(&floats);
    }
    if args.normalize {
        floats = normalize(&floats);
    }

    let floats_len = floats.len();
    println!("pub static SAMPLES: [f32;{floats_len}] = [");
    for float in floats {
        println!("    {float:.0}, ");
    }
    println!("];")
}

fn join_channels(header: Header, data: &[f32]) -> Vec<f32> {
    data.chunks(header.channel_count as usize)
        .map(|chunk| chunk.iter().sum::<f32>())
        .collect::<Vec<f32>>()
}

fn trim_start(data: &[f32]) -> Vec<f32> {
    let mut result = Vec::new();
    result.push(0.0);
    result.extend(
        data.iter()
            .skip_while(|sample| **sample == 0.0)
            .cloned()
            .collect::<Vec<f32>>(),
    );
    result
}

fn normalize(data: &[f32]) -> Vec<f32> {
    let mut max_amplitude: f32 = 0.0;
    data.iter()
        .map(|sample| sample.abs())
        .for_each(|sample| max_amplitude = max_amplitude.max(sample));

    let multiplier = if max_amplitude == 0.0 {
        1.0
    } else {
        1.0 / max_amplitude
    };

    data.iter()
        .map(|sample| sample * multiplier)
        .collect::<Vec<f32>>()
}
