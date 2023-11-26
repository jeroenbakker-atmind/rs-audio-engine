use crate::{
    algorithm::Algorithm,
    operator::{Operators, OperatorsNoteState},
};

pub struct Instrument {
    pub operators: Operators,
    pub algorithm: Algorithm,
}

impl Instrument {
    pub fn sample(
        &self,
        note_time: crate::Time,
        note_off: Option<crate::Time>,
        frequency: f32,
        state: &mut InstrumentNoteState,
    ) -> f32 {
        self.algorithm.sample(
            note_time,
            note_off,
            frequency,
            &self.operators,
            &mut state.operators,
        )
    }
}

#[derive(Default)]
pub struct InstrumentNoteState {
    pub operators: OperatorsNoteState,
}
