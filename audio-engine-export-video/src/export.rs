use std::path::PathBuf;
use video_rs::{Encoder, EncoderSettings, Locator, Time};

use crate::types::VideoType;

pub fn export_audio_to_video(
    filename: &str,
    samples: &[f32],
    sample_rate: f32,
    video_type: VideoType,
) {
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
    let num_frames = samples.len() / num_samples_per_frame as usize;

    for frame_number in 0..num_frames {
        print!(" - {}/{}\r", frame_number, num_frames);
        // This will create a smooth rainbow animation video!
        let sample_offset = (frame_number as f32 * num_samples_per_frame) as usize;
        let frame = video_type.create_frame(
            width,
            height,
            samples,
            sample_offset,
            num_samples_per_frame as usize,
        );

        encoder
            .encode(&frame, &position)
            .expect("failed to encode frame");

        position = position.aligned_with(&duration).add();
    }

    encoder.finish().expect("failed to finish encoder");
}
