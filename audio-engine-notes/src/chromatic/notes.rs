use crate::{ChromaticTone, Note};

pub type ChromaticNote = Note<ChromaticTone>;

impl Default for ChromaticNote {
    fn default() -> Self {
        ChromaticNote::new(ChromaticTone::C, 4)
    }
}

impl ChromaticNote {
    pub fn multiplier(&self) -> f32 {
        match self.octave {
            0 => 1.0 / 16.0,
            1 => 1.0 / 8.0,
            2 => 1.0 / 4.0,
            3 => 1.0 / 2.0,
            4 => 1.0,
            5 => 2.0,
            6 => 4.0,
            7 => 8.0,
            8 => 16.0,
            _ => 1.0,
        }
    }
    pub fn pitch(&self) -> f32 {
        self.tone.pitch_octave4() * self.multiplier()
    }
}
