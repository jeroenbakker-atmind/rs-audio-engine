use audio_engine_common::envelope::delay_attack_hold_decay_sustain_release::DelayAttackHoldDecaySustainRelease;
use audio_engine_instrument_fm::{
    algorithm::preset::FM_ALGORITHM_BASIC_D_MOD_ABC,
    instrument::FMInstrument,
    operator::{Operator, Operators},
    operator_frequency::{RATED_1, RATED_2, RATED_4, RATED_8},
};
use audio_engine_sequencer::instrument::Instrument;

pub fn create_fm_wip_instrument() -> Instrument {
    let mut instrument = FMInstrument::<DelayAttackHoldDecaySustainRelease> {
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
                frequency: RATED_8,
                level: 1.0,
                ..Operator::default()
            },
            d: Operator {
                frequency: RATED_1,
                level: 16.0,
                ..Operator::default()
            },
            ..Operators::<DelayAttackHoldDecaySustainRelease>::default()
        },
        algorithm_preset: FM_ALGORITHM_BASIC_D_MOD_ABC,
        ..FMInstrument::<DelayAttackHoldDecaySustainRelease>::default()
    };
    instrument.compile();
    Instrument::FM(instrument)
}
