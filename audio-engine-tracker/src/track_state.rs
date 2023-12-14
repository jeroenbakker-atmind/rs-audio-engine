use audio_engine_common::{level::Level, note_time::NoteTime};
use audio_engine_effect_delay::delay_state::DelayState;
use audio_engine_effect_distortion::distortion_state::DistortionState;
use audio_engine_sequencer::{
    instrument::InstrumentID, instrument_note_state::InstrumentNoteState,
};

#[derive(Clone)]
pub struct TrackState {
    /// Row index calculated from the speed from the start of the playback.
    pub global_row_index: u32,

    pub instrument_id: InstrumentID,
    pub instrument_note_state: InstrumentNoteState,
    pub note_pitch: f32,
    pub note_on: Option<NoteTime>,
    pub note_off: Option<NoteTime>,
    pub level: Level,

    pub delay_state: DelayState,
    pub distortion_state: DistortionState,
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
            delay_state: DelayState::default(),
            distortion_state: DistortionState::default(),
        }
    }
}
