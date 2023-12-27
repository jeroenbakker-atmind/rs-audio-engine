use ndarray::Array3;

use crate::{
    histogram::{create_histogram_frame, init_histogram_rendering},
    render_context::RenderContext,
    waveform::create_waveform_frame,
};

#[derive(Default, Copy, Clone, Debug)]
pub enum VideoType {
    #[default]
    Waveform,
    Histogram,
}

impl VideoType {
    pub fn init(&self, samples: &[f32], samples_per_frame: usize) -> RenderContext {
        match self {
            VideoType::Waveform => RenderContext::default(),
            VideoType::Histogram => init_histogram_rendering(samples, samples_per_frame),
        }
    }

    pub fn create_frame(
        &self,
        width: usize,
        height: usize,
        samples: &[f32],
        sample_offset: usize,
        samples_per_frame: usize,
        render_context: &RenderContext,
    ) -> Array3<u8> {
        match self {
            VideoType::Waveform => {
                create_waveform_frame(width, height, samples, sample_offset, samples_per_frame)
            }
            VideoType::Histogram => create_histogram_frame(
                width,
                height,
                samples,
                sample_offset,
                samples_per_frame,
                render_context,
            ),
        }
    }
}
