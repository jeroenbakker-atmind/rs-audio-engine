use crate::Time;
use audio_engine_common::{
    envelope::Envelope, level::Level, phase_time::PhaseTime, waveform::Waveform,
};

pub struct Operator<E>
where
    E: Envelope,
{
    pub waveform: Waveform,
    pub envelope: E,
    pub rate: f32,
    pub level: Level,
    pub phase: PhaseTime,
}

impl<E> Default for Operator<E>
where
    E: Envelope + Default,
{
    fn default() -> Self {
        Operator {
            waveform: Waveform::Sine,
            envelope: E::default(),
            rate: 1.0,
            level: 0.0,
            phase: PhaseTime::default(),
        }
    }
}

impl<E> Operator<E>
where
    E: Envelope,
{
    pub fn modulate(
        &self,
        note_time: Time,
        note_off: Option<Time>,
        frequency: f32,
        state: &mut OperatorNoteState,
    ) -> f32 {
        frequency + self.sample(note_time, note_off, frequency, state)
    }

    pub fn sample(
        &self,
        note_time: Time,
        note_off: Option<Time>,
        frequency: f32,
        state: &mut OperatorNoteState,
    ) -> f32 {
        let result = self.waveform.sample(&(state.phase_time + self.phase));
        self.waveform
            .advance(&mut state.phase_time, frequency, 44100.0);
        result * self.envelope.level(note_time, note_off) * self.level
    }
}

pub struct Operators<E>
where
    E: Envelope,
{
    pub a: Operator<E>,
    pub b: Operator<E>,
    pub c: Operator<E>,
    pub d: Operator<E>,
}

#[derive(Default)]
pub struct OperatorNoteState {
    pub phase_time: PhaseTime,
}

#[derive(Default)]
pub struct OperatorsNoteState {
    pub a: OperatorNoteState,
    pub b: OperatorNoteState,
    pub c: OperatorNoteState,
    pub d: OperatorNoteState,
}
