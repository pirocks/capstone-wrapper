use crate::memory_operand::MemoryOperandOrRegister8;
use crate::registers::Register8;
use capstone::arch::x86::{X86InsnDetail, X86OperandType};
use capstone::prelude::DetailsArchInsn;
use itertools::Itertools;

#[derive(Debug, Eq, PartialEq)]
pub struct SetBInstruction {
    target: MemoryOperandOrRegister8,
}

impl SetBInstruction {
    pub fn from_details(detail: &X86InsnDetail) -> Self {
        let operands = detail.operands().collect_vec();
        if operands.len() != 1 {
            todo!()
        }
        match &operands[0].op_type {
            X86OperandType::Reg(reg_id) => SetBInstruction {
                target: MemoryOperandOrRegister8::Reg(Register8::new(*reg_id)),
            },
            X86OperandType::Imm(_) => {
                todo!()
            }
            X86OperandType::Mem(_) => {
                todo!()
            }
            X86OperandType::Invalid => {
                todo!()
            }
        }
    }
}
