use audio_engine_common::{
    digital_sound::{sound::Sound, sound_state::SoundState},
    envelope::Envelope,
    note_time::NoteTime,
};

use crate::{
    algorithm::{
        compiled::{CompiledAlgorithm, CompiledAlgorithmState},
        preset::Algorithm,
    },
    operator::Operators,
};

#[derive(Debug, Clone)]
pub struct FMInstrument<E>
where
    E: Envelope + Copy + Clone,
{
    pub repeat: u8,
    pub algorithm_preset: Algorithm,
    pub operators: Operators<E>,
    pub algorithm: Option<CompiledAlgorithm>,
}

impl<E> Default for FMInstrument<E>
where
    E: Envelope + Copy + Default,
{
    fn default() -> Self {
        FMInstrument::<E> {
            repeat: 0,
            algorithm_preset: Algorithm::A,
            operators: Operators::<E>::default(),
            algorithm: None,
        }
    }
}

impl<E> FMInstrument<E>
where
    E: Envelope + Copy + Clone,
{
    pub fn compile(&mut self) {
        self.algorithm = Some(self.algorithm_preset.compile(self.repeat));
    }
}

impl<E> Sound for FMInstrument<E>
where
    E: Envelope + Copy,
{
    type SoundState = FMInstrumentNoteState;

    fn init_sound_state(&self) -> Self::SoundState {
        assert!(self.algorithm.is_some(), "Algorithm should have been compiled when creating the instrument by calling #FMInstrument::compile.");
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
        if let Some(program) = &self.algorithm {
            program.sample(
                note_time,
                note_off,
                note_pitch,
                sample_rate,
                &self.operators,
                &mut state.state,
            )
        } else {
            0.0
        }
    }
}

#[derive(Default, Clone)]
pub struct FMInstrumentNoteState {
    pub state: CompiledAlgorithmState,
}

impl SoundState for FMInstrumentNoteState {}
