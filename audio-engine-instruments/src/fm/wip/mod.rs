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

pub fn create_fm_wip_instrument() -> Instrument {
    let instrument = FMInstrument::<DelayAttackHoldDecaySustainRelease> {
        operators: Operators::<DelayAttackHoldDecaySustainRelease> {
            a: Operator {
                waveform: Waveform::Sine,
                rate: 2.0,
                level: 1.0,
                envelope: DelayAttackHoldDecaySustainRelease {
                    delay: 0.0,
                    attack: 0.0,
                    hold: 0.0,
                    decay: 0.0,
                    sustain: 1.0,
                    release: 0.0,
                },
                phase: PhaseTime::default(),
            },
            b: Operator {
                waveform: Waveform::Sine,
                rate: 4.0,
                level: 1.0,
                envelope: DelayAttackHoldDecaySustainRelease {
                    delay: 0.0,
                    attack: 0.0,
                    hold: 0.0,
                    decay: 0.0,
                    sustain: 1.0,
                    release: 0.0,
                },
                phase: PhaseTime::default(),
            },
            c: Operator {
                waveform: Waveform::Sine,
                rate: 12.0,
                level: 1.0,
                envelope: DelayAttackHoldDecaySustainRelease {
                    delay: 0.0,
                    attack: 0.0,
                    hold: 0.0,
                    decay: 0.0,
                    sustain: 1.0,
                    release: 0.0,
                },
                phase: PhaseTime::default(),
            },
            d: Operator {
                waveform: Waveform::Sine,
                rate: 1.0,
                level: 1.0,
                envelope: DelayAttackHoldDecaySustainRelease {
                    delay: 0.0,
                    attack: 0.0,
                    hold: 0.0,
                    decay: 0.0,
                    sustain: 1.0,
                    release: 0.0,
                },
                phase: PhaseTime::default(),
            },
            ..Operators::default()
        },
        algorithm: Algorithm::DModulatesABC,
    };
    Instrument::FM(instrument)
}
