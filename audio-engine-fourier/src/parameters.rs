#[derive(Debug, Clone, Copy)]
pub struct Parameters {
    /// Number of samples in the time domain.
    pub data_len: usize,
    /// Number of steps in the frequency domain.
    pub steps: usize,
    pub step_type: StepType,
}

#[derive(Debug, Clone, Copy)]
pub enum StepType {
    /// Each step is a integer semitone related to the buffer size. 
    /// step 0 is semitone 0
    Semitones,

    /// Each step is a linear interpolation between frequencies.
    /// Frequencies are related to the size of the buffer.
    FrequencyRange(f32, f32)
}

impl Parameters {
    /// Get the semitone of the given step.
    pub fn semitone(&self, step: usize) ->f32 {
        match self.step_type {
            StepType::Semitones => step as f32,
            StepType::FrequencyRange(start_frequency, end_frequency) => {
                todo!()
            }
        }
    }
}
