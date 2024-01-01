use audio_engine_instrument_sample::sample::Sample;
use audio_engine_sequencer::instrument::Instrument;

pub fn create_sample_pianos_piano_ax() -> Instrument {
    let instrument = Sample {
        start: 0,
        end: crate::samples::piano_ax::SAMPLES.len() - 1,
        is_looped: false,
        loop_start: 0,
        loop_end: 0,
        sample_rate_c4: 44100.0,
        data: crate::samples::piano_ax::SAMPLES.as_slice(),
    };
    Instrument::Sample(instrument)
}
