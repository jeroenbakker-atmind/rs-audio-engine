use audio_engine_common::envelope::Envelope;

use crate::{
    algorithm::Algorithm,
    operator::{Operators, OperatorsNoteState},
};

#[derive(Debug, Copy, Clone)]
pub struct Instrument<E>
where
    E: Envelope + Copy + Clone,
{
    pub operators: Operators<E>,
    pub algorithm: Algorithm,
}

impl<E> Instrument<E>
where
    E: Envelope + Copy,
{
    pub fn sample(
        &self,
        note_time: crate::Time,
        note_off: Option<crate::Time>,
        note_pitch: f32,
        sample_rate: f32,
        state: &mut InstrumentNoteState,
    ) -> f32 {
        self.algorithm.sample(
            note_time,
            note_off,
            note_pitch,
            sample_rate,
            &self.operators,
            &mut state.operators,
        )
    }
}

#[derive(Default, Copy, Clone)]
pub struct InstrumentNoteState {
    pub operators: OperatorsNoteState,
}

impl InstrumentNoteState {
    pub fn reset(&mut self) {
        self.operators.reset();
    }
}
