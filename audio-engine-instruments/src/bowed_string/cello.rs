use audio_engine_instrument_bowed_string::{instrument::BowedStringInstrument, string::String};
use audio_engine_sequencer::instrument::Instrument;

static CELLO_STRING_A3: String = String {
        radius: 3.75e-04,
        density: 3.7575e3,
        tension: 153.0,
        young_mod: 25e9,
        length: 0.69,
    };
static CELLO_STRING_D3: String = String {
        radius: 4.4e-04,
        density: 4.1104e3,
        tension: 102.6,
        young_mod: 25e9,
        length: 0.69,
    };
static CELLO_STRING_G2: String = String {
        radius: 6.05e-04,
        density: 5.3570e3,
        tension: 112.67,
        young_mod: 8.6e9,
        length: 0.69,
    };
static CELLO_STRING_C2: String = String {
        radius: 7.2e-4,
        density: 1.3017e4,
        tension: 172.74,
        young_mod: 22.4e9,
        length: 0.69,
    };

pub fn create_bowed_string_cello_instrument() -> Instrument {
    let mut cello = BowedStringInstrument::default();

    cello.add_string(CELLO_STRING_A3);
    cello.add_string(CELLO_STRING_D3);
    cello.add_string(CELLO_STRING_G2);
    cello.add_string(CELLO_STRING_C2);

    Instrument::BowedString(cello)
}
