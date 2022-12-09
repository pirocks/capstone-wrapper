use capstone::arch::x86::{X86InsnDetail, X86OperandType};
use capstone::prelude::DetailsArchInsn;
use itertools::Itertools;

use crate::memory_operand::{MemoryOperand, MemoryOperandOrRegister16, MemoryOperandOrRegister32, MemoryOperandOrRegister64, MemoryOperandOrRegister8};
use crate::registers::{OperandSize, Register16, Register32, Register64, Register8};

//https://www.felixcloutier.com/x86/mov
//https://www.felixcloutier.com/x86/mov-1
//https://www.felixcloutier.com/x86/mov-2
pub enum MovInstruction {
    Rm8R8 {
        src: Register8,
        dst: MemoryOperandOrRegister8,
    },
    Rm16R16 {
        src: Register16,
        dst: MemoryOperandOrRegister16,
    },
    Rm32R32 {
        src: Register32,
        dst: MemoryOperandOrRegister32,
    },
    Rm64R64 {
        src: Register64,
        dst: MemoryOperandOrRegister64,
    },
    R8Rm8 {
        src: MemoryOperandOrRegister8,
        dst: Register8,
    },
    R16Rm16 {
        src: MemoryOperandOrRegister16,
        dst: Register16,
    },
    R32Rm32 {
        src: MemoryOperandOrRegister32,
        dst: Register32,
    },
    R64Rm64 {
        src: MemoryOperandOrRegister64,
        dst: Register64,
    },
}

impl MovInstruction {
    pub fn from_detail(detail: &X86InsnDetail) -> Self {
        let operands = detail.operands().collect_vec();
        let dst_operand = &operands[0];
        let src_operand = &operands[1];
        let dst_operand_size = OperandSize::from_capstone_size(dst_operand.size);
        let src_operand_size = OperandSize::from_capstone_size(src_operand.size);
        match &dst_operand.op_type {
            X86OperandType::Reg(dst_reg_id) => {
                //todo handle segment register
                // let dst_general_register = GeneralRegister::new(*dst_reg_id, dst_operand_size);
                match &src_operand.op_type {
                    X86OperandType::Reg(src_reg_id) => {
                        assert_eq!(src_operand_size, dst_operand_size);
                        match src_operand_size {
                            OperandSize::QuadWord => {
                                let dst = Register64::new(*dst_reg_id);
                                let src = Register64::new(*src_reg_id);
                                MovInstruction::R64Rm64 { src: MemoryOperandOrRegister64::Reg(src), dst: dst }
                            }
                            OperandSize::DoubleWord => {
                                todo!()
                            }
                            OperandSize::Word => {
                                todo!()
                            }
                            OperandSize::HalfWord => {
                                todo!()
                            }
                        }
                    }
                    X86OperandType::Imm(_) => {
                        todo!()
                    }
                    X86OperandType::Mem(src_mem) => {
                        assert_eq!(src_operand_size, dst_operand_size);
                        match src_operand_size {
                            OperandSize::QuadWord => {
                                let dst = Register64::new(*dst_reg_id);
                                let src = MemoryOperandOrRegister64::Mem(MemoryOperand::from_mem(src_mem));
                                MovInstruction::R64Rm64 { src, dst }
                            }
                            OperandSize::DoubleWord => {
                                todo!()
                            }
                            OperandSize::Word => {
                                todo!()
                            }
                            OperandSize::HalfWord => {
                                todo!()
                            }
                        }
                    }
                    X86OperandType::Invalid => {
                        todo!()
                    }
                }
            }
            X86OperandType::Imm(_) => {
                todo!()
            }
            X86OperandType::Mem(mem) => {
                match dst_operand_size {
                    OperandSize::QuadWord => {
                        let dst = MemoryOperandOrRegister64::Mem(MemoryOperand::from_mem(mem));
                        match &src_operand.op_type {
                            X86OperandType::Reg(reg_id) => {
                                let src = Register64::new(*reg_id);
                                MovInstruction::Rm64R64 { src, dst }
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
                        todo!()
                    }
                    OperandSize::HalfWord => {
                        todo!()
                    }
                }
            }
            X86OperandType::Invalid => {
                todo!()
            }
        }
    }
}
