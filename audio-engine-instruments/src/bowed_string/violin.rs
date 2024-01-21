use audio_engine_instrument_bowed_string::instrument::BowedStringInstrument;
use audio_engine_instrument_bowed_string::string::{
    VIOLIN_STRING_A4, VIOLIN_STRING_D4, VIOLIN_STRING_E5, VIOLIN_STRING_G3,
};
use audio_engine_notes::{ChromaticNote, ChromaticTone, Pitch};
use audio_engine_sequencer::instrument::Instrument;

pub fn create_bowed_string_violin_instrument() -> Instrument {
    let mut cello = BowedStringInstrument::default();

    cello.add_string(
        VIOLIN_STRING_E5,
        Pitch::from(ChromaticNote::new(ChromaticTone::E, 5).pitch() as f64),
    );
    cello.add_string(
        VIOLIN_STRING_A4,
        Pitch::from(ChromaticNote::new(ChromaticTone::A, 4).pitch() as f64),
    );
    cello.add_string(
        VIOLIN_STRING_D4,
        Pitch::from(ChromaticNote::new(ChromaticTone::D, 4).pitch() as f64),
    );
    cello.add_string(
        VIOLIN_STRING_G3,
        Pitch::from(ChromaticNote::new(ChromaticTone::G, 3).pitch() as f64),
    );

    Instrument::BowedString(cello)
}
