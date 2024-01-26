use std::str::FromStr;

use crate::{ChromaticTone, Note, Pitch};

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

    /// Get the note one higher than the current instance.
    ///
    /// ```
    /// use audio_engine_notes::{ChromaticNote, ChromaticTone};
    /// assert_eq!(ChromaticNote::new(ChromaticTone::C, 4).one_note_lower(), ChromaticNote::new(ChromaticTone::B, 3));
    /// ```
    pub fn one_note_lower(self) -> ChromaticNote {
        if self.tone == ChromaticTone::C {
            ChromaticNote::new(ChromaticTone::B, self.octave - 1)
        } else {
            ChromaticNote::new(self.tone as u8 - 1, self.octave)
        }
    }
    /// Get the note one lower than the current instance.
    ///
    /// ```
    /// use audio_engine_notes::{ChromaticNote, ChromaticTone};
    /// assert_eq!(ChromaticNote::new(ChromaticTone::C, 4).one_note_higher(), ChromaticNote::new(ChromaticTone::CSharp, 4));
    /// ```
    pub fn one_note_higher(self) -> ChromaticNote {
        if self.tone == ChromaticTone::B {
            ChromaticNote::new(ChromaticTone::C, self.octave + 1)
        } else {
            ChromaticNote::new(self.tone as u8 + 1, self.octave)
        }
    }
}

impl From<Pitch> for ChromaticNote {
    fn from(value: Pitch) -> Self {
        let mut result = ChromaticNote::new(ChromaticTone::C, 0);
        while value.frequency > result.pitch() as f64 {
            result.octave += 1;
        }
        while value.frequency < result.pitch() as f64 {
            result = result.one_note_lower();
        }
        if (result.pitch() as f64 - value.frequency).abs()
            > (result.one_note_higher().pitch() as f64 - value.frequency).abs()
        {
            result.one_note_higher()
        } else {
            result
        }
    }
}

impl FromStr for ChromaticNote {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let octave = s[s.len() - 1..s.len()].parse::<u8>().unwrap();
        let tone = s[0..s.len() - 1].parse::<ChromaticTone>().unwrap();
        Ok(ChromaticNote::new(tone, octave))
    }
}

impl From<&str> for ChromaticNote {
    fn from(value: &str) -> Self {
        value.parse::<ChromaticNote>().unwrap()
    }
}
