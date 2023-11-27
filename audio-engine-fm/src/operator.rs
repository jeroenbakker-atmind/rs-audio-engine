use crate::{envelope::Envelope, waveform::Waveform, Level, Time};
use audio_engine_common::phase_time::PhaseTime;

pub struct Operator {
    pub waveform: Waveform,
    pub envelope: Envelope,
    pub rate: f32,
    pub level: Level,
    pub phase: PhaseTime,
}

impl Default for Operator {
    fn default() -> Self {
        Operator {
            waveform: Waveform::Sine,
            envelope: Envelope::default(),
            rate: 1.0,
            level: 0.0,
            phase: PhaseTime::default(),
        }
    }
}

impl Operator {
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
        let result = self.waveform.sample(&(state.phase_time + self.phase))
            * self.envelope.level(note_time, note_off)
            * self.level;
        self.waveform
            .advance(&mut state.phase_time, frequency, 44100.0);
        result
    }
}

pub struct Operators {
    pub a: Operator,
    pub b: Operator,
    pub c: Operator,
    pub d: Operator,
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