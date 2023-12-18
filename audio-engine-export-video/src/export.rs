use ndarray::Array3;
use std::path::PathBuf;
use video_rs::{Encoder, EncoderSettings, Locator, Options, Time};

pub fn export_audio_to_file(filename: &str, samples: &[f32], sample_rate: f32) {
    video_rs::init().unwrap();

    let height = 720;
    let width = 1280;

    let destination: Locator = PathBuf::from("export-song1.mp4").into();

    let settings = EncoderSettings::for_h264_yuv420p(width, height, false);

    let mut encoder = Encoder::new(&destination, settings).expect("failed to create encoder");

    let frame_rate = 24.0;
    let duration: Time = Time::from_nth_of_a_second(frame_rate as usize);
    let mut position = Time::zero();

    let mut sample_offset = 0.0;
    let num_samples_per_frame = sample_rate / frame_rate;
    let num_samples_per_column = num_samples_per_frame / 1280.0;
    let num_frames = samples.len() / num_samples_per_frame as usize;

    for i in 0..num_frames {
        // This will create a smooth rainbow animation video!
        let frame = create_frame(
            width,
            height,
            samples,
            sample_offset as usize,
            num_samples_per_column as usize,
        );

        encoder
            .encode(&frame, &position)
            .expect("failed to encode frame");

        position = position.aligned_with(&duration).add();
        sample_offset += num_samples_per_frame;
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
    let samples = samples
        .iter()
        .map(|sample| (sample * half_height as f32) as i32 + half_height)
        .collect::<Vec<i32>>();

    Array3::from_shape_fn((height, width, 3), |(x, y, z)| {
        let min_sample = *samples
            [sample_offset + samples_per_column * y..sample_offset + samples_per_column * (y + 1)]
            .iter()
            .min()
            .unwrap_or(&half_height);
        let max_sample = *samples
            [sample_offset + samples_per_column * y..sample_offset + samples_per_column * (y + 1)]
            .iter()
            .max()
            .unwrap_or(&half_height);

        if x as i32 >= min_sample && x as i32 <= max_sample {
            white[z]
        } else {
            black[z]
        }
    })
}
