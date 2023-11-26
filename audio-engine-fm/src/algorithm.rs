use crate::{
    operator::{Operators, OperatorsNoteState},
    Time,
};

pub enum Algorithm {
    /// Output carrier
    A,

    /// Output carriers A and B
    AB,

    // B modulates carrier A
    BModulatesA,
}

impl Algorithm {
    pub fn sample(
        &self,
        note_time: Time,
        note_off: Option<Time>,
        frequency: f32,
        operators: &Operators,
        operator_states: &mut OperatorsNoteState,
    ) -> f32 {
        match self {
            Algorithm::A => {
                operators
                    .a
                    .sample(note_time, note_off, frequency, &mut operator_states.a)
            }
            Algorithm::AB => {
                (operators
                    .a
                    .sample(note_time, note_off, frequency, &mut operator_states.a)
                    + operators
                        .b
                        .sample(note_time, note_off, frequency, &mut operator_states.b))
                    / 2.0
            }
            Algorithm::BModulatesA => operators.a.sample(
                note_time,
                note_off,
                operators
                    .b
                    .modulate(note_time, note_off, frequency, &mut operator_states.b),
                &mut operator_states.a,
            ),
        }
    }
}
