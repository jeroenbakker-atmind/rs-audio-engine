use self::{basic::Basic, dx7::compile_dx7};

use super::compiled::CompiledAlgorithm;
pub mod basic;
mod dx7;

pub const FM_ALGORITHM_BASIC_A: Algorithm = Algorithm::Basic(Basic::A);
pub const FM_ALGORITHM_BASIC_AB: Algorithm = Algorithm::Basic(Basic::AB);
pub const FM_ALGORITHM_BASIC_B_MOD_A: Algorithm = Algorithm::Basic(Basic::BModulatesA);
pub const FM_ALGORITHM_BASIC_D_MOD_ABC: Algorithm = Algorithm::Basic(Basic::DModulatesABC);
pub const FM_ALGORITHM_DX7_1: Algorithm = Algorithm::DX7(1);
pub const FM_ALGORITHM_DX7_2: Algorithm = Algorithm::DX7(2);
pub const FM_ALGORITHM_DX7_3: Algorithm = Algorithm::DX7(3);
pub const FM_ALGORITHM_DX7_4: Algorithm = Algorithm::DX7(4);
pub const FM_ALGORITHM_DX7_5: Algorithm = Algorithm::DX7(5);
pub const FM_ALGORITHM_DX7_6: Algorithm = Algorithm::DX7(6);
pub const FM_ALGORITHM_DX7_7: Algorithm = Algorithm::DX7(7);
pub const FM_ALGORITHM_DX7_8: Algorithm = Algorithm::DX7(8);
pub const FM_ALGORITHM_DX7_9: Algorithm = Algorithm::DX7(9);
pub const FM_ALGORITHM_DX7_10: Algorithm = Algorithm::DX7(10);
pub const FM_ALGORITHM_DX7_11: Algorithm = Algorithm::DX7(11);
pub const FM_ALGORITHM_DX7_12: Algorithm = Algorithm::DX7(12);
pub const FM_ALGORITHM_DX7_13: Algorithm = Algorithm::DX7(13);
pub const FM_ALGORITHM_DX7_14: Algorithm = Algorithm::DX7(14);
pub const FM_ALGORITHM_DX7_15: Algorithm = Algorithm::DX7(15);
pub const FM_ALGORITHM_DX7_16: Algorithm = Algorithm::DX7(16);
pub const FM_ALGORITHM_DX7_17: Algorithm = Algorithm::DX7(17);
pub const FM_ALGORITHM_DX7_18: Algorithm = Algorithm::DX7(18);
pub const FM_ALGORITHM_DX7_19: Algorithm = Algorithm::DX7(19);
pub const FM_ALGORITHM_DX7_20: Algorithm = Algorithm::DX7(20);
pub const FM_ALGORITHM_DX7_21: Algorithm = Algorithm::DX7(21);
pub const FM_ALGORITHM_DX7_22: Algorithm = Algorithm::DX7(22);
pub const FM_ALGORITHM_DX7_23: Algorithm = Algorithm::DX7(23);
pub const FM_ALGORITHM_DX7_24: Algorithm = Algorithm::DX7(24);
pub const FM_ALGORITHM_DX7_25: Algorithm = Algorithm::DX7(25);
pub const FM_ALGORITHM_DX7_26: Algorithm = Algorithm::DX7(26);
pub const FM_ALGORITHM_DX7_27: Algorithm = Algorithm::DX7(27);
pub const FM_ALGORITHM_DX7_28: Algorithm = Algorithm::DX7(28);
pub const FM_ALGORITHM_DX7_29: Algorithm = Algorithm::DX7(29);
pub const FM_ALGORITHM_DX7_30: Algorithm = Algorithm::DX7(30);
pub const FM_ALGORITHM_DX7_31: Algorithm = Algorithm::DX7(31);
pub const FM_ALGORITHM_DX7_32: Algorithm = Algorithm::DX7(32);

#[derive(Debug, Copy, Clone)]
pub enum Algorithm {
    Basic(Basic),
    DX7(u8),
}
impl Default for Algorithm {
    fn default() -> Self {
        Algorithm::Basic(Basic::default())
    }
}

impl Algorithm {
    pub fn compile(&self, repeat: u8) -> CompiledAlgorithm {
        match self {
            Algorithm::Basic(basic) => basic.compile(repeat),
            Algorithm::DX7(dx7) => compile_dx7(*dx7, repeat),
        }
    }
}
