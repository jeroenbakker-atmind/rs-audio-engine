use arguments::Arguments;
use audio_engine_notes::{ChromaticNote, ChromaticTone};
use tuner::Tuner;

mod arguments;
mod tuner;

fn main() {
    let arguments = Arguments {
        chromatic_note: ChromaticNote::new(ChromaticTone::C, 4),
        buffer_size: 4096,
    };

    let mut tuner = Tuner::new(arguments);
    tuner.start();
}
