use audio_engine_common::{
    envelope::delay_attack_hold_decay_sustain_release::DelayAttackHoldDecaySustainRelease,
    phase_time::PhaseTime, waveform::Waveform,
};
use audio_engine_fm::{
    algorithm::Algorithm,
    instrument::Instrument as FMInstrument,
    operator::{Operator, Operators},
};
use audio_engine_sequencer::instrument::Instrument;

pub fn create_fm_basic_sine_instrument() -> Instrument {
    let instrument = FMInstrument::<DelayAttackHoldDecaySustainRelease> {
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
                phase: PhaseTime::default(),
            },
            ..Operators::default()
        },
        algorithm: Algorithm::A,
    };
    Instrument::FM(instrument)
}
