use crate::registers::{Reg16WithRIP, Reg32WithRIP, Reg64WithRIP, Reg8};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum OperandSize {
    QuadWord,
    DoubleWord,
    Word,
    HalfWord,
}

impl OperandSize {
    pub fn from_capstone_size(size: u8) -> OperandSize {
        match size {
            8 => Self::QuadWord,
            4 => Self::DoubleWord,
            2 => Self::Word,
            1 => Self::HalfWord,
            _ => {
                panic!("Unexpected operand size from capstone")
            }
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum ImmediateOperand {
    Imm8(i8),
    Imm16(i16),
    Imm32(i32),
}

impl ImmediateOperand {
    pub fn from_capstone_displacement(capstone_displacement: i64) -> ImmediateOperand {
        let res: i32 = capstone_displacement.try_into().expect("But x86 doesn't have 64 bit displacements?");
        ImmediateOperand::Imm32(res)
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum X86Scale {
    One,
    Two,
    Four,
    Eight,
}

impl X86Scale {
    pub fn from_capstone_scale(capstone_scale: i32) -> X86Scale {
        match capstone_scale {
            1 => X86Scale::One,
            2 => X86Scale::Two,
            4 => X86Scale::Four,
            8 => X86Scale::Eight,
            _ => {
                panic!("Unexpected scale?")
            }
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum GeneralRegister {
    Reg64(Reg64WithRIP),
    Reg32(Reg32WithRIP),
    Reg16(Reg16WithRIP),
    Reg8(Reg8),
}


#[derive(Debug, Eq, PartialEq)]
pub struct MemoryOperand {
    pub base: GeneralRegister,
    pub scale: X86Scale,
    pub index: Option<GeneralRegister>,
    pub offset: ImmediateOperand,
}

impl MemoryOperand {
    pub fn base64(reg: Reg64WithRIP) -> MemoryOperand {
        Self{
            base: GeneralRegister::Reg64(reg),
            scale: X86Scale::One,
            index: None,
            offset: ImmediateOperand::Imm32(0),
        }
    }

    /*pub fn from_mem(mem: &X86OpMem) -> Self {
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
    }*/
}

