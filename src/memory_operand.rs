use capstone::arch::x86::X86OpMem;
use capstone::arch::x86::X86Reg::X86_REG_INVALID;

use crate::{ImmediateOperand, X86Scale};
use crate::registers::{GeneralRegister, OperandSize, Register16, Register32, Register64, Register8};

#[derive(Debug)]
pub enum MemoryOperandOrRegister8 {
    Reg(Register8),
    Mem(MemoryOperand),
}

#[derive(Debug)]
pub enum MemoryOperandOrRegister16 {
    Reg(Register16),
    Mem(MemoryOperand),
}

#[derive(Debug)]
pub enum MemoryOperandOrRegister32 {
    Reg(Register32),
    Mem(MemoryOperand),
}

#[derive(Debug)]
pub enum MemoryOperandOrRegister64 {
    Reg(Register64),
    Mem(MemoryOperand),
}

#[derive(Debug)]
pub struct MemoryOperand {
    base: GeneralRegister,
    scale: X86Scale,
    index: Option<GeneralRegister>,
    offset: ImmediateOperand,
}

impl MemoryOperand {
    pub fn from_mem(mem: &X86OpMem) -> Self {
        let base = GeneralRegister::new(mem.base(), OperandSize::QuadWord);//todo what about not 64 bit stuff
        let index = if mem.index() != X86_REG_INVALID.into() {
            Some(GeneralRegister::new(mem.index(), OperandSize::QuadWord))
        } else {
            None
        };
        MemoryOperand {
            base,
            scale: X86Scale::from_capstone_scale(mem.scale()),
            index,
            offset: ImmediateOperand::from_capstone_displacement(mem.disp()),
        }
    }
}
