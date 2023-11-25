use crate::{
    envelope::Envelope,
    sample::{NoUserData, SampleGenerator},
    waveform::Waveform,
    Level, Time,
};

pub struct Operator {
    pub waveform: Waveform,
    pub envelope: Envelope,
    pub rate: f32,
    pub level: Level,
}

impl Default for Operator {
    fn default() -> Self {
        Operator {
            waveform: Waveform::Sine,
            envelope: Envelope::default(),
            rate: 1.0,
            level: 0.0,
        }
    }
}

impl SampleGenerator for Operator {
    type U = NoUserData;

    fn sample(
        &self,
        note_time: Time,
        note_off: Option<Time>,
        frequency: f32,
        user_data: &Self::U,
    ) -> f32 {
        self.waveform
            .sample(note_time, note_off, frequency * self.rate, user_data)
            * self.envelope.level(note_time, note_off)
            * self.level
    }
}

pub struct Operators {
    pub a: Operator,
    pub b: Operator,
    pub c: Operator,
    pub d: Operator,
}
