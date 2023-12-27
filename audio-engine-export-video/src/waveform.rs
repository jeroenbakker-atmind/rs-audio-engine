use ndarray::Array3;

pub fn create_waveform_frame(
    width: usize,
    height: usize,
    samples: &[f32],
    sample_offset: usize,
    samples_per_frame: usize,
) -> Array3<u8> {
    let samples_per_column = samples_per_frame / width;
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
            let to_offset =
                (sample_offset + samples_per_column * (column + 2)).min(samples.len() - 1);
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
