use crate::{
    operator::Operators,
    sample::{SampleGenerator, NO_USER_DATA},
    Time,
};

pub enum Algorithm {
    A,
    BModulatesA,
}

impl SampleGenerator for Algorithm {
    type U = Operators;
    fn sample(
        &self,
        note_time: Time,
        note_off: Option<Time>,
        frequency: f32,
        user_data: &Self::U,
    ) -> f32 {
        match self {
            Algorithm::A => user_data
                .a
                .sample(note_time, note_off, frequency, &NO_USER_DATA),
            Algorithm::BModulatesA => user_data.a.sample(
                note_time,
                note_off,
                frequency
                    + user_data
                        .b
                        .sample(note_time, note_off, frequency, &NO_USER_DATA),
                &NO_USER_DATA,
            ),
        }
    }
}
