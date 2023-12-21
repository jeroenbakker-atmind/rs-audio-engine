use crate::{
    algorithm::compiled::{CompiledAlgorithm, StackID},
    operator::OperatorID,
};

fn next_step_id(compiled_algorithm: &CompiledAlgorithm) -> StackID {
    let mut result = None;
    for step in &compiled_algorithm.execution_steps {
        if let StackID::Index(stack_index) = step.stack_out {
            match result {
                None => result = Some(stack_index),
                Some(prev) => result = Some(prev.max(stack_index + 1)),
            }
        }
    }
    StackID::from(result.unwrap_or_default())
}

pub fn build_step(
    result: &mut CompiledAlgorithm,
    stack_in: Vec<StackID>,
    operator: OperatorID,
) -> StackID {
    let stack_out = next_step_id(result);
    result.add_step(stack_in, operator, stack_out);
    stack_out
}

pub fn build_steps(
    result: &mut CompiledAlgorithm,
    stack_in: Vec<StackID>,
    operators: &[OperatorID],
) -> StackID {
    let mut stack_out = stack_in;
    for operator in operators {
        stack_out = vec![build_step(result, stack_out, *operator)];
    }
    stack_out[0]
}

pub fn build_repeat(
    result: &mut CompiledAlgorithm,
    operators: &[OperatorID],
    repeat: u8,
) -> StackID {
    assert!(!operators.is_empty());
    assert!(result.execution_steps.is_empty());
    let mut stack_in = vec![];
    let mut stack_index = 0;
    for _ in 0..=repeat {
        for operator_id in operators {
            result.add_step(stack_in, *operator_id, StackID::from(stack_index));
            stack_in = vec![StackID::from(stack_index)];
            stack_index += 1;
        }
    }
    stack_in[0]
}

fn build_stack_size(result: &mut CompiledAlgorithm) {
    let mut max_stack_size = 0;
    for step in &result.execution_steps {
        for stack_id in &step.stack_in {
            if let StackID::Index(stack_index) = stack_id {
                max_stack_size = max_stack_size.max(*stack_index as usize);
            }
        }
        if let StackID::Index(stack_index) = step.stack_out {
            max_stack_size = max_stack_size.max(stack_index as usize);
        }
    }
    result.stack_size = max_stack_size + 1;
}

pub fn build_carrier_out(result: &mut CompiledAlgorithm, carriers: Vec<OperatorID>) {
    assert!(result.carrier_output.is_empty());
    result.carrier_output = carriers;
    build_stack_size(result);
}
