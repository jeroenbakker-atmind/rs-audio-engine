use audio_engine_instrument_bowed_string::{instrument::BowedStringInstrument, string::String};
use audio_engine_notes::{ChromaticNote, ChromaticTone, Pitch};
use audio_engine_sequencer::instrument::Instrument;

static VIOLIN_STRING_E5: String = String {
    radius: 1.65e-04,
    density: 4.7936e3,
    tension: 73.0,
    young_mod: 62.5e9,
    length: 0.32,
};

static VIOLIN_STRING_A4: String = String {
    radius: 3e-4,
    density: 2.5465e+3,
    tension: 57.1,
    young_mod: 19.5e9,
    length: 0.32,
};
static VIOLIN_STRING_D4: String = String {
    radius: 4.4e-4,
    density: 2.6471e3,
    tension: 56.88,
    young_mod: 4.56e9,
    length: 0.32,
};
static VIOLIN_STRING_G3: String = String {
    radius: 4.25e-4,
    density: 4.9167e3,
    tension: 43.9,
    young_mod: 4.79e9,
    length: 0.32,
};

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
