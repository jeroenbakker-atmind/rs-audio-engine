use audio_engine_common::{
    envelope::delay_attack_hold_decay_sustain_release::DelayAttackHoldDecaySustainRelease, id::ID,
    note_time::NoteTime,
};
use audio_engine_fm::instrument::FMInstrument;
use audio_engine_sample::sample::Sample;

use crate::instrument_note_state::InstrumentNoteState;

#[derive(Debug, Default, Copy, Clone)]
pub enum Instrument {
    #[default]
    None,
    FM(FMInstrument<DelayAttackHoldDecaySustainRelease>),
    Sample(Sample),
}

pub type InstrumentID = ID<Instrument>;

impl Instrument {
    pub fn sample(
        &self,
        note_time: NoteTime,
        note_off: Option<NoteTime>,
        note_pitch: f32,
        sample_rate: f32,
        note_state: &mut InstrumentNoteState,
    ) -> f32 {
        match self {
            Instrument::FM(instrument) => {
                if let InstrumentNoteState::FM(note_state) = note_state {
                    instrument.sample(note_time, note_off, note_pitch, sample_rate, note_state)
                } else {
                    0.0
                }
            }
            Instrument::Sample(sample) => {
                if let InstrumentNoteState::Sample(note_state) = note_state {
                    sample.sample(note_time, note_off, note_pitch, sample_rate, note_state)
                } else {
                    0.0
                }
            }
            Instrument::None => 0.0,
        }
    }
}
