use enum_iterator::Sequence;
use crate::registers::{Reg16WithRIP, Reg32WithRIP, Reg64WithRIP, Reg8};
use xed_sys::{xed_reg_enum_t, xed_uint_t};

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
        let res: i32 = capstone_displacement
            .try_into()
            .expect("But x86 doesn't have 64 bit displacements?");
        ImmediateOperand::Imm32(res)
    }

    pub fn to_xed(&self) -> u64{
        todo!("self:?")
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Sequence)]
pub enum X86Scale {
    One,
    Two,
    Four,
    Eight,
}

impl X86Scale {
    pub fn from_raw_scale(capstone_scale: i32) -> X86Scale {
        match capstone_scale {
            1 => X86Scale::One,
            2 => X86Scale::Two,
            4 => X86Scale::Four,
            8 => X86Scale::Eight,
            _ => {
                panic!("Unexpected scale?: {capstone_scale}")
            }
        }
    }

    pub fn to_xed(&self) -> xed_uint_t {
        match self {
            X86Scale::One => 1,
            X86Scale::Two => 2,
            X86Scale::Four => 4,
            X86Scale::Eight => 9,
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum GeneralReg {
    Reg64(Reg64WithRIP),
    Reg32(Reg32WithRIP),
    Reg16(Reg16WithRIP),
    Reg8(Reg8),
}

impl GeneralReg {

    pub fn bit_width(&self) -> usize{
        match self {
            GeneralReg::Reg64(_) => 64,
            GeneralReg::Reg32(_) => 32,
            GeneralReg::Reg16(_) => 16,
            GeneralReg::Reg8(_) => 8,
        }
    }

    pub fn try_new(reg: xed_reg_enum_t, width: Option<u32>) -> Option<Self> {
        use xed_sys::*;
        let class: xed_reg_class_enum_t = unsafe { xed_reg_class(reg) };
        Some(match class {
            XED_REG_CLASS_GPR => {
                if reg >= XED_REG_GPR64_FIRST && reg <= XED_REG_GPR64_LAST{
                    return Some(GeneralReg::Reg64(Reg64WithRIP::try_new(reg)?))
                }
                match width {
                    None => todo!(),
                    Some(64) => GeneralReg::Reg64(Reg64WithRIP::try_new(reg)?),
                    Some(32) => GeneralReg::Reg32(Reg32WithRIP::try_new(reg)?),
                    Some(16) => GeneralReg::Reg16(Reg16WithRIP::try_new(reg)?),
                    width => todo!("{width:?}")
                }
            }
            XED_REG_CLASS_GPR8 => GeneralReg::Reg8(Reg8::try_new(reg)?),
            XED_REG_CLASS_GPR16 => GeneralReg::Reg16(Reg16WithRIP::try_new(reg)?),
            XED_REG_CLASS_GPR32 => GeneralReg::Reg32(Reg32WithRIP::try_new(reg)?),
            XED_REG_CLASS_GPR64 => GeneralReg::Reg64(Reg64WithRIP::try_new(reg)?),
            XED_REG_CLASS_INVALID => {
                dbg!(reg);
                todo!()
            }
            _ => return None,
        })
    }

    pub fn to_xed(&self) -> xed_reg_enum_t{
        match self {
            GeneralReg::Reg64(reg) => reg.to_xed(),
            GeneralReg::Reg32(reg) => reg.to_xed(),
            GeneralReg::Reg16(reg) => reg.to_xed(),
            GeneralReg::Reg8(reg) => reg.to_xed()
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum GeneralReg3264 {
    Reg64(Reg64WithRIP),
    Reg32(Reg32WithRIP),
}

impl GeneralReg3264 {
    pub fn try_new(xed: xed_reg_enum_t) -> Option<Self> {
        use xed_sys::*;
        let class: xed_reg_class_enum_t = unsafe { xed_reg_class(xed) };
        Some(match class {
            XED_REG_CLASS_GPR32 => Self::Reg32(Reg32WithRIP::try_new(xed)?),
            XED_REG_CLASS_GPR64 => Self::Reg64(Reg64WithRIP::try_new(xed)?),
            _ => return None,
        })
    }

    pub fn to_xed(&self) -> xed_reg_enum_t{
        todo!("self:?")
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum GeneralReg163264 {
    Reg64(Reg64WithRIP),
    Reg32(Reg32WithRIP),
    Reg16(Reg16WithRIP),
}

impl GeneralReg163264 {
    pub fn try_new(xed: xed_reg_enum_t) -> Option<Self> {
        use xed_sys::*;
        let class: xed_reg_class_enum_t = unsafe { xed_reg_class(xed) };
        Some(match class {
            XED_REG_CLASS_GPR16 => Self::Reg16(Reg16WithRIP::try_new(xed)?),
            XED_REG_CLASS_GPR32 => Self::Reg32(Reg32WithRIP::try_new(xed)?),
            XED_REG_CLASS_GPR64 => Self::Reg64(Reg64WithRIP::try_new(xed)?),
            _ => return None,
        })
    }

    pub fn to_xed(&self) -> xed_reg_enum_t{
        todo!("self:?")
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct MemoryOperand {
    pub base: GeneralReg,
    pub scale: X86Scale,
    pub index: Option<GeneralReg>,
    pub offset: ImmediateOperand,
}

impl MemoryOperand {
    pub fn base64(reg: Reg64WithRIP) -> MemoryOperand {
        Self {
            base: GeneralReg::Reg64(reg),
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
