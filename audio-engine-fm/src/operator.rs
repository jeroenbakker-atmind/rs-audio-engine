use crate::operator_frequency::{OperatorFrequency, RATED_1};
use audio_engine_common::{
    digital_sound::sound::Sound, envelope::Envelope, level::Level, note_time::NoteTime,
    phase_time::PhaseTime, waveform::Waveform,
};

#[derive(Debug, Clone)]
pub struct Operator<E>
where
    E: Envelope,
{
    pub enable: bool,
    pub waveform: Waveform,
    pub envelope: E,
    pub frequency: OperatorFrequency,
    pub level: Level,
    pub phase: PhaseTime,
}

impl<E> Default for Operator<E>
where
    E: Envelope + Default,
{
    fn default() -> Self {
        Operator {
            enable: true,
            waveform: Waveform::Sine,
            envelope: E::default(),
            frequency: RATED_1,
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
        note_time: NoteTime,
        note_off: Option<NoteTime>,
        note_pitch: f32,
        sample_rate: f32,
        state: &mut OperatorNoteState,
    ) -> f32 {
        note_pitch + self.sample(note_time, note_off, note_pitch, sample_rate, state)
    }

    pub fn sample(
        &self,
        note_time: NoteTime,
        note_off: Option<NoteTime>,
        note_pitch: f32,
        sample_rate: f32,
        state: &mut OperatorNoteState,
    ) -> f32 {
        if !self.enable {
            return 0.0;
        }
        // TODO: add phase when initializing the note_state
        let result = self.waveform.sample(
            note_time,
            note_off,
            note_pitch,
            sample_rate,
            &mut state.phase_time,
        );
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
