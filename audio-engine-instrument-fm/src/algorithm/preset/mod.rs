use self::basic::Basic;

use super::compiled::CompiledAlgorithm;
pub mod basic;

pub const FM_ALGORITHM_BASIC_A: Algorithm = Algorithm::Basic(Basic::A);
pub const FM_ALGORITHM_BASIC_AB: Algorithm = Algorithm::Basic(Basic::AB);
pub const FM_ALGORITHM_BASIC_B_MOD_A: Algorithm = Algorithm::Basic(Basic::BModulatesA);
pub const FM_ALGORITHM_BASIC_D_MOD_ABC: Algorithm = Algorithm::Basic(Basic::DModulatesABC);

#[derive(Debug, Copy, Clone)]
pub enum Algorithm {
    Basic(Basic),
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
        }
    }
}
