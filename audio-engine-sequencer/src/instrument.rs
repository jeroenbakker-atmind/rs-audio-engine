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
            Instrument::FM(instrument) => instrument.sample(
                note_time,
                note_off,
                note_pitch,
                sample_rate,
                &mut note_state.fm_note_state,
            ),
            Instrument::Sample(sample) => sample.sample(
                note_time,
                note_off,
                note_pitch,
                sample_rate,
                &mut note_state.sample_note_state,
            ),
            Instrument::None => 0.0,
        }
    }
}
