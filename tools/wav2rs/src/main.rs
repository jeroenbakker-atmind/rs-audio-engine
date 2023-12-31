use std::{env, fs::File, path::Path};

use wav::{BitDepth, Header};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("invalid arguments");
    }

    let wav_filepath = &args[1];
    let mut wav_file = File::open(Path::new(wav_filepath)).unwrap();
    let (header, data) = wav::read(&mut wav_file).unwrap();
    let floats = convert(header, data);
    println!("static audio: Vec<f32> = vec![");
    for float in floats {
        println!("    {float}, ");
    }
    println!("];")
}

fn convert(header: Header, data: BitDepth) -> Vec<f32> {
    let mut samples = Vec::new();
    let mut started= false;
    match data {
        BitDepth::Sixteen(data) => {
            for channel_samples in data.chunks(header.channel_count as usize) {
                let sample = 
                    channel_samples
                        .iter()
                        .map(|a| *a as f32 / 32768.0)
                        .sum::<f32>();
                    if sample > 0.0 || started {
                        started=true;
                        samples.push(sample);
                    }
                
            }
        }

        _ => unimplemented!("unsupported bit depth."),
    }

    samples
}
