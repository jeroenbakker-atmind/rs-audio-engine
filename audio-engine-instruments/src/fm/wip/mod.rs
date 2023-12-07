use audio_engine_common::envelope::delay_attack_hold_decay_sustain_release::DelayAttackHoldDecaySustainRelease;
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
                rate: 2.0,
                level: 1.0,
                ..Operator::default()
            },
            b: Operator {
                rate: 4.0,
                level: 1.0,
                ..Operator::default()
            },
            c: Operator {
                rate: 12.0,
                level: 1.0,
                ..Operator::default()
            },
            d: Operator {
                rate: 1.0,
                level: 128.0,
                ..Operator::default()
            },
            ..Operators::default()
        },
        algorithm: Algorithm::DModulatesABC,
    };
    Instrument::FM(instrument)
}
