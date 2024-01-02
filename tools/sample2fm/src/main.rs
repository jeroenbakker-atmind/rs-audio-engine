use audio_engine_common::{
    digital_sound::sound::Sound,
    envelope::delay_attack_hold_decay_sustain_release::DelayAttackHoldDecaySustainRelease,
    waveform::Waveform,
};
use audio_engine_instrument_fm::{
    algorithm::preset::Algorithm,
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

fn calculate_distance(a: &[f32], b: &[f32]) -> f64 {
    a.iter().zip(b).map(|(a, b)| (a - b).abs() as f64).sum()
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

    for operator_1 in &operators {
        for operator_2 in &operators {
            for operator_3 in &operators {
                for operator_4 in &operators {
                    for operator_5 in &operators {
                        for operator_6 in &operators {
                            for algorithm in 0..32 {
                                let algorithm_preset = Algorithm::DX7(algorithm + 1);
                                for repeat in 0..7 {
                                    let mut instrument =
                                        FMInstrument::<DelayAttackHoldDecaySustainRelease> {
                                            algorithm_preset,
                                            repeat,
                                            operators:
                                                Operators::<DelayAttackHoldDecaySustainRelease> {
                                                    a: *operator_1,
                                                    b: *operator_2,
                                                    c: *operator_3,
                                                    d: *operator_4,
                                                    e: *operator_5,
                                                    f: *operator_6,
                                                },
                                                    ..FMInstrument::<
                                                        DelayAttackHoldDecaySustainRelease,
                                                    >::default(
                                                    )
                                        };
                                    instrument.compile();

                                    let mut sound_state = instrument.init_sound_state();
                                    let fm_samples = (0..extracted_samples.len())
                                        .map(|index| index as f32 / sample_rate)
                                        .map(|note_time| {
                                            instrument.sample(
                                                note_time,
                                                None,
                                                note_pitch,
                                                sample_rate,
                                                &mut sound_state,
                                            )
                                        })
                                        .collect::<Vec<f32>>();
                                    let distance =
                                        calculate_distance(extracted_samples, &fm_samples);

                                    if distance < best_distance {
                                        best_distance = distance;
                                        best_option = Some(instrument);
                                        num_options += 1;
                                        println!(
                                            "option: {num_options} distance: {best_distance:#?}"
                                        );
                                    }
                                }
                            }
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

    for ratio_i in 0..32 {
        let ratio = (ratio_i + 1) as f32 / 4.0;
        for level_i in 0..32 {
            let level = (level_i + 1) as f32 / 4.0;
            result.push(Operator::<DelayAttackHoldDecaySustainRelease> {
                enable: true,
                frequency: OperatorFrequency::Rated(ratio),
                level,
                waveform: Waveform::Sine,
                ..Operator::<DelayAttackHoldDecaySustainRelease>::default()
            });
        }
    }
    result.push(Operator::<DelayAttackHoldDecaySustainRelease> {
        enable: false,
        ..Operator::<DelayAttackHoldDecaySustainRelease>::default()
    });

    result
}
