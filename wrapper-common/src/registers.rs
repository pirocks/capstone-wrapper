use std::collections::{BTreeSet, HashSet};

use capstone::arch::x86::X86Operand;
use capstone::RegId;
use enum_iterator::Sequence;
use proc_macro2::Ident;
use quote::format_ident;
use serde::{Deserialize, Serialize};
use xed_sys::*;
use crate::operand_width::XedOperandWidth;

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum RegMMX {
    MM0,
    MM1,
    MM2,
    MM3,
    MM4,
    MM5,
    MM6,
    MM7,
}

impl RegMMX {
    pub fn try_new(reg: xed_reg_enum_t) -> Option<Self> {
        Some(match reg {
            XED_REG_MMX0 => RegMMX::MM0,
            XED_REG_MMX1 => RegMMX::MM1,
            XED_REG_MMX2 => RegMMX::MM2,
            XED_REG_MMX3 => RegMMX::MM3,
            XED_REG_MMX4 => RegMMX::MM4,
            XED_REG_MMX5 => RegMMX::MM5,
            XED_REG_MMX6 => RegMMX::MM6,
            XED_REG_MMX7 => RegMMX::MM7,
            _ => return None,
        })
    }

    pub fn to_xed(&self) -> xed_reg_enum_t {
        todo!("self:?")
    }
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Ord, PartialOrd,Serialize, Deserialize)]
pub enum RegXMM {
    XMM0,
    XMM1,
    XMM2,
    XMM3,
    XMM4,
    XMM5,
    XMM6,
    XMM7,
    XMM8,
    XMM9,
    XMM10,
    XMM11,
    XMM12,
    XMM13,
    XMM14,
    XMM15,
    XMM16,
    XMM17,
    XMM18,
    XMM19,
    XMM20,
    XMM21,
    XMM22,
    XMM23,
    XMM24,
    XMM25,
    XMM26,
    XMM27,
    XMM28,
    XMM29,
    XMM30,
    XMM31,
    XMM32,
}

impl RegXMM {
    pub fn try_new(reg: xed_reg_enum_t) -> Option<Self> {
        Some(match reg {
            XED_REG_XMM0 => RegXMM::XMM0,
            XED_REG_XMM1 => RegXMM::XMM1,
            XED_REG_XMM2 => RegXMM::XMM2,
            XED_REG_XMM3 => RegXMM::XMM3,
            XED_REG_XMM4 => RegXMM::XMM4,
            XED_REG_XMM5 => RegXMM::XMM5,
            XED_REG_XMM6 => RegXMM::XMM6,
            XED_REG_XMM7 => RegXMM::XMM7,
            XED_REG_XMM8 => RegXMM::XMM8,
            XED_REG_XMM9 => RegXMM::XMM9,
            XED_REG_XMM10 => RegXMM::XMM10,
            XED_REG_XMM11 => RegXMM::XMM11,
            XED_REG_XMM12 => RegXMM::XMM12,
            XED_REG_XMM13 => RegXMM::XMM13,
            XED_REG_XMM14 => RegXMM::XMM14,
            XED_REG_XMM15 => RegXMM::XMM15,
            XED_REG_XMM16 => RegXMM::XMM16,
            XED_REG_XMM17 => RegXMM::XMM17,
            XED_REG_XMM18 => RegXMM::XMM18,
            XED_REG_XMM19 => RegXMM::XMM19,
            XED_REG_XMM20 => RegXMM::XMM20,
            XED_REG_XMM21 => RegXMM::XMM21,
            XED_REG_XMM22 => RegXMM::XMM22,
            XED_REG_XMM23 => RegXMM::XMM23,
            XED_REG_XMM24 => RegXMM::XMM24,
            XED_REG_XMM25 => RegXMM::XMM25,
            XED_REG_XMM26 => RegXMM::XMM26,
            XED_REG_XMM27 => RegXMM::XMM27,
            XED_REG_XMM28 => RegXMM::XMM28,
            XED_REG_XMM29 => RegXMM::XMM29,
            XED_REG_XMM30 => RegXMM::XMM30,
            XED_REG_XMM31 => RegXMM::XMM31,
            _ => return None,
        })
    }

    pub fn to_declaration_string(&self) -> String {
        match self {
            RegXMM::XMM0 => "RegXMM::XMM0".to_string(),
            RegXMM::XMM1 => "RegXMM::XMM1".to_string(),
            RegXMM::XMM2 => "RegXMM::XMM2".to_string(),
            RegXMM::XMM3 => "RegXMM::XMM3".to_string(),
            RegXMM::XMM4 => "RegXMM::XMM4".to_string(),
            RegXMM::XMM5 => "RegXMM::XMM5".to_string(),
            RegXMM::XMM6 => "RegXMM::XMM6".to_string(),
            RegXMM::XMM7 => "RegXMM::XMM7".to_string(),
            RegXMM::XMM8 => "RegXMM::XMM8".to_string(),
            RegXMM::XMM9 => "RegXMM::XMM9".to_string(),
            RegXMM::XMM10 => "RegXMM::XMM10".to_string(),
            RegXMM::XMM11 => "RegXMM::XMM11".to_string(),
            RegXMM::XMM12 => "RegXMM::XMM12".to_string(),
            RegXMM::XMM13 => "RegXMM::XMM13".to_string(),
            RegXMM::XMM14 => "RegXMM::XMM14".to_string(),
            RegXMM::XMM15 => "RegXMM::XMM15".to_string(),
            RegXMM::XMM16 => "RegXMM::XMM16".to_string(),
            RegXMM::XMM17 => "RegXMM::XMM17".to_string(),
            RegXMM::XMM18 => "RegXMM::XMM18".to_string(),
            RegXMM::XMM19 => "RegXMM::XMM19".to_string(),
            RegXMM::XMM20 => "RegXMM::XMM20".to_string(),
            RegXMM::XMM21 => "RegXMM::XMM21".to_string(),
            RegXMM::XMM22 => "RegXMM::XMM22".to_string(),
            RegXMM::XMM23 => "RegXMM::XMM23".to_string(),
            RegXMM::XMM24 => "RegXMM::XMM24".to_string(),
            RegXMM::XMM25 => "RegXMM::XMM25".to_string(),
            RegXMM::XMM26 => "RegXMM::XMM26".to_string(),
            RegXMM::XMM27 => "RegXMM::XMM27".to_string(),
            RegXMM::XMM28 => "RegXMM::XMM28".to_string(),
            RegXMM::XMM29 => "RegXMM::XMM29".to_string(),
            RegXMM::XMM30 => "RegXMM::XMM30".to_string(),
            RegXMM::XMM31 => "RegXMM::XMM31".to_string(),
            RegXMM::XMM32 => "RegXMM::XMM32".to_string(),
        }
    }

    pub fn to_xed(&self) -> xed_reg_enum_t {
        todo!("self:?")
    }
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum RegYMM {
    YMM0,
    YMM1,
    YMM2,
    YMM3,
    YMM4,
    YMM5,
    YMM6,
    YMM7,
    YMM8,
    YMM9,
    YMM10,
    YMM11,
    YMM12,
    YMM13,
    YMM14,
    YMM15,
    YMM16,
    YMM17,
    YMM18,
    YMM19,
    YMM20,
    YMM21,
    YMM22,
    YMM23,
    YMM24,
    YMM25,
    YMM26,
    YMM27,
    YMM28,
    YMM29,
    YMM30,
    YMM31,
    YMM32,
}

impl RegYMM {
    pub fn try_new(reg: xed_reg_enum_t) -> Option<Self> {
        Some(match reg {
            XED_REG_YMM0 => RegYMM::YMM0,
            XED_REG_YMM1 => RegYMM::YMM1,
            XED_REG_YMM2 => RegYMM::YMM2,
            XED_REG_YMM3 => RegYMM::YMM3,
            XED_REG_YMM4 => RegYMM::YMM4,
            XED_REG_YMM5 => RegYMM::YMM5,
            XED_REG_YMM6 => RegYMM::YMM6,
            XED_REG_YMM7 => RegYMM::YMM7,
            XED_REG_YMM8 => RegYMM::YMM8,
            XED_REG_YMM9 => RegYMM::YMM9,
            XED_REG_YMM10 => RegYMM::YMM10,
            XED_REG_YMM11 => RegYMM::YMM11,
            XED_REG_YMM12 => RegYMM::YMM12,
            XED_REG_YMM13 => RegYMM::YMM13,
            XED_REG_YMM14 => RegYMM::YMM14,
            XED_REG_YMM15 => RegYMM::YMM15,
            XED_REG_YMM16 => RegYMM::YMM16,
            XED_REG_YMM17 => RegYMM::YMM17,
            XED_REG_YMM18 => RegYMM::YMM18,
            XED_REG_YMM19 => RegYMM::YMM19,
            XED_REG_YMM20 => RegYMM::YMM20,
            XED_REG_YMM21 => RegYMM::YMM21,
            XED_REG_YMM22 => RegYMM::YMM22,
            XED_REG_YMM23 => RegYMM::YMM23,
            XED_REG_YMM24 => RegYMM::YMM24,
            XED_REG_YMM25 => RegYMM::YMM25,
            XED_REG_YMM26 => RegYMM::YMM26,
            XED_REG_YMM27 => RegYMM::YMM27,
            XED_REG_YMM28 => RegYMM::YMM28,
            XED_REG_YMM29 => RegYMM::YMM29,
            XED_REG_YMM30 => RegYMM::YMM30,
            XED_REG_YMM31 => RegYMM::YMM31,
            _ => return None,
        })
    }

    pub fn to_declaration_string(&self) -> String {
        match self {
            RegYMM::YMM0 => "RegYMM::YMM0".to_string(),
            RegYMM::YMM1 => "RegYMM::YMM1".to_string(),
            RegYMM::YMM2 => "RegYMM::YMM2".to_string(),
            RegYMM::YMM3 => "RegYMM::YMM3".to_string(),
            RegYMM::YMM4 => "RegYMM::YMM4".to_string(),
            RegYMM::YMM5 => "RegYMM::YMM5".to_string(),
            RegYMM::YMM6 => "RegYMM::YMM6".to_string(),
            RegYMM::YMM7 => "RegYMM::YMM7".to_string(),
            RegYMM::YMM8 => "RegYMM::YMM8".to_string(),
            RegYMM::YMM9 => "RegYMM::YMM9".to_string(),
            RegYMM::YMM10 => "RegYMM::YMM10".to_string(),
            RegYMM::YMM11 => "RegYMM::YMM11".to_string(),
            RegYMM::YMM12 => "RegYMM::YMM12".to_string(),
            RegYMM::YMM13 => "RegYMM::YMM13".to_string(),
            RegYMM::YMM14 => "RegYMM::YMM14".to_string(),
            RegYMM::YMM15 => "RegYMM::YMM15".to_string(),
            RegYMM::YMM16 => "RegYMM::YMM16".to_string(),
            RegYMM::YMM17 => "RegYMM::YMM17".to_string(),
            RegYMM::YMM18 => "RegYMM::YMM18".to_string(),
            RegYMM::YMM19 => "RegYMM::YMM19".to_string(),
            RegYMM::YMM20 => "RegYMM::YMM20".to_string(),
            RegYMM::YMM21 => "RegYMM::YMM21".to_string(),
            RegYMM::YMM22 => "RegYMM::YMM22".to_string(),
            RegYMM::YMM23 => "RegYMM::YMM23".to_string(),
            RegYMM::YMM24 => "RegYMM::YMM24".to_string(),
            RegYMM::YMM25 => "RegYMM::YMM25".to_string(),
            RegYMM::YMM26 => "RegYMM::YMM26".to_string(),
            RegYMM::YMM27 => "RegYMM::YMM27".to_string(),
            RegYMM::YMM28 => "RegYMM::YMM28".to_string(),
            RegYMM::YMM29 => "RegYMM::YMM29".to_string(),
            RegYMM::YMM30 => "RegYMM::YMM30".to_string(),
            RegYMM::YMM31 => "RegYMM::YMM31".to_string(),
            RegYMM::YMM32 => "RegYMM::YMM32".to_string(),
        }
    }

    pub fn to_xed(&self) -> xed_reg_enum_t {
        todo!("self:?")
    }
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Ord, PartialOrd,Serialize, Deserialize)]
pub enum RegZMM {
    ZMM0,
    ZMM1,
    ZMM2,
    ZMM3,
    ZMM4,
    ZMM5,
    ZMM6,
    ZMM7,
    ZMM8,
    ZMM9,
    ZMM10,
    ZMM11,
    ZMM12,
    ZMM13,
    ZMM14,
    ZMM15,
    ZMM16,
    ZMM17,
    ZMM18,
    ZMM19,
    ZMM20,
    ZMM21,
    ZMM22,
    ZMM23,
    ZMM24,
    ZMM25,
    ZMM26,
    ZMM27,
    ZMM28,
    ZMM29,
    ZMM30,
    ZMM31,
    ZMM32,
}

impl RegZMM {
    pub fn try_new(reg: xed_reg_enum_t) -> Option<Self> {
        Some(match reg {
            XED_REG_ZMM0 => RegZMM::ZMM0,
            XED_REG_ZMM1 => RegZMM::ZMM1,
            XED_REG_ZMM2 => RegZMM::ZMM2,
            XED_REG_ZMM3 => RegZMM::ZMM3,
            XED_REG_ZMM4 => RegZMM::ZMM4,
            XED_REG_ZMM5 => RegZMM::ZMM5,
            XED_REG_ZMM6 => RegZMM::ZMM6,
            XED_REG_ZMM7 => RegZMM::ZMM7,
            XED_REG_ZMM8 => RegZMM::ZMM8,
            XED_REG_ZMM9 => RegZMM::ZMM9,
            XED_REG_ZMM10 => RegZMM::ZMM10,
            XED_REG_ZMM11 => RegZMM::ZMM11,
            XED_REG_ZMM12 => RegZMM::ZMM12,
            XED_REG_ZMM13 => RegZMM::ZMM13,
            XED_REG_ZMM14 => RegZMM::ZMM14,
            XED_REG_ZMM15 => RegZMM::ZMM15,
            XED_REG_ZMM16 => RegZMM::ZMM16,
            XED_REG_ZMM17 => RegZMM::ZMM17,
            XED_REG_ZMM18 => RegZMM::ZMM18,
            XED_REG_ZMM19 => RegZMM::ZMM19,
            XED_REG_ZMM20 => RegZMM::ZMM20,
            XED_REG_ZMM21 => RegZMM::ZMM21,
            XED_REG_ZMM22 => RegZMM::ZMM22,
            XED_REG_ZMM23 => RegZMM::ZMM23,
            XED_REG_ZMM24 => RegZMM::ZMM24,
            XED_REG_ZMM25 => RegZMM::ZMM25,
            XED_REG_ZMM26 => RegZMM::ZMM26,
            XED_REG_ZMM27 => RegZMM::ZMM27,
            XED_REG_ZMM28 => RegZMM::ZMM28,
            XED_REG_ZMM29 => RegZMM::ZMM29,
            XED_REG_ZMM30 => RegZMM::ZMM30,
            XED_REG_ZMM31 => RegZMM::ZMM31,
            _ => return None,
        })
    }

    pub fn to_declaration_string(&self) -> String {
        match self {
            RegZMM::ZMM0 => "RegZMM::ZMM0".to_string(),
            RegZMM::ZMM1 => "RegZMM::ZMM1".to_string(),
            RegZMM::ZMM2 => "RegZMM::ZMM2".to_string(),
            RegZMM::ZMM3 => "RegZMM::ZMM3".to_string(),
            RegZMM::ZMM4 => "RegZMM::ZMM4".to_string(),
            RegZMM::ZMM5 => "RegZMM::ZMM5".to_string(),
            RegZMM::ZMM6 => "RegZMM::ZMM6".to_string(),
            RegZMM::ZMM7 => "RegZMM::ZMM7".to_string(),
            RegZMM::ZMM8 => "RegZMM::ZMM8".to_string(),
            RegZMM::ZMM9 => "RegZMM::ZMM9".to_string(),
            RegZMM::ZMM10 => "RegZMM::ZMM10".to_string(),
            RegZMM::ZMM11 => "RegZMM::ZMM11".to_string(),
            RegZMM::ZMM12 => "RegZMM::ZMM12".to_string(),
            RegZMM::ZMM13 => "RegZMM::ZMM13".to_string(),
            RegZMM::ZMM14 => "RegZMM::ZMM14".to_string(),
            RegZMM::ZMM15 => "RegZMM::ZMM15".to_string(),
            RegZMM::ZMM16 => "RegZMM::ZMM16".to_string(),
            RegZMM::ZMM17 => "RegZMM::ZMM17".to_string(),
            RegZMM::ZMM18 => "RegZMM::ZMM18".to_string(),
            RegZMM::ZMM19 => "RegZMM::ZMM19".to_string(),
            RegZMM::ZMM20 => "RegZMM::ZMM20".to_string(),
            RegZMM::ZMM21 => "RegZMM::ZMM21".to_string(),
            RegZMM::ZMM22 => "RegZMM::ZMM22".to_string(),
            RegZMM::ZMM23 => "RegZMM::ZMM23".to_string(),
            RegZMM::ZMM24 => "RegZMM::ZMM24".to_string(),
            RegZMM::ZMM25 => "RegZMM::ZMM25".to_string(),
            RegZMM::ZMM26 => "RegZMM::ZMM26".to_string(),
            RegZMM::ZMM27 => "RegZMM::ZMM27".to_string(),
            RegZMM::ZMM28 => "RegZMM::ZMM28".to_string(),
            RegZMM::ZMM29 => "RegZMM::ZMM29".to_string(),
            RegZMM::ZMM30 => "RegZMM::ZMM30".to_string(),
            RegZMM::ZMM31 => "RegZMM::ZMM31".to_string(),
            RegZMM::ZMM32 => "RegZMM::ZMM32".to_string(),
        }
    }

    pub fn to_xed(&self) -> xed_reg_enum_t {
        todo!("self:?")
    }
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum RegTMM {
    TMM0,
    TMM1,
    TMM2,
    TMM3,
    TMM4,
    TMM5,
    TMM6,
    TMM7,
}

impl RegTMM {
    pub fn try_new(reg: xed_reg_enum_t) -> Option<Self> {
        Some(match reg {
            XED_REG_TMM0 => RegTMM::TMM0,
            XED_REG_TMM1 => RegTMM::TMM1,
            XED_REG_TMM2 => RegTMM::TMM2,
            XED_REG_TMM3 => RegTMM::TMM3,
            XED_REG_TMM4 => RegTMM::TMM4,
            XED_REG_TMM5 => RegTMM::TMM5,
            XED_REG_TMM6 => RegTMM::TMM6,
            XED_REG_TMM7 => RegTMM::TMM7,
            _ => return None,
        })
    }

    pub fn to_xed(&self) -> xed_reg_enum_t {
        todo!("self:?")
    }
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq,Ord, PartialOrd, Serialize, Deserialize)]
pub enum RegMask {
    K0,
    K1,
    K2,
    K3,
    K4,
    K5,
    K6,
    K7,
}

impl RegMask {
    pub fn try_new(reg: xed_reg_enum_t) -> Option<Self> {
        Some(match reg {
            XED_REG_K0 => RegMask::K0,
            XED_REG_K1 => RegMask::K1,
            XED_REG_K2 => RegMask::K2,
            XED_REG_K3 => RegMask::K3,
            XED_REG_K4 => RegMask::K4,
            XED_REG_K5 => RegMask::K5,
            XED_REG_K6 => RegMask::K6,
            XED_REG_K7 => RegMask::K7,
            _ => return None,
        })
    }

    pub fn to_declaration_string(&self) -> String {
        match self {
            RegMask::K0 => "RegMask::K0".to_string(),
            RegMask::K1 => "RegMask::K1".to_string(),
            RegMask::K2 => "RegMask::K2".to_string(),
            RegMask::K3 => "RegMask::K3".to_string(),
            RegMask::K4 => "RegMask::K4".to_string(),
            RegMask::K5 => "RegMask::K5".to_string(),
            RegMask::K6 => "RegMask::K6".to_string(),
            RegMask::K7 => "RegMask::K7".to_string(),
        }
    }

    pub fn to_xed(&self) -> xed_reg_enum_t {
        todo!("self:?")
    }
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Ord, PartialOrd,Serialize, Deserialize, Sequence)]
pub enum Reg64WithRIP {
    RAX,
    RBX,
    RCX,
    RDX,
    RSI,
    RDI,
    RBP,
    RSP,
    R8,
    R9,
    R10,
    R11,
    R12,
    R13,
    R14,
    R15,
    RIP,
}

impl Reg64WithRIP {
    pub fn try_new(reg: xed_reg_enum_t) -> Option<Self> {
        Some(match reg {
            XED_REG_RAX => Self::RAX,
            XED_REG_RBX => Self::RBX,
            XED_REG_RCX => Self::RCX,
            XED_REG_RDX => Self::RDX,
            XED_REG_RSI => Self::RSI,
            XED_REG_RDI => Self::RDI,
            XED_REG_RBP => Self::RBP,
            XED_REG_RSP => Self::RSP,
            XED_REG_R8 => Self::R8,
            XED_REG_R9 => Self::R9,
            XED_REG_R10 => Self::R10,
            XED_REG_R11 => Self::R11,
            XED_REG_R12 => Self::R12,
            XED_REG_R13 => Self::R13,
            XED_REG_R14 => Self::R14,
            XED_REG_R15 => Self::R15,
            XED_REG_RIP => Self::RIP,
            _ => return None,
        })
    }

    pub fn try_unwrap_reg64_without_rip(&self) -> Option<Reg64WithoutRIP> {
        match self {
            Reg64WithRIP::RAX => Some(Reg64WithoutRIP::RAX),
            Reg64WithRIP::RBX => Some(Reg64WithoutRIP::RBX),
            Reg64WithRIP::RCX => Some(Reg64WithoutRIP::RCX),
            Reg64WithRIP::RDX => Some(Reg64WithoutRIP::RDX),
            Reg64WithRIP::RSI => Some(Reg64WithoutRIP::RSI),
            Reg64WithRIP::RDI => Some(Reg64WithoutRIP::RDI),
            Reg64WithRIP::RBP => Some(Reg64WithoutRIP::RBP),
            Reg64WithRIP::RSP => Some(Reg64WithoutRIP::RSP),
            Reg64WithRIP::R8 => Some(Reg64WithoutRIP::R8),
            Reg64WithRIP::R9 => Some(Reg64WithoutRIP::R9),
            Reg64WithRIP::R10 => Some(Reg64WithoutRIP::R10),
            Reg64WithRIP::R11 => Some(Reg64WithoutRIP::R11),
            Reg64WithRIP::R12 => Some(Reg64WithoutRIP::R12),
            Reg64WithRIP::R13 => Some(Reg64WithoutRIP::R13),
            Reg64WithRIP::R14 => Some(Reg64WithoutRIP::R14),
            Reg64WithRIP::R15 => Some(Reg64WithoutRIP::R15),
            Reg64WithRIP::RIP => None,
        }
    }

    pub fn to_declaration_string(&self) -> String {
        match self {
            Reg64WithRIP::RAX => "Reg64WithRIP::RAX".to_string(),
            Reg64WithRIP::RBX => "Reg64WithRIP::RBX".to_string(),
            Reg64WithRIP::RCX => "Reg64WithRIP::RCX".to_string(),
            Reg64WithRIP::RDX => "Reg64WithRIP::RDX".to_string(),
            Reg64WithRIP::RSI => "Reg64WithRIP::RSI".to_string(),
            Reg64WithRIP::RDI => "Reg64WithRIP::RDI".to_string(),
            Reg64WithRIP::RBP => "Reg64WithRIP::RBP".to_string(),
            Reg64WithRIP::RSP => "Reg64WithRIP::RSP".to_string(),
            Reg64WithRIP::R8 => "Reg64WithRIP::R8".to_string(),
            Reg64WithRIP::R9 => "Reg64WithRIP::R9".to_string(),
            Reg64WithRIP::R10 => "Reg64WithRIP::R10".to_string(),
            Reg64WithRIP::R11 => "Reg64WithRIP::R11".to_string(),
            Reg64WithRIP::R12 => "Reg64WithRIP::R12".to_string(),
            Reg64WithRIP::R13 => "Reg64WithRIP::R13".to_string(),
            Reg64WithRIP::R14 => "Reg64WithRIP::R14".to_string(),
            Reg64WithRIP::R15 => "Reg64WithRIP::R15".to_string(),
            Reg64WithRIP::RIP => "Reg64WithRIP::RIP".to_string(),
        }
    }

    pub fn to_xed(&self) -> xed_reg_enum_t {
        match self {
            Self::RAX => XED_REG_RAX,
            Self::RBX => XED_REG_RBX,
            Self::RCX => XED_REG_RCX,
            Self::RDX => XED_REG_RDX,
            Self::RSI => XED_REG_RSI,
            Self::RDI => XED_REG_RDI,
            Self::RBP => XED_REG_RBP,
            Self::RSP => XED_REG_RSP,
            Self::R8 => XED_REG_R8,
            Self::R9 => XED_REG_R9,
            Self::R10 => XED_REG_R10,
            Self::R11 => XED_REG_R11,
            Self::R12 => XED_REG_R12,
            Self::R13 => XED_REG_R13,
            Self::R14 => XED_REG_R14,
            Self::R15 => XED_REG_R15,
            Self::RIP => XED_REG_RIP,
        }
    }
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum Reg64WithoutRIP {
    RAX,
    RBX,
    RCX,
    RDX,
    RSI,
    RDI,
    RBP,
    RSP,
    R8,
    R9,
    R10,
    R11,
    R12,
    R13,
    R14,
    R15,
}

impl Reg64WithoutRIP {
    pub fn try_new(reg: xed_reg_enum_t) -> Option<Self> {
        Some(match reg {
            XED_REG_RAX => Self::RAX,
            XED_REG_RBX => Self::RBX,
            XED_REG_RCX => Self::RCX,
            XED_REG_RDX => Self::RDX,
            XED_REG_RSI => Self::RSI,
            XED_REG_RDI => Self::RDI,
            XED_REG_RBP => Self::RBP,
            XED_REG_RSP => Self::RSP,
            XED_REG_R8 => Self::R8,
            XED_REG_R9 => Self::R9,
            XED_REG_R10 => Self::R10,
            XED_REG_R11 => Self::R11,
            XED_REG_R12 => Self::R12,
            XED_REG_R13 => Self::R13,
            XED_REG_R14 => Self::R14,
            XED_REG_R15 => Self::R15,
            _ => return None,
        })
    }

    pub fn to_xed(&self) -> xed_reg_enum_t {
        match self {
            Self::RAX => XED_REG_RAX,
            Self::RBX => XED_REG_RBX,
            Self::RCX => XED_REG_RCX,
            Self::RDX => XED_REG_RDX,
            Self::RSI => XED_REG_RSI,
            Self::RDI => XED_REG_RDI,
            Self::RBP => XED_REG_RBP,
            Self::RSP => XED_REG_RSP,
            Self::R8 => XED_REG_R8,
            Self::R9 => XED_REG_R9,
            Self::R10 => XED_REG_R10,
            Self::R11 => XED_REG_R11,
            Self::R12 => XED_REG_R12,
            Self::R13 => XED_REG_R13,
            Self::R14 => XED_REG_R14,
            Self::R15 => XED_REG_R15,
        }
    }
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Ord, PartialOrd,Serialize, Deserialize, Sequence)]
pub enum Reg32WithRIP {
    EAX,
    EBX,
    ECX,
    EDX,
    ESI,
    EDI,
    EBP,
    ESP,
    R8D,
    R9D,
    R10D,
    R11D,
    R12D,
    R13D,
    R14D,
    R15D,
    EIP,
}

impl Reg32WithRIP {
    pub fn try_new(reg: xed_reg_enum_t) -> Option<Self> {
        Some(match reg {
            XED_REG_EAX => Self::EAX,
            XED_REG_EBX => Self::EBX,
            XED_REG_ECX => Self::ECX,
            XED_REG_EDX => Self::EDX,
            XED_REG_ESI => Self::ESI,
            XED_REG_EDI => Self::EDI,
            XED_REG_EBP => Self::EBP,
            XED_REG_ESP => Self::ESP,
            XED_REG_R8D => Self::R8D,
            XED_REG_R9D => Self::R9D,
            XED_REG_R10D => Self::R10D,
            XED_REG_R11D => Self::R11D,
            XED_REG_R12D => Self::R12D,
            XED_REG_R13D => Self::R13D,
            XED_REG_R14D => Self::R14D,
            XED_REG_R15D => Self::R15D,
            XED_REG_EIP => Self::EIP,
            _ => return None,
        })
    }

    pub fn try_unwrap_reg32_without_rip(&self) -> Option<Reg32WithoutRIP> {
        match self {
            Reg32WithRIP::EAX => Some(Reg32WithoutRIP::EAX),
            Reg32WithRIP::EBX => Some(Reg32WithoutRIP::EBX),
            Reg32WithRIP::ECX => Some(Reg32WithoutRIP::ECX),
            Reg32WithRIP::EDX => Some(Reg32WithoutRIP::EDX),
            Reg32WithRIP::ESI => Some(Reg32WithoutRIP::ESI),
            Reg32WithRIP::EDI => Some(Reg32WithoutRIP::EDI),
            Reg32WithRIP::EBP => Some(Reg32WithoutRIP::EBP),
            Reg32WithRIP::ESP => Some(Reg32WithoutRIP::ESP),
            Reg32WithRIP::R8D => Some(Reg32WithoutRIP::R8D),
            Reg32WithRIP::R9D => Some(Reg32WithoutRIP::R9D),
            Reg32WithRIP::R10D => Some(Reg32WithoutRIP::R10D),
            Reg32WithRIP::R11D => Some(Reg32WithoutRIP::R11D),
            Reg32WithRIP::R12D => Some(Reg32WithoutRIP::R12D),
            Reg32WithRIP::R13D => Some(Reg32WithoutRIP::R13D),
            Reg32WithRIP::R14D => Some(Reg32WithoutRIP::R14D),
            Reg32WithRIP::R15D => Some(Reg32WithoutRIP::R15D),
            Reg32WithRIP::EIP => None,
        }
    }

    pub fn to_declaration_string(&self) -> String {
        match self {
            Reg32WithRIP::EAX => "Reg32WithRIP::EAX".to_string(),
            Reg32WithRIP::EBX => "Reg32WithRIP::EBX".to_string(),
            Reg32WithRIP::ECX => "Reg32WithRIP::ECX".to_string(),
            Reg32WithRIP::EDX => "Reg32WithRIP::EDX".to_string(),
            Reg32WithRIP::ESI => "Reg32WithRIP::ESI".to_string(),
            Reg32WithRIP::EDI => "Reg32WithRIP::EDI".to_string(),
            Reg32WithRIP::EBP => "Reg32WithRIP::EBP".to_string(),
            Reg32WithRIP::ESP => "Reg32WithRIP::ESP".to_string(),
            Reg32WithRIP::R8D => "Reg32WithRIP::R8D".to_string(),
            Reg32WithRIP::R9D => "Reg32WithRIP::R9D".to_string(),
            Reg32WithRIP::R10D => "Reg32WithRIP::R10D".to_string(),
            Reg32WithRIP::R11D => "Reg32WithRIP::R11D".to_string(),
            Reg32WithRIP::R12D => "Reg32WithRIP::R12D".to_string(),
            Reg32WithRIP::R13D => "Reg32WithRIP::R13D".to_string(),
            Reg32WithRIP::R14D => "Reg32WithRIP::R14D".to_string(),
            Reg32WithRIP::R15D => "Reg32WithRIP::R15D".to_string(),
            Reg32WithRIP::EIP => "Reg32WithRIP::EIP".to_string(),
        }
    }

    pub fn to_xed(&self) -> xed_reg_enum_t {
        match self {
            Self::EAX => XED_REG_EAX,
            Self::EBX => XED_REG_EBX,
            Self::ECX => XED_REG_ECX,
            Self::EDX => XED_REG_EDX,
            Self::ESI => XED_REG_ESI,
            Self::EDI => XED_REG_EDI,
            Self::EBP => XED_REG_EBP,
            Self::ESP => XED_REG_ESP,
            Self::R8D => XED_REG_R8D,
            Self::R9D => XED_REG_R9D,
            Self::R10D => XED_REG_R10D,
            Self::R11D => XED_REG_R11D,
            Self::R12D => XED_REG_R12D,
            Self::R13D => XED_REG_R13D,
            Self::R14D => XED_REG_R14D,
            Self::R15D => XED_REG_R15D,
            Self::EIP => XED_REG_EIP,
        }
    }
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize, Sequence)]
pub enum Reg32WithoutRIP {
    EAX,
    EBX,
    ECX,
    EDX,
    ESI,
    EDI,
    EBP,
    ESP,
    R8D,
    R9D,
    R10D,
    R11D,
    R12D,
    R13D,
    R14D,
    R15D,
}

impl Reg32WithoutRIP {
    pub fn try_new(reg: xed_reg_enum_t) -> Option<Self> {
        Some(match reg {
            XED_REG_EAX => Self::EAX,
            XED_REG_EBX => Self::EBX,
            XED_REG_ECX => Self::ECX,
            XED_REG_EDX => Self::EDX,
            XED_REG_ESI => Self::ESI,
            XED_REG_EDI => Self::EDI,
            XED_REG_EBP => Self::EBP,
            XED_REG_ESP => Self::ESP,
            XED_REG_R8D => Self::R8D,
            XED_REG_R9D => Self::R9D,
            XED_REG_R10D => Self::R10D,
            XED_REG_R11D => Self::R11D,
            XED_REG_R12D => Self::R12D,
            XED_REG_R13D => Self::R13D,
            XED_REG_R14D => Self::R14D,
            XED_REG_R15D => Self::R15D,
            _ => return None,
        })
    }

    pub fn to_xed(&self) -> xed_reg_enum_t {
        todo!("self:?")
    }
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq,Ord, PartialOrd, Serialize, Deserialize, Sequence)]
pub enum Reg16WithRIP {
    AX,
    BX,
    CX,
    DX,
    SI,
    DI,
    BP,
    SP,
    R8W,
    R9W,
    R10W,
    R11W,
    R12W,
    R13W,
    R14W,
    R15W,
    IP,
}

impl Reg16WithRIP {
    pub fn try_new(reg: xed_reg_enum_t) -> Option<Self> {
        Some(match reg {
            XED_REG_AX => Self::AX,
            XED_REG_BX => Self::BX,
            XED_REG_CX => Self::CX,
            XED_REG_DX => Self::DX,
            XED_REG_SI => Self::SI,
            XED_REG_DI => Self::DI,
            XED_REG_BP => Self::BP,
            XED_REG_SP => Self::SP,
            XED_REG_R8W => Self::R8W,
            XED_REG_R9W => Self::R9W,
            XED_REG_R10W => Self::R10W,
            XED_REG_R11W => Self::R11W,
            XED_REG_R12W => Self::R12W,
            XED_REG_R13W => Self::R13W,
            XED_REG_R14W => Self::R14W,
            XED_REG_R15W => Self::R15W,
            XED_REG_IP => Self::IP,
            _ => return None,
        })
    }

    pub fn try_unwrap_reg16_without_rip(&self) -> Option<Reg16WithoutRIP> {
        match self {
            Reg16WithRIP::AX => Some(Reg16WithoutRIP::AX),
            Reg16WithRIP::BX => Some(Reg16WithoutRIP::BX),
            Reg16WithRIP::CX => Some(Reg16WithoutRIP::CX),
            Reg16WithRIP::DX => Some(Reg16WithoutRIP::DX),
            Reg16WithRIP::SI => Some(Reg16WithoutRIP::SI),
            Reg16WithRIP::DI => Some(Reg16WithoutRIP::DI),
            Reg16WithRIP::BP => Some(Reg16WithoutRIP::BP),
            Reg16WithRIP::SP => Some(Reg16WithoutRIP::SP),
            Reg16WithRIP::R8W => Some(Reg16WithoutRIP::R8W),
            Reg16WithRIP::R9W => Some(Reg16WithoutRIP::R9W),
            Reg16WithRIP::R10W => Some(Reg16WithoutRIP::R10W),
            Reg16WithRIP::R11W => Some(Reg16WithoutRIP::R11W),
            Reg16WithRIP::R12W => Some(Reg16WithoutRIP::R12W),
            Reg16WithRIP::R13W => Some(Reg16WithoutRIP::R13W),
            Reg16WithRIP::R14W => Some(Reg16WithoutRIP::R14W),
            Reg16WithRIP::R15W => Some(Reg16WithoutRIP::R15W),
            Reg16WithRIP::IP => None,
        }
    }

    pub fn to_declaration_string(&self) -> String {
        match self {
            Reg16WithRIP::AX => "Reg16WithRIP::AX".to_string(),
            Reg16WithRIP::BX => "Reg16WithRIP::BX".to_string(),
            Reg16WithRIP::CX => "Reg16WithRIP::CX".to_string(),
            Reg16WithRIP::DX => "Reg16WithRIP::DX".to_string(),
            Reg16WithRIP::SI => "Reg16WithRIP::SI".to_string(),
            Reg16WithRIP::DI => "Reg16WithRIP::DI".to_string(),
            Reg16WithRIP::BP => "Reg16WithRIP::BP".to_string(),
            Reg16WithRIP::SP => "Reg16WithRIP::SP".to_string(),
            Reg16WithRIP::R8W => "Reg16WithRIP::R8W".to_string(),
            Reg16WithRIP::R9W => "Reg16WithRIP::R9W".to_string(),
            Reg16WithRIP::R10W => "Reg16WithRIP::R10W".to_string(),
            Reg16WithRIP::R11W => "Reg16WithRIP::R11W".to_string(),
            Reg16WithRIP::R12W => "Reg16WithRIP::R12W".to_string(),
            Reg16WithRIP::R13W => "Reg16WithRIP::R13W".to_string(),
            Reg16WithRIP::R14W => "Reg16WithRIP::R14W".to_string(),
            Reg16WithRIP::R15W => "Reg16WithRIP::R15W".to_string(),
            Reg16WithRIP::IP => "Reg16WithRIP::IP".to_string(),
        }
    }

    pub fn to_xed(&self) -> xed_reg_enum_t {
        match self {
            Self::AX => XED_REG_AX,
            Self::BX => XED_REG_BX,
            Self::CX => XED_REG_CX,
            Self::DX => XED_REG_DX,
            Self::SI => XED_REG_SI,
            Self::DI => XED_REG_DI,
            Self::BP => XED_REG_BP,
            Self::SP => XED_REG_SP,
            Self::R8W => XED_REG_R8W,
            Self::R9W => XED_REG_R9W,
            Self::R10W => XED_REG_R10W,
            Self::R11W => XED_REG_R11W,
            Self::R12W => XED_REG_R12W,
            Self::R13W => XED_REG_R13W,
            Self::R14W => XED_REG_R14W,
            Self::R15W => XED_REG_R15W,
            Self::IP => XED_REG_IP,
        }
    }
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize, Sequence)]
pub enum Reg16WithoutRIP {
    AX,
    BX,
    CX,
    DX,
    SI,
    DI,
    BP,
    SP,
    R8W,
    R9W,
    R10W,
    R11W,
    R12W,
    R13W,
    R14W,
    R15W,
}

impl Reg16WithoutRIP {
    pub fn try_new(reg: xed_reg_enum_t) -> Option<Self> {
        Some(match reg {
            XED_REG_AX => Self::AX,
            XED_REG_BX => Self::BX,
            XED_REG_CX => Self::CX,
            XED_REG_DX => Self::DX,
            XED_REG_SI => Self::SI,
            XED_REG_DI => Self::DI,
            XED_REG_BP => Self::BP,
            XED_REG_SP => Self::SP,
            XED_REG_R8W => Self::R8W,
            XED_REG_R9W => Self::R9W,
            XED_REG_R10W => Self::R10W,
            XED_REG_R11W => Self::R11W,
            XED_REG_R12W => Self::R12W,
            XED_REG_R13W => Self::R13W,
            XED_REG_R14W => Self::R14W,
            XED_REG_R15W => Self::R15W,
            _ => return None,
        })
    }
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Ord, PartialOrd,Serialize, Deserialize, Sequence)]
pub enum Reg8 {
    AL,
    AH,
    BL,
    BH,
    CL,
    CH,
    DL,
    DH,
    SIL,
    DIL,
    BPL,
    SPL,
    R8B,
    R9B,
    R10B,
    R11B,
    R12B,
    R13B,
    R14B,
    R15B,
}

impl Reg8 {
    pub fn try_new(reg: xed_reg_enum_t) -> Option<Self> {
        Some(match reg {
            XED_REG_AL => Self::AL,
            XED_REG_AH => Self::AH,
            XED_REG_BL => Self::BL,
            XED_REG_BH => Self::BH,
            XED_REG_CL => Self::CL,
            XED_REG_CH => Self::CH,
            XED_REG_DL => Self::DL,
            XED_REG_DH => Self::DH,
            XED_REG_SIL => Self::SIL,
            XED_REG_DIL => Self::DIL,
            XED_REG_BPL => Self::BPL,
            XED_REG_SPL => Self::SPL,
            XED_REG_R8B => Self::R8B,
            XED_REG_R9B => Self::R9B,
            XED_REG_R10B => Self::R10B,
            XED_REG_R11B => Self::R11B,
            XED_REG_R12B => Self::R12B,
            XED_REG_R13B => Self::R13B,
            XED_REG_R14B => Self::R14B,
            XED_REG_R15B => Self::R15B,
            _ => return None,
        })
    }

    pub fn to_declaration_string(&self) -> String {
        match self {
            Reg8::AL => "Reg8::AL".to_string(),
            Reg8::AH => "Reg8::AH".to_string(),
            Reg8::BL => "Reg8::BL".to_string(),
            Reg8::BH => "Reg8::BH".to_string(),
            Reg8::CL => "Reg8::CL".to_string(),
            Reg8::CH => "Reg8::CH".to_string(),
            Reg8::DL => "Reg8::DL".to_string(),
            Reg8::DH => "Reg8::DH".to_string(),
            Reg8::SIL => "Reg8::SIL".to_string(),
            Reg8::DIL => "Reg8::DIL".to_string(),
            Reg8::BPL => "Reg8::BPL".to_string(),
            Reg8::SPL => "Reg8::SPL".to_string(),
            Reg8::R8B => "Reg8::R8B".to_string(),
            Reg8::R9B => "Reg8::R9B".to_string(),
            Reg8::R10B => "Reg8::R10B".to_string(),
            Reg8::R11B => "Reg8::R11B".to_string(),
            Reg8::R12B => "Reg8::R12B".to_string(),
            Reg8::R13B => "Reg8::R13B".to_string(),
            Reg8::R14B => "Reg8::R14B".to_string(),
            Reg8::R15B => "Reg8::R15B".to_string(),
        }
    }

    pub fn to_xed(&self) -> xed_reg_enum_t {
        match self {
            Self::AL => XED_REG_AL,
            Self::AH => XED_REG_AH,
            Self::BL => XED_REG_BL,
            Self::BH => XED_REG_BH,
            Self::CL => XED_REG_CL,
            Self::CH => XED_REG_CH,
            Self::DL => XED_REG_DL,
            Self::DH => XED_REG_DH,
            Self::SIL => XED_REG_SIL,
            Self::DIL => XED_REG_DIL,
            Self::BPL => XED_REG_BPL,
            Self::SPL => XED_REG_SPL,
            Self::R8B => XED_REG_R8B,
            Self::R9B => XED_REG_R9B,
            Self::R10B => XED_REG_R10B,
            Self::R11B => XED_REG_R11B,
            Self::R12B => XED_REG_R12B,
            Self::R13B => XED_REG_R13B,
            Self::R14B => XED_REG_R14B,
            Self::R15B => XED_REG_R15B,
        }
    }
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Ord, PartialOrd,Serialize, Deserialize)]
pub enum RegFloat {
    ST0,
    ST1,
    ST2,
    ST3,
    ST4,
    ST5,
    ST6,
    ST7,
}

impl RegFloat {
    pub fn try_new(reg: xed_reg_enum_t) -> Option<Self> {
        Some(match reg {
            XED_REG_ST0 => Self::ST0,
            XED_REG_ST1 => Self::ST1,
            XED_REG_ST2 => Self::ST2,
            XED_REG_ST3 => Self::ST3,
            XED_REG_ST4 => Self::ST4,
            XED_REG_ST5 => Self::ST5,
            XED_REG_ST6 => Self::ST6,
            XED_REG_ST7 => Self::ST7,
            _ => return None,
        })
    }

    pub fn to_xed(&self) -> xed_reg_enum_t {
        todo!("self:?")
    }
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq,Ord, PartialOrd, Serialize, Deserialize)]
pub enum RegFloatControl {
    X87CONTROL,
    X87STATUS,
    X87TAG,
    X87POP,
    X87PUSH,
    X87POP2,
}

impl RegFloatControl {
    pub fn try_new(reg: xed_reg_enum_t) -> Option<Self> {
        Some(match reg {
            XED_REG_X87CONTROL => Self::X87CONTROL,
            XED_REG_X87STATUS => Self::X87STATUS,
            XED_REG_X87TAG => Self::X87TAG,
            XED_REG_X87POP => Self::X87POP,
            XED_REG_X87PUSH => Self::X87PUSH,
            XED_REG_X87POP2 => Self::X87POP2,
            _ => return None,
        })
    }

    pub fn to_declaration_string(&self) -> String {
        match self {
            RegFloatControl::X87CONTROL => "RegFloatControl::X87CONTROL".to_string(),
            RegFloatControl::X87STATUS => "RegFloatControl::X87STATUS".to_string(),
            RegFloatControl::X87TAG => "RegFloatControl::X87TAG".to_string(),
            RegFloatControl::X87POP => "RegFloatControl::X87POP".to_string(),
            RegFloatControl::X87PUSH => "RegFloatControl::X87PUSH".to_string(),
            RegFloatControl::X87POP2 => "RegFloatControl::X87POP2".to_string(),
        }
    }

    pub fn to_xed(&self) -> xed_reg_enum_t {
        todo!("self:?")
    }
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum RegBnd {
    BND0,
    BND1,
    BND2,
    BND3,
}

impl RegBnd {
    pub fn try_new(reg: xed_reg_enum_t) -> Option<Self> {
        Some(match reg {
            XED_REG_BND0 => Self::BND0,
            XED_REG_BND1 => Self::BND1,
            XED_REG_BND2 => Self::BND2,
            XED_REG_BND3 => Self::BND3,
            _ => return None,
        })
    }

    pub fn to_xed(&self) -> xed_reg_enum_t {
        todo!("self:?")
    }
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum RegBndConfig {
    BNDCFGU,
    BNDSTATUS,
}

impl RegBndConfig {
    pub fn try_new(reg: xed_reg_enum_t) -> Option<Self> {
        Some(match reg {
            XED_REG_BNDCFGU => Self::BNDCFGU,
            XED_REG_BNDSTATUS => Self::BNDSTATUS,
            _ => return None,
        })
    }
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Ord, PartialOrd,Serialize, Deserialize)]
pub enum RegSpecial {
    GDTR,
    LDTR,
    IDTR,
    TR,
    TSC,
    TSCAUX,
    MSRS,
    UIF,
    SSP,
}

impl RegSpecial {
    pub fn try_new(reg: xed_reg_enum_t) -> Option<Self> {
        Some(match reg {
            XED_REG_GDTR => Self::GDTR,
            XED_REG_LDTR => Self::LDTR,
            XED_REG_IDTR => Self::IDTR,
            XED_REG_TR => Self::TR,
            XED_REG_TSC => Self::TSC,
            XED_REG_TSCAUX => Self::TSCAUX,
            XED_REG_MSRS => Self::MSRS,
            XED_REG_UIF => Self::UIF,
            XED_REG_SSP => Self::SSP,
            _ => return None,
        })
    }

    pub fn to_declaration_string(&self) -> String {
        match self {
            RegSpecial::GDTR => "RegSpecial::GDTR".to_string(),
            RegSpecial::LDTR => "RegSpecial::LDTR".to_string(),
            RegSpecial::IDTR => "RegSpecial::IDTR".to_string(),
            RegSpecial::TR => "RegSpecial::TR".to_string(),
            RegSpecial::TSC => "RegSpecial::TSC".to_string(),
            RegSpecial::TSCAUX => "RegSpecial::TSCAUX".to_string(),
            RegSpecial::MSRS => "RegSpecial::MSRS".to_string(),
            RegSpecial::UIF => "RegSpecial::UIF".to_string(),
            RegSpecial::SSP => "RegSpecial::SSP".to_string(),
        }
    }

    pub fn to_xed(&self) -> xed_reg_enum_t {
        todo!("self:?")
    }
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Ord, PartialOrd,Serialize, Deserialize)]
pub enum RegControl {
    //several of these exist in encoding but may not actually exist
    CR0,
    CR1,
    CR2,
    CR3,
    CR4,
    CR5,
    CR6,
    CR7,
    CR8,
    CR9,
    CR10,
    CR11,
    CR12,
    CR13,
    CR14,
    CR15,
}

impl RegControl {
    pub fn try_new(reg: xed_reg_enum_t) -> Option<Self> {
        Some(match reg {
            XED_REG_CR0 => Self::CR0,
            XED_REG_CR1 => Self::CR1,
            XED_REG_CR2 => Self::CR2,
            XED_REG_CR3 => Self::CR3,
            XED_REG_CR4 => Self::CR4,
            XED_REG_CR5 => Self::CR5,
            XED_REG_CR6 => Self::CR6,
            XED_REG_CR7 => Self::CR7,
            XED_REG_CR8 => Self::CR8,
            XED_REG_CR9 => Self::CR9,
            XED_REG_CR10 => Self::CR10,
            XED_REG_CR11 => Self::CR11,
            XED_REG_CR12 => Self::CR12,
            XED_REG_CR13 => Self::CR13,
            XED_REG_CR14 => Self::CR14,
            XED_REG_CR15 => Self::CR15,
            _ => return None,
        })
    }

    pub fn to_declaration_string(&self) -> String {
        match self {
            RegControl::CR0 => "RegControl::CR0".to_string(),
            RegControl::CR1 => "RegControl::CR1".to_string(),
            RegControl::CR2 => "RegControl::CR2".to_string(),
            RegControl::CR3 => "RegControl::CR3".to_string(),
            RegControl::CR4 => "RegControl::CR4".to_string(),
            RegControl::CR5 => "RegControl::CR5".to_string(),
            RegControl::CR6 => "RegControl::CR6".to_string(),
            RegControl::CR7 => "RegControl::CR7".to_string(),
            RegControl::CR8 => "RegControl::CR8".to_string(),
            RegControl::CR9 => "RegControl::CR9".to_string(),
            RegControl::CR10 => "RegControl::CR10".to_string(),
            RegControl::CR11 => "RegControl::CR11".to_string(),
            RegControl::CR12 => "RegControl::CR12".to_string(),
            RegControl::CR13 => "RegControl::CR13".to_string(),
            RegControl::CR14 => "RegControl::CR14".to_string(),
            RegControl::CR15 => "RegControl::CR15".to_string(),
        }
    }

    pub fn to_xed(&self) -> xed_reg_enum_t {
        todo!("self:?")
    }
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Ord, PartialOrd,Serialize, Deserialize)]
pub enum RegControlExtra {
    EFER,
    XCR0,
    XSS,
    MXCSR,
}

impl RegControlExtra {
    pub fn try_new(reg: xed_reg_enum_t) -> Option<Self> {
        Some(match reg {
            XED_REG_XCR0 => Self::XCR0,
            XED_REG_MXCSR => Self::MXCSR,
            _ => return None,
        })
    }

    pub fn to_declaration_string(&self) -> String {
        match self {
            RegControlExtra::EFER => "RegControlExtra::EFER".to_string(),
            RegControlExtra::XCR0 => "RegControlExtra::XCR0".to_string(),
            RegControlExtra::XSS => "RegControlExtra::XSS".to_string(),
            RegControlExtra::MXCSR => "RegControlExtra::MXCSR".to_string(),
        }
    }
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Ord, PartialOrd,Serialize, Deserialize)]
#[derive(Sequence)]
pub enum RegSegment {
    CS,
    DS,
    SS,
    ES,
    FS,
    GS,
}

impl RegSegment {
    pub fn try_new(reg: xed_reg_enum_t) -> Option<Self> {
        Some(match reg {
            XED_REG_CS => Self::CS,
            XED_REG_DS => Self::DS,
            XED_REG_SS => Self::SS,
            XED_REG_ES => Self::ES,
            XED_REG_FS => Self::FS,
            XED_REG_GS => Self::GS,
            _ => return None,
        })
    }

    pub fn to_declaration_string(&self) -> String {
        match self {
            RegSegment::CS => "RegSegment::CS".to_string(),
            RegSegment::DS => "RegSegment::DS".to_string(),
            RegSegment::SS => "RegSegment::SS".to_string(),
            RegSegment::ES => "RegSegment::ES".to_string(),
            RegSegment::FS => "RegSegment::FS".to_string(),
            RegSegment::GS => "RegSegment::GS".to_string(),
        }
    }

    pub fn to_xed(&self) -> xed_reg_enum_t {
        match self {
            Self::CS => XED_REG_CS,
            Self::DS => XED_REG_DS,
            Self::SS => XED_REG_SS,
            Self::ES => XED_REG_ES,
            Self::FS => XED_REG_FS,
            Self::GS => XED_REG_GS,
        }
    }
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Ord, PartialOrd,Serialize, Deserialize)]
pub enum RegSegmentBase {
    FSBase,
    GSBase,
}

impl RegSegmentBase {
    pub fn try_new(reg: xed_reg_enum_t) -> Option<Self> {
        Some(match reg {
            XED_REG_FSBASE => Self::FSBase,
            XED_REG_GSBASE => Self::GSBase,
            _ => return None,
        })
    }

    pub fn to_declaration_string(&self) -> String {
        match self {
            RegSegmentBase::FSBase => "RegSegmentBase::FSBase".to_string(),
            RegSegmentBase::GSBase => "RegSegmentBase::GSBase".to_string(),
        }
    }
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum RegDebug {
    DR0,
    DR1,
    DR2,
    DR3,
    // 4 and 5 exist in encoding but may not actually exist
    DR4,
    DR5,
    DR6,
    DR7,
}

impl RegDebug {
    pub fn try_new(reg: xed_reg_enum_t) -> Option<Self> {
        Some(match reg {
            XED_REG_DR0 => Self::DR0,
            XED_REG_DR1 => Self::DR1,
            XED_REG_DR2 => Self::DR2,
            XED_REG_DR3 => Self::DR3,
            XED_REG_DR4 => Self::DR4,
            XED_REG_DR5 => Self::DR5,
            XED_REG_DR6 => Self::DR6,
            XED_REG_DR7 => Self::DR7,
            _ => return None,
        })
    }

    pub fn to_xed(&self) -> xed_reg_enum_t {
        todo!("self:?")
    }
}



#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum OperandWidth{
    _8,
    _16,
    _32,
    _64,
    _80,
    _94,
    _108,
    _112,
    _128,
    _224,
    _256,
    _384,
    _512,
    _4096,
    _Tile
}

impl OperandWidth{
    pub fn width_for_mem(width: XedOperandWidth) -> HashSet<Self>{
        [match width {
            XedOperandWidth::MEM32REAL => Self::_32,
            XedOperandWidth::M64INT => Self::_64,
            XedOperandWidth::MEM32INT => Self::_32,
            XedOperandWidth::MEM16INT => Self::_16,
            XedOperandWidth::M64REAL => Self::_64,
            XedOperandWidth::MEM80REAL => Self::_80,
            XedOperandWidth::MEM80DEC => Self::_80,
            XedOperandWidth::MEM14 => Self::_112,
            XedOperandWidth::MEM16 => Self::_16,
            XedOperandWidth::MEM28 => Self::_224,
            XedOperandWidth::MEM94 => Self::_94,
            XedOperandWidth::MEM108 => Self::_108,
            XedOperandWidth::B => Self::_8,
            XedOperandWidth::D => Self::_32,
            XedOperandWidth::A16 => Self::_16,
            XedOperandWidth::A32 => Self::_32,
            XedOperandWidth::Q => Self::_64,
            XedOperandWidth::DQ => Self::_128,
            XedOperandWidth::QQ => Self::_256,
            XedOperandWidth::V => return [Self::_8, Self::_16, Self::_32, Self::_64].into_iter().collect(),
            XedOperandWidth::P2 => return [Self::_16, Self::_32, Self::_64].into_iter().collect(),
            XedOperandWidth::P => return [Self::_16, Self::_32, Self::_64].into_iter().collect(),
            XedOperandWidth::PS => return [Self::_128, Self::_256, Self::_512].into_iter().collect(),
            XedOperandWidth::PD => return [Self::_128, Self::_256, Self::_512].into_iter().collect(),
            XedOperandWidth::XUD => return [Self::_128, Self::_256, Self::_512].into_iter().collect(),
            XedOperandWidth::XUQ => return [Self::_128, Self::_256, Self::_512].into_iter().collect(),
            XedOperandWidth::Z => return [Self::_16, Self::_32, Self::_64].into_iter().collect(),
            XedOperandWidth::Y => return [Self::_32, Self::_64].into_iter().collect(),
            XedOperandWidth::W => Self::_16,
            XedOperandWidth::MFPXENV => Self::_512,
            XedOperandWidth::MPREFETCH => Self::_8,
            XedOperandWidth::S64 => return [Self::_16, Self::_64].into_iter().collect(),
            XedOperandWidth::S => return [Self::_16, Self::_32].into_iter().collect(),
            XedOperandWidth::SS => Self::_32,
            XedOperandWidth::SD => Self::_64,
            XedOperandWidth::MXSAVE => Self::_4096,
            XedOperandWidth::BND32 => Self::_32,
            XedOperandWidth::BND64 => Self::_64,
            XedOperandWidth::ZD => Self::_512,
            XedOperandWidth::M384 => Self::_384,
            XedOperandWidth::WRD => Self::_16,
            XedOperandWidth::PTR => Self::_Tile,
            XedOperandWidth::VV => Self::_128,
            width => todo!("{width:?}")
        }].into_iter().collect()
    }

    pub fn width_for_imm(width: XedOperandWidth) -> HashSet<Self>{
        [match width {
            XedOperandWidth::B => Self::_8,
            XedOperandWidth::Z => return [Self::_8, Self::_16, Self::_32].into_iter().collect(),
            XedOperandWidth::V => return [Self::_8, Self::_16, Self::_32, Self::_64].into_iter().collect(),
            XedOperandWidth::W => Self::_16,
            XedOperandWidth::D => Self::_32,
            width => todo!("{width:?}")
        }].into_iter().collect()
    }

    pub fn to_xed_width_bits(&self) -> usize{
        match self {
            OperandWidth::_8 => 8,
            OperandWidth::_16 => 16,
            OperandWidth::_32 => 32,
            OperandWidth::_64 => 64,
            OperandWidth::_80 => 80,
            OperandWidth::_94 => 94,
            OperandWidth::_108 => 108,
            OperandWidth::_112 => 112,
            OperandWidth::_128 => 128,
            OperandWidth::_224 => 224,
            OperandWidth::_256 => 256,
            OperandWidth::_384 => 384,
            OperandWidth::_512 => 512,
            OperandWidth::_4096 => 4096,
            OperandWidth::_Tile => 8192
        }
    }

    pub fn to_string(&self) -> String{
        match self {
            OperandWidth::_8 => "_8".to_string(),
            OperandWidth::_16 => "_16".to_string(),
            OperandWidth::_32 => "_32".to_string(),
            OperandWidth::_64 => "_64".to_string(),
            OperandWidth::_80 => "_80".to_string(),
            OperandWidth::_94 => "_94".to_string(),
            OperandWidth::_108 => "_108".to_string(),
            OperandWidth::_112 => "_112".to_string(),
            OperandWidth::_128 => "_128".to_string(),
            OperandWidth::_224 => "_224".to_string(),
            OperandWidth::_256 => "_256".to_string(),
            OperandWidth::_384 => "_384".to_string(),
            OperandWidth::_512 => "_512".to_string(),
            OperandWidth::_4096 => "_4096".to_string(),
            OperandWidth::_Tile => "_Tile".to_string(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
pub enum RegisterType {
    AllMmx,
    AllXmm16,
    AllXmm32,
    SomeXmm(BTreeSet<RegXMM>),
    SingleXmm(RegXMM),
    AllYmm16,
    AllYmm32,
    AllZmm32,
    SomeZmm(BTreeSet<RegZMM>),
    AllTmm,
    AllMask,
    SomeMask(BTreeSet<RegMask>),
    AllGP64WithoutRIP,
    AllGP64WithRIP,
    SingleGP64(Reg64WithRIP),
    AllGP32WithoutRIP,
    AllGP32WithRIP,
    SomeGP32(BTreeSet<Reg32WithRIP>),
    SingleGP32(Reg32WithRIP),
    AllGP16WithoutRIP,
    AllGP16WithRIP,
    SomeGP16(BTreeSet<Reg16WithRIP>),
    SingleGP16(Reg16WithRIP),
    AllGP8,
    SomeGP8(BTreeSet<Reg8>),
    SingleGP8(Reg8),
    AllFloat,
    SingleFloat(RegFloat),
    AllBnd,
    AllSegment,
    SingleSegment(RegSegment),
    SomeSegment(BTreeSet<RegSegment>),
    AllDebug,
    SomeControl(BTreeSet<RegControl>),
    AllControl,
    SingleControl(RegControl),
    SomeControlExtra(BTreeSet<RegControlExtra>),
    SingleSegmentBase(RegSegmentBase),
    SingleSpecial(RegSpecial),
    SingleFloatControl(RegFloatControl),
    Multiple(BTreeSet<RegisterType>),
}

impl RegisterType {
    pub fn field_as_width(&self, width: OperandWidth) -> Self {
        match self {
            RegisterType::Multiple(multiple) => {
                let mut res = None;
                for elem in multiple {
                    if elem.widths().contains(&width){
                        assert!(res.is_none());
                        res = Some(elem);
                    }
                }
                return res.unwrap().clone();
            }
            other => other.clone()
        }
    }

    pub fn widths(&self) -> HashSet<OperandWidth>{
        [match self {
            RegisterType::AllMmx => {
                OperandWidth::_128
            }
            RegisterType::AllXmm16 => {
                OperandWidth::_128
            }
            RegisterType::AllXmm32 => {
                OperandWidth::_128
            }
            RegisterType::SomeXmm(_) => {
                OperandWidth::_128
            }
            RegisterType::SingleXmm(_) => {
                OperandWidth::_128
            }
            RegisterType::AllYmm16 => {
                OperandWidth::_256
            }
            RegisterType::AllYmm32 => {
                OperandWidth::_256
            }
            RegisterType::AllZmm32 => {
                OperandWidth::_512
            }
            RegisterType::SomeZmm(_) => {
                OperandWidth::_512
            }
            RegisterType::AllTmm => {
                todo!()
            }
            RegisterType::AllMask => {
                OperandWidth::_64
            }
            RegisterType::SomeMask(_) => {
                todo!()
            }
            RegisterType::AllGP64WithoutRIP => {
                OperandWidth::_64
            }
            RegisterType::AllGP64WithRIP => {
                OperandWidth::_64
            }
            RegisterType::SingleGP64(_) => {
                OperandWidth::_64
            }
            RegisterType::AllGP32WithoutRIP => {
                OperandWidth::_32
            }
            RegisterType::AllGP32WithRIP => {
                OperandWidth::_32
            }
            RegisterType::SomeGP32(_) => {
                OperandWidth::_32
            }
            RegisterType::SingleGP32(_) => {
                OperandWidth::_32
            }
            RegisterType::AllGP16WithoutRIP => {
                OperandWidth::_16
            }
            RegisterType::AllGP16WithRIP => {
                OperandWidth::_16
            }
            RegisterType::SomeGP16(_) => {
                OperandWidth::_16
            }
            RegisterType::SingleGP16(_) => {
                OperandWidth::_16
            }
            RegisterType::AllGP8 => {
                OperandWidth::_8
            }
            RegisterType::SomeGP8(_) => {
                OperandWidth::_8
            }
            RegisterType::SingleGP8(_) => {
                OperandWidth::_8
            }
            RegisterType::AllFloat => {
                OperandWidth::_80
            }
            RegisterType::SingleFloat(_) => {
                todo!()
            }
            RegisterType::AllBnd => {
                todo!()
            }
            RegisterType::AllSegment => {
                todo!()
            }
            RegisterType::SingleSegment(_) => {
                todo!()
            }
            RegisterType::SomeSegment(_) => {
                todo!()
            }
            RegisterType::AllDebug => {
                todo!()
            }
            RegisterType::SomeControl(_) => {
                todo!()
            }
            RegisterType::AllControl => {
                todo!()
            }
            RegisterType::SingleControl(_) => {
                todo!()
            }
            RegisterType::SomeControlExtra(_) => {
                todo!()
            }
            RegisterType::SingleSegmentBase(_) => {
                todo!()
            }
            RegisterType::SingleSpecial(_) => {
                todo!()
            }
            RegisterType::SingleFloatControl(_) => {
                todo!()
            }
            RegisterType::Multiple(multiple) => {
                return multiple.iter().flat_map(|reg|reg.widths()).collect()
            }
        }].into_iter().collect()
    }

    pub fn is_of_type(&self, reg: &Register) -> bool {
        match self {
            RegisterType::AllMmx => match reg {
                Register::Mmx(_) => true,
                _ => false,
            },
            RegisterType::AllXmm16 => {
                todo!()
            }
            RegisterType::AllXmm32 => match reg {
                Register::Xmm(_) => true,
                _ => false,
            },
            RegisterType::SomeXmm(_) => {
                todo!()
            }
            RegisterType::SingleXmm(_) => {
                todo!()
            }
            RegisterType::AllYmm16 => {
                todo!()
            }
            RegisterType::AllYmm32 => match reg {
                Register::Ymm(_) => true,
                _ => false,
            },
            RegisterType::AllZmm32 => match reg {
                Register::Zmm(_) => true,
                _ => false,
            },
            RegisterType::SomeZmm(_) => {
                todo!()
            }
            RegisterType::AllTmm => match reg {
                Register::Tmm(_) => true,
                _ => false,
            },
            RegisterType::AllMask => match reg {
                Register::Mask(_) => true,
                _ => false,
            },
            RegisterType::SomeMask(_) => {
                todo!()
            }
            RegisterType::AllGP64WithoutRIP => match reg {
                Register::GP64(reg) => {
                    if let Reg64WithRIP::RIP = reg {
                        false
                    } else {
                        true
                    }
                }
                _ => false,
            },
            RegisterType::AllGP64WithRIP => {
                todo!()
            }
            RegisterType::SingleGP64(single_gp64) => match reg {
                Register::GP64(gp64) => gp64 == single_gp64,
                _ => false,
            },
            RegisterType::AllGP32WithoutRIP => match reg {
                Register::GP32(reg) => {
                    if let Reg32WithRIP::EIP = reg {
                        false
                    } else {
                        true
                    }
                }
                _ => false,
            },
            RegisterType::AllGP32WithRIP => match reg {
                Register::GP32(_) => true,
                _ => false,
            },
            RegisterType::SomeGP32(_) => {
                todo!()
            }
            RegisterType::SingleGP32(single_gp32) => match reg {
                Register::GP32(gp32) => gp32 == single_gp32,
                _ => false,
            },
            RegisterType::AllGP16WithRIP => {
                todo!()
            }
            RegisterType::AllGP16WithoutRIP => match reg {
                Register::GP16(gp16) => {
                    if let Reg16WithRIP::IP = gp16 {
                        false
                    } else {
                        true
                    }
                }
                _ => false,
            },
            RegisterType::SomeGP16(_) => {
                todo!()
            }
            RegisterType::SingleGP16(single_reg16) => match reg {
                Register::GP16(gp16) => gp16 == single_reg16,
                _ => false,
            },
            RegisterType::AllGP8 => match reg {
                Register::GP8(_) => true,
                _ => false,
            },
            RegisterType::SomeGP8(regs) => match reg {
                Register::GP8(gp8) => regs.contains(&gp8),
                _ => false,
            },
            RegisterType::SingleGP8(single_gp8) => match reg {
                Register::GP8(gp8) => gp8 == single_gp8,
                _ => false,
            },
            RegisterType::AllFloat => match reg {
                Register::Float(_) => true,
                _ => false,
            },
            RegisterType::SingleFloat(_) => {
                todo!()
            }
            RegisterType::AllBnd => match reg {
                Register::Bnd(_) => true,
                _ => false,
            },
            RegisterType::AllSegment => match reg {
                Register::Segment(_) => true,
                _ => false,
            },
            RegisterType::SingleSegment(_seg) => match reg {
                Register::Segment(_) => todo!(),
                _ => false,
            },
            RegisterType::SomeSegment(_some_segs) => match reg {
                Register::Segment(_) => todo!(),
                _ => false,
            },
            RegisterType::AllDebug => match reg {
                Register::Debug(_) => true,
                _ => false,
            },
            RegisterType::SomeControl(_) => {
                todo!()
            }
            RegisterType::SingleControl(_) => {
                todo!()
            }
            RegisterType::SomeControlExtra(_) => {
                todo!()
            }
            RegisterType::SingleSegmentBase(_) => {
                todo!()
            }
            RegisterType::SingleSpecial(_) => {
                todo!()
            }
            RegisterType::SingleFloatControl(_) => {
                todo!()
            }
            RegisterType::Multiple(_) => {
                todo!()
            }
            RegisterType::AllControl => {
                todo!()
            }
        }
    }
    pub fn type_to_rust_type(&self) -> Ident {
        match self {
            RegisterType::AllMmx => format_ident!("RegMMX"),
            RegisterType::AllXmm16 => format_ident!("RegXMM"),
            RegisterType::AllXmm32 => format_ident!("RegXMM"),
            RegisterType::SomeXmm(_) => format_ident!("RegXMM"),
            RegisterType::SingleXmm(_) => format_ident!("RegXMM"),
            RegisterType::AllYmm16 => format_ident!("RegYMM"),
            RegisterType::AllYmm32 => format_ident!("RegYMM"),
            RegisterType::AllZmm32 => format_ident!("RegZMM"),
            RegisterType::SomeZmm(_) => format_ident!("RegZMM"),
            RegisterType::AllTmm => format_ident!("RegTMM"),
            RegisterType::AllMask => format_ident!("RegMask"),
            RegisterType::SomeMask(_) => format_ident!("RegZMM"),
            RegisterType::AllGP64WithoutRIP => format_ident!("Reg64WithoutRIP"),
            RegisterType::AllGP64WithRIP => format_ident!("Reg64WithRIP"),
            RegisterType::SingleGP64(_) => format_ident!("Reg64WithRIP"),
            RegisterType::AllGP32WithoutRIP => format_ident!("Reg32WithoutRIP"),
            RegisterType::AllGP32WithRIP => format_ident!("Reg32WithRIP"),
            RegisterType::SomeGP32(_) => format_ident!("Reg32WithRIP"),
            RegisterType::SingleGP32(_) => format_ident!("Reg32WithRIP"),
            RegisterType::AllGP16WithoutRIP => format_ident!("Reg16WithoutRIP"),
            RegisterType::AllGP16WithRIP => format_ident!("Reg16WithRIP"),
            RegisterType::SomeGP16(_) => format_ident!("Reg16WithRIP"),
            RegisterType::SingleGP16(_) => format_ident!("Reg16WithRIP"),
            RegisterType::AllGP8 => format_ident!("Reg8"),
            RegisterType::SomeGP8(_) => format_ident!("Reg8"),
            RegisterType::SingleGP8(_) => format_ident!("Reg8"),
            RegisterType::AllFloat => format_ident!("RegFloat"),
            RegisterType::SingleFloat(_) => format_ident!("RegFloat"),
            RegisterType::AllBnd => format_ident!("RegBnd"),
            RegisterType::AllSegment => format_ident!("RegSegment"),
            RegisterType::SingleSegment(_) => format_ident!("RegSegment"),
            RegisterType::SomeSegment(_) => format_ident!("RegSegment"),
            RegisterType::AllDebug => format_ident!("RegDebug"),
            RegisterType::SomeControl(_) => format_ident!("RegControl"),
            RegisterType::AllControl => format_ident!("RegControl"),
            RegisterType::SingleControl(_) => format_ident!("RegControl"),
            RegisterType::SomeControlExtra(_) => todo!("SomeControlExtra"),
            RegisterType::SingleSegmentBase(_) => todo!("SingleSegmentBase"),
            RegisterType::SingleSpecial(_) => format_ident!("RegSpecial"),
            RegisterType::SingleFloatControl(_) => format_ident!("RegFloatControl"),
            RegisterType::Multiple(multiple) => {
                if multiple == &[RegisterType::AllGP64WithRIP, RegisterType::AllGP32WithRIP].into_iter().collect()
                {
                    format_ident!("GeneralReg3264")
                } else if multiple == &[
                    RegisterType::AllGP64WithRIP,
                    RegisterType::AllGP32WithRIP,
                    RegisterType::AllGP16WithRIP,
                    RegisterType::AllGP8,
                ].into_iter().collect()
                {
                    format_ident!("GeneralReg")
                } else if multiple
                    == &[
                    RegisterType::AllGP64WithRIP,
                    RegisterType::AllGP32WithRIP,
                    RegisterType::AllGP16WithRIP,
                ].into_iter().collect()
                {
                    format_ident!("GeneralReg163264")
                } else {
                    dbg!(multiple);
                    panic!();
                }
            }
        }
    }
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum Register {
    Mmx(RegMMX),
    Xmm(RegXMM),
    Ymm(RegYMM),
    Zmm(RegZMM),
    Tmm(RegTMM),
    Mask(RegMask),
    GP64(Reg64WithRIP),
    GP32(Reg32WithRIP),
    GP16(Reg16WithRIP),
    GP8(Reg8),
    Float(RegFloat),
    FloatControl(RegFloatControl),
    Bnd(RegBnd),
    BndConfig(RegBndConfig),
    Special(RegSpecial),
    Debug(RegDebug),
    Control(RegControl),
    ControlExtra(RegControlExtra),
    Segment(()),
}

impl Register {
    pub fn try_new(reg: xed_reg_enum_t) -> Option<Self> {
        let class: xed_reg_class_enum_t = unsafe { xed_reg_class(reg) };
        Some(match class {
            XED_REG_CLASS_BNDCFG => Register::BndConfig(RegBndConfig::try_new(reg)?),
            XED_REG_CLASS_BNDSTAT => Register::BndConfig(RegBndConfig::try_new(reg)?),
            XED_REG_CLASS_BOUND => Register::Bnd(RegBnd::try_new(reg)?),
            XED_REG_CLASS_CR => Register::Control(RegControl::try_new(reg)?),
            XED_REG_CLASS_DR => Register::Debug(RegDebug::try_new(reg)?),
            XED_REG_CLASS_FLAGS => todo!(),
            XED_REG_CLASS_GPR => Register::GP64(Reg64WithRIP::try_new(reg)?),
            XED_REG_CLASS_GPR16 => Register::GP16(Reg16WithRIP::try_new(reg)?),
            XED_REG_CLASS_GPR32 => Register::GP32(Reg32WithRIP::try_new(reg)?),
            XED_REG_CLASS_GPR64 => Register::GP64(Reg64WithRIP::try_new(reg)?),
            XED_REG_CLASS_GPR8 => Register::GP8(Reg8::try_new(reg)?),
            XED_REG_CLASS_IP => todo!(),
            XED_REG_CLASS_MASK => Register::Mask(RegMask::try_new(reg)?),
            XED_REG_CLASS_MMX => Register::Mmx(RegMMX::try_new(reg)?),
            XED_REG_CLASS_MSR => Register::Special(RegSpecial::try_new(reg)?),
            XED_REG_CLASS_MXCSR => Register::Special(RegSpecial::try_new(reg)?),
            XED_REG_CLASS_PSEUDO => todo!(),
            XED_REG_CLASS_PSEUDOX87 => todo!(),
            XED_REG_CLASS_SR => todo!(),
            XED_REG_CLASS_TMP => todo!(),
            XED_REG_CLASS_TREG => Register::Tmm(RegTMM::try_new(reg)?),
            XED_REG_CLASS_UIF => todo!(),
            XED_REG_CLASS_X87 => Register::Float(RegFloat::try_new(reg)?),
            XED_REG_CLASS_XCR => Register::ControlExtra(RegControlExtra::try_new(reg)?),
            XED_REG_CLASS_XMM => Register::Xmm(RegXMM::try_new(reg)?),
            XED_REG_CLASS_YMM => Register::Ymm(RegYMM::try_new(reg)?),
            XED_REG_CLASS_ZMM => Register::Zmm(RegZMM::try_new(reg)?),
            _ => return None,
        })
    }

    pub fn is_of_type(&self, _register_type: RegisterType) {
        todo!()
    }

    pub fn unwrap_regmmx(&self) -> RegMMX {
        todo!()
    }

    pub fn unwrap_regxmm(&self) -> RegXMM {
        todo!()
    }

    pub fn unwrap_regymm(&self) -> RegYMM {
        todo!()
    }

    pub fn unwrap_regzmm(&self) -> RegZMM {
        todo!()
    }

    pub fn unwrap_regtmm(&self) -> RegTMM {
        todo!()
    }

    pub fn unwrap_regmask(&self) -> RegMask {
        todo!()
    }

    pub fn unwrap_regfloat(&self) -> RegFloat {
        todo!()
    }

    pub fn unwrap_regsegment(&self) -> RegSegment {
        todo!()
    }

    pub fn unwrap_reg64withrip(&self) -> Reg64WithRIP {
        todo!()
    }

    pub fn unwrap_reg32withrip(&self) -> Reg32WithRIP {
        todo!()
    }

    pub fn unwrap_reg16withrip(&self) -> Reg16WithRIP {
        todo!()
    }

    pub fn unwrap_reg8(&self) -> Reg8 {
        todo!()
    }

    pub fn unwrap_reg64withoutrip(&self) -> Reg64WithoutRIP {
        match self {
            Register::GP64(reg64) => reg64.try_unwrap_reg64_without_rip().unwrap(),
            _ => panic!(),
        }
    }

    pub fn unwrap_reg32withoutrip(&self) -> Reg32WithoutRIP {
        match self {
            Register::GP32(gp32) => gp32.try_unwrap_reg32_without_rip().unwrap(),
            _ => panic!(),
        }
    }

    pub fn unwrap_reg16withoutrip(&self) -> Reg16WithoutRIP {
        match self {
            Register::GP16(gp16) => gp16.try_unwrap_reg16_without_rip().unwrap(),
            _ => panic!(),
        }
    }

    pub fn unwrap_regbnd(&self) -> RegBnd {
        todo!()
    }

    pub fn unwrap_regdebug(&self) -> RegDebug {
        todo!()
    }

    pub fn unwrap_regcontrol(&self) -> RegControl {
        todo!()
    }

    pub fn unwrap_regcontrolextra(&self) -> RegControlExtra {
        todo!()
    }

    pub fn from_capstone(reg_id: RegId, _operand: &X86Operand) -> Self {
        use capstone::arch::x86::X86Reg::*;
        match reg_id.0 as u32 {
            X86_REG_INVALID => {
                todo!()
            }
            X86_REG_AH => Self::GP8(Reg8::AH),
            X86_REG_AL => Self::GP8(Reg8::AL),
            X86_REG_AX => Self::GP16(Reg16WithRIP::AX),
            X86_REG_BH => Self::GP8(Reg8::BH),
            X86_REG_BL => Self::GP8(Reg8::BL),
            X86_REG_BP => Self::GP16(Reg16WithRIP::BP),
            X86_REG_BPL => {
                todo!()
            }
            X86_REG_BX => Self::GP16(Reg16WithRIP::BX),
            X86_REG_CH => Self::GP8(Reg8::CH),
            X86_REG_CL => Self::GP8(Reg8::CL),
            X86_REG_CS => {
                todo!()
            }
            X86_REG_CX => Self::GP16(Reg16WithRIP::CX),
            X86_REG_DH => Self::GP8(Reg8::DH),
            X86_REG_DI => Self::GP16(Reg16WithRIP::DI),
            X86_REG_DIL => {
                todo!()
            }
            X86_REG_DL => Self::GP8(Reg8::DL),
            X86_REG_DS => {
                todo!()
            }
            X86_REG_DX => Self::GP16(Reg16WithRIP::DX),
            X86_REG_EAX => Self::GP32(Reg32WithRIP::EAX),
            X86_REG_EBP => Self::GP32(Reg32WithRIP::EBP),
            X86_REG_EBX => Self::GP32(Reg32WithRIP::EBX),
            X86_REG_ECX => Self::GP32(Reg32WithRIP::ECX),
            X86_REG_EDI => Self::GP32(Reg32WithRIP::EDI),
            X86_REG_EDX => Self::GP32(Reg32WithRIP::EDX),
            X86_REG_EFLAGS => {
                todo!()
            }
            X86_REG_EIP => {
                todo!()
            }
            X86_REG_EIZ => {
                todo!()
            }
            X86_REG_ES => {
                todo!()
            }
            X86_REG_ESI => {
                todo!()
            }
            X86_REG_ESP => {
                todo!()
            }
            X86_REG_FPSW => {
                todo!()
            }
            X86_REG_FS => {
                todo!()
            }
            X86_REG_GS => {
                todo!()
            }
            X86_REG_IP => {
                todo!()
            }
            X86_REG_RAX => Self::GP64(Reg64WithRIP::RAX),
            X86_REG_RBP => Self::GP64(Reg64WithRIP::RBP),
            X86_REG_RBX => Self::GP64(Reg64WithRIP::RBX),
            X86_REG_RCX => Self::GP64(Reg64WithRIP::RCX),
            X86_REG_RDI => Self::GP64(Reg64WithRIP::RDI),
            X86_REG_RDX => Self::GP64(Reg64WithRIP::RDX),
            X86_REG_RIP => Self::GP64(Reg64WithRIP::RIP),
            X86_REG_RIZ => {
                todo!()
            }
            X86_REG_RSI => Self::GP64(Reg64WithRIP::RSI),
            X86_REG_RSP => Self::GP64(Reg64WithRIP::RSP),
            X86_REG_SI => {
                todo!()
            }
            X86_REG_SIL => {
                todo!()
            }
            X86_REG_SP => {
                todo!()
            }
            X86_REG_SPL => {
                todo!()
            }
            X86_REG_SS => {
                todo!()
            }
            X86_REG_CR0 => Self::Control(RegControl::CR0),
            X86_REG_CR1 => Self::Control(RegControl::CR1),
            X86_REG_CR2 => Self::Control(RegControl::CR2),
            X86_REG_CR3 => Self::Control(RegControl::CR3),
            X86_REG_CR4 => Self::Control(RegControl::CR4),
            X86_REG_CR5 => Self::Control(RegControl::CR5),
            X86_REG_CR6 => Self::Control(RegControl::CR6),
            X86_REG_CR7 => Self::Control(RegControl::CR7),
            X86_REG_CR8 => Self::Control(RegControl::CR8),
            X86_REG_CR9 => Self::Control(RegControl::CR9),
            X86_REG_CR10 => Self::Control(RegControl::CR10),
            X86_REG_CR11 => Self::Control(RegControl::CR11),
            X86_REG_CR12 => Self::Control(RegControl::CR12),
            X86_REG_CR13 => Self::Control(RegControl::CR13),
            X86_REG_CR14 => Self::Control(RegControl::CR14),
            X86_REG_CR15 => Self::Control(RegControl::CR15),
            X86_REG_DR0 => Self::Debug(RegDebug::DR0),
            X86_REG_DR1 => Self::Debug(RegDebug::DR1),
            X86_REG_DR2 => Self::Debug(RegDebug::DR2),
            X86_REG_DR3 => Self::Debug(RegDebug::DR3),
            X86_REG_DR4 => Self::Debug(RegDebug::DR4),
            X86_REG_DR5 => Self::Debug(RegDebug::DR5),
            X86_REG_DR6 => Self::Debug(RegDebug::DR6),
            X86_REG_DR7 => Self::Debug(RegDebug::DR7),
            X86_REG_DR8 => {
                todo!()
            }
            X86_REG_DR9 => {
                todo!()
            }
            X86_REG_DR10 => {
                todo!()
            }
            X86_REG_DR11 => {
                todo!()
            }
            X86_REG_DR12 => {
                todo!()
            }
            X86_REG_DR13 => {
                todo!()
            }
            X86_REG_DR14 => {
                todo!()
            }
            X86_REG_DR15 => {
                todo!()
            }
            X86_REG_FP0 => {
                todo!()
            }
            X86_REG_FP1 => {
                todo!()
            }
            X86_REG_FP2 => {
                todo!()
            }
            X86_REG_FP3 => {
                todo!()
            }
            X86_REG_FP4 => {
                todo!()
            }
            X86_REG_FP5 => {
                todo!()
            }
            X86_REG_FP6 => {
                todo!()
            }
            X86_REG_FP7 => {
                todo!()
            }
            X86_REG_K0 => Self::Mask(RegMask::K0),
            X86_REG_K1 => Self::Mask(RegMask::K1),
            X86_REG_K2 => Self::Mask(RegMask::K2),
            X86_REG_K3 => Self::Mask(RegMask::K3),
            X86_REG_K4 => Self::Mask(RegMask::K4),
            X86_REG_K5 => Self::Mask(RegMask::K5),
            X86_REG_K6 => Self::Mask(RegMask::K6),
            X86_REG_K7 => Self::Mask(RegMask::K7),
            X86_REG_MM0 => Self::Mmx(RegMMX::MM0),
            X86_REG_MM1 => Self::Mmx(RegMMX::MM1),
            X86_REG_MM2 => Self::Mmx(RegMMX::MM2),
            X86_REG_MM3 => Self::Mmx(RegMMX::MM3),
            X86_REG_MM4 => Self::Mmx(RegMMX::MM4),
            X86_REG_MM5 => Self::Mmx(RegMMX::MM5),
            X86_REG_MM6 => Self::Mmx(RegMMX::MM6),
            X86_REG_MM7 => Self::Mmx(RegMMX::MM7),
            X86_REG_R8 => {
                todo!()
            }
            X86_REG_R9 => {
                todo!()
            }
            X86_REG_R10 => {
                todo!()
            }
            X86_REG_R11 => {
                todo!()
            }
            X86_REG_R12 => {
                todo!()
            }
            X86_REG_R13 => {
                todo!()
            }
            X86_REG_R14 => {
                todo!()
            }
            X86_REG_R15 => {
                todo!()
            }
            X86_REG_ST0 => Self::Float(RegFloat::ST0),
            X86_REG_ST1 => Self::Float(RegFloat::ST1),
            X86_REG_ST2 => Self::Float(RegFloat::ST2),
            X86_REG_ST3 => Self::Float(RegFloat::ST3),
            X86_REG_ST4 => Self::Float(RegFloat::ST4),
            X86_REG_ST5 => Self::Float(RegFloat::ST5),
            X86_REG_ST6 => Self::Float(RegFloat::ST6),
            X86_REG_ST7 => Self::Float(RegFloat::ST7),
            X86_REG_XMM0 => Self::Xmm(RegXMM::XMM0),
            X86_REG_XMM1 => Self::Xmm(RegXMM::XMM1),
            X86_REG_XMM2 => Self::Xmm(RegXMM::XMM2),
            X86_REG_XMM3 => Self::Xmm(RegXMM::XMM3),
            X86_REG_XMM4 => Self::Xmm(RegXMM::XMM4),
            X86_REG_XMM5 => Self::Xmm(RegXMM::XMM5),
            X86_REG_XMM6 => Self::Xmm(RegXMM::XMM6),
            X86_REG_XMM7 => Self::Xmm(RegXMM::XMM7),
            X86_REG_XMM8 => Self::Xmm(RegXMM::XMM8),
            X86_REG_XMM9 => Self::Xmm(RegXMM::XMM9),
            X86_REG_XMM10 => Self::Xmm(RegXMM::XMM10),
            X86_REG_XMM11 => Self::Xmm(RegXMM::XMM11),
            X86_REG_XMM12 => Self::Xmm(RegXMM::XMM12),
            X86_REG_XMM13 => Self::Xmm(RegXMM::XMM13),
            X86_REG_XMM14 => Self::Xmm(RegXMM::XMM14),
            X86_REG_XMM15 => Self::Xmm(RegXMM::XMM15),
            X86_REG_XMM16 => Self::Xmm(RegXMM::XMM16),
            X86_REG_XMM17 => Self::Xmm(RegXMM::XMM17),
            X86_REG_XMM18 => Self::Xmm(RegXMM::XMM18),
            X86_REG_XMM19 => Self::Xmm(RegXMM::XMM19),
            X86_REG_XMM20 => Self::Xmm(RegXMM::XMM20),
            X86_REG_XMM21 => Self::Xmm(RegXMM::XMM21),
            X86_REG_XMM22 => Self::Xmm(RegXMM::XMM22),
            X86_REG_XMM23 => Self::Xmm(RegXMM::XMM23),
            X86_REG_XMM24 => Self::Xmm(RegXMM::XMM24),
            X86_REG_XMM25 => Self::Xmm(RegXMM::XMM25),
            X86_REG_XMM26 => Self::Xmm(RegXMM::XMM26),
            X86_REG_XMM27 => Self::Xmm(RegXMM::XMM27),
            X86_REG_XMM28 => Self::Xmm(RegXMM::XMM28),
            X86_REG_XMM29 => Self::Xmm(RegXMM::XMM29),
            X86_REG_XMM30 => Self::Xmm(RegXMM::XMM30),
            X86_REG_XMM31 => Self::Xmm(RegXMM::XMM31),
            X86_REG_YMM0 => Self::Ymm(RegYMM::YMM0),
            X86_REG_YMM1 => Self::Ymm(RegYMM::YMM1),
            X86_REG_YMM2 => Self::Ymm(RegYMM::YMM2),
            X86_REG_YMM3 => Self::Ymm(RegYMM::YMM3),
            X86_REG_YMM4 => Self::Ymm(RegYMM::YMM4),
            X86_REG_YMM5 => Self::Ymm(RegYMM::YMM5),
            X86_REG_YMM6 => Self::Ymm(RegYMM::YMM6),
            X86_REG_YMM7 => Self::Ymm(RegYMM::YMM7),
            X86_REG_YMM8 => Self::Ymm(RegYMM::YMM8),
            X86_REG_YMM9 => Self::Ymm(RegYMM::YMM9),
            X86_REG_YMM10 => Self::Ymm(RegYMM::YMM10),
            X86_REG_YMM11 => Self::Ymm(RegYMM::YMM11),
            X86_REG_YMM12 => Self::Ymm(RegYMM::YMM12),
            X86_REG_YMM13 => Self::Ymm(RegYMM::YMM13),
            X86_REG_YMM14 => Self::Ymm(RegYMM::YMM14),
            X86_REG_YMM15 => Self::Ymm(RegYMM::YMM15),
            X86_REG_YMM16 => Self::Ymm(RegYMM::YMM16),
            X86_REG_YMM17 => Self::Ymm(RegYMM::YMM17),
            X86_REG_YMM18 => Self::Ymm(RegYMM::YMM18),
            X86_REG_YMM19 => Self::Ymm(RegYMM::YMM19),
            X86_REG_YMM20 => Self::Ymm(RegYMM::YMM20),
            X86_REG_YMM21 => Self::Ymm(RegYMM::YMM21),
            X86_REG_YMM22 => Self::Ymm(RegYMM::YMM22),
            X86_REG_YMM23 => Self::Ymm(RegYMM::YMM23),
            X86_REG_YMM24 => Self::Ymm(RegYMM::YMM24),
            X86_REG_YMM25 => Self::Ymm(RegYMM::YMM25),
            X86_REG_YMM26 => Self::Ymm(RegYMM::YMM26),
            X86_REG_YMM27 => Self::Ymm(RegYMM::YMM27),
            X86_REG_YMM28 => Self::Ymm(RegYMM::YMM28),
            X86_REG_YMM29 => Self::Ymm(RegYMM::YMM29),
            X86_REG_YMM30 => Self::Ymm(RegYMM::YMM30),
            X86_REG_YMM31 => Self::Ymm(RegYMM::YMM31),
            X86_REG_ZMM0 => Self::Zmm(RegZMM::ZMM0),
            X86_REG_ZMM1 => Self::Zmm(RegZMM::ZMM1),
            X86_REG_ZMM2 => Self::Zmm(RegZMM::ZMM2),
            X86_REG_ZMM3 => Self::Zmm(RegZMM::ZMM3),
            X86_REG_ZMM4 => Self::Zmm(RegZMM::ZMM4),
            X86_REG_ZMM5 => Self::Zmm(RegZMM::ZMM5),
            X86_REG_ZMM6 => Self::Zmm(RegZMM::ZMM6),
            X86_REG_ZMM7 => Self::Zmm(RegZMM::ZMM7),
            X86_REG_ZMM8 => Self::Zmm(RegZMM::ZMM8),
            X86_REG_ZMM9 => Self::Zmm(RegZMM::ZMM9),
            X86_REG_ZMM10 => Self::Zmm(RegZMM::ZMM10),
            X86_REG_ZMM11 => Self::Zmm(RegZMM::ZMM11),
            X86_REG_ZMM12 => Self::Zmm(RegZMM::ZMM12),
            X86_REG_ZMM13 => Self::Zmm(RegZMM::ZMM13),
            X86_REG_ZMM14 => Self::Zmm(RegZMM::ZMM14),
            X86_REG_ZMM15 => Self::Zmm(RegZMM::ZMM15),
            X86_REG_ZMM16 => Self::Zmm(RegZMM::ZMM16),
            X86_REG_ZMM17 => Self::Zmm(RegZMM::ZMM17),
            X86_REG_ZMM18 => Self::Zmm(RegZMM::ZMM18),
            X86_REG_ZMM19 => Self::Zmm(RegZMM::ZMM19),
            X86_REG_ZMM20 => Self::Zmm(RegZMM::ZMM20),
            X86_REG_ZMM21 => Self::Zmm(RegZMM::ZMM21),
            X86_REG_ZMM22 => Self::Zmm(RegZMM::ZMM22),
            X86_REG_ZMM23 => Self::Zmm(RegZMM::ZMM23),
            X86_REG_ZMM24 => Self::Zmm(RegZMM::ZMM24),
            X86_REG_ZMM25 => Self::Zmm(RegZMM::ZMM25),
            X86_REG_ZMM26 => Self::Zmm(RegZMM::ZMM26),
            X86_REG_ZMM27 => Self::Zmm(RegZMM::ZMM27),
            X86_REG_ZMM28 => Self::Zmm(RegZMM::ZMM28),
            X86_REG_ZMM29 => Self::Zmm(RegZMM::ZMM29),
            X86_REG_ZMM30 => Self::Zmm(RegZMM::ZMM30),
            X86_REG_ZMM31 => Self::Zmm(RegZMM::ZMM31),
            X86_REG_R8B => {
                todo!()
            }
            X86_REG_R9B => {
                todo!()
            }
            X86_REG_R10B => {
                todo!()
            }
            X86_REG_R11B => {
                todo!()
            }
            X86_REG_R12B => {
                todo!()
            }
            X86_REG_R13B => {
                todo!()
            }
            X86_REG_R14B => {
                todo!()
            }
            X86_REG_R15B => {
                todo!()
            }
            X86_REG_R8D => {
                todo!()
            }
            X86_REG_R9D => {
                todo!()
            }
            X86_REG_R10D => {
                todo!()
            }
            X86_REG_R11D => {
                todo!()
            }
            X86_REG_R12D => {
                todo!()
            }
            X86_REG_R13D => {
                todo!()
            }
            X86_REG_R14D => {
                todo!()
            }
            X86_REG_R15D => {
                todo!()
            }
            X86_REG_R8W => Self::GP16(Reg16WithRIP::R8W),
            X86_REG_R9W => Self::GP16(Reg16WithRIP::R9W),
            X86_REG_R10W => Self::GP16(Reg16WithRIP::R10W),
            X86_REG_R11W => Self::GP16(Reg16WithRIP::R11W),
            X86_REG_R12W => Self::GP16(Reg16WithRIP::R12W),
            X86_REG_R13W => Self::GP16(Reg16WithRIP::R13W),
            X86_REG_R14W => Self::GP16(Reg16WithRIP::R14W),
            X86_REG_R15W => Self::GP16(Reg16WithRIP::R15W),
            X86_REG_BND0 => {
                todo!()
            }
            X86_REG_BND1 => {
                todo!()
            }
            X86_REG_BND2 => {
                todo!()
            }
            X86_REG_BND3 => {
                todo!()
            }
            X86_REG_ENDING => {
                todo!()
            }
            _ => {
                todo!()
            }
        }
    }
}
