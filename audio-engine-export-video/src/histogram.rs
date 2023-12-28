use ndarray::Array3;

use crate::render_context::RenderContext;

pub fn init_histogram_rendering(samples: &[f32], samples_per_frame: usize) -> RenderContext {
    let mut result = RenderContext::default();
    result.init_fourier_series(samples, samples_per_frame);
    result
}

pub fn create_histogram_frame(
    width: usize,
    height: usize,
    _samples: &[f32],
    sample_offset: usize,
    samples_per_frame: usize,
    render_context: &RenderContext,
) -> Array3<u8> {
    let frame_no = sample_offset / samples_per_frame;
    Array3::from_shape_fn((height, width, 3), |(x, y, _)| {
        if x <= frame_no * 4 {
            let fourier_series = render_context.fourier_series.get(frame_no - x / 4).unwrap();
            let complex = fourier_series.amplitudes[y / 8];
            let amplitude = ((complex.1) * 4.0).clamp(0.0, 1.0);
            (amplitude * 255.0) as u8
        } else {
            0_u8
        }
    })
}
