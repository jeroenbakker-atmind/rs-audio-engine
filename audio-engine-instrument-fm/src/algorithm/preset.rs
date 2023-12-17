use crate::operator::OperatorID;

use super::compiled::{CompiledAlgorithm, StackID};

#[derive(Debug, Default, Copy, Clone)]
pub enum Algorithm {
    /// Output carrier
    #[default]
    A,

    /// Output carriers A and B
    AB,

    // B modulates carrier A
    BModulatesA,

    // D modulates Carrier A, B and C
    DModulatesABC,
}

impl Algorithm {
    pub fn compile(&self) -> CompiledAlgorithm {
        let mut result = CompiledAlgorithm::default();
        match self {
            Algorithm::A => {
                result.stack_size = 2;
                result.add_step(
                    vec![StackID::from(0)],
                    OperatorID::from(0),
                    StackID::from(1),
                );
                result.carrier_output = vec![StackID::from(1)];
            }
            Algorithm::AB => {
                result.stack_size = 3;
                result.add_step(
                    vec![StackID::from(0)],
                    OperatorID::from(0),
                    StackID::from(1),
                );
                result.add_step(
                    vec![StackID::from(0)],
                    OperatorID::from(1),
                    StackID::from(2),
                );
                result.carrier_output = vec![StackID::from(1), StackID::from(2)];
            }
            Algorithm::BModulatesA => {
                result.stack_size = 3;
                result.add_step(
                    vec![StackID::from(0)],
                    OperatorID::from(1),
                    StackID::from(1),
                );
                result.add_step(
                    vec![StackID::from(0), StackID::from(1)],
                    OperatorID::from(0),
                    StackID::from(2),
                );
                result.carrier_output = vec![StackID::from(2)];
            }
            Algorithm::DModulatesABC => {
                result.stack_size = 5;
                result.add_step(
                    vec![StackID::from(0)],
                    OperatorID::from(3),
                    StackID::from(1),
                );
                result.add_step(
                    vec![StackID::from(0), StackID::from(1)],
                    OperatorID::from(0),
                    StackID::from(2),
                );
                result.add_step(
                    vec![StackID::from(0), StackID::from(1)],
                    OperatorID::from(1),
                    StackID::from(3),
                );
                result.add_step(
                    vec![StackID::from(0), StackID::from(1)],
                    OperatorID::from(2),
                    StackID::from(4),
                );
                result.carrier_output = vec![StackID::from(2), StackID::from(3), StackID::from(4)];
            }
        }
        result
    }
}
