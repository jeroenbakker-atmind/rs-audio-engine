use audio_engine_instrument_bowed_string::{instrument::BowedStringInstrument, string::String};
use audio_engine_sequencer::instrument::Instrument;

pub fn create_bowed_string_cello_instrument() -> Instrument {
    let mut cello = BowedStringInstrument::default();

    cello.string = String {
        radius: 7.2e-4,
        density: 1.3017e4,
        tension: 172.74,
        young_mod: 22.4e9,
        length: 0.69,
    };
    
    Instrument::BowedString(cello)
}
