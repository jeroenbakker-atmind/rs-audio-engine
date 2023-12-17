use audio_engine_common::{
    digital_sound::{sound::Sound, sound_state::SoundState},
    envelope::Envelope,
    note_time::NoteTime,
};

use crate::{
    algorithm::compiled::{CompiledAlgorithm, CompiledAlgorithmState},
    operator::{Operators, OperatorsNoteState},
};

#[derive(Debug, Clone)]
pub struct FMInstrument<E>
where
    E: Envelope + Copy + Clone,
{
    pub operators: Operators<E>,
    pub algorithm: CompiledAlgorithm,
}

impl<E> Sound for FMInstrument<E>
where
    E: Envelope + Copy,
{
    type SoundState = FMInstrumentNoteState;

    fn init_sound_state(&self) -> Self::SoundState {
        Self::SoundState::default()
    }

    fn sample(
        &self,
        note_time: NoteTime,
        note_off: Option<NoteTime>,
        note_pitch: f32,
        sample_rate: f32,
        state: &mut Self::SoundState,
    ) -> f32 {
        self.algorithm.sample(
            note_time,
            note_off,
            note_pitch,
            sample_rate,
            &self.operators,
            &mut state.state,
        )
    }
}

#[derive(Default, Clone)]
pub struct FMInstrumentNoteState {
    pub state: CompiledAlgorithmState,
}

impl SoundState for FMInstrumentNoteState {}
