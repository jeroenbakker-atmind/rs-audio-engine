use crate::{
    sample::{NoUserData, SampleGenerator},
    Time,
};

pub enum Waveform {
    Sine,
    Block,
    Saw,
}

impl SampleGenerator for Waveform {
    type U = NoUserData;

    fn sample(
        &self,
        note_time: Time,
        _note_off: Option<Time>,
        frequency: f32,
        _user_data: &Self::U,
    ) -> f32 {
        let block_time = (note_time * frequency) % 1.0;
        match self {
            Waveform::Sine => (block_time * std::f32::consts::TAU).sin(),
            Waveform::Block => {
                if block_time < 0.5 {
                    -1.0
                } else {
                    1.0
                }
            }
            Waveform::Saw => block_time * 2.0 - 1.0,
        }
    }
}
