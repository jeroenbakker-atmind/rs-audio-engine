use crate::Time;

pub struct NoUserData {}
pub const NO_USER_DATA: NoUserData = NoUserData {};

pub trait SampleGenerator {
    type U: Sized;
    fn sample(
        &self,
        note_time: Time,
        note_off: Option<Time>,
        frequency: f32,
        user_data: &Self::U,
    ) -> f32;
}
