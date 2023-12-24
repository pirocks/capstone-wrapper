use wrapper_common::memory_operand::GeneralReg;
use crate::semantics2::expression::{Expression, Flag};
use crate::semantics2::state::ConcreteX86MachineState64;
use crate::x86_machine::semantics_builder::FlagTag;

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
    InstructionSyncPoint {
        interruptable: bool,
    },
    UndefinedException,
    CalculateFlags { flag_tag: FlagTag, left: &'arena Expression<'arena>, right: &'arena Expression<'arena> },
}

fn apply_instructions_to_concrete<'arena>(concrete: ConcreteX86MachineState64, instructions: &[InstructionSemanticsStep<'arena>]) -> ConcreteX86MachineState64 {
    for instruction in instructions.iter() {
        match instruction {
            InstructionSemanticsStep::Conditional { condition, true_semantics, false_semantics } => {
                condition.apply_concrete(&concrete);
            }
            InstructionSemanticsStep::SetRegister { zero_upper, register, value } => {

            }
            InstructionSemanticsStep::SetFlag { flag, value } => {

            }
            InstructionSemanticsStep::InstructionSyncPoint { interruptable } => {

            }
            InstructionSemanticsStep::UndefinedException => {

            }
            InstructionSemanticsStep::CalculateFlags { flag_tag, left, right } => {

            }
        }
    }
    concrete
}
