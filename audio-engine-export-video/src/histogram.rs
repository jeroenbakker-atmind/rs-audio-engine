use audio_engine_fourier::to_frequency_domain::ToFrequencyDomain;
use ndarray::Array3;

pub fn create_histogram_frame(
    width: usize,
    height: usize,
    samples: &[f32],
    sample_offset: usize,
    samples_per_frame: usize,
) -> Array3<u8> {
    let frame_samples = &samples[sample_offset..sample_offset + samples_per_frame];
    let fourier_series = frame_samples.to_frequency_domain(samples_per_frame, 0);

    Array3::from_shape_fn((height, width, 3), |(x, y, z)| {
        let complex = fourier_series.amplitudes[y / 8];
        let amplitude = ((complex.1) * 4.0).clamp(0.0, 1.0);
        let color_value = (amplitude * 255.0) as u8;
        color_value
    })
}
