use audio_engine_common::{digital_sound::parameters::NoteParameters, envelope::Envelope, id::ID};

use crate::operator::{OperatorID, OperatorNoteState, Operators};

use super::compiled::StackID;

#[derive(Debug, Default, Clone)]
pub struct ExecutionStep {
    /// Add the list of values together forming the input.
    pub stack_in: Vec<StackID>,
    /// Operator to execute
    pub operator_index: OperatorID,
    /// Where to write the result of the operator.
    pub stack_out: StackID,
}

impl ExecutionStep {
    pub fn execute<E: Envelope>(
        &self,
        parameters: &NoteParameters,
        operators: &Operators<E>,
        step_state: &mut OperatorNoteState,
        stack: &mut [f32],
    ) {
        let note_pitch_base = parameters.note_pitch;
        let note_pitch_modulator = self.sum_inputs(stack);
        if let (Some(operator), StackID::Index(index)) =
            (operators.get_operator(self.operator_index), self.stack_out)
        {
            let result = operator.sample(
                parameters.note_time,
                parameters.note_off,
                note_pitch_base,
                note_pitch_modulator,
                parameters.sample_rate,
                step_state,
            );
            stack[index as usize] = result;
        }
    }

    /// Sum all input values from the stack for this execution step.
    fn sum_inputs(&self, stack: &[f32]) -> f32 {
        self.stack_in
            .iter()
            .filter_map(|s| {
                if let StackID::Index(index) = s {
                    Some(index)
                } else {
                    None
                }
            })
            .map(|s| stack[*s as usize])
            .sum::<f32>()
    }
}

pub type ExecutionStepID = ID;
