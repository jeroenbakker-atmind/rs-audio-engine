use audio_engine_common::digital_sound::{sound::Sound, sound_state::SoundState};
use audio_engine_instrument_bowed_string::{
    instrument_state::BowedStringInstrumentState, processor::DefaultStringProcessor,
};
use audio_engine_instrument_fm::instrument::FMInstrumentNoteState;
use audio_engine_instrument_piano::note_state::PianoNoteState;
use audio_engine_instrument_sample::sample_note_state::SampleNoteState;

use crate::instrument::Instrument;

#[derive(Default, Clone)]
pub enum InstrumentNoteState {
    #[default]
    None,
    FM(FMInstrumentNoteState),
    Sample(SampleNoteState),
    Piano(PianoNoteState),
    BowedString(BowedStringInstrumentState<DefaultStringProcessor>),
}

impl InstrumentNoteState {
    fn is_none(&self) -> bool {
        match self {
            InstrumentNoteState::None => true,
            _ => false,
        }
    }
}

impl SoundState for InstrumentNoteState {}

impl InstrumentNoteState {
    pub fn reset(&mut self, instrument: Option<&Instrument>) {
        match instrument {
            None | Some(Instrument::None) => *self = Self::None,
            Some(Instrument::FM(instrument)) => *self = Self::FM(instrument.init_sound_state()),
            Some(Instrument::Sample(instrument)) => {
                *self = Self::Sample(instrument.init_sound_state())
            }
            Some(Instrument::Piano(piano)) => *self = Self::Piano(piano.init_sound_state()),
            Some(Instrument::BowedString(instrument)) => {
                if self.is_none() {
                    *self = Self::BowedString(instrument.init_sound_state());
                }
            }
        }
    }
}
