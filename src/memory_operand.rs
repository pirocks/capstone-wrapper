use capstone::arch::x86::X86OpMem;
use capstone::arch::x86::X86Reg::X86_REG_INVALID;

use crate::registers::{GeneralRegister, OperandSize, Register16, Register32, Register64, Register8, SegmentRegister};
use crate::{ImmediateOperand, X86Scale};

//todo consider templating to avoid duplication

#[derive(Debug, Eq, PartialEq)]
pub enum MemoryOperandOrRegister8 {
    Reg(Register8),
    Mem(MemoryOperand),
}

impl MemoryOperandOrRegister8 {
    pub fn unwrap_reg(&self) -> Register8 {
        match self {
            MemoryOperandOrRegister8::Reg(reg) => *reg,
            MemoryOperandOrRegister8::Mem(_) => {
                panic!("Expected register")
            }
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum MemoryOperandOrRegister16 {
    Reg(Register16),
    Mem(MemoryOperand),
}

impl MemoryOperandOrRegister16 {
    pub fn unwrap_reg(&self) -> Register16 {
        match self {
            MemoryOperandOrRegister16::Reg(reg) => *reg,
            MemoryOperandOrRegister16::Mem(_) => {
                panic!("Expected register")
            }
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum MemoryOperandOrRegister32 {
    Reg(Register32),
    Mem(MemoryOperand),
}

impl MemoryOperandOrRegister32 {
    pub fn unwrap_reg(&self) -> Register32 {
        match self {
            MemoryOperandOrRegister32::Reg(reg) => *reg,
            MemoryOperandOrRegister32::Mem(_) => {
                panic!("Expected register")
            }
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum MemoryOperandOrRegister64 {
    Reg(Register64),
    Mem(MemoryOperand),
}

impl MemoryOperandOrRegister64 {
    pub fn unwrap_reg(&self) -> Register64 {
        match self {
            MemoryOperandOrRegister64::Reg(reg) => *reg,
            MemoryOperandOrRegister64::Mem(_) => {
                panic!("Expected register")
            }
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct MemoryOperand {
    pub seg: Option<SegmentRegister>,
    pub base: GeneralRegister,
    pub scale: X86Scale,
    pub index: Option<GeneralRegister>,
    pub offset: ImmediateOperand,
}

impl MemoryOperand {
    pub fn base64(reg: Register64) -> MemoryOperand {
        Self {
            seg: None,
            base: GeneralRegister::Register64(reg),
            scale: X86Scale::One,
            index: None,
            offset: ImmediateOperand::Imm32(0),
        }
    }

    pub fn from_mem(mem: &X86OpMem) -> Self {
        let base = GeneralRegister::new(mem.base(), OperandSize::QuadWord); //todo what about not 64 bit stuff
        let index = if mem.index() != X86_REG_INVALID.into() {
            Some(GeneralRegister::new(mem.index(), OperandSize::QuadWord))
        } else {
            None
        };
        MemoryOperand {
            seg: todo!(),
            base,
            scale: X86Scale::from_capstone_scale(mem.scale()),
            index,
            offset: ImmediateOperand::from_capstone_displacement(mem.disp()),
        }
    }
}
