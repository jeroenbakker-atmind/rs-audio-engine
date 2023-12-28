#[derive(Debug, Clone, Copy)]
pub struct Parameters {
    /// Number of samples in the time domain.
    pub data_len: usize,
    /// Number of integer steps in the frequency domain.
    pub steps: usize,
}
