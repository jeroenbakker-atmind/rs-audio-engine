use audio_engine_instrument_piano::instrument::PianoInstrument;
use audio_engine_sequencer::instrument::Instrument;

pub fn create_piano_piano_instrument() -> Instrument {
    let piano = PianoInstrument::default();
    Instrument::Piano(piano)
}
