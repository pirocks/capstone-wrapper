use capstone::arch::x86::{X86InsnDetail, X86OperandType};
use capstone::prelude::DetailsArchInsn;
use itertools::Itertools;
use crate::memory_operand::{MemoryOperandOrRegister16, MemoryOperandOrRegister64};
use crate::registers::{OperandSize, Register16, Register64};

pub enum AddInstruction{
    R64Rm64{
        target: MemoryOperandOrRegister64,
        to_add: Register64
    }
}

impl AddInstruction {
    pub fn from_details(detail: &X86InsnDetail) -> Self {
        let operands = detail.operands().collect_vec();
        if operands.len() != 2{
            todo!()
        }
        let target_op_size = OperandSize::from_capstone_size(operands[0].size);
        match &operands[0].op_type {
            X86OperandType::Reg(reg_id) => {
                match target_op_size {
                    OperandSize::QuadWord => {
                        let target = MemoryOperandOrRegister64::Reg(Register64::new(*reg_id));
                        match &operands[1].op_type {
                            X86OperandType::Reg(reg_id) => {
                                let to_add = Register64::new(*reg_id);
                                AddInstruction::R64Rm64 { target, to_add }
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
                    OperandSize::DoubleWord => {
                        todo!()
                    }
                    OperandSize::Word => {
                        let target = MemoryOperandOrRegister16::Reg(Register16::new(*reg_id));
                        match &operands[1].op_type {
                            X86OperandType::Reg(_) => {
                                todo!()
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
                    OperandSize::HalfWord => {
                        todo!()
                    }
                }
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

