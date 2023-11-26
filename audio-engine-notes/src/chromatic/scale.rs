use crate::{ChromaticTone, Scale};

#[derive(Default, PartialEq, Clone, Copy, Debug)]
pub struct ChromaticScale {}

impl Scale for ChromaticScale {
    type Tones = ChromaticTone;

    fn tones_per_octave(&self) -> usize {
        12
    }
}
