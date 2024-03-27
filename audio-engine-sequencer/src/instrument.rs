use audio_engine_common::{
    digital_sound::{parameters::NoteParameters, sound::Sound},
    envelope::delay_attack_hold_decay_sustain_release::DelayAttackHoldDecaySustainRelease,
    id::ID,
};
use audio_engine_instrument_bowed_string::{
    instrument::BowedStringInstrument, processor::DefaultStringProcessor,
};
use audio_engine_instrument_fm::instrument::FMInstrument;
use audio_engine_instrument_piano::{instrument::PianoInstrument, instrument2::PianoInstrument2};
use audio_engine_instrument_sample::sample::Sample;

use crate::instrument_note_state::InstrumentNoteState;

#[derive(Debug, Default, Clone)]
pub enum Instrument {
    #[default]
    None,
    FM(FMInstrument<DelayAttackHoldDecaySustainRelease>),
    Sample(Sample),
    Piano(PianoInstrument),
    Piano2(PianoInstrument2),
    BowedString(BowedStringInstrument<DefaultStringProcessor>),
}

pub type InstrumentID = ID;

impl Sound for Instrument {
    type SoundState = InstrumentNoteState;
    type Parameters = NoteParameters;

    fn init_sound_state(&self) -> Self::SoundState {
        match self {
            Self::None => InstrumentNoteState::None,
            Self::FM(fm) => InstrumentNoteState::FM(fm.init_sound_state()),
            Self::Sample(sample) => InstrumentNoteState::Sample(sample.init_sound_state()),
            Self::Piano(piano) => InstrumentNoteState::Piano(piano.init_sound_state()),
            Self::Piano2(piano) => InstrumentNoteState::Piano2(piano.init_sound_state()),
            Self::BowedString(bowed_string) => {
                InstrumentNoteState::BowedString(bowed_string.init_sound_state())
            }
        }
    }

    fn sample(&self, parameters: &Self::Parameters, state: &mut Self::SoundState) -> f32 {
        match self {
            Instrument::FM(instrument) => {
                if let InstrumentNoteState::FM(state) = state {
                    instrument.sample(parameters, state)
                } else {
                    0.0
                }
            }
            Instrument::Piano(piano) => {
                if let InstrumentNoteState::Piano(state) = state {
                    piano.sample(parameters, state)
                } else {
                    0.0
                }
            }
            Instrument::Piano2(piano) => {
                if let InstrumentNoteState::Piano2(state) = state {
                    piano.sample(parameters, state)
                } else {
                    0.0
                }
            }
            Instrument::Sample(sample) => {
                if let InstrumentNoteState::Sample(state) = state {
                    sample.sample(parameters, state)
                } else {
                    0.0
                }
            }
            Instrument::BowedString(bowed_string) => {
                if let InstrumentNoteState::BowedString(state) = state {
                    bowed_string.sample(parameters, state)
                } else {
                    0.0
                }
            }
            Instrument::None => 0.0,
        }
    }
}
