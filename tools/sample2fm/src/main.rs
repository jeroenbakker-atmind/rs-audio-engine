use audio_engine_common::{
    digital_sound::{parameters::NoteParameters, sound::Sound},
    envelope::delay_attack_hold_decay_sustain_release::DelayAttackHoldDecaySustainRelease,
    waveform::Waveform,
};
use audio_engine_instrument_fm::{
    algorithm::{
        compiled::CompiledAlgorithm,
        preset::{Algorithm, FM_ALGORITHM_BASIC_A},
    },
    instrument::FMInstrument,
    operator::{Operator, Operators},
    operator_frequency::OperatorFrequency,
};
use audio_engine_instruments::InstrumentLibrary;
use audio_engine_notes::{ChromaticNote, ChromaticTone};
use audio_engine_sequencer::instrument::Instrument;

fn main() {
    let sample_rate = 44100.0;
    let pitch_c4 = ChromaticNote::new(ChromaticTone::C, 4).pitch();
    let piano_ax = InstrumentLibrary::SamplePianosPianoAX.create();
    if let Instrument::Sample(sample) = piano_ax {
        let samples = sample.data;

        let extracted_samples = extract(samples);
        println!("{extracted_samples:#?}");
        let best_option = find_best_instrument(&extracted_samples, pitch_c4, sample_rate);
        println!("{best_option:#?}");
    }
}

fn extract(samples: &[f32]) -> Vec<f32> {
    let mut result = Vec::new();
    let wave_start = find_wave_start(samples);
    let wave_length = 44100 / ChromaticNote::new(ChromaticTone::C, 4).pitch() as usize;
    let num_waves = 10;

    let mut wave_end = wave_start + num_waves * wave_length;
    while samples[wave_end] > 0.0 {
        wave_end -= 1
    }
    while samples[wave_end] < 0.0 {
        wave_end += 1
    }

    result.extend_from_slice(&samples[wave_start..=wave_end]);

    result
}

fn find_wave_start(samples: &[f32]) -> usize {
    samples
        .iter()
        .skip_while(|sample| **sample > -0.5)
        .position(|sample| *sample < 0.0)
        .unwrap()
}

fn find_best_instrument(
    extracted_samples: &[f32],
    note_pitch: f32,
    sample_rate: f32,
) -> Option<FMInstrument<DelayAttackHoldDecaySustainRelease>> {
    let operators = generate_operators();
    println!("generated {} operators", operators.len());

    let mut num_options = 0;
    let mut best_distance = f64::MAX;
    let mut best_option = None;

    let compiled_algorithms = (1..=32)
        .zip(0..=7)
        .map(|(alg, repeat)| Algorithm::DX7(alg).compile(repeat))
        .collect::<Vec<CompiledAlgorithm>>();

    for operator_1 in &operators {
        for operator_2 in &operators {
            for operator_3 in &operators {
                for operator_4 in &operators {
                    for operator_5 in &operators {
                        for operator_6 in &operators {
                            let instruments = compiled_algorithms
                                .iter()
                                .map(|compiled_algorithm| FMInstrument::<
                                    DelayAttackHoldDecaySustainRelease,
                                > {
                                    algorithm: Some(compiled_algorithm.clone()),
                                    algorithm_preset: FM_ALGORITHM_BASIC_A,
                                    repeat: 0,
                                    operators: Operators::<DelayAttackHoldDecaySustainRelease> {
                                        a: *operator_1,
                                        b: *operator_2,
                                        c: *operator_3,
                                        d: *operator_4,
                                        e: *operator_5,
                                        f: *operator_6,
                                    },
                                })
                                .collect::<Vec<FMInstrument<DelayAttackHoldDecaySustainRelease>>>();

                            let distances = instruments
                                .iter()
                                .map(|instrument| {
                                    let mut sound_state = instrument.init_sound_state();
                                    let mut distance = 0.0;
                                    for (index, sample) in extracted_samples.iter().enumerate() {
                                        let note_time = index as f32 / sample_rate;
                                        let fm_sample = instrument.sample(
                                            &NoteParameters {
                                                note_time,
                                                note_off: None,
                                                note_pitch,
                                                sample_rate,
                                            },
                                            &mut sound_state,
                                        );
                                        let sample_distance = (sample - fm_sample).abs();
                                        distance += sample_distance as f64;
                                        if distance > best_distance {
                                            distance = f64::MAX;
                                            break;
                                        }
                                    }

                                    distance
                                })
                                .collect::<Vec<f64>>();

                            instruments
                                .iter()
                                .zip(distances)
                                .for_each(|(instrument, distance)| {
                                    if distance < best_distance {
                                        best_distance = distance;
                                        best_option = Some(instrument.clone());
                                        num_options += 1;
                                        println!(
                                            "option: {num_options} distance: {best_distance:#?}"
                                        );
                                    }
                                })
                        }
                    }
                }
            }
        }
    }
    best_option
}

fn generate_operators() -> Vec<Operator<DelayAttackHoldDecaySustainRelease>> {
    let mut result = Vec::new();

    for ratio in [
        0.1, 0.2, 0.4, 0.5, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0,
        14.0, 15.0, 16.0, 0.3, 0.6, 0.7, 0.8, 0.9,
    ] {
        for level in [
            0.0, 0.1, 0.2, 0.5, 1.0, 2.0, 3.0, 4.0, 6.0, 8.0, 12.0, 16.0, 24.0, 32.0, 64.0, 0.4,
            0.6, 0.7, 0.8, 0.9,
        ] {
            result.push(Operator::<DelayAttackHoldDecaySustainRelease> {
                enable: true,
                frequency: OperatorFrequency::Rated(ratio),
                level,
                waveform: Waveform::Sine,
                ..Operator::<DelayAttackHoldDecaySustainRelease>::default()
            });
        }
    }
    result
}
