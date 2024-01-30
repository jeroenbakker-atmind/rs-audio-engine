use audio_engine_common::digital_sound::{
    benchmark::bench_realtime_factor_single, parameters::NoteParameters,
};
use audio_engine_instrument_bowed_string::{
    instrument::BowedStringInstrument,
    processor::modal::ModalProcessor,
    processor::{modal_var1::ModalVar1Processor, DefaultStringProcessor, StringProcessor},
    string::CELLO_STRING_G2,
};
use audio_engine_notes::{ChromaticNote, ChromaticTone};

fn bench_string<P>() -> f32
where
    P: StringProcessor + Sized + Clone + Default,
{
    let mut sound = BowedStringInstrument::<P>::default();
    let pitch = ChromaticNote::new(ChromaticTone::G, 2).pitch();
    sound.add_string(CELLO_STRING_G2, pitch);

    let mut parameters = NoteParameters {
        note_time: 0.0,
        note_off: None,
        note_pitch: pitch,
        gain: 1.0,
        sample_rate: 44100.0,
    };

    bench_realtime_factor_single(&sound, &mut parameters, 5.0)
}

fn main() {
    let realtime_factor = bench_string::<DefaultStringProcessor>();
    println!("Benchmark DefaultStringProcessor: realtime_factor={realtime_factor}");
    let realtime_factor = bench_string::<ModalProcessor>();
    println!("Benchmark ModalProcessor:         realtime_factor={realtime_factor}");
    let realtime_factor = bench_string::<ModalVar1Processor>();
    println!("Benchmark ModalVar1Processor:     realtime_factor={realtime_factor}");
}
