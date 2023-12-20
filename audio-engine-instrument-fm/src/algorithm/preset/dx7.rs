use crate::{
    algorithm::compiled::{CompiledAlgorithm, StackID},
    operator::{OPERATOR_1, OPERATOR_2, OPERATOR_3, OPERATOR_4, OPERATOR_5, OPERATOR_6},
};

pub fn compile_dx7(dx7: u8, repeat: u8) -> CompiledAlgorithm {
    match dx7 {
        1 => compile_dx7_1(repeat),
        2 => compile_dx7_2(repeat),
        3 => compile_dx7_3(repeat),
        4 => compile_dx7_4(repeat),
        5 => compile_dx7_5(repeat),
        6 => compile_dx7_6(repeat),
        7 => compile_dx7_7(repeat),
        8 => compile_dx7_8(repeat),
        9 => compile_dx7_9(repeat),
        10 => compile_dx7_10(repeat),
        11 => compile_dx7_11(repeat),
        12 => compile_dx7_12(repeat),
        13 => compile_dx7_13(repeat),
        14 => compile_dx7_14(repeat),
        15 => compile_dx7_15(repeat),
        16 => compile_dx7_16(repeat),
        17 => compile_dx7_17(repeat),
        18 => compile_dx7_18(repeat),
        19 => compile_dx7_19(repeat),
        20 => compile_dx7_20(repeat),
        21 => compile_dx7_21(repeat),
        22 => compile_dx7_22(repeat),
        23 => compile_dx7_23(repeat),
        24 => compile_dx7_24(repeat),
        25 => compile_dx7_25(repeat),
        26 => compile_dx7_26(repeat),
        27 => compile_dx7_27(repeat),
        28 => compile_dx7_28(repeat),
        29 => compile_dx7_29(repeat),
        30 => compile_dx7_30(repeat),
        31 => compile_dx7_31(repeat),
        32 => compile_dx7_32(repeat),
        _ => CompiledAlgorithm::default(),
    }
}

#[cfg_attr(doc, aquamarine::aquamarine)]
/// Compile DX& algorithm 1.
///
/// ```mermaid
/// flowchart
///     6 --> 6
///     6 --> 5 --> 4 --> 3 --> out
///     2 --> 1 --> out
/// ```
pub fn compile_dx7_1(repeat: u8) -> CompiledAlgorithm {
    let mut result = CompiledAlgorithm::default();

    let out_6 = StackID::from(repeat);
    let out_5 = StackID::from(repeat + 1);
    let out_4 = StackID::from(repeat + 2);
    let out_3 = StackID::from(repeat + 3);
    let out_2 = StackID::from(repeat + 4);
    let out_1 = StackID::from(repeat + 5);

    result.stack_size = (repeat + 6) as usize;

    let mut stack_in = vec![];
    for stack_index in 0..repeat {
        result.add_step(stack_in, OPERATOR_6, StackID::from(stack_index));
        stack_in = vec![StackID::from(stack_index)];
    }
    result.add_step(stack_in, OPERATOR_6, out_6);
    result.add_step(vec![out_6], OPERATOR_5, out_5);
    result.add_step(vec![out_5], OPERATOR_4, out_4);
    result.add_step(vec![out_4], OPERATOR_3, out_3);
    result.add_step(vec![], OPERATOR_2, out_2);
    result.add_step(vec![out_2], OPERATOR_1, out_1);

    result.carrier_output = vec![out_1, out_3];

    result
}
fn compile_dx7_2(repeat: u8) -> CompiledAlgorithm {
    todo!()
}
fn compile_dx7_3(repeat: u8) -> CompiledAlgorithm {
    todo!()
}
fn compile_dx7_4(repeat: u8) -> CompiledAlgorithm {
    todo!()
}
fn compile_dx7_5(repeat: u8) -> CompiledAlgorithm {
    todo!()
}
fn compile_dx7_6(repeat: u8) -> CompiledAlgorithm {
    todo!()
}
fn compile_dx7_7(repeat: u8) -> CompiledAlgorithm {
    todo!()
}
fn compile_dx7_8(repeat: u8) -> CompiledAlgorithm {
    todo!()
}
fn compile_dx7_9(repeat: u8) -> CompiledAlgorithm {
    todo!()
}
fn compile_dx7_10(repeat: u8) -> CompiledAlgorithm {
    todo!()
}
fn compile_dx7_11(repeat: u8) -> CompiledAlgorithm {
    todo!()
}
fn compile_dx7_12(repeat: u8) -> CompiledAlgorithm {
    todo!()
}
fn compile_dx7_13(repeat: u8) -> CompiledAlgorithm {
    todo!()
}
fn compile_dx7_14(repeat: u8) -> CompiledAlgorithm {
    todo!()
}
fn compile_dx7_15(repeat: u8) -> CompiledAlgorithm {
    todo!()
}
fn compile_dx7_16(repeat: u8) -> CompiledAlgorithm {
    todo!()
}
fn compile_dx7_17(repeat: u8) -> CompiledAlgorithm {
    todo!()
}
fn compile_dx7_18(repeat: u8) -> CompiledAlgorithm {
    todo!()
}
fn compile_dx7_19(repeat: u8) -> CompiledAlgorithm {
    todo!()
}
fn compile_dx7_20(repeat: u8) -> CompiledAlgorithm {
    todo!()
}
fn compile_dx7_21(repeat: u8) -> CompiledAlgorithm {
    todo!()
}
fn compile_dx7_22(repeat: u8) -> CompiledAlgorithm {
    todo!()
}
fn compile_dx7_23(repeat: u8) -> CompiledAlgorithm {
    todo!()
}
fn compile_dx7_24(repeat: u8) -> CompiledAlgorithm {
    todo!()
}
fn compile_dx7_25(repeat: u8) -> CompiledAlgorithm {
    todo!()
}
fn compile_dx7_26(repeat: u8) -> CompiledAlgorithm {
    todo!()
}
fn compile_dx7_27(repeat: u8) -> CompiledAlgorithm {
    todo!()
}
fn compile_dx7_28(repeat: u8) -> CompiledAlgorithm {
    todo!()
}
fn compile_dx7_29(repeat: u8) -> CompiledAlgorithm {
    todo!()
}

fn compile_dx7_30(repeat: u8) -> CompiledAlgorithm {
    todo!()
}
fn compile_dx7_31(repeat: u8) -> CompiledAlgorithm {
    todo!()
}
fn compile_dx7_32(repeat: u8) -> CompiledAlgorithm {
    todo!()
}
