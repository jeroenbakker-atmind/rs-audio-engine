use crate::{
    algorithm::{
        builder::{build_carrier_out, build_repeat, build_step, build_steps},
        compiled::CompiledAlgorithm,
    },
    operator::{OPERATOR_1, OPERATOR_2, OPERATOR_3, OPERATOR_4, OPERATOR_5, OPERATOR_6},
};

pub fn compile_dx7(dx7: u8, repeat: u8) -> CompiledAlgorithm {
    let compiled_algorithm = match dx7 {
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
    };
    debug_assert!(is_valid_dx7_algorithm(&compiled_algorithm));
    compiled_algorithm
}

/// Is the given compiled algorithm a valid DX7 algorithm.
///
/// Validation rules that are checked:
///
/// - All operators should be connected (direct or indirect) to the output.
fn is_valid_dx7_algorithm(_compiled_algorithm: &CompiledAlgorithm) -> bool {
    true
}

/// Compile #FM_ALGORITHM_DX7_1
pub fn compile_dx7_1(repeat: u8) -> CompiledAlgorithm {
    let mut result = CompiledAlgorithm::default();

    let out_6 = build_repeat(&mut result, &[OPERATOR_6], repeat);
    let out_3 = build_steps(
        &mut result,
        vec![out_6],
        &[OPERATOR_5, OPERATOR_4, OPERATOR_3],
    );
    let out_1 = build_steps(&mut result, vec![], &[OPERATOR_2, OPERATOR_1]);
    build_carrier_out(&mut result, vec![out_1, out_3]);

    result
}

/// Compile #FM_ALGORITHM_DX7_2
fn compile_dx7_2(repeat: u8) -> CompiledAlgorithm {
    let mut result = CompiledAlgorithm::default();

    let out_2 = build_repeat(&mut result, &[OPERATOR_2], repeat);
    let out_1 = build_step(&mut result, vec![out_2], OPERATOR_1);
    let out_3 = build_steps(
        &mut result,
        vec![],
        &[OPERATOR_6, OPERATOR_5, OPERATOR_4, OPERATOR_3],
    );
    build_carrier_out(&mut result, vec![out_1, out_3]);

    result
}

/// Compile #FM_ALGORITHM_DX7_3
fn compile_dx7_3(repeat: u8) -> CompiledAlgorithm {
    let mut result = CompiledAlgorithm::default();
    let out_6 = build_repeat(&mut result, &[OPERATOR_6], repeat);
    let out_4 = build_steps(&mut result, vec![out_6], &[OPERATOR_5, OPERATOR_4]);
    let out_1 = build_steps(&mut result, vec![], &[OPERATOR_3, OPERATOR_2, OPERATOR_1]);
    build_carrier_out(&mut result, vec![out_1, out_4]);
    result
}

/// Compile #FM_ALGORITHM_DX7_4
fn compile_dx7_4(repeat: u8) -> CompiledAlgorithm {
    let mut result = CompiledAlgorithm::default();
    let out_4 = build_repeat(&mut result, &[OPERATOR_6, OPERATOR_5, OPERATOR_4], repeat);
    let out_1 = build_steps(&mut result, vec![], &[OPERATOR_3, OPERATOR_2, OPERATOR_1]);
    build_carrier_out(&mut result, vec![out_1, out_4]);
    result
}

/// Compile #FM_ALGORITHM_DX7_5
fn compile_dx7_5(repeat: u8) -> CompiledAlgorithm {
    let mut result = CompiledAlgorithm::default();
    let out_6 = build_repeat(&mut result, &[OPERATOR_6], repeat);
    let out_5 = build_step(&mut result, vec![out_6], OPERATOR_5);
    let out_3 = build_steps(&mut result, vec![], &[OPERATOR_4, OPERATOR_3]);
    let out_1 = build_steps(&mut result, vec![], &[OPERATOR_2, OPERATOR_1]);
    build_carrier_out(&mut result, vec![out_1, out_3, out_5]);
    result
}

/// Compile #FM_ALGORITHM_DX7_6
fn compile_dx7_6(repeat: u8) -> CompiledAlgorithm {
    let mut result = CompiledAlgorithm::default();
    let out_5 = build_repeat(&mut result, &[OPERATOR_6, OPERATOR_5], repeat);
    let out_3 = build_steps(&mut result, vec![], &[OPERATOR_4, OPERATOR_3]);
    let out_1 = build_steps(&mut result, vec![], &[OPERATOR_2, OPERATOR_1]);
    build_carrier_out(&mut result, vec![out_1, out_3, out_5]);
    result
}

/// Compile #FM_ALGORITHM_DX7_7
fn compile_dx7_7(repeat: u8) -> CompiledAlgorithm {
    let mut result = CompiledAlgorithm::default();
    let out_5 = build_repeat(&mut result, &[OPERATOR_6, OPERATOR_5], repeat);
    let out_4 = build_step(&mut result, vec![], OPERATOR_4);
    let out_3 = build_step(&mut result, vec![out_5, out_4], OPERATOR_3);
    let out_1 = build_steps(&mut result, vec![], &[OPERATOR_2, OPERATOR_1]);
    build_carrier_out(&mut result, vec![out_1, out_3]);
    result
}

/// Compile #FM_ALGORITHM_DX7_8
fn compile_dx7_8(repeat: u8) -> CompiledAlgorithm {
    let mut result = CompiledAlgorithm::default();
    let out_4 = build_repeat(&mut result, &[OPERATOR_4], repeat);
    let out_5 = build_steps(&mut result, vec![], &[OPERATOR_6, OPERATOR_5]);
    let out_3 = build_step(&mut result, vec![out_5, out_4], OPERATOR_3);
    let out_1 = build_steps(&mut result, vec![], &[OPERATOR_2, OPERATOR_1]);
    build_carrier_out(&mut result, vec![out_1, out_3]);
    result
}

/// Compile #FM_ALGORITHM_DX7_9
fn compile_dx7_9(repeat: u8) -> CompiledAlgorithm {
    let mut result = CompiledAlgorithm::default();
    let out_2 = build_repeat(&mut result, &[OPERATOR_2], repeat);
    let out_1 = build_step(&mut result, vec![out_2], OPERATOR_1);
    let out_4 = build_step(&mut result, vec![], OPERATOR_4);
    let out_5 = build_steps(&mut result, vec![], &[OPERATOR_6, OPERATOR_5]);
    let out_3 = build_step(&mut result, vec![out_4, out_5], OPERATOR_3);
    build_carrier_out(&mut result, vec![out_1, out_3]);
    result
}

/// Compile #FM_ALGORITHM_DX7_10
fn compile_dx7_10(repeat: u8) -> CompiledAlgorithm {
    let mut result = CompiledAlgorithm::default();
    let out_3 = build_repeat(&mut result, &[OPERATOR_3], repeat);
    let out_1 = build_steps(&mut result, vec![out_3], &[OPERATOR_2, OPERATOR_1]);
    let out_6 = build_step(&mut result, vec![], OPERATOR_6);
    let out_5 = build_step(&mut result, vec![], OPERATOR_5);
    let out_4 = build_step(&mut result, vec![out_6, out_5], OPERATOR_4);
    build_carrier_out(&mut result, vec![out_1, out_4]);
    result
}

/// Compile #FM_ALGORITHM_DX7_11
fn compile_dx7_11(repeat: u8) -> CompiledAlgorithm {
    let mut result = CompiledAlgorithm::default();
    let out_6 = build_repeat(&mut result, &[OPERATOR_6], repeat);
    let out_1 = build_steps(&mut result, vec![], &[OPERATOR_3, OPERATOR_2, OPERATOR_1]);
    let out_5 = build_step(&mut result, vec![], OPERATOR_5);
    let out_4 = build_step(&mut result, vec![out_6, out_5], OPERATOR_4);
    build_carrier_out(&mut result, vec![out_1, out_4]);
    result
}

/// Compile #FM_ALGORITHM_DX7_12
fn compile_dx7_12(repeat: u8) -> CompiledAlgorithm {
    let mut result = CompiledAlgorithm::default();
    let out_2 = build_repeat(&mut result, &[OPERATOR_2], repeat);
    let out_1 = build_step(&mut result, vec![out_2], OPERATOR_1);
    let out_4 = build_step(&mut result, vec![], OPERATOR_4);
    let out_5 = build_step(&mut result, vec![], OPERATOR_5);
    let out_6 = build_step(&mut result, vec![], OPERATOR_6);
    let out_3 = build_step(&mut result, vec![out_4, out_5, out_6], OPERATOR_3);
    build_carrier_out(&mut result, vec![out_1, out_3]);
    result
}

/// Compile #FM_ALGORITHM_DX7_13
fn compile_dx7_13(repeat: u8) -> CompiledAlgorithm {
    let mut result = CompiledAlgorithm::default();
    let out_6 = build_repeat(&mut result, &[OPERATOR_6], repeat);
    let out_1 = build_steps(&mut result, vec![], &[OPERATOR_2, OPERATOR_1]);
    let out_4 = build_step(&mut result, vec![], OPERATOR_4);
    let out_5 = build_step(&mut result, vec![], OPERATOR_5);
    let out_3 = build_step(&mut result, vec![out_4, out_5, out_6], OPERATOR_3);
    build_carrier_out(&mut result, vec![out_1, out_3]);
    result
}

/// Compile #FM_ALGORITHM_DX7_14
fn compile_dx7_14(repeat: u8) -> CompiledAlgorithm {
    let mut result = CompiledAlgorithm::default();
    let out_6 = build_repeat(&mut result, &[OPERATOR_6], repeat);
    let out_1 = build_steps(&mut result, vec![], &[OPERATOR_2, OPERATOR_1]);
    let out_5 = build_step(&mut result, vec![], OPERATOR_5);
    let out_3 = build_steps(&mut result, vec![out_6, out_5], &[OPERATOR_4, OPERATOR_3]);
    build_carrier_out(&mut result, vec![out_1, out_3]);
    result
}

/// Compile #FM_ALGORITHM_DX7_15
fn compile_dx7_15(repeat: u8) -> CompiledAlgorithm {
    let mut result = CompiledAlgorithm::default();
    let out_2 = build_repeat(&mut result, &[OPERATOR_2], repeat);
    let out_1 = build_step(&mut result, vec![out_2], OPERATOR_1);
    let out_6 = build_step(&mut result, vec![], OPERATOR_6);
    let out_5 = build_step(&mut result, vec![], OPERATOR_5);
    let out_3 = build_steps(&mut result, vec![out_6, out_5], &[OPERATOR_4, OPERATOR_3]);
    build_carrier_out(&mut result, vec![out_1, out_3]);
    result
}

/// Compile #FM_ALGORITHM_DX7_16
fn compile_dx7_16(repeat: u8) -> CompiledAlgorithm {
    let mut result = CompiledAlgorithm::default();
    let out_6 = build_repeat(&mut result, &[OPERATOR_6], repeat);
    let out_5 = build_step(&mut result, vec![out_6], OPERATOR_5);
    let out_3 = build_steps(&mut result, vec![], &[OPERATOR_4, OPERATOR_3]);
    let out_2 = build_step(&mut result, vec![], OPERATOR_2);
    let out_1 = build_step(&mut result, vec![out_5, out_3, out_2], OPERATOR_1);
    build_carrier_out(&mut result, vec![out_1, out_3]);
    result
}

/// Compile #FM_ALGORITHM_DX7_17
fn compile_dx7_17(repeat: u8) -> CompiledAlgorithm {
    let mut result = CompiledAlgorithm::default();
    let out_2 = build_repeat(&mut result, &[OPERATOR_2], repeat);
    let out_5 = build_steps(&mut result, vec![], &[OPERATOR_6, OPERATOR_5]);
    let out_3 = build_steps(&mut result, vec![], &[OPERATOR_4, OPERATOR_3]);
    let out_1 = build_step(&mut result, vec![out_5, out_3, out_2], OPERATOR_1);
    build_carrier_out(&mut result, vec![out_1, out_3]);
    result
}

/// Compile #FM_ALGORITHM_DX7_18
fn compile_dx7_18(repeat: u8) -> CompiledAlgorithm {
    todo!()
}

/// Compile #FM_ALGORITHM_DX7_19
fn compile_dx7_19(repeat: u8) -> CompiledAlgorithm {
    todo!()
}
/// Compile #FM_ALGORITHM_DX7_20
fn compile_dx7_20(repeat: u8) -> CompiledAlgorithm {
    todo!()
}

/// Compile #FM_ALGORITHM_DX7_21
fn compile_dx7_21(repeat: u8) -> CompiledAlgorithm {
    todo!()
}

/// Compile #FM_ALGORITHM_DX7_22
fn compile_dx7_22(repeat: u8) -> CompiledAlgorithm {
    todo!()
}

/// Compile #FM_ALGORITHM_DX7_23
fn compile_dx7_23(repeat: u8) -> CompiledAlgorithm {
    todo!()
}

/// Compile #FM_ALGORITHM_DX7_24
fn compile_dx7_24(repeat: u8) -> CompiledAlgorithm {
    todo!()
}

/// Compile #FM_ALGORITHM_DX7_25
fn compile_dx7_25(repeat: u8) -> CompiledAlgorithm {
    todo!()
}

/// Compile #FM_ALGORITHM_DX7_26
fn compile_dx7_26(repeat: u8) -> CompiledAlgorithm {
    todo!()
}

/// Compile #FM_ALGORITHM_DX7_27
fn compile_dx7_27(repeat: u8) -> CompiledAlgorithm {
    todo!()
}

/// Compile #FM_ALGORITHM_DX7_28
fn compile_dx7_28(repeat: u8) -> CompiledAlgorithm {
    todo!()
}

/// Compile #FM_ALGORITHM_DX7_29
fn compile_dx7_29(repeat: u8) -> CompiledAlgorithm {
    todo!()
}

/// Compile #FM_ALGORITHM_DX7_30
fn compile_dx7_30(repeat: u8) -> CompiledAlgorithm {
    todo!()
}

/// Compile #FM_ALGORITHM_DX7_31
fn compile_dx7_31(repeat: u8) -> CompiledAlgorithm {
    let mut result = CompiledAlgorithm::default();
    let out_6 = build_repeat(&mut result, &[OPERATOR_6], repeat);
    let out_5 = build_step(&mut result, vec![out_6], OPERATOR_5);
    let out_4 = build_step(&mut result, vec![], OPERATOR_4);
    let out_3 = build_step(&mut result, vec![], OPERATOR_3);
    let out_2 = build_step(&mut result, vec![], OPERATOR_2);
    let out_1 = build_step(&mut result, vec![], OPERATOR_1);
    build_carrier_out(&mut result, vec![out_1, out_2, out_3, out_4, out_5]);
    result
}

/// Compile #FM_ALGORITHM_DX7_32
fn compile_dx7_32(repeat: u8) -> CompiledAlgorithm {
    let mut result = CompiledAlgorithm::default();
    let out_6 = build_repeat(&mut result, &[OPERATOR_6], repeat);
    let out_5 = build_step(&mut result, vec![], OPERATOR_5);
    let out_4 = build_step(&mut result, vec![], OPERATOR_4);
    let out_3 = build_step(&mut result, vec![], OPERATOR_3);
    let out_2 = build_step(&mut result, vec![], OPERATOR_2);
    let out_1 = build_step(&mut result, vec![], OPERATOR_1);
    build_carrier_out(&mut result, vec![out_1, out_2, out_3, out_4, out_5, out_6]);
    result
}
