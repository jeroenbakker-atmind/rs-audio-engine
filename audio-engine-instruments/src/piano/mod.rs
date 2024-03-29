use audio_engine_instrument_piano::{instrument::PianoInstrument, instrument2::PianoInstrument2};
use audio_engine_sequencer::instrument::Instrument;

pub fn create_piano_piano_instrument() -> Instrument {
    let piano = PianoInstrument::default();
    Instrument::Piano(piano)
}

pub fn create_piano_piano2_instrument() -> Instrument {
    let piano = PianoInstrument2::default();
    Instrument::Piano2(piano)
}
