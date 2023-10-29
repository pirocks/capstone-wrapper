use crate::memory_operand::{
    MemoryOperand, MemoryOperandOrRegister16, MemoryOperandOrRegister32, MemoryOperandOrRegister64,
    MemoryOperandOrRegister8,
};
use crate::registers::{OperandSize, Register16, Register32, Register64, Register8};
use crate::utils::{imm_i16, imm_i32, imm_i8};
use capstone::arch::x86::{X86InsnDetail, X86Operand, X86OperandType};
use capstone::prelude::DetailsArchInsn;
use itertools::Itertools;

#[derive(Debug, Eq, PartialEq)]
pub enum AddInstruction {
    ALimm8 {
        imm8: i8,
    },
    AXimm16 {
        imm8: i16,
    },
    EAXimm32 {
        imm16: i32,
    },
    RAXimm32 {
        imm32: i32,
    },
    Rm8imm8 {
        target: MemoryOperandOrRegister8,
        imm8: i8,
    },
    Rm16imm16 {
        target: MemoryOperandOrRegister16,
        imm16: i16,
    },
    Rm32imm32 {
        target: MemoryOperandOrRegister32,
        imm32: i32,
    },
    Rm64imm32 {
        target: MemoryOperandOrRegister64,
        imm32: i32,
    },
    Rm16imm8 {
        target: MemoryOperandOrRegister16,
        imm8: i8,
    },
    Rm32imm8 {
        target: MemoryOperandOrRegister32,
        imm8: i8,
    },
    Rm64imm8 {
        target: MemoryOperandOrRegister64,
        imm8: i8,
    },
    Rm8R8 {
        target: MemoryOperandOrRegister8,
        to_add: Register8,
    },
    Rm16R16 {
        target: MemoryOperandOrRegister16,
        to_add: Register16,
    },
    Rm32R32 {
        target: MemoryOperandOrRegister32,
        to_add: Register32,
    },
    Rm64R64 {
        target: MemoryOperandOrRegister64,
        to_add: Register64,
    },
    R8Rm8 {
        target: Register8,
        to_add: MemoryOperandOrRegister8,
    },
    R16Rm16 {
        target: Register16,
        to_add: MemoryOperandOrRegister16,
    },
    R32Rm32 {
        target: Register32,
        to_add: MemoryOperandOrRegister32,
    },
    R64Rm64 {
        target: Register64,
        to_add: MemoryOperandOrRegister64,
    },
}

impl AddInstruction {
    pub fn from_details(detail: &X86InsnDetail) -> Self {
        let operands = detail.operands().collect_vec();
        if operands.len() != 2 {
            todo!()
        }
        let target_op_size = OperandSize::from_capstone_size(operands[0].size);
        match &operands[0].op_type {
            X86OperandType::Reg(reg_id) => match target_op_size {
                OperandSize::QuadWord => {
                    let target = MemoryOperandOrRegister64::Reg(Register64::new(*reg_id));
                    Self::with_target64(&operands[1], target)
                }
                OperandSize::DoubleWord => {
                    let target = MemoryOperandOrRegister32::Reg(Register32::new(*reg_id));
                    Self::with_target32(&operands[1], target)
                }
                OperandSize::Word => {
                    let target = MemoryOperandOrRegister16::Reg(Register16::new(*reg_id));
                    Self::with_target16(&operands[1], target)
                }
                OperandSize::HalfWord => {
                    let target = MemoryOperandOrRegister8::Reg(Register8::new(*reg_id));
                    Self::with_target8(&operands[1], target)
                }
            },
            X86OperandType::Mem(mem) => match target_op_size {
                OperandSize::QuadWord => {
                    let target = MemoryOperandOrRegister64::Mem(MemoryOperand::from_mem(mem));
                    Self::with_target64(&operands[1], target)
                }
                OperandSize::DoubleWord => {
                    let target = MemoryOperandOrRegister32::Mem(MemoryOperand::from_mem(mem));
                    Self::with_target32(&operands[1], target)
                }
                OperandSize::Word => {
                    let target = MemoryOperandOrRegister16::Mem(MemoryOperand::from_mem(mem));
                    Self::with_target16(&operands[1], target)
                }
                OperandSize::HalfWord => {
                    let target = MemoryOperandOrRegister8::Mem(MemoryOperand::from_mem(mem));
                    Self::with_target8(&operands[1], target)
                }
            },
            X86OperandType::Imm(_) | X86OperandType::Invalid => {
                panic!("Unexpected first operand type")
            }
        }
    }

    fn with_target8(next_operand: &X86Operand, target: MemoryOperandOrRegister8) -> AddInstruction {
        match &next_operand.op_type {
            X86OperandType::Reg(reg) => AddInstruction::Rm8R8 {
                target,
                to_add: Register8::new(*reg),
            },
            X86OperandType::Imm(imm) => {
                assert_eq!(
                    OperandSize::from_capstone_size(next_operand.size),
                    OperandSize::HalfWord
                );
                AddInstruction::Rm8imm8 {
                    target,
                    imm8: imm_i8(*imm),
                }
            }
            X86OperandType::Mem(mem) => AddInstruction::R8Rm8 {
                target: target.unwrap_reg(),
                to_add: MemoryOperandOrRegister8::Mem(MemoryOperand::from_mem(mem)),
            },
            X86OperandType::Invalid => {
                todo!()
            }
        }
    }

    fn with_target16(
        next_operand: &X86Operand,
        target: MemoryOperandOrRegister16,
    ) -> AddInstruction {
        match &next_operand.op_type {
            X86OperandType::Reg(reg) => AddInstruction::Rm16R16 {
                target,
                to_add: Register16::new(*reg),
            },
            X86OperandType::Imm(imm) => match OperandSize::from_capstone_size(next_operand.size) {
                OperandSize::QuadWord | OperandSize::DoubleWord => {
                    panic!("Unexpected large constant")
                }
                OperandSize::Word => AddInstruction::Rm16imm16 {
                    target,
                    imm16: imm_i16(*imm),
                },
                OperandSize::HalfWord => AddInstruction::Rm16imm8 {
                    target,
                    imm8: imm_i8(*imm),
                },
            },
            X86OperandType::Mem(mem) => AddInstruction::R16Rm16 {
                target: target.unwrap_reg(),
                to_add: MemoryOperandOrRegister16::Mem(MemoryOperand::from_mem(mem)),
            },
            X86OperandType::Invalid => {
                todo!()
            }
        }
    }

    fn with_target32(
        next_operand: &X86Operand,
        target: MemoryOperandOrRegister32,
    ) -> AddInstruction {
        match &next_operand.op_type {
            X86OperandType::Reg(reg) => AddInstruction::Rm32R32 {
                target,
                to_add: Register32::new(*reg),
            },
            X86OperandType::Imm(imm) => match OperandSize::from_capstone_size(next_operand.size) {
                OperandSize::DoubleWord => AddInstruction::Rm32imm32 {
                    target,
                    imm32: imm_i32(*imm),
                },
                OperandSize::QuadWord | OperandSize::Word => {
                    panic!("Unexpected constant size")
                }
                OperandSize::HalfWord => AddInstruction::Rm32imm8 {
                    target,
                    imm8: imm_i8(*imm),
                },
            },
            X86OperandType::Mem(mem) => AddInstruction::R32Rm32 {
                target: target.unwrap_reg(),
                to_add: MemoryOperandOrRegister32::Mem(MemoryOperand::from_mem(mem)),
            },
            X86OperandType::Invalid => {
                todo!()
            }
        }
    }

    fn with_target64(
        next_operand: &X86Operand,
        target: MemoryOperandOrRegister64,
    ) -> AddInstruction {
        match &next_operand.op_type {
            X86OperandType::Reg(reg_id) => {
                let to_add = Register64::new(*reg_id);
                AddInstruction::Rm64R64 { target, to_add }
            }
            X86OperandType::Imm(imm) => match OperandSize::from_capstone_size(next_operand.size) {
                OperandSize::QuadWord => AddInstruction::Rm64imm32 {
                    target,
                    imm32: imm_i32(*imm),
                },
                OperandSize::DoubleWord | OperandSize::Word => {
                    panic!("Unexpected constant size")
                }
                OperandSize::HalfWord => AddInstruction::Rm64imm8 {
                    target,
                    imm8: imm_i8(*imm),
                },
            },
            X86OperandType::Mem(mem) => AddInstruction::R64Rm64 {
                target: target.unwrap_reg(),
                to_add: MemoryOperandOrRegister64::Mem(MemoryOperand::from_mem(mem)),
            },
            X86OperandType::Invalid => {
                todo!()
            }
        }
    }
}

#[cfg(test)]
pub mod test {
    use crate::add_instruction::AddInstruction;
    use crate::memory_operand::{
        MemoryOperand, MemoryOperandOrRegister16, MemoryOperandOrRegister32,
        MemoryOperandOrRegister64, MemoryOperandOrRegister8,
    };
    use crate::registers::{GeneralRegister, Register16, Register32, Register64, Register8};
    use crate::utils::get_function_bytes;
    use crate::ImmediateOperand::Imm32;
    use crate::{disassemble, function_end_guard, X86Instruction, X86Scale};
    use std::arch::asm;
    use std::ffi::c_void;

    #[no_mangle]
    fn add_instruction_variants() {
        unsafe {
            asm!(
                // todo rust uses gas to assemble this and it appears there is no way to convince gas
                // to use certain encodings. This could be fixed nasm and the strict keyword, but thats
                // an issue for another day.
                "add al, -9",
                "add ax, 0x3",
                "add eax, -9",
                "add rax, -9",
                "add byte ptr [rcx], -9",
                "add word ptr [rdx], -99",
                "add dword ptr [rax + rcx*8 - 5],  -999",
                "add qword ptr [rax], -9999999",
                "add byte ptr [rax], bl",
                "add word ptr [rax], cx",
                "add dword ptr [rax], edx",
                "add qword ptr [rax], r9",
                "add bl, byte ptr [rax]",
                "add cx, word ptr [rax]",
                "add edx, dword ptr [rax]",
                "add r9, qword ptr [rax]",
            );
            function_end_guard!();
        }
    }

    #[test]
    pub fn disassemble_add_instruction_variants() {
        let raw_function_ptr = add_instruction_variants as *const c_void;
        let function_bytes = get_function_bytes(raw_function_ptr);
        let res = disassemble(function_bytes, raw_function_ptr as u64).unwrap();
        assert_eq!(
            &res[1],
            &X86Instruction::Add(AddInstruction::Rm8imm8 {
                target: MemoryOperandOrRegister8::Reg(Register8::AL),
                imm8: -9,
            })
        );
        assert_eq!(
            &res[2],
            &X86Instruction::Add(AddInstruction::Rm16imm16 {
                target: MemoryOperandOrRegister16::Reg(Register16::AX),
                imm16: 3,
            })
        );
        assert_eq!(
            &res[3],
            &X86Instruction::Add(AddInstruction::Rm32imm32 {
                target: MemoryOperandOrRegister32::Reg(Register32::EAX),
                imm32: -9,
            })
        );
        assert_eq!(
            &res[4],
            &X86Instruction::Add(AddInstruction::Rm64imm32 {
                target: MemoryOperandOrRegister64::Reg(Register64::RAX),
                imm32: -9,
            })
        );
        assert_eq!(
            &res[5],
            &X86Instruction::Add(AddInstruction::Rm8imm8 {
                target: MemoryOperandOrRegister8::Mem(MemoryOperand::base64(Register64::RCX)),
                imm8: -9,
            })
        );
        assert_eq!(
            &res[6],
            &X86Instruction::Add(AddInstruction::Rm16imm16 {
                target: MemoryOperandOrRegister16::Mem(MemoryOperand::base64(Register64::RDX)),
                imm16: -99,
            })
        );
        assert_eq!(
            &res[7],
            &X86Instruction::Add(AddInstruction::Rm32imm32 {
                target: MemoryOperandOrRegister32::Mem(MemoryOperand {
                    base: GeneralRegister::Register64(Register64::RAX),
                    scale: X86Scale::Eight,
                    index: Some(GeneralRegister::Register64(Register64::RCX)),
                    offset: Imm32(-5),
                }),
                imm32: -999,
            })
        );
        assert_eq!(
            &res[8],
            &X86Instruction::Add(AddInstruction::Rm64imm32 {
                target: MemoryOperandOrRegister64::Mem(MemoryOperand::base64(Register64::RAX)),
                imm32: -9999999,
            })
        );
        assert_eq!(
            &res[9],
            &X86Instruction::Add(AddInstruction::Rm8R8 {
                target: MemoryOperandOrRegister8::Mem(MemoryOperand::base64(Register64::RAX)),
                to_add: Register8::BL,
            })
        );
        assert_eq!(
            &res[10],
            &X86Instruction::Add(AddInstruction::Rm16R16 {
                target: MemoryOperandOrRegister16::Mem(MemoryOperand::base64(Register64::RAX)),
                to_add: Register16::CX,
            })
        );
        assert_eq!(
            &res[11],
            &X86Instruction::Add(AddInstruction::Rm32R32 {
                target: MemoryOperandOrRegister32::Mem(MemoryOperand::base64(Register64::RAX)),
                to_add: Register32::EDX,
            })
        );
        assert_eq!(
            &res[12],
            &X86Instruction::Add(AddInstruction::Rm64R64 {
                target: MemoryOperandOrRegister64::Mem(MemoryOperand::base64(Register64::RAX)),
                to_add: Register64::R9,
            })
        );
        assert_eq!(
            &res[13],
            &X86Instruction::Add(AddInstruction::R8Rm8 {
                to_add: MemoryOperandOrRegister8::Mem(MemoryOperand::base64(Register64::RAX)),
                target: Register8::BL,
            })
        );
        assert_eq!(
            &res[14],
            &X86Instruction::Add(AddInstruction::R16Rm16 {
                to_add: MemoryOperandOrRegister16::Mem(MemoryOperand::base64(Register64::RAX)),
                target: Register16::CX,
            })
        );
        assert_eq!(
            &res[15],
            &X86Instruction::Add(AddInstruction::R32Rm32 {
                to_add: MemoryOperandOrRegister32::Mem(MemoryOperand::base64(Register64::RAX)),
                target: Register32::EDX,
            })
        );
        assert_eq!(
            &res[16],
            &X86Instruction::Add(AddInstruction::R64Rm64 {
                to_add: MemoryOperandOrRegister64::Mem(MemoryOperand::base64(Register64::RAX)),
                target: Register64::R9,
            })
        );
    }
}
