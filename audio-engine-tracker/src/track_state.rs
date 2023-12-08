use audio_engine_common::{level::Level, note_time::NoteTime};
use audio_engine_sequencer::{
    instrument::InstrumentID, instrument_note_state::InstrumentNoteState,
};

#[derive(Copy, Clone)]
pub struct TrackState {
    /// Row index calculated from the speed from the start of the playback.
    pub global_row_index: u32,

    pub instrument_id: InstrumentID,
    pub instrument_note_state: InstrumentNoteState,
    pub note_pitch: f32,
    pub note_on: Option<NoteTime>,
    pub note_off: Option<NoteTime>,
    pub level: Level,
}

impl Default for TrackState {
    fn default() -> Self {
        Self {
            global_row_index: u32::MAX,
            instrument_id: InstrumentID::NotSet,
            instrument_note_state: InstrumentNoteState::default(),
            note_pitch: 0.0,
            note_on: None,
            note_off: None,
            level: 0.0,
        }
    }
}
