use crate::{
    operator::Operators,
    sample::{SampleGenerator, NO_USER_DATA},
    Time,
};

pub enum Algorithm {
    /// Output carrier
    A,

    // B modulates carrier A
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
                user_data.b.modulate(note_time, note_off, frequency),
                &NO_USER_DATA,
            ),
        }
    }
}
