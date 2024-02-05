use audio_engine_common::{
    level::Level,
    note_duration::{NoteDuration, NoteDurationType},
};
use audio_engine_notes::Pitch;
use audio_engine_sequencer::instrument_index::InstrumentIndex;

use crate::link_selection::LinkSelection;

#[derive(Debug, Copy, Clone)]
pub struct Node {
    pub grid_location: (f32, f32),
    pub instrument: InstrumentIndex,
    pub note_pitch: Pitch,
    pub note_level: Level,
    /// Duration of a single played note. This excludes repeats. When repeat is active the note_duration will be applied to all repeated notes.
    pub note_duration: NoteDuration,
    pub repeat: usize,
    pub repeat_delay: NoteDuration,
    pub link_selection: LinkSelection,
}

impl Default for Node {
    fn default() -> Self {
        Self {
            grid_location: (0.0, 0.0),
            instrument: 0.into(),
            note_pitch: 440.0.into(),
            note_level: 1.0,
            repeat: 0,
            note_duration: NoteDuration::Duration(NoteDurationType::Full, 1),
            repeat_delay: NoteDuration::default(),
            link_selection: LinkSelection::default(),
        }
    }
}
