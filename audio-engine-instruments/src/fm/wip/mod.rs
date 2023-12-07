use audio_engine_common::envelope::delay_attack_hold_decay_sustain_release::DelayAttackHoldDecaySustainRelease;
use audio_engine_fm::{
    algorithm::Algorithm,
    instrument::Instrument as FMInstrument,
    operator::{Operator, Operators},
    operator_frequency::{RATED_1, RATED_12, RATED_2, RATED_4},
};
use audio_engine_sequencer::instrument::Instrument;

pub fn create_fm_wip_instrument() -> Instrument {
    let instrument = FMInstrument::<DelayAttackHoldDecaySustainRelease> {
        operators: Operators::<DelayAttackHoldDecaySustainRelease> {
            a: Operator {
                frequency: RATED_2,
                level: 1.0,
                ..Operator::default()
            },
            b: Operator {
                frequency: RATED_4,
                level: 1.0,
                ..Operator::default()
            },
            c: Operator {
                frequency: RATED_12,
                level: 1.0,
                ..Operator::default()
            },
            d: Operator {
                frequency: RATED_1,
                level: 16.0,
                ..Operator::default()
            },
        },
        algorithm: Algorithm::DModulatesABC,
    };
    Instrument::FM(instrument)
}
