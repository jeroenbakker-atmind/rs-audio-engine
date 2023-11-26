use crate::{ChromaticTone, Note};

pub type ChromaticNote = Note<ChromaticTone>;

impl Default for ChromaticNote {
    fn default() -> Self {
        ChromaticNote::new(ChromaticTone::C, 4)
    }
}
