use audio_engine_instrument_bowed_string::instrument::BowedStringInstrument;
use audio_engine_sequencer::instrument::Instrument;

pub fn create_bowed_string_cello_instrument() -> Instrument {
    let mut cello = BowedStringInstrument::default();
    Instrument::BowedString(cello)
}
