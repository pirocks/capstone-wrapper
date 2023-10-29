use std::collections::BTreeSet;
use std::fmt::Debug;
use std::hash::Hash;
use std::iter;

use itertools::Itertools;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::operand_index::OperandIndexError;
use crate::operands::{Operand, OperandImm};
use crate::registers::{
    Reg16WithRIP, Reg32WithRIP, Reg64WithRIP, Reg8, RegControl, RegControlExtra, RegFloat,
    RegFloatControl, RegMask, RegSegment, RegSegmentBase, RegSpecial, RegXMM, RegZMM, RegisterType,
};

#[derive(Debug, Error)]
pub enum FromRawError {
    #[error(transparent)]
    OperandIndex(#[from] OperandIndexError),
    #[error(transparent)]
    OperandFromStr(#[from] OperandFromStr),
    #[error("multiple operands with the same index")]
    MultipleOperandsWithSameIndex,
}

#[derive(Debug, Error)]
pub enum OperandFromStr {
    #[error("Unknown register val: {val}")]
    UnknownRegister { val: String },
    #[error("Unknown imm val: {val}")]
    UnknownImm { val: String },
    #[error("Missing xml val field")]
    MissingVal,
    #[error("Relative operand width unexpected: {width:?}")]
    UnexpectedWidthRel { width: Option<String> },
    #[error("Unexpected type: {r#type}")]
    UnexpectedType { r#type: String },
    #[error("Encountered an unknown memory operand")]
    UnknownMemoryOperand,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum OperandType {
    Reg(RegisterType),
    Mem(MemoryOperandType),
    Imm(Imm),
    ImmSpecific(i64),
    Flags(Flags),
    Agen(Agen),
    Rel8,
    Rel16,
    Rel32,
}

fn single_set<T: Eq + PartialEq + Hash + Debug + Ord + PartialOrd>(t: T) -> BTreeSet<T> {
    BTreeSet::from_iter(iter::once(t))
}

impl OperandType {
    pub fn to_declaration_string(&self) -> String {
        match self {
            OperandType::Reg(reg) => {
                let reg = match reg {
                    RegisterType::AllMmx => "RegisterType::AllMmx".to_string(),
                    RegisterType::AllXmm16 => "RegisterType::AllXmm16".to_string(),
                    RegisterType::AllXmm32 => "RegisterType::AllXmm32".to_string(),
                    RegisterType::AllYmm16 => "RegisterType::AllYmm16".to_string(),
                    RegisterType::AllYmm32 => "RegisterType::AllYmm32".to_string(),
                    RegisterType::AllZmm32 => "RegisterType::AllZmm32".to_string(),
                    RegisterType::AllTmm => "RegisterType::AllTmm".to_string(),
                    RegisterType::AllMask => "RegisterType::AllMask".to_string(),
                    RegisterType::AllGP64WithoutRIP => {
                        "RegisterType::AllGP64WithoutRIP".to_string()
                    }
                    RegisterType::AllGP64WithRIP => "RegisterType::AllGP64WithRIP".to_string(),
                    RegisterType::AllGP32WithoutRIP => {
                        "RegisterType::AllGP32WithoutRIP".to_string()
                    }
                    RegisterType::AllGP32WithRIP => "RegisterType::AllGP32WithRIP".to_string(),
                    RegisterType::AllGP16WithoutRIP => {
                        "RegisterType::AllGP16WithoutRIP".to_string()
                    }
                    RegisterType::AllGP16WithRIP => "RegisterType::AllGP16WithRIP".to_string(),
                    RegisterType::AllGP8 => "RegisterType::AllGP8".to_string(),
                    RegisterType::AllFloat => "RegisterType::AllFloat".to_string(),
                    RegisterType::AllBnd => "RegisterType::AllBnd".to_string(),
                    RegisterType::AllSegment => "RegisterType::AllSegment".to_string(),
                    RegisterType::AllDebug => "RegisterType::AllDebug".to_string(),
                    RegisterType::SingleXmm(inner) => {
                        format!("RegisterType::SingleXmm({})", inner.to_declaration_string())
                    }
                    RegisterType::SingleGP8(inner) => {
                        format!("RegisterType::SingleGP8({})", inner.to_declaration_string())
                    }
                    RegisterType::SingleGP16(inner) => format!(
                        "RegisterType::SingleGP16({})",
                        inner.to_declaration_string()
                    ),
                    RegisterType::SingleGP32(single) => format!(
                        "RegisterType::SingleGP32({})",
                        single.to_declaration_string()
                    ),
                    RegisterType::SingleGP64(inner) => format!(
                        "RegisterType::SingleGP64({})",
                        inner.to_declaration_string()
                    ),
                    RegisterType::SingleSegment(inner) => format!(
                        "RegisterType::SingleSegment({})",
                        inner.to_declaration_string()
                    ),
                    RegisterType::SingleSegmentBase(inner) => format!(
                        "RegisterType::SingleSegmentBase({})",
                        inner.to_declaration_string()
                    ),
                    RegisterType::SingleFloat(float) => {
                        let float = match float {
                            RegFloat::ST0 => "RegFloat::ST0".to_string(),
                            RegFloat::ST1 => "RegFloat::ST1".to_string(),
                            RegFloat::ST2 => "RegFloat::ST2".to_string(),
                            RegFloat::ST3 => "RegFloat::ST3".to_string(),
                            RegFloat::ST4 => "RegFloat::ST4".to_string(),
                            RegFloat::ST5 => "RegFloat::ST5".to_string(),
                            RegFloat::ST6 => "RegFloat::ST6".to_string(),
                            RegFloat::ST7 => "RegFloat::ST7".to_string(),
                        };
                        format!("RegisterType::SingleFloat({float})")
                    }
                    RegisterType::SingleControl(control) => format!(
                        "RegisterType::SingleControl({})",
                        control.to_declaration_string()
                    ),
                    RegisterType::SingleSpecial(special) => format!(
                        "RegisterType::SingleSpecial({})",
                        special.to_declaration_string()
                    ),
                    RegisterType::SingleFloatControl(float_control) => format!(
                        "RegisterType::SingleFloatControl({})",
                        float_control.to_declaration_string()
                    ),
                    RegisterType::SomeSegment(inner) => {
                        format!(
                            "RegisterType::SomeSegment([{}].into_iter().collect())",
                            inner
                                .iter()
                                .map(|inner| inner.to_declaration_string())
                                .join(",")
                        )
                    }
                    RegisterType::SomeXmm(inner) => {
                        format!(
                            "RegisterType::SomeXmm([{}].into_iter().collect())",
                            inner
                                .iter()
                                .map(|inner| inner.to_declaration_string())
                                .join(",")
                        )
                    }
                    RegisterType::SomeZmm(inner) => {
                        format!(
                            "RegisterType::SomeZmm([{}].into_iter().collect())",
                            inner
                                .iter()
                                .map(|inner| inner.to_declaration_string())
                                .join(",")
                        )
                    }
                    RegisterType::SomeMask(inner) => {
                        format!(
                            "RegisterType::SomeMask([{}].into_iter().collect())",
                            inner
                                .iter()
                                .map(|inner| inner.to_declaration_string())
                                .join(",")
                        )
                    }
                    RegisterType::SomeGP32(inner) => {
                        format!(
                            "RegisterType::SomeGP32([{}].into_iter().collect())",
                            inner
                                .iter()
                                .map(|inner| inner.to_declaration_string())
                                .join(",")
                        )
                    }
                    RegisterType::SomeGP16(inner) => {
                        format!(
                            "RegisterType::SomeGP16([{}].into_iter().collect())",
                            inner
                                .iter()
                                .map(|inner| inner.to_declaration_string())
                                .join(",")
                        )
                    }
                    RegisterType::SomeGP8(inner) => {
                        format!(
                            "RegisterType::SomeGP8([{}].into_iter().collect())",
                            inner
                                .iter()
                                .map(|inner| inner.to_declaration_string())
                                .join(",")
                        )
                    }
                    RegisterType::SomeControl(inner) => {
                        format!(
                            "RegisterType::SomeControl([{}].into_iter().collect())",
                            inner
                                .iter()
                                .map(|inner| inner.to_declaration_string())
                                .join(",")
                        )
                    }
                    RegisterType::SomeControlExtra(inner) => {
                        format!(
                            "RegisterType::SomeControlExtra([{}].into_iter().collect())",
                            inner
                                .iter()
                                .map(|inner| inner.to_declaration_string())
                                .join(",")
                        )
                    }
                    RegisterType::Multiple(_) => {
                        todo!()
                    }
                    RegisterType::AllControl => {
                        todo!()
                    }
                };
                format!("OperandType::Reg({reg})")
            }
            OperandType::Mem(MemoryOperandType {
                vsib,
                kind,
                load,
                store,
            }) => {
                let vsib = match vsib {
                    None => "None".to_string(),
                    Some(vrk) => match vrk {
                        VectorRegisterKind::XMM => "Some(VectorRegisterKind::XMM)".to_string(),
                        VectorRegisterKind::YMM => "Some(VectorRegisterKind::YMM)".to_string(),
                        VectorRegisterKind::ZMM => "Some(VectorRegisterKind::ZMM)".to_string(),
                    },
                };
                let kind = match kind {
                    MemoryOperandTypeKind::MemTile => "MemoryOperandTypeKind::MemTile".to_string(),
                    MemoryOperandTypeKind::MemStruct => {
                        "MemoryOperandTypeKind::MemStruct".to_string()
                    }
                    MemoryOperandTypeKind::Mem512 => "MemoryOperandTypeKind::Mem512".to_string(),
                    MemoryOperandTypeKind::Mem384 => "MemoryOperandTypeKind::Mem384".to_string(),
                    MemoryOperandTypeKind::Mem320 => "MemoryOperandTypeKind::Mem320".to_string(),
                    MemoryOperandTypeKind::Mem256 => "MemoryOperandTypeKind::Mem256".to_string(),
                    MemoryOperandTypeKind::Mem192 => "MemoryOperandTypeKind::Mem192".to_string(),
                    MemoryOperandTypeKind::Mem128 => "MemoryOperandTypeKind::Mem128".to_string(),
                    MemoryOperandTypeKind::Mem160 => "MemoryOperandTypeKind::Mem160".to_string(),
                    MemoryOperandTypeKind::Mem80 => "MemoryOperandTypeKind::Mem80".to_string(),
                    MemoryOperandTypeKind::Mem64 => "MemoryOperandTypeKind::Mem64".to_string(),
                    MemoryOperandTypeKind::Mem48 => "MemoryOperandTypeKind::Mem48".to_string(),
                    MemoryOperandTypeKind::Mem32 => "MemoryOperandTypeKind::Mem32".to_string(),
                    MemoryOperandTypeKind::Mem16 => "MemoryOperandTypeKind::Mem16".to_string(),
                    MemoryOperandTypeKind::Mem8 => "MemoryOperandTypeKind::Mem8".to_string(),
                };
                format!("OperandType::Mem(MemoryOperandType{{ vsib:{}, kind:{}, load: {}, store: {} }})", vsib, kind, load, store)
            }
            OperandType::Imm(imm) => match imm {
                Imm::Imm8 => "OperandType::Imm(Imm::Imm8)".to_string(),
                Imm::Imm16 => "OperandType::Imm(Imm::Imm16)".to_string(),
                Imm::Imm32 => "OperandType::Imm(Imm::Imm32)".to_string(),
                Imm::Imm64 => "OperandType::Imm(Imm::Imm64)".to_string(),
            },
            OperandType::ImmSpecific(inner) => format!("OperandType::ImmSpecific({inner})"),
            OperandType::Flags(Flags {}) => "OperandType::Flags(Flags{})".to_string(),
            OperandType::Agen(Agen {}) => "OperandType::Agen(Agen{})".to_string(),
            OperandType::Rel8 => "OperandType::Rel8".to_string(),
            OperandType::Rel16 => "OperandType::Rel16".to_string(),
            OperandType::Rel32 => "OperandType::Rel32".to_string(),
        }
    }

    /*    pub fn from_capstone(operand: &X86Operand) -> Self {
            match operand.op_type {
                X86OperandType::Reg(reg) => {
                    Self::Reg(Register::from_capstone(reg, operand))
                }
                X86OperandType::Imm(imm) => {
                    if operand.size == 64 {
                        Self::Imm(Imm::Imm64)
                    } else {
                        todo!()
                    }
                }
                X86OperandType::Mem(mem) => {
                    todo!()
                }
                X86OperandType::Invalid => {
                    todo!()
                }
            }
        }
    */
    pub fn is_of_type(&self, operand: &Operand) -> bool {
        match self {
            OperandType::Reg(reg_type) => match operand {
                Operand::Reg(reg) => reg_type.is_of_type(reg),
                _ => false,
            },
            OperandType::Mem(_mem_type) => match operand {
                Operand::Reg(_) | Operand::Imm(_) => false,
            },
            OperandType::Imm(imm_type) => match operand {
                Operand::Reg(_) => false,
                Operand::Imm(imm) => match imm_type {
                    Imm::Imm8 => matches!(imm, OperandImm::Imm8(_)),
                    Imm::Imm16 => matches!(imm, OperandImm::Imm16(_)),
                    Imm::Imm32 => matches!(imm, OperandImm::Imm32(_)),
                    Imm::Imm64 => matches!(imm, OperandImm::Imm64(_)),
                },
            },
            OperandType::ImmSpecific(imm_specific) => match operand {
                Operand::Reg(_) => false,
                Operand::Imm(imm_inner) => match imm_inner {
                    OperandImm::Imm64(inner) => *inner as i64 == *imm_specific,
                    OperandImm::Imm32(inner) => *inner as i64 == *imm_specific,
                    OperandImm::Imm16(inner) => *inner as i64 == *imm_specific,
                    OperandImm::Imm8(inner) => *inner as i64 == *imm_specific,
                },
            },
            OperandType::Flags(_) => match operand {
                Operand::Reg(_) => false,
                Operand::Imm(_) => {
                    todo!()
                }
            },
            OperandType::Agen(_) => match operand {
                Operand::Reg(_) => false,
                Operand::Imm(_) => {
                    todo!()
                }
            },
            OperandType::Rel8 => match operand {
                Operand::Reg(_) => false,
                Operand::Imm(_) => {
                    todo!()
                }
            },
            OperandType::Rel16 => match operand {
                Operand::Reg(_) => false,
                Operand::Imm(_) => {
                    todo!()
                }
            },
            OperandType::Rel32 => match operand {
                Operand::Reg(_) => false,
                Operand::Imm(_) => {
                    todo!()
                }
            },
        }
    }

    pub fn compatible_with(&self, _other: &Self) -> bool {
        match self {
            OperandType::Reg(_) => {
                todo!()
            }
            OperandType::Mem(_) => {
                todo!()
            }
            OperandType::Imm(_) => {
                todo!()
            }
            OperandType::ImmSpecific(_) => {
                todo!()
            }
            OperandType::Flags(_) => {
                todo!()
            }
            OperandType::Agen(_) => {
                todo!()
            }
            OperandType::Rel8 => {
                todo!()
            }
            OperandType::Rel16 => {
                todo!()
            }
            OperandType::Rel32 => {
                todo!()
            }
        }
    }

    pub fn from_reg(val: impl AsRef<str>) -> Result<OperandType, OperandFromStr> {
        Ok(OperandType::Reg(match val.as_ref() {
            "RAX" => RegisterType::SingleGP64(Reg64WithRIP::RAX),
            "RBX" => RegisterType::SingleGP64(Reg64WithRIP::RBX),
            "RCX" => RegisterType::SingleGP64(Reg64WithRIP::RCX),
            "RDX" => RegisterType::SingleGP64(Reg64WithRIP::RDX),
            "RSP" => RegisterType::SingleGP64(Reg64WithRIP::RSP),
            "RSI" => RegisterType::SingleGP64(Reg64WithRIP::RSI),
            "RDI" => RegisterType::SingleGP64(Reg64WithRIP::RDI),
            "RBP" => RegisterType::SingleGP64(Reg64WithRIP::RBP),
            "RIP" => RegisterType::SingleGP64(Reg64WithRIP::RIP),
            "R11" => RegisterType::SingleGP64(Reg64WithRIP::R11),
            "EAX" => RegisterType::SingleGP32(Reg32WithRIP::EAX),
            "EBX" => RegisterType::SingleGP32(Reg32WithRIP::EBX),
            "ECX" => RegisterType::SingleGP32(Reg32WithRIP::ECX),
            "EDX" => RegisterType::SingleGP32(Reg32WithRIP::EDX),
            "SP" => RegisterType::SingleGP16(Reg16WithRIP::SP),
            "AX" => RegisterType::SingleGP16(Reg16WithRIP::AX),
            "DX" => RegisterType::SingleGP16(Reg16WithRIP::DX),
            "BP" => RegisterType::SingleGP16(Reg16WithRIP::BP),
            "AL" => RegisterType::SingleGP8(Reg8::AL),
            "CL" => RegisterType::SingleGP8(Reg8::CL),
            "AH" => RegisterType::SingleGP8(Reg8::AH),
            "FS" => RegisterType::SingleSegment(RegSegment::FS),
            "FSBASE" => RegisterType::SingleSegmentBase(RegSegmentBase::FSBase),
            "GS" => RegisterType::SingleSegment(RegSegment::GS),
            "GSBASE" => RegisterType::SingleSegmentBase(RegSegmentBase::GSBase),
            "SS" => RegisterType::SingleSegment(RegSegment::SS),
            "TR" => RegisterType::SingleSpecial(RegSpecial::TR),
            "CR0" => RegisterType::SingleControl(RegControl::CR0),
            "XCR0" => RegisterType::SomeControlExtra(single_set(RegControlExtra::XCR0)),
            "GDTR" => RegisterType::SingleSpecial(RegSpecial::GDTR),
            "IDTR" => RegisterType::SingleSpecial(RegSpecial::IDTR),
            "LDTR" => RegisterType::SingleSpecial(RegSpecial::LDTR),
            "MXCSR" => RegisterType::SomeControlExtra(single_set(RegControlExtra::MXCSR)),
            "MSRS" => RegisterType::SingleSpecial(RegSpecial::MSRS),
            "TSC" => RegisterType::SingleSpecial(RegSpecial::TSC),
            "TSCAUX" => RegisterType::SingleSpecial(RegSpecial::TSCAUX),
            "X87CONTROL" => RegisterType::SingleFloatControl(RegFloatControl::X87CONTROL),
            "X87STATUS" => RegisterType::SingleFloatControl(RegFloatControl::X87STATUS),
            "X87TAG" => RegisterType::SingleFloatControl(RegFloatControl::X87TAG),
            "X87POP" => RegisterType::SingleFloatControl(RegFloatControl::X87POP),
            "X87PUSH" => RegisterType::SingleFloatControl(RegFloatControl::X87PUSH),
            "X87POP2" => RegisterType::SingleFloatControl(RegFloatControl::X87POP2),
            "SSP" => RegisterType::SingleSpecial(RegSpecial::SSP),
            "ST(0)" => RegisterType::SingleFloat(RegFloat::ST0),
            "ST(1)" => RegisterType::SingleFloat(RegFloat::ST1),
            "XMM0" => RegisterType::SingleXmm(RegXMM::XMM0),
            "XMM1" => RegisterType::SingleXmm(RegXMM::XMM1),
            "XMM2" => RegisterType::SingleXmm(RegXMM::XMM2),
            "XMM3" => RegisterType::SingleXmm(RegXMM::XMM3),
            "XMM4" => RegisterType::SingleXmm(RegXMM::XMM4),
            "XMM5" => RegisterType::SingleXmm(RegXMM::XMM5),
            "XMM6" => RegisterType::SingleXmm(RegXMM::XMM6),
            "XMM7" => RegisterType::SingleXmm(RegXMM::XMM7),
            "BND0,BND1,BND2,BND3" => RegisterType::AllBnd,
            "UIF" => RegisterType::SingleSpecial(RegSpecial::UIF),
            "EAX,ECX,EDX,EBX,ESP,EBP,ESI,EDI,R8D,R9D,R10D,R11D,R12D,R13D,R14D,R15D" => RegisterType::AllGP32WithoutRIP,
            "AX,CX,DX,BX,SP,BP,SI,DI,R8W,R9W,R10W,R11W,R12W,R13W,R14W,R15W" => RegisterType::AllGP16WithoutRIP,
            "AL,CL,DL,BL,SPL,BPL,SIL,DIL,R8B,R9B,R10B,R11B,R12B,R13B,R14B,R15B" => RegisterType::AllGP8,
            "AL,CL,DL,BL,R8B,R9B,R10B,R11B" => RegisterType::SomeGP8(BTreeSet::from_iter(vec![Reg8::AL, Reg8::CL, Reg8::DL, Reg8::BL, Reg8::R8B, Reg8::R9B, Reg8::R10B, Reg8::R11B].into_iter())),
            "AH,CH,DH,BH" => RegisterType::SomeGP8(BTreeSet::from_iter(vec![Reg8::AH, Reg8::CH, Reg8::DH, Reg8::BH].into_iter())),
            "AL,CL,DL,BL" => RegisterType::SomeGP8(BTreeSet::from_iter(vec![Reg8::AL, Reg8::CL, Reg8::DL, Reg8::BL].into_iter())),
            "ES,SS,DS,FS,GS" => RegisterType::SomeSegment(BTreeSet::from_iter(vec![RegSegment::ES, RegSegment::SS, RegSegment::DS, RegSegment::FS, RegSegment::GS].into_iter())),
            "ES,CS,SS,DS,FS,GS" => RegisterType::AllSegment,
            "CR0,CR2,CR3,CR4,CR8" => RegisterType::SomeControl(BTreeSet::from_iter(vec![RegControl::CR0, RegControl::CR2, RegControl::CR3, RegControl::CR4, RegControl::CR8].into_iter())),
            "DR0,DR1,DR2,DR3,DR4,DR5,DR6,DR7" => RegisterType::AllDebug,
            "AX,CX,DX,BX,SP,BP,SI,DI" => RegisterType::SomeGP16(BTreeSet::from_iter(vec![Reg16WithRIP::AX, Reg16WithRIP::CX, Reg16WithRIP::DX, Reg16WithRIP::BX, Reg16WithRIP::SP, Reg16WithRIP::BP, Reg16WithRIP::SI, Reg16WithRIP::DI].into_iter())),
            "EAX,ECX,EDX,EBX,ESP,EBP,ESI,EDI" => RegisterType::SomeGP32(BTreeSet::from_iter(vec![Reg32WithRIP::EAX, Reg32WithRIP::ECX, Reg32WithRIP::EDX, Reg32WithRIP::EDX, Reg32WithRIP::ESP, Reg32WithRIP::EBP, Reg32WithRIP::ESI, Reg32WithRIP::EDI])),
            "SPL,BPL,SIL,DIL,R12B,R13B,R14B,R15B" => RegisterType::SomeGP8(BTreeSet::from_iter(vec![Reg8::SPL, Reg8::BPL, Reg8::SIL, Reg8::DIL, Reg8::R12B, Reg8::R13B, Reg8::R14B, Reg8::R15B].into_iter())),
            "MM0,MM1,MM2,MM3,MM4,MM5,MM6,MM7" => RegisterType::AllMmx,
            "TMM0,TMM1,TMM2,TMM3,TMM4,TMM5,TMM6,TMM7" => RegisterType::AllTmm,
            "RAX,RCX,RDX,RBX,RSP,RBP,RSI,RDI,R8,R9,R10,R11,R12,R13,R14,R15" => RegisterType::AllGP64WithoutRIP,
            "XMM0,XMM1,XMM2,XMM3,XMM4,XMM5,XMM6,XMM7,XMM8,XMM9,XMM10,XMM11,XMM12,XMM13,XMM14,XMM15" => RegisterType::AllXmm16,
            "YMM0,YMM1,YMM2,YMM3,YMM4,YMM5,YMM6,YMM7,YMM8,YMM9,YMM10,YMM11,YMM12,YMM13,YMM14,YMM15" => RegisterType::AllYmm16,
            "ZMM0,ZMM1,ZMM2,ZMM3,ZMM4,ZMM5,ZMM6,ZMM7,ZMM8,ZMM9,ZMM10,ZMM11,ZMM12,ZMM13,ZMM14,ZMM15,ZMM16,ZMM17,ZMM18,ZMM19,ZMM20,ZMM21,ZMM22,ZMM23,ZMM24,ZMM25,ZMM26,ZMM27,ZMM28,ZMM29,ZMM30,ZMM31" => RegisterType::AllZmm32,
            "YMM0,YMM1,YMM2,YMM3,YMM4,YMM5,YMM6,YMM7,YMM8,YMM9,YMM10,YMM11,YMM12,YMM13,YMM14,YMM15,YMM16,YMM17,YMM18,YMM19,YMM20,YMM21,YMM22,YMM23,YMM24,YMM25,YMM26,YMM27,YMM28,YMM29,YMM30,YMM31" => RegisterType::AllYmm32,
            "XMM0,XMM1,XMM2,XMM3,XMM4,XMM5,XMM6,XMM7,XMM8,XMM9,XMM10,XMM11,XMM12,XMM13,XMM14,XMM15,XMM16,XMM17,XMM18,XMM19,XMM20,XMM21,XMM22,XMM23,XMM24,XMM25,XMM26,XMM27,XMM28,XMM29,XMM30,XMM31" => RegisterType::AllXmm32,
            "ZMM0,ZMM4,ZMM8,ZMM12,ZMM16,ZMM20,ZMM24,ZMM28" => RegisterType::SomeZmm(BTreeSet::from_iter(vec![RegZMM::ZMM0, RegZMM::ZMM4, RegZMM::ZMM8, RegZMM::ZMM12, RegZMM::ZMM16, RegZMM::ZMM20, RegZMM::ZMM24, RegZMM::ZMM28].into_iter())),
            "XMM0,XMM4,XMM8,XMM12,XMM16,XMM20,XMM24,XMM28" => RegisterType::SomeXmm(BTreeSet::from_iter(vec![RegXMM::XMM0, RegXMM::XMM4, RegXMM::XMM8, RegXMM::XMM12, RegXMM::XMM16, RegXMM::XMM20, RegXMM::XMM24, RegXMM::XMM28].into_iter())),
            "ST(0),ST(1),ST(2),ST(3),ST(4),ST(5),ST(6),ST(7)" => RegisterType::AllFloat,
            "K1,K2,K3,K4,K5,K6,K7" => RegisterType::SomeMask(BTreeSet::from_iter(vec![RegMask::K0, RegMask::K1, RegMask::K2, RegMask::K3, RegMask::K4, RegMask::K5, RegMask::K6, RegMask::K7].into_iter())),
            "K0,K1,K2,K3,K4,K5,K6,K7" => RegisterType::AllMask,
            "K7,K6,K5,K4,K3,K2,K1,K0" => RegisterType::AllMask,
            "K0,K2,K4,K6" => RegisterType::SomeMask(BTreeSet::from_iter(vec![RegMask::K0, RegMask::K2, RegMask::K4, RegMask::K6].into_iter())),
            val => return Err(OperandFromStr::UnknownRegister { val: val.to_string() })
        }))
    }

    pub fn from_mem(
        xtype: Option<impl AsRef<str>>,
        memory_prefix: Option<impl AsRef<str>>,
        width: Option<impl AsRef<str>>,
        vsib: Option<impl AsRef<str>>,
        load: bool,
        store: bool,
    ) -> Result<OperandType, OperandFromStr> {
        let memory_operand_kind = match (
            xtype.as_ref().map(|s| s.as_ref()),
            memory_prefix.as_ref().map(|s| s.as_ref()),
            width.as_ref().map(|s| s.as_ref()),
        ) {
            (Some("struct"), _, Some("80")) => MemoryOperandTypeKind::Mem80,
            (Some("struct"), _, Some("64")) => MemoryOperandTypeKind::Mem64,
            (Some("struct"), _, Some("48")) => MemoryOperandTypeKind::Mem48,
            (Some("struct"), _, Some("32")) => MemoryOperandTypeKind::Mem32,
            (Some("struct"), None, _) => MemoryOperandTypeKind::MemStruct,
            (Some("struct"), Some("zmmword ptr"), _) => MemoryOperandTypeKind::MemStruct,
            (_, Some("zmmword ptr"), _) => MemoryOperandTypeKind::Mem512,
            (_, Some("ymmword ptr"), _) => MemoryOperandTypeKind::Mem256,
            (_, Some("xmmword ptr"), _) => MemoryOperandTypeKind::Mem128,
            (_, Some("tbyte ptr"), _) => MemoryOperandTypeKind::Mem80,
            (_, Some("qword ptr"), _) => MemoryOperandTypeKind::Mem64,
            (_, Some("dword ptr"), _) => MemoryOperandTypeKind::Mem32,
            (_, Some("word ptr"), _) => MemoryOperandTypeKind::Mem16,
            (_, Some("byte ptr"), _) => MemoryOperandTypeKind::Mem8,
            (_, _, Some("512")) => MemoryOperandTypeKind::Mem512,
            (_, _, Some("384")) => MemoryOperandTypeKind::Mem384,
            (_, _, Some("320")) => MemoryOperandTypeKind::Mem320,
            (_, _, Some("256")) => MemoryOperandTypeKind::Mem256,
            (_, _, Some("192")) => MemoryOperandTypeKind::Mem192,
            (_, _, Some("160")) => MemoryOperandTypeKind::Mem160,
            (_, _, Some("128")) => MemoryOperandTypeKind::Mem128,
            (_, _, Some("80")) => MemoryOperandTypeKind::Mem80,
            (_, _, Some("64")) => MemoryOperandTypeKind::Mem64,
            (_, _, Some("32")) => MemoryOperandTypeKind::Mem32,
            (_, _, Some("16")) => MemoryOperandTypeKind::Mem16,
            (_, _, Some("8")) => MemoryOperandTypeKind::Mem8,
            _ => {
                if xtype.as_ref().map(|s| s.as_ref()) == Some("u32") {
                    MemoryOperandTypeKind::MemTile
                } else {
                    return Err(OperandFromStr::UnknownMemoryOperand);
                }
            }
        };

        Ok(OperandType::Mem(MemoryOperandType {
            vsib: match vsib.as_ref().map(|s| s.as_ref()) {
                None => None,
                Some("XMM") => Some(VectorRegisterKind::XMM),
                Some("YMM") => Some(VectorRegisterKind::YMM),
                Some("ZMM") => Some(VectorRegisterKind::ZMM),
                a => todo!("{a:?}"),
            },
            kind: memory_operand_kind,
            load,
            store,
        }))
    }

    pub fn new(
        r#type: &String,
        xtype: Option<impl AsRef<str>>,
        val: Option<impl AsRef<str>>,
        memory_prefix: Option<impl AsRef<str>>,
        width: Option<impl AsRef<str>>,
        vsib: Option<impl AsRef<str>>,
        load: Option<impl AsRef<str>>,
        store: Option<impl AsRef<str>>,
    ) -> Result<OperandType, OperandFromStr> {
        match r#type.as_str() {
            "mem" => Self::from_mem(
                xtype,
                memory_prefix,
                width,
                vsib,
                load.map(|inner| inner.as_ref() == "1").unwrap_or(false),
                store.map(|inner| inner.as_ref() == "1").unwrap_or(false),
            ),
            "reg" => match val {
                None => Err(OperandFromStr::MissingVal.into()),
                Some(val) => Self::from_reg(val),
            },
            "imm" => Ok(match val.as_ref().map(|s| s.as_ref()) {
                None => OperandType::Imm(match width.as_ref().map(|s| s.as_ref()) {
                    Some("8") => Imm::Imm8,
                    Some("16") => Imm::Imm16,
                    Some("32") => Imm::Imm32,
                    Some("64") => Imm::Imm64,
                    _ => {
                        todo!()
                    }
                }),
                Some("0") => OperandType::ImmSpecific(0),
                Some("1") => OperandType::ImmSpecific(1),
                Some(val) => {
                    return Err(OperandFromStr::UnknownImm {
                        val: val.to_string(),
                    })
                }
            }),
            "flags" => Ok(OperandType::Flags(Flags {})),
            "agen" => Ok(OperandType::Agen(Agen {})),
            "relbr" => Ok(match width.as_ref().map(|s| s.as_ref()) {
                Some("32") => OperandType::Rel32,
                Some("16") => OperandType::Rel16,
                Some("8") => OperandType::Rel8,
                width => {
                    return Err(OperandFromStr::UnexpectedWidthRel {
                        width: width.map(|s| s.to_string()),
                    }
                    .into());
                }
            }),
            r#type => Err(OperandFromStr::UnexpectedType {
                r#type: r#type.to_string(),
            }
            .into()),
        }
    }

    pub fn to_identifier_string(&self) -> String {
        match self {
            OperandType::Reg(reg) => match reg {
                RegisterType::AllMmx => "MMX".to_string(),
                RegisterType::AllXmm16 => "XMM".to_string(),
                RegisterType::AllXmm32 => "XMM".to_string(),
                RegisterType::SomeXmm(_) => "XMM".to_string(),
                RegisterType::AllYmm16 => "YMM".to_string(),
                RegisterType::AllYmm32 => "YMM".to_string(),
                RegisterType::AllZmm32 => "ZMM".to_string(),
                RegisterType::SomeZmm(_) => "ZMM".to_string(),
                RegisterType::AllTmm => "TMM".to_string(),
                RegisterType::AllMask => "K".to_string(),
                RegisterType::SomeMask(_) => "K".to_string(),
                RegisterType::AllGP64WithoutRIP => "R64".to_string(),
                RegisterType::AllGP64WithRIP => "R64".to_string(),
                RegisterType::AllGP32WithoutRIP => "R32".to_string(),
                RegisterType::AllGP32WithRIP => "R32".to_string(),
                RegisterType::SomeGP32(_) => "R32".to_string(),
                RegisterType::AllGP16WithoutRIP => "R16".to_string(),
                RegisterType::AllGP16WithRIP => "R16".to_string(),
                RegisterType::SomeGP16(_) => "R16".to_string(),
                RegisterType::AllGP8 => "R8".to_string(),
                RegisterType::SomeGP8(some) => {
                    if some
                        == &BTreeSet::from_iter(
                            vec![Reg8::AL, Reg8::BL, Reg8::CL, Reg8::DL].into_iter(),
                        )
                    {
                        format!("R8L")
                    } else if some
                        == &BTreeSet::from_iter(
                            vec![Reg8::AH, Reg8::BH, Reg8::CH, Reg8::DH].into_iter(),
                        )
                    {
                        format!("R8H")
                    } else {
                        format!("R8Some{}", some.len())
                    }
                }
                RegisterType::AllFloat => "ST".to_string(),
                RegisterType::SingleFloat(float_reg) => match float_reg {
                    RegFloat::ST0 => "ST0".to_string(),
                    RegFloat::ST1 => "ST1".to_string(),
                    RegFloat::ST2 => "ST2".to_string(),
                    RegFloat::ST3 => "ST3".to_string(),
                    RegFloat::ST4 => "ST4".to_string(),
                    RegFloat::ST5 => "ST5".to_string(),
                    RegFloat::ST6 => "ST6".to_string(),
                    RegFloat::ST7 => "ST7".to_string(),
                },
                RegisterType::AllBnd => "BND".to_string(),
                RegisterType::AllSegment => "SEG".to_string(),
                RegisterType::SomeSegment(_) => "SEG".to_string(),
                RegisterType::AllDebug => "DR".to_string(),
                RegisterType::SomeControl(_) => "CR".to_string(),
                RegisterType::SomeControlExtra(_) => "CR".to_string(),
                RegisterType::SingleSegmentBase(segment) => match segment {
                    RegSegmentBase::FSBase => "FSBASE".to_string(),
                    RegSegmentBase::GSBase => "GSBASE".to_string(),
                },
                RegisterType::SingleFloatControl(reg) => match reg {
                    RegFloatControl::X87CONTROL => "X87CONTROL".to_string(),
                    RegFloatControl::X87STATUS => "X87STATUS".to_string(),
                    RegFloatControl::X87TAG => "X87TAG".to_string(),
                    RegFloatControl::X87POP => "X87POP".to_string(),
                    RegFloatControl::X87PUSH => "X87PUSH".to_string(),
                    RegFloatControl::X87POP2 => "X87POP2".to_string(),
                },
                RegisterType::SingleSegment(segment) => match segment {
                    RegSegment::CS => "CS".to_string(),
                    RegSegment::DS => "DS".to_string(),
                    RegSegment::SS => "SS".to_string(),
                    RegSegment::ES => "ES".to_string(),
                    RegSegment::FS => "FS".to_string(),
                    RegSegment::GS => "GS".to_string(),
                },
                RegisterType::SingleGP64(reg64) => match reg64 {
                    Reg64WithRIP::RAX => "RAX".to_string(),
                    Reg64WithRIP::RBX => "RBX".to_string(),
                    Reg64WithRIP::RCX => "RCX".to_string(),
                    Reg64WithRIP::RDX => "RDX".to_string(),
                    Reg64WithRIP::RSI => "RSI".to_string(),
                    Reg64WithRIP::RDI => "RDI".to_string(),
                    Reg64WithRIP::RBP => "RBP".to_string(),
                    Reg64WithRIP::RSP => "RSP".to_string(),
                    Reg64WithRIP::R8 => "R8".to_string(),
                    Reg64WithRIP::R9 => "R9".to_string(),
                    Reg64WithRIP::R10 => "R10".to_string(),
                    Reg64WithRIP::R11 => "R11".to_string(),
                    Reg64WithRIP::R12 => "R12".to_string(),
                    Reg64WithRIP::R13 => "R13".to_string(),
                    Reg64WithRIP::R14 => "R14".to_string(),
                    Reg64WithRIP::R15 => "R15".to_string(),
                    Reg64WithRIP::RIP => "RIP".to_string(),
                },
                RegisterType::SingleGP32(reg32) => match reg32 {
                    Reg32WithRIP::EAX => "EAX".to_string(),
                    Reg32WithRIP::EBX => "EBX".to_string(),
                    Reg32WithRIP::ECX => "ECX".to_string(),
                    Reg32WithRIP::EDX => "EDX".to_string(),
                    Reg32WithRIP::ESI => "ESI".to_string(),
                    Reg32WithRIP::EDI => "EDI".to_string(),
                    Reg32WithRIP::EBP => "EBP".to_string(),
                    Reg32WithRIP::ESP => "ESP".to_string(),
                    Reg32WithRIP::R8D => "R8D".to_string(),
                    Reg32WithRIP::R9D => "R9D".to_string(),
                    Reg32WithRIP::R10D => "R10D".to_string(),
                    Reg32WithRIP::R11D => "R11D".to_string(),
                    Reg32WithRIP::R12D => "R12D".to_string(),
                    Reg32WithRIP::R13D => "R13D".to_string(),
                    Reg32WithRIP::R14D => "R14D".to_string(),
                    Reg32WithRIP::R15D => "R15D".to_string(),
                    Reg32WithRIP::EIP => "EIP".to_string(),
                },
                RegisterType::SingleGP16(reg16) => match reg16 {
                    Reg16WithRIP::AX => "AX".to_string(),
                    Reg16WithRIP::BX => "BX".to_string(),
                    Reg16WithRIP::CX => "CX".to_string(),
                    Reg16WithRIP::DX => "DX".to_string(),
                    Reg16WithRIP::SI => "SI".to_string(),
                    Reg16WithRIP::DI => "DI".to_string(),
                    Reg16WithRIP::BP => "BP".to_string(),
                    Reg16WithRIP::SP => "SP".to_string(),
                    Reg16WithRIP::R8W => "R8W".to_string(),
                    Reg16WithRIP::R9W => "R9W".to_string(),
                    Reg16WithRIP::R10W => "R10W".to_string(),
                    Reg16WithRIP::R11W => "R11W".to_string(),
                    Reg16WithRIP::R12W => "R12W".to_string(),
                    Reg16WithRIP::R13W => "R13W".to_string(),
                    Reg16WithRIP::R14W => "R14W".to_string(),
                    Reg16WithRIP::R15W => "R15W".to_string(),
                    Reg16WithRIP::IP => "IP".to_string(),
                },
                RegisterType::SingleGP8(reg8) => match reg8 {
                    Reg8::AL => "AL".to_string(),
                    Reg8::AH => "AH".to_string(),
                    Reg8::BL => "BL".to_string(),
                    Reg8::BH => "BH".to_string(),
                    Reg8::CL => "CL".to_string(),
                    Reg8::CH => "CH".to_string(),
                    Reg8::DL => "DL".to_string(),
                    Reg8::DH => "DH".to_string(),
                    Reg8::SIL => "SIL".to_string(),
                    Reg8::DIL => "DIL".to_string(),
                    Reg8::BPL => "BPL".to_string(),
                    Reg8::SPL => "SPL".to_string(),
                    Reg8::R8B => "R8B".to_string(),
                    Reg8::R9B => "R9B".to_string(),
                    Reg8::R10B => "R10B".to_string(),
                    Reg8::R11B => "R11B".to_string(),
                    Reg8::R12B => "R12B".to_string(),
                    Reg8::R13B => "R13B".to_string(),
                    Reg8::R14B => "R14B".to_string(),
                    Reg8::R15B => "R15B".to_string(),
                },
                RegisterType::SingleXmm(xmm) => match xmm {
                    RegXMM::XMM0 => "XMM0".to_string(),
                    RegXMM::XMM1 => "XMM1".to_string(),
                    RegXMM::XMM2 => "XMM2".to_string(),
                    RegXMM::XMM3 => "XMM3".to_string(),
                    RegXMM::XMM4 => "XMM4".to_string(),
                    RegXMM::XMM5 => "XMM5".to_string(),
                    RegXMM::XMM6 => "XMM6".to_string(),
                    RegXMM::XMM7 => "XMM7".to_string(),
                    RegXMM::XMM8 => "XMM8".to_string(),
                    RegXMM::XMM9 => "XMM9".to_string(),
                    RegXMM::XMM10 => "XMM10".to_string(),
                    RegXMM::XMM11 => "XMM11".to_string(),
                    RegXMM::XMM12 => "XMM12".to_string(),
                    RegXMM::XMM13 => "XMM13".to_string(),
                    RegXMM::XMM14 => "XMM14".to_string(),
                    RegXMM::XMM15 => "XMM15".to_string(),
                    RegXMM::XMM16 => "XMM16".to_string(),
                    RegXMM::XMM17 => "XMM17".to_string(),
                    RegXMM::XMM18 => "XMM18".to_string(),
                    RegXMM::XMM19 => "XMM19".to_string(),
                    RegXMM::XMM20 => "XMM20".to_string(),
                    RegXMM::XMM21 => "XMM21".to_string(),
                    RegXMM::XMM22 => "XMM22".to_string(),
                    RegXMM::XMM23 => "XMM23".to_string(),
                    RegXMM::XMM24 => "XMM24".to_string(),
                    RegXMM::XMM25 => "XMM25".to_string(),
                    RegXMM::XMM26 => "XMM26".to_string(),
                    RegXMM::XMM27 => "XMM27".to_string(),
                    RegXMM::XMM28 => "XMM28".to_string(),
                    RegXMM::XMM29 => "XMM29".to_string(),
                    RegXMM::XMM30 => "XMM30".to_string(),
                    RegXMM::XMM31 => "XMM31".to_string(),
                    RegXMM::XMM32 => "XMM32".to_string(),
                },
                RegisterType::SingleSpecial(special) => match special {
                    RegSpecial::GDTR => "GDTR".to_string(),
                    RegSpecial::LDTR => "LDTR".to_string(),
                    RegSpecial::IDTR => "IDTR".to_string(),
                    RegSpecial::TR => "TR".to_string(),
                    RegSpecial::TSC => "TSC".to_string(),
                    RegSpecial::TSCAUX => "TSCAUX".to_string(),
                    RegSpecial::MSRS => "MSRS".to_string(),
                    RegSpecial::UIF => "UIF".to_string(),
                    RegSpecial::SSP => "SSP".to_string(),
                },
                RegisterType::SingleControl(control) => match control {
                    RegControl::CR0 => "CR0".to_string(),
                    RegControl::CR1 => "CR1".to_string(),
                    RegControl::CR2 => "CR2".to_string(),
                    RegControl::CR3 => "CR3".to_string(),
                    RegControl::CR4 => "CR4".to_string(),
                    RegControl::CR5 => "CR5".to_string(),
                    RegControl::CR6 => "CR6".to_string(),
                    RegControl::CR7 => "CR7".to_string(),
                    RegControl::CR8 => "CR8".to_string(),
                    RegControl::CR9 => "CR9".to_string(),
                    RegControl::CR10 => "CR10".to_string(),
                    RegControl::CR11 => "CR11".to_string(),
                    RegControl::CR12 => "CR12".to_string(),
                    RegControl::CR13 => "CR13".to_string(),
                    RegControl::CR14 => "CR14".to_string(),
                    RegControl::CR15 => "CR15".to_string(),
                },
                RegisterType::Multiple(_) => {
                    todo!()
                }
                RegisterType::AllControl => {
                    todo!()
                }
            },
            OperandType::Mem(mem) => {
                (match mem.kind {
                    MemoryOperandTypeKind::MemTile => "MemTile".to_string(),
                    MemoryOperandTypeKind::MemStruct => "MemStruct".to_string(),
                    MemoryOperandTypeKind::Mem512 => "Mem512".to_string(),
                    MemoryOperandTypeKind::Mem384 => "Mem384".to_string(),
                    MemoryOperandTypeKind::Mem320 => "Mem320".to_string(),
                    MemoryOperandTypeKind::Mem256 => "Mem256".to_string(),
                    MemoryOperandTypeKind::Mem192 => "Mem192".to_string(),
                    MemoryOperandTypeKind::Mem128 => "Mem128".to_string(),
                    MemoryOperandTypeKind::Mem160 => "Mem160".to_string(),
                    MemoryOperandTypeKind::Mem80 => "Mem80".to_string(),
                    MemoryOperandTypeKind::Mem64 => "Mem64".to_string(),
                    MemoryOperandTypeKind::Mem48 => "Mem48".to_string(),
                    MemoryOperandTypeKind::Mem32 => "Mem32".to_string(),
                    MemoryOperandTypeKind::Mem16 => "Mem16".to_string(),
                    MemoryOperandTypeKind::Mem8 => "Mem8".to_string(),
                }) + (match &mem.vsib {
                    None => "",
                    Some(VectorRegisterKind::XMM) => "VSIBXMM",
                    Some(VectorRegisterKind::YMM) => "VSIBYMM",
                    Some(VectorRegisterKind::ZMM) => "VSIBZMM",
                })
            }
            OperandType::Imm(imm) => match imm {
                Imm::Imm8 => "Imm8".to_string(),
                Imm::Imm16 => "Imm16".to_string(),
                Imm::Imm32 => "Imm32".to_string(),
                Imm::Imm64 => "Imm64".to_string(),
            },
            OperandType::ImmSpecific(imm) => {
                format!("Imm{imm}")
            }
            OperandType::Flags(_) => "Flags".to_string(),
            OperandType::Agen(_) => "Agen".to_string(),
            OperandType::Rel8 => "Rel8".to_string(),
            OperandType::Rel16 => "Rel16".to_string(),
            OperandType::Rel32 => "Rel32".to_string(),
        }
    }
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum VectorRegisterKind {
    XMM,
    YMM,
    ZMM,
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct MemoryOperandType {
    pub vsib: Option<VectorRegisterKind>,
    pub kind: MemoryOperandTypeKind,
    pub load: bool,
    pub store: bool,
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum MemoryOperandTypeKind {
    MemTile,
    MemStruct,
    Mem512,
    Mem384,
    Mem320,
    Mem256,
    Mem192,
    Mem128,
    Mem160,
    Mem80,
    Mem64,
    Mem48,
    Mem32,
    Mem16,
    Mem8,
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct Flags;

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct Agen;

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum Imm {
    Imm8,
    Imm16,
    Imm32,
    Imm64,
}
