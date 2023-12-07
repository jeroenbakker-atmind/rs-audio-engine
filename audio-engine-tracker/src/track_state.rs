use audio_engine_common::{level::Level, note_time::NoteTime};
use audio_engine_sample::sample_note_state::SampleNoteState;
use audio_engine_sequencer::instrument::InstrumentID;

#[derive(Copy, Clone)]
pub struct TrackState {
    /// Row index calculated from the speed from the start of the playback.
    pub global_row_index: u32,

    pub instrument_id: InstrumentID,
    pub instrument_note_state: audio_engine_fm::instrument::InstrumentNoteState,
    pub sample_note_state: SampleNoteState,
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
            instrument_note_state: audio_engine_fm::instrument::InstrumentNoteState::default(),
            sample_note_state: SampleNoteState::default(),
            note_pitch: 0.0,
            note_on: None,
            note_off: None,
            level: 0.0,
        }
    }
}
