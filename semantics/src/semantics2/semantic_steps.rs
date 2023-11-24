use wrapper_common::memory_operand::GeneralReg;
use crate::semantics2::expression::{Expression, Flag};

pub enum ZeroUpper {
    ZeroUpper,
    NoZeroUpper,
}

pub enum InstructionSemanticsStep<'arena> {
    Conditional {
        condition: &'arena Expression<'arena>,
        true_semantics: Vec<InstructionSemanticsStep<'arena>>,
        false_semantics: Vec<InstructionSemanticsStep<'arena>>,
    },
    SetRegister {
        zero_upper: ZeroUpper,
        register: GeneralReg,
        value: &'arena Expression<'arena>,
    },
    SetFlag {
        flag: Flag,
        value: &'arena Expression<'arena>,
    },
    UndefinedException,
}
