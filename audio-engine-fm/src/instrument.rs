use crate::{
    algorithm::Algorithm,
    operator::Operators,
    sample::{NoUserData, SampleGenerator},
};

pub struct Instrument {
    pub operators: Operators,
    pub algorithm: Algorithm,
}

impl SampleGenerator for Instrument {
    type U = NoUserData;
    fn sample(
        &self,
        note_time: crate::Time,
        note_off: Option<crate::Time>,
        frequency: f32,
        _user_data: &Self::U,
    ) -> f32 {
        self.algorithm
            .sample(note_time, note_off, frequency, &self.operators)
    }
}
