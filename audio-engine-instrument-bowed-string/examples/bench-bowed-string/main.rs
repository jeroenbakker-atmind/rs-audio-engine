use audio_engine_common::digital_sound::{
    benchmark::bench_realtime_factor_single, parameters::NoteParameters,
};
use audio_engine_instrument_bowed_string::{
    instrument::BowedStringInstrument, string::CELLO_STRING_G2,
};
use audio_engine_notes::{ChromaticNote, ChromaticTone};

fn main() {
    let mut sound = BowedStringInstrument::default();
    let pitch = ChromaticNote::new(ChromaticTone::G, 2).pitch();
    sound.add_string(CELLO_STRING_G2, pitch);

    let mut parameters = NoteParameters {
        note_time: 0.0,
        note_off: None,
        note_pitch: pitch,
        gain: 1.0,
        sample_rate: 44100.0,
    };

    let realtime_factor = bench_realtime_factor_single(&sound, &mut parameters, 5.0);
    println!("Benchmark BowedString: realtime_factor={realtime_factor}");
}
