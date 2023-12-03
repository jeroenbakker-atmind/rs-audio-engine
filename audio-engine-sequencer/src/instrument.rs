use audio_engine_common::{
    envelope::delay_attack_hold_decay_sustain_release::DelayAttackHoldDecaySustainRelease, id::ID,
    note_time::NoteTime,
};
use audio_engine_fm::instrument::{Instrument as FMInstrument, InstrumentNoteState};

#[derive(Default, Copy, Clone)]
pub enum Instrument {
    #[default]
    None,
    FM(FMInstrument<DelayAttackHoldDecaySustainRelease>),
    // Sample(SampleInstrument),
}

pub type InstrumentID = ID<Instrument>;

impl Instrument {
    pub fn sample(
        &self,
        note_time: NoteTime,
        note_off: Option<NoteTime>,
        frequency: f32,
        note_state: &mut InstrumentNoteState,
    ) -> f32 {
        match self {
            Instrument::FM(instrument) => {
                instrument.sample(note_time, note_off, frequency, note_state)
            }
            Instrument::None => 0.0,
        }
    }
}
