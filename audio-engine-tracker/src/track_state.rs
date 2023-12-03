use audio_engine_common::note_time::NoteTime;
use audio_engine_sequencer::instrument::InstrumentID;

pub struct TrackState {
    /// Row index calculated from the speed from the start of the playback.
    pub global_row_index: u32,

    pub instrument_id: InstrumentID,
    pub instrument_note_state: audio_engine_fm::instrument::InstrumentNoteState,
    pub frequency: f32,
    pub note_on: Option<NoteTime>,
    pub note_off: Option<NoteTime>,
}

impl Default for TrackState {
    fn default() -> Self {
        Self {
            global_row_index: u32::MAX,
            instrument_id: InstrumentID::NotSet,
            instrument_note_state: audio_engine_fm::instrument::InstrumentNoteState::default(),
            frequency: 0.0,
            note_on: None,
            note_off: None,
        }
    }
}
