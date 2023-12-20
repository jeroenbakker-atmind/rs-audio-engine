use audio_engine_common::{envelope::Envelope, id::ID, note_time::NoteTime};

use crate::operator::{OperatorID, OperatorNoteState, Operators};

use super::execution_step::ExecutionStep;

pub type StackID = ID;

#[derive(Debug, Default, Clone)]
pub struct CompiledAlgorithm {
    pub execution_steps: Vec<ExecutionStep>,
    pub stack_size: usize,
    pub carrier_output: Vec<StackID>,
}

impl CompiledAlgorithm {
    pub fn add_step(&mut self, stack_in: Vec<StackID>, operator: OperatorID, stack_out: StackID) {
        let step = ExecutionStep {
            stack_in,
            operator_index: operator,
            stack_out,
        };
        self.execution_steps.push(step);
    }
}

#[derive(Debug, Default, Clone)]
pub struct CompiledAlgorithmState {
    pub stack: Vec<f32>,
    pub execution_step_state: Vec<OperatorNoteState>,
}

impl CompiledAlgorithm {
    pub fn sample<E: Envelope>(
        &self,
        note_time: NoteTime,
        note_off: Option<NoteTime>,
        note_pitch: f32,
        sample_rate: f32,
        operators: &Operators<E>,
        note_state: &mut CompiledAlgorithmState,
    ) -> f32 {
        self.init_state(note_state);
        for (step, step_state) in self
            .execution_steps
            .iter()
            .zip(note_state.execution_step_state.iter_mut())
        {
            step.execute(
                note_time,
                note_off,
                note_pitch,
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
        // TODO: Unclear if we want to average or just add and use envelopes?
        if !self.carrier_output.is_empty() {
            result /= self.carrier_output.len() as f32;
        }
        result
    }

    fn init_state(&self, note_state: &mut CompiledAlgorithmState) {
        note_state.stack.reserve_exact(self.stack_size);
        note_state.stack.resize(self.stack_size, f32::default());
        note_state
            .execution_step_state
            .reserve_exact(self.execution_steps.len());
        note_state
            .execution_step_state
            .resize(self.execution_steps.len(), OperatorNoteState::default());
    }
}
