use crate::{ChromaticTone, Note};

pub type ChromaticNote = Note<ChromaticTone>;

impl Default for ChromaticNote {
    fn default() -> Self {
        ChromaticNote::new(ChromaticTone::C, 4)
    }
}

impl ChromaticNote {
    /// Calculate the multiplier to move a tone (tuned at octave 4) to the octave in self.octave.
    ///
    /// ```
    /// use audio_engine_notes::{ChromaticNote, ChromaticTone};
    /// assert_eq!(1.0 / 16.0, ChromaticNote::new(ChromaticTone::C, 0).multiplier_octave4());
    /// assert_eq!(1.0 / 8.0, ChromaticNote::new(ChromaticTone::C, 1).multiplier_octave4());
    /// assert_eq!(1.0 / 4.0, ChromaticNote::new(ChromaticTone::C, 2).multiplier_octave4());
    /// assert_eq!(1.0 / 2.0, ChromaticNote::new(ChromaticTone::C, 3).multiplier_octave4());
    /// assert_eq!(1.0, ChromaticNote::new(ChromaticTone::C, 4).multiplier_octave4());
    /// assert_eq!(2.0, ChromaticNote::new(ChromaticTone::C, 5).multiplier_octave4());
    /// assert_eq!(4.0, ChromaticNote::new(ChromaticTone::C, 6).multiplier_octave4());
    /// assert_eq!(8.0, ChromaticNote::new(ChromaticTone::C, 7).multiplier_octave4());
    /// assert_eq!(16.0, ChromaticNote::new(ChromaticTone::C, 8).multiplier_octave4());
    /// ```
    pub fn multiplier_octave4(&self) -> f32 {
        2.0_f32.powi(self.octave as i32 - 4)
    }

    /// Get the pitch of self.
    ///
    /// Result is in Hz.
    ///
    /// ```
    /// use audio_engine_notes::{ChromaticNote, ChromaticTone};
    /// assert_eq!(440.0, ChromaticNote::new(ChromaticTone::A, 4).pitch());
    /// ```
    pub fn pitch(&self) -> f32 {
        self.tone.pitch_octave4() * self.multiplier_octave4()
    }
}
