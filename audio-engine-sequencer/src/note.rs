use audio_engine_common::envelope::attack_hold_decay_sustain_release::AttackHoldDecaySustainRelease;
use audio_engine_fm::{
    instrument::{Instrument, InstrumentNoteState},
    Time,
};

pub struct Note {
    pub instrument: Instrument<AttackHoldDecaySustainRelease>,
    pub instrument_state: InstrumentNoteState,
    pub frequency: f32,
    pub note_on: Time,
    pub note_off: Option<Time>,
}
