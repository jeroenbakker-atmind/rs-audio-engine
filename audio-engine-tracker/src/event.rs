use audio_engine_notes::ChromaticNote;
use audio_engine_sequencer::instrument::InstrumentID;

#[derive(Debug, Default, Copy, Clone)]
pub enum Event {
    #[default]
    Empty,
    NoteOn(ChromaticNote, InstrumentID),
    NoteRelease,
    NoteOff,

    PatternEnd,
}
