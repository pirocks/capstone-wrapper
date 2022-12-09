use capstone::arch::x86::{X86InsnDetail, X86OperandType};
use capstone::prelude::DetailsArchInsn;
use itertools::Itertools;

use crate::memory_operand::{MemoryOperandOrRegister16, MemoryOperandOrRegister32, MemoryOperandOrRegister64, MemoryOperandOrRegister8};
use crate::registers::{OperandSize, Register16};

//https://www.felixcloutier.com/x86/sal:sar:shl:shr
pub enum ShlInstruction {
    Rm8_1 {
        target: MemoryOperandOrRegister8
    },
    Rm8CL {
        target: MemoryOperandOrRegister8
    },
    Rm8Imm8 {
        target: MemoryOperandOrRegister8,
        imm8: i8,
    },
    Rm16_1 {
        target: MemoryOperandOrRegister16,
    },
    Rm16CL {
        target: MemoryOperandOrRegister16,
    },
    Rm16Imm8 {
        target: MemoryOperandOrRegister16,
        imm8: i8,
    },
    Rm32_1 {
        target: MemoryOperandOrRegister32,
    },
    Rm32CL {
        target: MemoryOperandOrRegister32,
    },
    Rm32Imm8 {
        target: MemoryOperandOrRegister32,
        imm8: i8,
    },
    Rm64_1 {
        target: MemoryOperandOrRegister64,
    },
    Rm64CL {
        target: MemoryOperandOrRegister64,
    },
    Rm64Imm8 {
        target: MemoryOperandOrRegister64,
        imm8: i8,
    },
}

impl ShlInstruction {
    pub fn from_details(detail: &X86InsnDetail) -> Self {
        let operands = detail.operands().collect_vec();
        if operands.len() == 1 {
            todo!()
        } else if operands.len() == 2 {
            let target_operand_size = OperandSize::from_capstone_size(operands[0].size);
            match &operands[0].op_type {
                X86OperandType::Reg(reg_id) => {
                    match target_operand_size {
                        OperandSize::QuadWord => {
                            todo!()
                        }
                        OperandSize::DoubleWord => {
                            todo!()
                        }
                        OperandSize::Word => {
                            let target = MemoryOperandOrRegister16::Reg(Register16::new(*reg_id));
                            match &operands[1].op_type{
                                X86OperandType::Reg(_) => {
                                    todo!()
                                }
                                X86OperandType::Imm(imm) => {
                                    let imm8 = (*imm).try_into().expect("malformed data from capstone?");
                                    ShlInstruction::Rm16Imm8 { target, imm8 }
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
        } else {
            todo!()
        }
    }
}
