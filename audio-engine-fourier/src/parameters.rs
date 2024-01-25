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
    FrequencyRange(FrequencyRange)
}

#[derive(Debug, Clone, Copy)]
pub struct FrequencyRange {
    pub start_frequency: f32,
    pub end_frequenct: f32,
    pub sample_rate: f32,
}

impl Parameters {
    /// Get the semitone of the given step.
    #[deprecated]
    pub fn semitone(&self, step: usize) ->f32 {
        match self.step_type {
            StepType::Semitones => step as f32,
            StepType::FrequencyRange(_params) => {
               self.steps as f32 / self.period_duration(step)
            }
        }
    }

    pub fn frequency(&self, step: usize) -> f32 {
        match self.step_type {
            StepType::Semitones=> {
                todo!()
            },
            StepType::FrequencyRange(range)=> {
                let step_f32 = step as f32 / self.steps as f32 ;
                let frequency = range.end_frequenct * step_f32 + range.start_frequency * (1.0 - step_f32);
                frequency
            }
        }
    }

    /// How many samples are needed for a single period of a step.
    pub fn period_duration(&self, step:usize) -> f32 {
        match self.step_type {
            StepType::Semitones=> {
                1.0 / (step as f32 + 1.0)
            },
            StepType::FrequencyRange(range)=> {
                let frequency = self.frequency(step);
                range.sample_rate / frequency
            }
        }

    }
}
