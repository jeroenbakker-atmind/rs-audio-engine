use self::{basic::Basic, dx7::compile_dx7};

use super::compiled::CompiledAlgorithm;
pub mod basic;
mod dx7;

pub const FM_ALGORITHM_BASIC_A: Algorithm = Algorithm::Basic(Basic::A);
pub const FM_ALGORITHM_BASIC_AB: Algorithm = Algorithm::Basic(Basic::AB);
pub const FM_ALGORITHM_BASIC_B_MOD_A: Algorithm = Algorithm::Basic(Basic::BModulatesA);
pub const FM_ALGORITHM_BASIC_D_MOD_ABC: Algorithm = Algorithm::Basic(Basic::DModulatesABC);

#[cfg_attr(doc, aquamarine::aquamarine)]
/// DX7 algorithm 1.
/// ```mermaid
/// flowchart
///     6 -->|repeat| 6
///     6 --> 5 --> 4 --> 3 --> out
///     2 --> 1 --> out
/// ```
pub const FM_ALGORITHM_DX7_1: Algorithm = Algorithm::DX7(1);

#[cfg_attr(doc, aquamarine::aquamarine)]
/// DX7 algorithm 2
///
/// ```mermaid
/// flowchart
///     2 -->|repeat| 2
///     2 --> 1 --> out
///     6 --> 5 --> 4 --> 3 --> out
/// ```
pub const FM_ALGORITHM_DX7_2: Algorithm = Algorithm::DX7(2);

#[cfg_attr(doc, aquamarine::aquamarine)]
/// DX7 algorithm 3
///
/// ```mermaid
/// flowchart
///     6 -->|repeat| 6
///     6 --> 5 --> 4 --> out
///     3 --> 2 --> 1 --> out
/// ```
pub const FM_ALGORITHM_DX7_3: Algorithm = Algorithm::DX7(3);

#[cfg_attr(doc, aquamarine::aquamarine)]
/// DX7 algorithm 4
///
/// ```mermaid
/// flowchart
///     6 --> 5 --> 4 -->|repeat| 6
///     4 --> out
///     3 --> 2 --> 1 --> out
/// ```
pub const FM_ALGORITHM_DX7_4: Algorithm = Algorithm::DX7(4);

#[cfg_attr(doc, aquamarine::aquamarine)]
/// DX7 algorithm 5
///
/// ```mermaid
/// flowchart
///     6 -->|repeat| 6
///     6 --> 5 --> out
///     4 --> 3 --> out
///     2 --> 1 --> out
/// ```
pub const FM_ALGORITHM_DX7_5: Algorithm = Algorithm::DX7(5);

#[cfg_attr(doc, aquamarine::aquamarine)]
/// DX7 algorithm 6
///
/// ```mermaid
/// flowchart
///     6 --> 5 -->|repeat| 6
///     5 --> out
///     4 --> 3 --> out
///     2 --> 1 --> out
/// ```
pub const FM_ALGORITHM_DX7_6: Algorithm = Algorithm::DX7(6);

#[cfg_attr(doc, aquamarine::aquamarine)]
/// DX7 algorithm 7
///
/// ```mermaid
/// flowchart
///     6 -->|repeat| 6
///     5 --> 3
///     4 --> 3 --> out
///     2 --> 1 --> out
/// ```
pub const FM_ALGORITHM_DX7_7: Algorithm = Algorithm::DX7(7);

#[cfg_attr(doc, aquamarine::aquamarine)]
/// DX7 algorithm 8
///
/// ```mermaid
/// flowchart
///     4 -->|repeat| 4
///     6 --> 5 --> 3
///     4 --> 3 --> out
///     2 --> 1 --> out
/// ```
pub const FM_ALGORITHM_DX7_8: Algorithm = Algorithm::DX7(8);

#[cfg_attr(doc, aquamarine::aquamarine)]
/// DX7 algorithm 9
///
/// ```mermaid
/// flowchart
///     2 -->|repeat| 2
///     2 --> 1 --> out
///     6 --> 5 --> 3
///     4 --> 3 --> out
/// ```
pub const FM_ALGORITHM_DX7_9: Algorithm = Algorithm::DX7(9);

#[cfg_attr(doc, aquamarine::aquamarine)]
/// DX7 algorithm 10
///
/// ```mermaid
/// flowchart
///     3 -->|repeat| 3
///     3 --> 2 --> 1 --> out
///     6 --> 4
///     5 --> 4 --> out
/// ```
pub const FM_ALGORITHM_DX7_10: Algorithm = Algorithm::DX7(10);

#[cfg_attr(doc, aquamarine::aquamarine)]
/// DX7 algorithm 11
///
/// ```mermaid
/// flowchart
///     3 --> 2 --> 1
///     6 -->|repeat| 6
///     6 --> 4 --> out
///     5 --> 4
/// ```
pub const FM_ALGORITHM_DX7_11: Algorithm = Algorithm::DX7(11);

#[cfg_attr(doc, aquamarine::aquamarine)]
/// DX7 algorithm 12
///
/// ```mermaid
/// flowchart
///     2 -->|repeat| 2
///     2 --> 1 --> out
///     4 --> 3 --> out
///     5 --> 3
///     6 --> 3
/// ```
pub const FM_ALGORITHM_DX7_12: Algorithm = Algorithm::DX7(12);

#[cfg_attr(doc, aquamarine::aquamarine)]
/// DX7 algorithm 13
///
/// ```mermaid
/// flowchart
///     2 --> 1 --> out
///     4 --> 3 --> out
///     5 --> 3
///     6 -->|repeat| 6
///     6 --> 3
/// ```
pub const FM_ALGORITHM_DX7_13: Algorithm = Algorithm::DX7(13);

#[cfg_attr(doc, aquamarine::aquamarine)]
/// DX7 algorithm 14
///
/// ```mermaid
/// flowchart
///     6 -->|repeat| 6
///     2 --> 1 --> out
///     6 --> 4 --> 3 --> out
///     5 --> 4
/// ```
pub const FM_ALGORITHM_DX7_14: Algorithm = Algorithm::DX7(14);

#[cfg_attr(doc, aquamarine::aquamarine)]
/// DX7 algorithm 15
///
/// ```mermaid
/// flowchart
///     2 -->|repeat| 2
///     2 --> 1 --> out
///     6 --> 4 --> 3 --> out
///     5 --> 4
/// ```
pub const FM_ALGORITHM_DX7_15: Algorithm = Algorithm::DX7(15);

#[cfg_attr(doc, aquamarine::aquamarine)]
/// DX7 algorithm 16
///
/// ```mermaid
/// flowchart
///     6 -->|repeat| 6
///     6 --> 5 --> 1
///     4 --> 3 --> 1
///     2 --> 1 --> out
/// ```
pub const FM_ALGORITHM_DX7_16: Algorithm = Algorithm::DX7(16);

#[cfg_attr(doc, aquamarine::aquamarine)]
/// DX7 algorithm 17
///
/// ```mermaid
/// flowchart
///     2 -->|repeat| 2
///     6 --> 5 --> 1
///     4 --> 3 --> 1
///     2 --> 1 --> out
/// ```
pub const FM_ALGORITHM_DX7_17: Algorithm = Algorithm::DX7(17);

#[cfg_attr(doc, aquamarine::aquamarine)]
/// DX7 algorithm 18
///
/// ```mermaid
/// flowchart
///     3 -->|repeat| 3
///     6 --> 5 --> 4 --> 1 --> out
///     3 --> 1
///     2 --> 1
/// ```
pub const FM_ALGORITHM_DX7_18: Algorithm = Algorithm::DX7(18);

#[cfg_attr(doc, aquamarine::aquamarine)]
/// DX7 algorithm 19
///
/// ```mermaid
/// flowchart
///     6 -->|repeat| 6
///     6 --> 4 --> out
///     6 --> 5 --> out
///     3 --> 2 --> 1 --> out
/// ```
pub const FM_ALGORITHM_DX7_19: Algorithm = Algorithm::DX7(19);

#[cfg_attr(doc, aquamarine::aquamarine)]
/// DX7 algorithm 20
///
/// ```mermaid
/// flowchart
///     3 -->|repeat| 3
///     3 --> 1 --> out
///     3 --> 2 --> out
///     5 --> 4 --> out
///     6 --> 4
/// ```
pub const FM_ALGORITHM_DX7_20: Algorithm = Algorithm::DX7(20);

#[cfg_attr(doc, aquamarine::aquamarine)]
/// DX7 algorithm 21
///
/// ```mermaid
/// flowchart
///     3 -->|repeat| 3
///     3 --> 1 --> out
///     3 --> 2 --> out
///     6 --> 4 --> out
///     6 --> 5 --> out
/// ```
pub const FM_ALGORITHM_DX7_21: Algorithm = Algorithm::DX7(21);

#[cfg_attr(doc, aquamarine::aquamarine)]
/// DX7 algorithm 22
///
/// ```mermaid
/// flowchart
///     6 -->|repeat| 6
///     2 --> 1 --> out
///     6 --> 3 --> out
///     6 --> 4 --> out
///     6 --> 5 --> out
/// ```
pub const FM_ALGORITHM_DX7_22: Algorithm = Algorithm::DX7(22);

#[cfg_attr(doc, aquamarine::aquamarine)]
/// DX7 algorithm 23
///
/// ```mermaid
/// flowchart
///     6 -->|repeat| 6
///     1 --> out
///     3 --> 2 --> out
///     6 --> 4 --> out
///     6 --> 5 --> out
/// ```
pub const FM_ALGORITHM_DX7_23: Algorithm = Algorithm::DX7(23);

#[cfg_attr(doc, aquamarine::aquamarine)]
/// DX7 algorithm 24
///
/// ```mermaid
/// flowchart
///     6 -->|repeat| 6
///     1 --> out
///     2 --> out
///     6 --> 3 --> out
///     6 --> 4 --> out
///     6 --> 5 --> out
/// ```
pub const FM_ALGORITHM_DX7_24: Algorithm = Algorithm::DX7(24);

#[cfg_attr(doc, aquamarine::aquamarine)]
/// DX7 algorithm 25
///
/// ```mermaid
/// flowchart
///     6 -->|repeat| 6
///     1 --> out
///     2 --> out
///     3 --> out
///     6 --> 4 --> out
///     6 --> 5 --> out
/// ```
pub const FM_ALGORITHM_DX7_25: Algorithm = Algorithm::DX7(25);

#[cfg_attr(doc, aquamarine::aquamarine)]
/// DX7 algorithm 26
///
/// ```mermaid
/// flowchart
///     6 -->|repeat| 6
///     1 --> out
///     3 --> 2 --> out
///     5 --> 4 --> out
///     6 --> 4 --> out
/// ```
pub const FM_ALGORITHM_DX7_26: Algorithm = Algorithm::DX7(26);

#[cfg_attr(doc, aquamarine::aquamarine)]
/// DX7 algorithm 27
///
/// ```mermaid
/// flowchart
///     3 -->|repeat| 3
///     1 --> out
///     3 --> 2 --> out
///     5 --> 4 --> out
///     6 --> 4 --> out
/// ```
pub const FM_ALGORITHM_DX7_27: Algorithm = Algorithm::DX7(27);

#[cfg_attr(doc, aquamarine::aquamarine)]
/// DX7 algorithm 28
///
/// ```mermaid
/// flowchart
///     5 -->|repeat| 5
///     2 --> 1 --> out
///     5 --> 4 --> 3 --> out
///     6 --> out
/// ```
pub const FM_ALGORITHM_DX7_28: Algorithm = Algorithm::DX7(28);

#[cfg_attr(doc, aquamarine::aquamarine)]
/// DX7 algorithm 29
///
/// ```mermaid
/// flowchart
///     6 -->|repeat| 6
///     1 --> out
///     2 --> out
///     4 --> 3 --> out
///     6 --> 5 --> out
/// ```
pub const FM_ALGORITHM_DX7_29: Algorithm = Algorithm::DX7(29);

#[cfg_attr(doc, aquamarine::aquamarine)]
/// DX7 algorithm 30
///
/// ```mermaid
/// flowchart
///     5 -->|repeat| 5
///     1 --> out
///     2 --> out
///     5 --> 4 --> 3 --> out
///     6 --> out
/// ```
pub const FM_ALGORITHM_DX7_30: Algorithm = Algorithm::DX7(30);

#[cfg_attr(doc, aquamarine::aquamarine)]
/// DX7 algorithm 31
///
/// ```mermaid
/// flowchart
///     6 -->|repeat| 6
///     6 --> 5 --> out
///     4 --> out
///     3 --> out
///     2 --> out
///     1 --> out
/// ```
pub const FM_ALGORITHM_DX7_31: Algorithm = Algorithm::DX7(31);

#[cfg_attr(doc, aquamarine::aquamarine)]
/// DX7 algorithm 32
///
/// ```mermaid
/// flowchart
///     6 -->|repeat| 6
///     6 --> out
///     5 --> out
///     4 --> out
///     3 --> out
///     2 --> out
///     1 --> out
/// ```
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
