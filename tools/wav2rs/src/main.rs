use std::{env, fs::File, path::Path};

use convert_f32s::Convert2F32;
use wav::Header;

mod convert_f32s;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("invalid arguments");
    }

    let wav_filepath = &args[1];
    let mut wav_file = File::open(Path::new(wav_filepath)).unwrap();
    let (header, data) = wav::read(&mut wav_file).unwrap();
    let floats = data.to_f32s();
    let floats = join_channels(header, &floats);
    let floats = trim_start(&floats);
    let floats_len = floats.len();
    println!("pub static SAMPLES: [f32;{floats_len}] = [");
    for float in floats {
        println!("    {float:.0}, ");
    }
    println!("];")
}

fn join_channels(header: Header, data: &[f32]) -> Vec<f32> {
    let mut samples = Vec::new();
    for channel_samples in data.chunks(header.channel_count as usize) {
        let sample = channel_samples.iter().sum::<f32>();
        samples.push(sample);
    }

    samples
}

fn trim_start(data: &[f32]) -> Vec<f32> {
    data.iter()
        .skip_while(|sample| **sample == 0.0)
        .cloned()
        .collect::<Vec<f32>>()
}
