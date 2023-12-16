use audio_engine_common::{envelope::Envelope, id::ID, note_time::NoteTime};

use crate::operator::{Operator, OperatorNoteState, Operators, OperatorsNoteState};

pub type OperatorID<E> = ID<Operator<E>>;

pub struct ExecutionStep<E>
where
    E: Envelope,
{
    /// Add the list of values together forming the input.
    pub stack_in: Vec<StackID>,
    /// Operator to execute
    pub operator: OperatorID<E>,
    /// Where to write the result of the operator.
    pub stack_out: StackID,
}

impl<E> ExecutionStep<E>
where
    E: Envelope,
{
    fn execute(
        &self,
        note_time: NoteTime,
        note_off: Option<NoteTime>,
        sample_rate: f32,
        operators: &Operators<E>,
        step_state: &mut OperatorNoteState,
        stack: &mut [f32],
    ) {
        unimplemented!()
    }
}

pub type ExecutionStepID<E> = ID<ExecutionStep<E>>;
pub type StackID = ID<f32>;

pub struct CompiledAlgorithm<E>
where
    E: Envelope,
{
    pub execution_steps: Vec<ExecutionStep<E>>,
    pub stack_size: usize,
    pub carrier_output: Vec<StackID>,
}

pub struct CompiledAlgorithmState {
    pub stack: Vec<f32>,
    pub execution_step_state: Vec<OperatorNoteState>,
}

impl<E> CompiledAlgorithm<E>
where
    E: Envelope,
{
    pub fn sample(
        &self,
        note_time: NoteTime,
        note_off: Option<NoteTime>,
        note_pitch: f32,
        sample_rate: f32,
        operators: &Operators<E>,
        note_state: &mut CompiledAlgorithmState,
    ) -> f32
    where
        E: Envelope,
    {
        self.init_state(note_pitch, note_state);
        for (step, step_state) in self
            .execution_steps
            .iter()
            .zip(note_state.execution_step_state.iter_mut())
        {
            step.execute(
                note_time,
                note_off,
                sample_rate,
                operators,
                step_state,
                &mut note_state.stack,
            );
        }

        // Combine carrier result.
        let mut result = 0.0;
        for carrier_id in &self.carrier_output {
            if let StackID::Index(index) = carrier_id {
                result += note_state.stack[*index as usize];
            }
        }
        result
    }

    fn init_state(&self, note_pitch: f32, note_state: &mut CompiledAlgorithmState) {
        note_state.stack.reserve_exact(self.stack_size);
        note_state.stack.resize(self.stack_size, f32::default());
        note_state
            .execution_step_state
            .reserve_exact(self.execution_steps.len());
        note_state
            .execution_step_state
            .resize(self.execution_steps.len(), OperatorNoteState::default());
        note_state.stack[0] = note_pitch;
    }
}
