use audio_engine_common::{envelope::Envelope, id::ID, note_time::NoteTime};

use crate::operator::{OperatorID, OperatorNoteState, Operators};

use super::execution_step::ExecutionStep;

pub type StackID = ID;

/// Contains a compiled version of an algorithm.
///
/// Algorithms needs to be compiled before execution.
///
/// # Stack
///
/// During execution the execution steps can read and will write into the stack
/// During the compilation the located will be reserved where the data needs to
/// be read from and where the result of the operator should be stored.
///
/// # Output
///
/// Operators write there result to the stack. [carrier_output] points to location
/// on the stack where the carrier has written its result to.
///
/// # Unrolling repeat
///
/// During compilation the operator repeat is unrolled resulting in a vector of
/// [ExecutionStep]. They are stored in [CompiledAlgorithm::execution_steps].
///
/// Unrolling ensures that each step can have its own execution state and is
/// independent from the number of times a specific operator is used.
///
/// # TODO
///
/// - Optimize execution by removing execution steps won't change the final
///   output. This happens when certain operators are disabled or when their
///   gain is zero.
/// - Currently all execution steps write to their own reserved space on the
///   stack. Stack space can be saved when operators are sequential and don't
///   branch. Or in other way when not read from anymore, other operators
///   may reuse already reserved stack space.
#[derive(Debug, Default, Clone)]
pub struct CompiledAlgorithm {
    /// A compiled algorithm is a vec of execution steps that are executed in order.
    pub execution_steps: Vec<ExecutionStep>,

    /// Needed stack size for execution
    ///
    /// Execution steps read and write to a stack. This attribute should be at equal (or higher) than the
    /// highest stack position an execution step reads from or writes to.
    ///
    /// See [ExecutionStep::stack_in] and [ExecutionStep::stack_out].
    ///
    /// The actual stack is located at [CompiledAlgorithmState::stack]
    pub stack_size: usize,

    /// Where on the stack have the carriers written their result to.
    ///
    /// These results are combined using addition for the final output
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
        self.execute_steps(
            note_time,
            note_off,
            note_pitch,
            sample_rate,
            operators,
            note_state,
        );
        self.sum_carrier_result(note_state)
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

    fn execute_steps<E: Envelope>(
        &self,
        note_time: NoteTime,
        note_off: Option<NoteTime>,
        note_pitch: f32,
        sample_rate: f32,
        operators: &Operators<E>,
        note_state: &mut CompiledAlgorithmState,
    ) {
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
    }

    fn sum_carrier_result(&self, note_state: &CompiledAlgorithmState) -> f32 {
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
}
