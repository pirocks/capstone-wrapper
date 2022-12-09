use capstone::arch::x86::{X86InsnDetail, X86OperandType};
use capstone::Insn;
use capstone::prelude::DetailsArchInsn;
use itertools::Itertools;
use crate::condition_code_flag::JumpConditionCode;
use crate::registers::OperandSize;

#[derive(Debug, Eq, PartialEq)]
pub struct  JCCInstruction{
    absolute_target: u64,
    condition_code: JumpConditionCode
}

impl JCCInstruction {
    pub fn from_details(_instruction: &Insn, detail: &X86InsnDetail, op_code: JumpConditionCode) -> Self {
        let operands = detail.operands().collect_vec();
        if operands.len() != 1{
            todo!()
        }
        let operand_size = OperandSize::from_capstone_size(operands[0].size);
        match &operands[0].op_type {
            X86OperandType::Reg(_) => {
                todo!()
            }
            X86OperandType::Imm(imm) => {
                JCCInstruction{
                    absolute_target: *imm as u64,
                    condition_code: op_code
                }
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
