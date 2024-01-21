use audio_engine_instrument_bowed_string::{
    instrument::BowedStringInstrument,
    string::{CELLO_STRING_A3, CELLO_STRING_C2, CELLO_STRING_D3, CELLO_STRING_G2},
};
use audio_engine_notes::{ChromaticNote, ChromaticTone, Pitch};
use audio_engine_sequencer::instrument::Instrument;

pub fn create_bowed_string_cello_instrument() -> Instrument {
    let mut cello = BowedStringInstrument::default();

    cello.add_string(
        CELLO_STRING_A3,
        Pitch::from(ChromaticNote::new(ChromaticTone::A, 3).pitch() as f64),
    );
    cello.add_string(
        CELLO_STRING_D3,
        Pitch::from(ChromaticNote::new(ChromaticTone::D, 3).pitch() as f64),
    );
    cello.add_string(
        CELLO_STRING_G2,
        Pitch::from(ChromaticNote::new(ChromaticTone::G, 2).pitch() as f64),
    );
    cello.add_string(
        CELLO_STRING_C2,
        Pitch::from(ChromaticNote::new(ChromaticTone::C, 2).pitch() as f64),
    );

    Instrument::BowedString(cello)
}
