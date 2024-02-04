use audio_engine_common::{level::Level, note_duration::NoteDuration};
use audio_engine_notes::Pitch;
use audio_engine_sequencer::instrument_index::InstrumentIndex;

use crate::link_selection::LinkSelection;

#[derive(Debug, Copy, Clone)]
pub struct Node {
    pub instrument: InstrumentIndex,
    pub note_level: Level,
    pub note_pitch: Pitch,
    pub repeat: usize,
    pub repeat_delay: NoteDuration,
    pub link_selection: LinkSelection,
}

impl Default for Node {
    fn default() -> Self {
        Self {
            instrument: 0.into(),
            note_pitch: 440.0.into(),
            note_level: 1.0,
            repeat: 0,
            repeat_delay: NoteDuration::default(),
            link_selection: LinkSelection::default(),
        }
    }
}
