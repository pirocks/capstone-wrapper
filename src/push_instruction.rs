use capstone::arch::x86::{X86InsnDetail, X86OperandType};
use capstone::prelude::DetailsArchInsn;
use itertools::Itertools;
use crate::{ImmediateOperand};
use crate::memory_operand::MemoryOperand;
use crate::registers::{GeneralRegisterWordAndBigger, OperandSize, SegmentRegister};

//https://www.felixcloutier.com/x86/push
#[derive(Debug, Eq, PartialEq)]
pub enum PushInstruction {
    MemoryOperand {
        memory_operand: MemoryOperand
    },
    Register {
        register: GeneralRegisterWordAndBigger
    },
    Imm {
        imm: ImmediateOperand
    },
    PushSegment {
        segment_register: SegmentRegister
    },
}

impl PushInstruction {
    pub fn from_detail(detail: &X86InsnDetail) -> Self {
        let operands = detail.operands().collect_vec();
        let first_operand = &operands[0];
        let operand_size = OperandSize::from_capstone_size(first_operand.size);
        match first_operand.op_type {
            X86OperandType::Reg(register_id) => {
                let register = GeneralRegisterWordAndBigger::new(register_id, operand_size);
                PushInstruction::Register { register }
            }
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
