use crate::operator_frequency::{OperatorFrequency, RATED_1};
use audio_engine_common::{
    digital_sound::{parameters::NoteParameters, sound::Sound},
    envelope::Envelope,
    id::ID,
    level::Level,
    note_time::NoteTime,
    phase_time::PhaseTime,
    waveform::{state::WaveformState, Waveform},
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
pub type OperatorID = ID;
pub const OPERATOR_A: OperatorID = OperatorID::Index(0);
pub const OPERATOR_B: OperatorID = OperatorID::Index(1);
pub const OPERATOR_C: OperatorID = OperatorID::Index(2);
pub const OPERATOR_D: OperatorID = OperatorID::Index(3);
pub const OPERATOR_E: OperatorID = OperatorID::Index(4);
pub const OPERATOR_F: OperatorID = OperatorID::Index(5);
pub const OPERATOR_1: OperatorID = OperatorID::Index(0);
pub const OPERATOR_2: OperatorID = OperatorID::Index(1);
pub const OPERATOR_3: OperatorID = OperatorID::Index(2);
pub const OPERATOR_4: OperatorID = OperatorID::Index(3);
pub const OPERATOR_5: OperatorID = OperatorID::Index(4);
pub const OPERATOR_6: OperatorID = OperatorID::Index(5);

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

// TODO: Use Sound
impl<E> Operator<E>
where
    E: Envelope,
{
    pub fn sample(
        &self,
        note_time: NoteTime,
        note_off: Option<NoteTime>,
        note_pitch_base: f32,
        note_pitch_modulator: f32,
        sample_rate: f32,
        state: &mut OperatorNoteState,
    ) -> f32 {
        if !self.enable {
            return 0.0;
        }
        let note_pitch = self.frequency.apply(note_pitch_base) + note_pitch_modulator;
        // TODO: add phase when initializing the note_state
        self.waveform.sample(
            &NoteParameters {
                note_time,
                note_off,
                note_pitch,
                gain: self.envelope.level(note_time, note_off) * self.level,
                sample_rate,
            },
            &mut state.waveform,
        )
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
    pub e: Operator<E>,
    pub f: Operator<E>,
}

impl<E> Operators<E>
where
    E: Envelope,
{
    pub fn get_operator(&self, operator_index: OperatorID) -> Option<&Operator<E>> {
        if let OperatorID::Index(index) = operator_index {
            match index {
                0 => Some(&self.a),
                1 => Some(&self.b),
                2 => Some(&self.c),
                3 => Some(&self.d),
                4 => Some(&self.e),
                5 => Some(&self.f),
                _ => None,
            }
        } else {
            None
        }
    }
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
            e: Operator::<E>::default(),
            f: Operator::<E>::default(),
        }
    }
}

#[derive(Debug, Default, Copy, Clone)]
pub struct OperatorNoteState {
    pub waveform: WaveformState,
}

impl OperatorNoteState {
    pub fn reset(&mut self) {
        // TODO should use create_state, as the state is getting more settings.
        self.waveform = WaveformState::default();
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
