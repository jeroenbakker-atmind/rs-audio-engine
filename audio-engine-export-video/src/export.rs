use ndarray::Array3;
use std::path::PathBuf;
use video_rs::{Encoder, EncoderSettings, Locator, Time};

pub fn export_audio_to_file(filename: &str, samples: &[f32], sample_rate: f32) {
    video_rs::init().unwrap();

    let height = 720;
    let width = 1280;

    let destination: Locator = PathBuf::from(filename).into();

    let settings = EncoderSettings::for_h264_yuv420p(width, height, false);

    let mut encoder = Encoder::new(&destination, settings).expect("failed to create encoder");

    let frame_rate = 24.0;
    let duration: Time = Time::from_nth_of_a_second(frame_rate as usize);
    let mut position = Time::zero();

    let num_samples_per_frame = sample_rate / frame_rate;
    let num_samples_per_column = num_samples_per_frame / 1280.0;
    let num_frames = samples.len() / num_samples_per_frame as usize;

    for frame_number in 0..num_frames {
        // This will create a smooth rainbow animation video!
        let sample_offset = (frame_number as f32 * num_samples_per_frame) as usize;
        let frame = create_frame(
            width,
            height,
            samples,
            sample_offset,
            num_samples_per_column as usize,
        );

        encoder
            .encode(&frame, &position)
            .expect("failed to encode frame");

        position = position.aligned_with(&duration).add();
    }

    encoder.finish().expect("failed to finish encoder");
}

fn create_frame(
    width: usize,
    height: usize,
    samples: &[f32],
    sample_offset: usize,
    samples_per_column: usize,
) -> Array3<u8> {
    let black: [u8; 3] = [0, 0, 0];
    let white: [u8; 3] = [0, 255, 0];
    let half_height = (height / 2) as i32;
    let scalar = 100;
    let samples = samples
        .iter()
        .map(|sample| (sample * scalar as f32) as i32 + half_height)
        .collect::<Vec<i32>>();
    let min_max_per_column = (0..width)
        .map(|column| {
            let mut from_offset = sample_offset + samples_per_column * (column - 1);
            if from_offset > samples.len() {
                from_offset = 0;
            }
            let mut to_offset = sample_offset + samples_per_column * (column + 2);
            if to_offset > samples.len() {
                to_offset = samples.len() - 1;
            }
            (from_offset, to_offset)
        })
        .map(|(from_offset, to_offset)| {
            let min_sample = *samples[from_offset..to_offset]
                .iter()
                .min()
                .unwrap_or(&half_height);
            let max_sample = *samples[from_offset..to_offset]
                .iter()
                .max()
                .unwrap_or(&half_height);
            (min_sample, max_sample)
        })
        .collect::<Vec<(i32, i32)>>();

    Array3::from_shape_fn((height, width, 3), |(x, y, z)| {
        let (min_sample, max_sample) = min_max_per_column[y];
        if x as i32 >= min_sample && x as i32 <= max_sample {
            white[z]
        } else {
            black[z]
        }
    })
}
