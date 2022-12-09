use capstone::arch::x86::{X86InsnDetail, X86OperandType};
use capstone::prelude::DetailsArchInsn;
use itertools::Itertools;
use crate::memory_operand::MemoryOperandOrRegister8;
use crate::registers::{OperandSize, Register8};

#[derive(Debug, Eq, PartialEq)]
pub enum TestInstruction {
    Rm8Imm8{
        rm8: MemoryOperandOrRegister8,
        imm8: i8
    }
}

impl TestInstruction {
    pub fn from_details(detail: &X86InsnDetail) -> Self {
        let operands = detail.operands().collect_vec();
        if operands.len() != 2 {
            todo!()
        }
        let first_op_size = OperandSize::from_capstone_size(operands[0].size);
        match &operands[0].op_type {
            X86OperandType::Reg(reg_id) => {
                match first_op_size {
                    OperandSize::QuadWord => {
                        todo!()
                    }
                    OperandSize::DoubleWord => {
                        todo!()
                    }
                    OperandSize::Word => {
                        todo!()
                    }
                    OperandSize::HalfWord => {
                        let op_one = MemoryOperandOrRegister8::Reg(Register8::new(*reg_id));
                        let operand_two_size = OperandSize::from_capstone_size(operands[1].size);
                        match &operands[1].op_type {
                            X86OperandType::Reg(_) => {
                                todo!()
                            }
                            X86OperandType::Imm(imm) => {
                                match operand_two_size {
                                    OperandSize::QuadWord => {
                                        todo!()
                                    }
                                    OperandSize::DoubleWord => {
                                        todo!()
                                    }
                                    OperandSize::Word => {
                                        todo!()
                                    }
                                    OperandSize::HalfWord => {
                                        let imm8 = (*imm).try_into().expect("Bad immediate from capstone");
                                        TestInstruction::Rm8Imm8 { rm8: op_one, imm8 }
                                    }
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
