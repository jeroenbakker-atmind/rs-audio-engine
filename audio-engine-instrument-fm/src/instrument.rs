use audio_engine_common::{
    digital_sound::{parameters::NoteParameters, sound::Sound, sound_state::SoundState},
    envelope::Envelope,
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
            algorithm_preset: Algorithm::default(),
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
    type Parameters = NoteParameters;

    fn init_sound_state(&self) -> Self::SoundState {
        assert!(self.algorithm.is_some(), "Algorithm should have been compiled when creating the instrument by calling #FMInstrument::compile.");
        Self::SoundState::default()
    }

    fn sample(&self, parameters: &Self::Parameters, state: &mut Self::SoundState) -> f32 {
        if let Some(program) = &self.algorithm {
            program.sample(parameters, &self.operators, &mut state.state)
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
