use crate::Time;
use audio_engine_common::{
    envelope::Envelope, level::Level, phase_time::PhaseTime, waveform::Waveform,
};

#[derive(Debug, Clone)]
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
        sample_rate: f32,
        state: &mut OperatorNoteState,
    ) -> f32 {
        frequency + self.sample(note_time, note_off, frequency, sample_rate, state)
    }

    pub fn sample(
        &self,
        note_time: Time,
        note_off: Option<Time>,
        frequency: f32,
        sample_rate: f32,
        state: &mut OperatorNoteState,
    ) -> f32 {
        let result = self.waveform.sample(&(state.phase_time + self.phase));
        self.waveform
            .advance(&mut state.phase_time, frequency, sample_rate);
        result * self.envelope.level(note_time, note_off) * self.level
    }
}

#[derive(Debug, Clone)]
pub struct Operators<E>
where
    E: Envelope,
{
    pub a: Operator<E>,
    pub b: Operator<E>,
    pub c: Operator<E>,
    pub d: Operator<E>,
}

impl<E> Copy for Operator<E> where E: Envelope + Copy {}
impl<E> Copy for Operators<E> where E: Envelope + Copy {}

impl<E> Default for Operators<E>
where
    E: Envelope + Default,
{
    fn default() -> Self {
        Operators {
            a: Operator::<E>::default(),
            b: Operator::<E>::default(),
            c: Operator::<E>::default(),
            d: Operator::<E>::default(),
        }
    }
}

#[derive(Default, Copy, Clone)]
pub struct OperatorNoteState {
    pub phase_time: PhaseTime,
}

impl OperatorNoteState {
    pub fn reset(&mut self) {
        self.phase_time = PhaseTime::default();
    }
}

#[derive(Default, Copy, Clone)]
pub struct OperatorsNoteState {
    pub a: OperatorNoteState,
    pub b: OperatorNoteState,
    pub c: OperatorNoteState,
    pub d: OperatorNoteState,
}

impl OperatorsNoteState {
    pub fn reset(&mut self) {
        self.a.reset();
        self.b.reset();
        self.c.reset();
        self.d.reset();
    }
}
