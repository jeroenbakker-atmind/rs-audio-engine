use audio_engine_common::{envelope::Envelope, note_time::NoteTime};

use crate::{
    algorithm_compiled::CompiledAlgorithm,
    operator::{Operators, OperatorsNoteState},
};

#[derive(Debug, Default, Copy, Clone)]
pub enum Algorithm {
    /// Output carrier
    #[default]
    A,

    /// Output carriers A and B
    AB,

    // B modulates carrier A
    BModulatesA,

    // D modulates Carrier A, B and C
    DModulatesABC,
}

impl Algorithm {
    pub fn compile<E>(&self) -> CompiledAlgorithm<E>
    where
        E: Envelope,
    {
        unimplemented!()
    }
}

impl Algorithm {
    pub fn sample<E>(
        &self,
        note_time: NoteTime,
        note_off: Option<NoteTime>,
        note_pitch: f32,
        sample_rate: f32,
        operators: &Operators<E>,
        operator_states: &mut OperatorsNoteState,
    ) -> f32
    where
        E: Envelope,
    {
        match self {
            Algorithm::A => operators.a.sample(
                note_time,
                note_off,
                note_pitch,
                sample_rate,
                &mut operator_states.a,
            ),
            Algorithm::AB => {
                operators.a.sample(
                    note_time,
                    note_off,
                    note_pitch,
                    sample_rate,
                    &mut operator_states.a,
                ) + operators.b.sample(
                    note_time,
                    note_off,
                    note_pitch,
                    sample_rate,
                    &mut operator_states.b,
                )
            }
            Algorithm::BModulatesA => operators.a.sample(
                note_time,
                note_off,
                operators.b.modulate(
                    note_time,
                    note_off,
                    note_pitch,
                    sample_rate,
                    &mut operator_states.b,
                ),
                sample_rate,
                &mut operator_states.a,
            ),
            Algorithm::DModulatesABC => {
                let d_result = operators.d.modulate(
                    note_time,
                    note_off,
                    note_pitch,
                    sample_rate,
                    &mut operator_states.d,
                );
                let a_result = operators.a.sample(
                    note_time,
                    note_off,
                    d_result,
                    sample_rate,
                    &mut operator_states.a,
                );
                let b_result = operators.b.sample(
                    note_time,
                    note_off,
                    d_result,
                    sample_rate,
                    &mut operator_states.b,
                );
                let c_result = operators.c.sample(
                    note_time,
                    note_off,
                    d_result,
                    sample_rate,
                    &mut operator_states.c,
                );
                (a_result + b_result + c_result) / 3.0
            }
        }
    }
}
