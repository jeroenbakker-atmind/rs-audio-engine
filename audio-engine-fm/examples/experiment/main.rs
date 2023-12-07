use audio_engine_common::{
    envelope::delay_attack_hold_decay_sustain_release::DelayAttackHoldDecaySustainRelease,
    waveform::Waveform,
};
use audio_engine_fm::{
    algorithm::Algorithm,
    instrument::{Instrument, InstrumentNoteState},
    operator::{Operator, Operators},
    Time,
};
use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    Sample,
};

fn main() -> Result<(), ()> {
    let host = cpal::default_host();
    let device = host.default_output_device().unwrap();
    let config = device.default_output_config().unwrap();

    play_tone(&device, &config.into())
}

fn play_tone(device: &cpal::Device, config: &cpal::StreamConfig) -> Result<(), ()> {
    let sample_rate = config.sample_rate.0 as f32;
    let channels = config.channels as usize;

    let mut sample_num = 0_u64;
    let instrument = Instrument::<DelayAttackHoldDecaySustainRelease> {
        operators: Operators::<DelayAttackHoldDecaySustainRelease> {
            a: Operator {
                waveform: Waveform::Sine,
                rate: 1.0,
                level: 1.0,
                envelope: DelayAttackHoldDecaySustainRelease {
                    delay: 0.0,
                    attack: 0.1,
                    hold: 0.0,
                    decay: 0.1,
                    sustain: 0.7,
                    release: 1.0,
                },
                ..Operator::default()
            },
            b: Operator {
                waveform: Waveform::Triangle,
                rate: 2.0,
                level: 64.0,
                ..Operator::default()
            },
            ..Operators::default()
        },
        algorithm: Algorithm::BModulatesA,
    };
    let mut instrument_state = InstrumentNoteState::default();
    let frequency = 437.0;

    let mut next_value = move || {
        sample_num += 1;
        let sample_time = sample_num as Time / sample_rate;
        instrument.sample(
            sample_time,
            None,
            frequency,
            sample_rate,
            &mut instrument_state,
        )
    };

    let err_fn = |err| eprintln!("an error occurred on stream: {}", err);

    let stream = device
        .build_output_stream(
            config,
            move |output: &mut [f32], _: &cpal::OutputCallbackInfo| {
                for frame in output.chunks_mut(channels) {
                    let value = f32::from_sample(next_value());
                    for sample in frame.iter_mut() {
                        *sample = value;
                    }
                }
            },
            err_fn,
            None,
        )
        .unwrap();
    stream.play().unwrap();

    std::thread::sleep(std::time::Duration::from_millis(10000));

    Ok(())
}
