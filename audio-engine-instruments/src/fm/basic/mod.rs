use audio_engine_common::{
    envelope::delay_attack_hold_decay_sustain_release::DelayAttackHoldDecaySustainRelease,
    waveform::Waveform,
};
use audio_engine_instrument_fm::{
    algorithm::Algorithm,
    instrument::FMInstrument,
    operator::{Operator, Operators},
    operator_frequency::RATED_1,
};
use audio_engine_sequencer::instrument::Instrument;

pub mod saw_ramp_down;
pub mod saw_ramp_up;
pub mod sine;
pub mod square;
pub mod triangle;

fn create_fm_waveform_instrument(waveform: Waveform) -> Instrument {
    let instrument = FMInstrument::<DelayAttackHoldDecaySustainRelease> {
        operators: Operators::<DelayAttackHoldDecaySustainRelease> {
            a: Operator {
                waveform,
                frequency: RATED_1,
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
            ..Operators::default()
        },
        algorithm: Algorithm::A,
    };
    Instrument::FM(instrument)
}
