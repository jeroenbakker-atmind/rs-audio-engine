use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

use audio_engine_common::phase_time::PhaseTime;
use audio_engine_common::waveform::shape::morph::MorphShape;
use audio_engine_common::waveform::shape::shape_sample;
fn main() {
    // For reading and opening files

    let path = Path::new(r"morph-shapes.png");
    let file = File::create(path).unwrap();
    let w = BufWriter::new(file);

    let shape_dim = 64;
    let num_shapes_per_param = 8;
    let image_dim: usize = shape_dim * num_shapes_per_param;

    let mut encoder = png::Encoder::new(w, image_dim as u32, image_dim as u32); // Width is 2 pixels and height is 1.
    encoder.set_color(png::ColorType::Grayscale);
    encoder.set_depth(png::BitDepth::Eight);
    encoder.set_source_gamma(png::ScaledFloat::from_scaled(45455)); // 1.0 / 2.2, scaled by 100000
    encoder.set_source_gamma(png::ScaledFloat::new(1.0 / 2.2)); // 1.0 / 2.2, unscaled, but rounded
    let source_chromaticities = png::SourceChromaticities::new(
        // Using unscaled instantiation here
        (0.31270, 0.32900),
        (0.64000, 0.33000),
        (0.30000, 0.60000),
        (0.15000, 0.06000),
    );
    encoder.set_source_chromaticities(source_chromaticities);
    let mut writer = encoder.write_header().unwrap();

    let mut data = vec![0_u8; image_dim * image_dim];
    for shape_y_param in 0..num_shapes_per_param {
        for shape_x_param in 0..num_shapes_per_param {
            let shape = MorphShape::new(
                shape_x_param as f32 / (num_shapes_per_param - 1) as f32,
                shape_y_param as f32 / (num_shapes_per_param - 1) as f32,
            );
            for shape_x in 0..shape_dim {
                let time = shape_x as f32 / shape_dim as f32;
                let phase_time = PhaseTime { time };
                let sample = shape_sample(&shape, phase_time, 16).clamp(-1.0, 1.0);
                let from_y = (sample * 0.5 + 0.5) * 32.0;
                if from_y >= 0.0 && from_y < shape_dim as f32 {
                    let shape_y = from_y as usize;

                    let image_offset = (shape_y_param * shape_dim + shape_y) * image_dim
                        + (shape_x_param * shape_dim + shape_x);
                    data[image_offset] = 255;
                }
            }
        }
    }
    writer.write_image_data(data.as_slice()).unwrap(); // Save
}
