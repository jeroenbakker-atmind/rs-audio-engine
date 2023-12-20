use crate::{
    algorithm::compiled::{CompiledAlgorithm, StackID},
    operator::{OPERATOR_A, OPERATOR_B, OPERATOR_C, OPERATOR_D},
};

#[derive(Debug, Default, Copy, Clone)]
pub enum Basic {
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

impl Basic {
    pub fn compile(&self, repeat: u8) -> CompiledAlgorithm {
        let mut result = CompiledAlgorithm::default();
        match self {
            Basic::A => {
                let a_out = StackID::from(0);
                result.stack_size = 1;
                result.add_step(vec![], OPERATOR_A, a_out);
                result.carrier_output = vec![a_out];
            }
            Basic::AB => {
                let a_out = StackID::from(0);
                let b_out = StackID::from(1);
                result.stack_size = 2;
                result.add_step(vec![], OPERATOR_A, a_out);
                result.add_step(vec![], OPERATOR_B, b_out);
                result.carrier_output = vec![a_out, b_out];
            }
            Basic::BModulatesA => {
                let b_out = StackID::from(0);
                let a_out = StackID::from(1);
                result.stack_size = 2;
                result.add_step(vec![], OPERATOR_B, b_out);
                result.add_step(vec![b_out], OPERATOR_A, a_out);
                result.carrier_output = vec![a_out];
            }
            Basic::DModulatesABC => {
                let d_out = StackID::from(0);
                let a_out = StackID::from(1);
                let c_out = StackID::from(2);
                let b_out = StackID::from(3);
                result.stack_size = 4;

                result.add_step(vec![], OPERATOR_D, d_out);
                result.add_step(vec![d_out], OPERATOR_A, a_out);
                result.add_step(vec![d_out], OPERATOR_B, b_out);
                result.add_step(vec![d_out], OPERATOR_C, c_out);
                result.carrier_output = vec![a_out, b_out, c_out];
            }
        }
        result
    }
}
