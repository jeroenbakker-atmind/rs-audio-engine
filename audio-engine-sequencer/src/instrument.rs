use audio_engine_common::{
    digital_sound::sound::Sound,
    envelope::delay_attack_hold_decay_sustain_release::DelayAttackHoldDecaySustainRelease, id::ID,
    note_time::NoteTime,
};
use audio_engine_instrument_fm::instrument::FMInstrument;
use audio_engine_instrument_piano::instrument::PianoInstrument;
use audio_engine_instrument_sample::sample::Sample;

use crate::instrument_note_state::InstrumentNoteState;

#[derive(Debug, Default, Clone)]
pub enum Instrument {
    #[default]
    None,
    FM(FMInstrument<DelayAttackHoldDecaySustainRelease>),
    Sample(Sample),
    Piano(PianoInstrument),
}

pub type InstrumentID = ID;

impl Sound for Instrument {
    type SoundState = InstrumentNoteState;
    fn init_sound_state(&self) -> Self::SoundState {
        match self {
            Self::None => InstrumentNoteState::None,
            Self::FM(fm) => InstrumentNoteState::FM(fm.init_sound_state()),
            Self::Sample(sample) => InstrumentNoteState::Sample(sample.init_sound_state()),
            Self::Piano(piano) => InstrumentNoteState::Piano(piano.init_sound_state()),
        }
    }

    fn sample(
        &self,
        note_time: NoteTime,
        note_off: Option<NoteTime>,
        note_pitch: f32,
        sample_rate: f32,
        state: &mut Self::SoundState,
    ) -> f32 {
        match self {
            Instrument::FM(instrument) => {
                if let InstrumentNoteState::FM(state) = state {
                    instrument.sample(note_time, note_off, note_pitch, sample_rate, state)
                } else {
                    0.0
                }
            }
            Instrument::Piano(piano) => {
                if let InstrumentNoteState::Piano(state) = state {
                    piano.sample(note_time, note_off, note_pitch, sample_rate, state)
                } else {
                    0.0
                }
            }
            Instrument::Sample(sample) => {
                if let InstrumentNoteState::Sample(state) = state {
                    sample.sample(note_time, note_off, note_pitch, sample_rate, state)
                } else {
                    0.0
                }
            }
            Instrument::None => 0.0,
        }
    }
}
