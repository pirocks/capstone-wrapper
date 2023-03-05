use std::collections::HashSet;
use std::fmt::Debug;
use std::hash::Hash;
use std::iter;

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::operand_index::OperandIndexError;
use crate::registers::{Reg16WithRIP, Reg32WithRIP, Reg64WithRIP, Reg8, RegControl, RegControlExtra, RegFloat, RegFloatControl, RegisterType, RegMask, RegSegment, RegSegmentBase, RegSpecial, RegXMM, RegZMM};

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
    UnknownRegister {
        val: String
    },
    #[error("Unknown imm val: {val}")]
    UnknownImm {
        val: String
    },
    #[error("Missing xml val field")]
    MissingVal,
    #[error("Relative operand width unexpected: {width:?}")]
    UnexpectedWidthRel {
        width: Option<String>
    },
    #[error("Unexpected type: {r#type}")]
    UnexpectedType {
        r#type: String
    },
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

fn single_set<T: Eq + PartialEq + Hash + Debug>(t: T) -> HashSet<T> {
    HashSet::from_iter(iter::once(t))
}


impl OperandType {
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
            "TR" => RegisterType::SomeSpecial(single_set(RegSpecial::TR)),
            "CR0" => RegisterType::SomeControl(single_set(RegControl::CR0)),
            "XCR0" => RegisterType::SomeControlExtra(single_set(RegControlExtra::XCR0)),
            "GDTR" => RegisterType::SomeSpecial(single_set(RegSpecial::GDTR)),
            "IDTR" => RegisterType::SomeSpecial(single_set(RegSpecial::IDTR)),
            "LDTR" => RegisterType::SomeSpecial(single_set(RegSpecial::LDTR)),
            "MXCSR" => RegisterType::SomeControlExtra(single_set(RegControlExtra::MXCSR)),
            "MSRS" => RegisterType::SomeSpecial(single_set(RegSpecial::MSRS)),
            "TSC" => RegisterType::SomeSpecial(single_set(RegSpecial::TSC)),
            "TSCAUX" => RegisterType::SomeSpecial(single_set(RegSpecial::TSCAUX)),
            "X87CONTROL" => RegisterType::SomeFloatControl(single_set(RegFloatControl::X87CONTROL)),
            "X87STATUS" => RegisterType::SomeFloatControl(single_set(RegFloatControl::X87STATUS)),
            "X87TAG" => RegisterType::SomeFloatControl(single_set(RegFloatControl::X87TAG)),
            "X87POP" => RegisterType::SomeFloatControl(single_set(RegFloatControl::X87POP)),
            "X87PUSH" => RegisterType::SomeFloatControl(single_set(RegFloatControl::X87PUSH)),
            "X87POP2" => RegisterType::SomeFloatControl(single_set(RegFloatControl::X87POP2)),
            "SSP" => RegisterType::SomeSpecial(single_set(RegSpecial::SSP)),
            "ST(0)" => RegisterType::SomeFloat(single_set(RegFloat::ST0)),
            "ST(1)" => RegisterType::SomeFloat(single_set(RegFloat::ST1)),
            "XMM0" => RegisterType::SomeXmm(single_set(RegXMM::XMM0)),
            "XMM1" => RegisterType::SomeXmm(single_set(RegXMM::XMM1)),
            "XMM2" => RegisterType::SomeXmm(single_set(RegXMM::XMM2)),
            "XMM3" => RegisterType::SomeXmm(single_set(RegXMM::XMM3)),
            "XMM4" => RegisterType::SomeXmm(single_set(RegXMM::XMM4)),
            "XMM5" => RegisterType::SomeXmm(single_set(RegXMM::XMM5)),
            "XMM6" => RegisterType::SomeXmm(single_set(RegXMM::XMM6)),
            "XMM7" => RegisterType::SomeXmm(single_set(RegXMM::XMM7)),
            "BND0,BND1,BND2,BND3" => RegisterType::AllBnd,
            "UIF" => RegisterType::SomeSpecial(HashSet::from_iter(iter::once(RegSpecial::UIF))),
            "EAX,ECX,EDX,EBX,ESP,EBP,ESI,EDI,R8D,R9D,R10D,R11D,R12D,R13D,R14D,R15D" => RegisterType::AllGP32WithoutRIP,
            "AX,CX,DX,BX,SP,BP,SI,DI,R8W,R9W,R10W,R11W,R12W,R13W,R14W,R15W" => RegisterType::AllGP16WithoutRIP,
            "AL,CL,DL,BL,SPL,BPL,SIL,DIL,R8B,R9B,R10B,R11B,R12B,R13B,R14B,R15B" => RegisterType::AllGP8,
            "AL,CL,DL,BL,R8B,R9B,R10B,R11B" => RegisterType::SomeGP8(HashSet::from_iter(vec![Reg8::AL, Reg8::CL, Reg8::DL, Reg8::BL, Reg8::R8B, Reg8::R9B, Reg8::R10B, Reg8::R11B].into_iter())),
            "AH,CH,DH,BH" => RegisterType::SomeGP8(HashSet::from_iter(vec![Reg8::AH, Reg8::CH, Reg8::DH, Reg8::BH].into_iter())),
            "AL,CL,DL,BL" => RegisterType::SomeGP8(HashSet::from_iter(vec![Reg8::AL, Reg8::CL, Reg8::DL, Reg8::BL].into_iter())),
            "ES,SS,DS,FS,GS" => RegisterType::SomeSegment(HashSet::from_iter(vec![RegSegment::ES, RegSegment::SS, RegSegment::DS, RegSegment::FS, RegSegment::GS].into_iter())),
            "ES,CS,SS,DS,FS,GS" => RegisterType::AllSegment,
            "CR0,CR2,CR3,CR4,CR8" => RegisterType::SomeControl(HashSet::from_iter(vec![RegControl::CR0, RegControl::CR2, RegControl::CR3, RegControl::CR4, RegControl::CR8].into_iter())),
            "DR0,DR1,DR2,DR3,DR4,DR5,DR6,DR7" => RegisterType::AllDebug,
            "AX,CX,DX,BX,SP,BP,SI,DI" => RegisterType::SomeGP16(HashSet::from_iter(vec![Reg16WithRIP::AX, Reg16WithRIP::CX, Reg16WithRIP::DX, Reg16WithRIP::BX, Reg16WithRIP::SP, Reg16WithRIP::BP, Reg16WithRIP::SI, Reg16WithRIP::DI].into_iter())),
            "EAX,ECX,EDX,EBX,ESP,EBP,ESI,EDI" => RegisterType::SomeGP32(HashSet::from_iter(vec![Reg32WithRIP::EAX, Reg32WithRIP::ECX, Reg32WithRIP::EDX, Reg32WithRIP::EDX, Reg32WithRIP::ESP, Reg32WithRIP::EBP, Reg32WithRIP::ESI, Reg32WithRIP::EDI])),
            "SPL,BPL,SIL,DIL,R12B,R13B,R14B,R15B" => RegisterType::SomeGP8(HashSet::from_iter(vec![Reg8::SPL, Reg8::BPL, Reg8::SIL, Reg8::DIL, Reg8::R12B, Reg8::R13B, Reg8::R14B, Reg8::R15B].into_iter())),
            "MM0,MM1,MM2,MM3,MM4,MM5,MM6,MM7" => RegisterType::AllMmx,
            "TMM0,TMM1,TMM2,TMM3,TMM4,TMM5,TMM6,TMM7" => RegisterType::AllTmm,
            "RAX,RCX,RDX,RBX,RSP,RBP,RSI,RDI,R8,R9,R10,R11,R12,R13,R14,R15" => RegisterType::AllGP64WithoutRIP,
            "XMM0,XMM1,XMM2,XMM3,XMM4,XMM5,XMM6,XMM7,XMM8,XMM9,XMM10,XMM11,XMM12,XMM13,XMM14,XMM15" => RegisterType::AllXmm16,
            "YMM0,YMM1,YMM2,YMM3,YMM4,YMM5,YMM6,YMM7,YMM8,YMM9,YMM10,YMM11,YMM12,YMM13,YMM14,YMM15" => RegisterType::AllYmm16,
            "ZMM0,ZMM1,ZMM2,ZMM3,ZMM4,ZMM5,ZMM6,ZMM7,ZMM8,ZMM9,ZMM10,ZMM11,ZMM12,ZMM13,ZMM14,ZMM15,ZMM16,ZMM17,ZMM18,ZMM19,ZMM20,ZMM21,ZMM22,ZMM23,ZMM24,ZMM25,ZMM26,ZMM27,ZMM28,ZMM29,ZMM30,ZMM31" => RegisterType::AllZmm32,
            "YMM0,YMM1,YMM2,YMM3,YMM4,YMM5,YMM6,YMM7,YMM8,YMM9,YMM10,YMM11,YMM12,YMM13,YMM14,YMM15,YMM16,YMM17,YMM18,YMM19,YMM20,YMM21,YMM22,YMM23,YMM24,YMM25,YMM26,YMM27,YMM28,YMM29,YMM30,YMM31" => RegisterType::AllYmm32,
            "XMM0,XMM1,XMM2,XMM3,XMM4,XMM5,XMM6,XMM7,XMM8,XMM9,XMM10,XMM11,XMM12,XMM13,XMM14,XMM15,XMM16,XMM17,XMM18,XMM19,XMM20,XMM21,XMM22,XMM23,XMM24,XMM25,XMM26,XMM27,XMM28,XMM29,XMM30,XMM31" => RegisterType::AllXmm32,
            "ZMM0,ZMM4,ZMM8,ZMM12,ZMM16,ZMM20,ZMM24,ZMM28" => RegisterType::SomeZmm(HashSet::from_iter(vec![RegZMM::ZMM0, RegZMM::ZMM4, RegZMM::ZMM8, RegZMM::ZMM12, RegZMM::ZMM16, RegZMM::ZMM20, RegZMM::ZMM24, RegZMM::ZMM28].into_iter())),
            "XMM0,XMM4,XMM8,XMM12,XMM16,XMM20,XMM24,XMM28" => RegisterType::SomeXmm(HashSet::from_iter(vec![RegXMM::XMM0, RegXMM::XMM4, RegXMM::XMM8, RegXMM::XMM12, RegXMM::XMM16, RegXMM::XMM20, RegXMM::XMM24, RegXMM::XMM28].into_iter())),
            "ST(0),ST(1),ST(2),ST(3),ST(4),ST(5),ST(6),ST(7)" => RegisterType::AllFloat,
            "K1,K2,K3,K4,K5,K6,K7" => RegisterType::SomeMask(HashSet::from_iter(vec![RegMask::K0, RegMask::K1, RegMask::K2, RegMask::K3, RegMask::K4, RegMask::K5, RegMask::K6, RegMask::K7].into_iter())),
            "K0,K1,K2,K3,K4,K5,K6,K7" => RegisterType::AllMask,
            "K7,K6,K5,K4,K3,K2,K1,K0" => RegisterType::AllMask,
            "K0,K2,K4,K6" => RegisterType::SomeMask(HashSet::from_iter(vec![RegMask::K0, RegMask::K2, RegMask::K4, RegMask::K6].into_iter())),
            val => return Err(OperandFromStr::UnknownRegister { val: val.to_string() })
        }))
    }

    pub fn from_mem(
        xtype: Option<impl AsRef<str>>,
        memory_prefix: Option<impl AsRef<str>>,
        width: Option<impl AsRef<str>>,
    ) -> Result<OperandType, OperandFromStr> {
        Ok(OperandType::Mem(match (xtype.as_ref().map(|s| s.as_ref()), memory_prefix.as_ref().map(|s| s.as_ref()), width.as_ref().map(|s| s.as_ref())) {
            (Some("struct"), _, Some("80")) => MemoryOperandType::Mem80,
            (Some("struct"), _, Some("64")) => MemoryOperandType::Mem64,
            (Some("struct"), _, Some("48")) => MemoryOperandType::Mem48,
            (Some("struct"), _, Some("32")) => MemoryOperandType::Mem32,
            (Some("struct"), None, _) => MemoryOperandType::MemStruct,
            (Some("struct"), Some("zmmword ptr"), _) => MemoryOperandType::MemStruct,
            (_, Some("zmmword ptr"), _) => MemoryOperandType::Mem512,
            (_, Some("ymmword ptr"), _) => MemoryOperandType::Mem256,
            (_, Some("xmmword ptr"), _) => MemoryOperandType::Mem128,
            (_, Some("tbyte ptr"), _) => MemoryOperandType::Mem80,
            (_, Some("qword ptr"), _) => MemoryOperandType::Mem64,
            (_, Some("dword ptr"), _) => MemoryOperandType::Mem32,
            (_, Some("word ptr"), _) => MemoryOperandType::Mem16,
            (_, Some("byte ptr"), _) => MemoryOperandType::Mem8,
            (_, _, Some("512")) => MemoryOperandType::Mem512,
            (_, _, Some("384")) => MemoryOperandType::Mem384,
            (_, _, Some("320")) => MemoryOperandType::Mem320,
            (_, _, Some("256")) => MemoryOperandType::Mem256,
            (_, _, Some("192")) => MemoryOperandType::Mem192,
            (_, _, Some("160")) => MemoryOperandType::Mem160,
            (_, _, Some("128")) => MemoryOperandType::Mem128,
            (_, _, Some("80")) => MemoryOperandType::Mem80,
            (_, _, Some("64")) => MemoryOperandType::Mem64,
            (_, _, Some("32")) => MemoryOperandType::Mem32,
            (_, _, Some("16")) => MemoryOperandType::Mem16,
            (_, _, Some("8")) => MemoryOperandType::Mem8,
            _ => {
                if xtype.as_ref().map(|s| s.as_ref()) == Some("u32") {
                    MemoryOperandType::MemTile
                } else {
                    return Err(OperandFromStr::UnknownMemoryOperand);
                }
            }
        }))
    }

    pub fn new(
        r#type: &String,
        xtype: Option<impl AsRef<str>>,
        val: Option<impl AsRef<str>>,
        memory_prefix: Option<impl AsRef<str>>,
        width: Option<impl AsRef<str>>,
    ) -> Result<OperandType, OperandFromStr> {
        match r#type.as_str() {
            "mem" => {
                Self::from_mem(xtype, memory_prefix, width)
            }
            "reg" => {
                match val {
                    None => {
                        Err(OperandFromStr::MissingVal.into())
                    }
                    Some(val) => {
                        Self::from_reg(val)
                    }
                }
            }
            "imm" => {
                Ok(match val.as_ref().map(|s| s.as_ref()) {
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
                    Some(val) => return Err(OperandFromStr::UnknownImm { val: val.to_string() })
                })
            }
            "flags" => {
                Ok(OperandType::Flags(Flags {}))
            }
            "agen" => {
                Ok(OperandType::Agen(Agen {}))
            }
            "relbr" => {
                Ok(match width.as_ref().map(|s| s.as_ref()) {
                    Some("32") => OperandType::Rel32,
                    Some("16") => OperandType::Rel16,
                    Some("8") => OperandType::Rel8,
                    width => {
                        return Err(OperandFromStr::UnexpectedWidthRel { width: width.map(|s| s.to_string()) }.into());
                    }
                })
            }
            r#type => {
                Err(OperandFromStr::UnexpectedType { r#type: r#type.to_string() }.into())
            }
        }
    }

    pub fn to_identifier_string(&self) -> String {
        match self {
            OperandType::Reg(reg) => {
                match reg {
                    RegisterType::AllMmx => "MMX".to_string(),
                    RegisterType::SomeMmx(_) => "MMX".to_string(),
                    RegisterType::AllXmm16 => "XMM".to_string(),
                    RegisterType::AllXmm32 => "XMM".to_string(),
                    RegisterType::SomeXmm(_) => "XMM".to_string(),
                    RegisterType::AllYmm16 => "YMM".to_string(),
                    RegisterType::AllYmm32 => "YMM".to_string(),
                    RegisterType::SomeYmm(_) => "YMM".to_string(),
                    RegisterType::AllZmm32 => "ZMM".to_string(),
                    RegisterType::SomeZmm(_) => "ZMM".to_string(),
                    RegisterType::AllTmm => "TMM".to_string(),
                    RegisterType::SomeTmm(_) => "TMM".to_string(),
                    RegisterType::AllMask => "K".to_string(),
                    RegisterType::SomeMask(_) => "K".to_string(),
                    RegisterType::AllGP64WithoutRIP => "R64".to_string(),
                    RegisterType::AllGP64WithRIP => "R64".to_string(),
                    RegisterType::SomeGP64(_) => "R64".to_string(),
                    RegisterType::AllGP32WithoutRIP => "R32".to_string(),
                    RegisterType::AllGP32WithRIP => "R32".to_string(),
                    RegisterType::SomeGP32(_) => "R32".to_string(),
                    RegisterType::AllGP16WithoutRIP => "R16".to_string(),
                    RegisterType::AllGP16WithRIP => "R16".to_string(),
                    RegisterType::SomeGP16(_) => "R16".to_string(),
                    RegisterType::AllGP8 => "R8".to_string(),
                    RegisterType::SomeGP8(some) => {
                        if some == &HashSet::from_iter(vec![Reg8::AL, Reg8::BL, Reg8::CL, Reg8::DL].into_iter()) {
                            format!("R8L")
                        } else if some == &HashSet::from_iter(vec![Reg8::AH, Reg8::BH, Reg8::CH, Reg8::DH].into_iter()) {
                            format!("R8H")
                        } else {
                            format!("R8Some{}", some.len())
                        }
                    }
                    RegisterType::AllFloat => "ST".to_string(),
                    RegisterType::SomeFloat(_) => "ST".to_string(),
                    RegisterType::AllBnd => "BND".to_string(),
                    RegisterType::SomeBnd(_) => "BND".to_string(),
                    RegisterType::AllSegment => "SEG".to_string(),
                    RegisterType::SomeSegment(_) => "SEG".to_string(),
                    RegisterType::AllDebug => "DR".to_string(),
                    RegisterType::SomeDebug(_) => "DR".to_string(),
                    RegisterType::AllControlRegisters => "CR".to_string(),
                    RegisterType::SomeControl(_) => "CR".to_string(),
                    RegisterType::SomeControlExtra(_) => "CR".to_string(),
                    RegisterType::SingleSegmentBase(segment) => match segment {
                        RegSegmentBase::FSBase => {
                            "FSBASE".to_string()
                        }
                        RegSegmentBase::GSBase => {
                            "GSBASE".to_string()
                        }
                    },
                    RegisterType::SomeSpecial(_) => "SPECIAL".to_string(),
                    RegisterType::SomeFloatControl(_) => "FPCONTROL".to_string(),
                    RegisterType::SingleSegment(segment) => {
                        match segment {
                            RegSegment::CS => {
                                "CS".to_string()
                            }
                            RegSegment::DS => {
                                "DS".to_string()
                            }
                            RegSegment::SS => {
                                "SS".to_string()
                            }
                            RegSegment::ES => {
                                "ES".to_string()
                            }
                            RegSegment::FS => {
                                "FS".to_string()
                            }
                            RegSegment::GS => {
                                "GS".to_string()
                            }
                        }
                    }
                    RegisterType::SingleGP64(_) => {
                        todo!()
                    }
                    RegisterType::SingleGP32(_) => {
                        todo!()
                    }
                    RegisterType::SingleGP16(reg16) => {
                        match reg16 {
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
                        }
                    }
                    RegisterType::SingleGP8(reg8) => {
                        match reg8 {
                            Reg8::AL => {
                                "AL".to_string()
                            }
                            Reg8::AH => {
                                "AH".to_string()
                            }
                            Reg8::BL => {
                                "BL".to_string()
                            }
                            Reg8::BH => {
                                "BH".to_string()
                            }
                            Reg8::CL => {
                                "CL".to_string()
                            }
                            Reg8::CH => {
                                "CH".to_string()
                            }
                            Reg8::DL => {
                                "DL".to_string()
                            }
                            Reg8::DH => {
                                "DH".to_string()
                            }
                            Reg8::SIL => {
                                "SIL".to_string()
                            }
                            Reg8::DIL => {
                                "DIL".to_string()
                            }
                            Reg8::BPL => {
                                "BPL".to_string()
                            }
                            Reg8::SPL => {
                                "SPL".to_string()
                            }
                            Reg8::R8B => {
                                "R8B".to_string()
                            }
                            Reg8::R9B => {
                                "R9B".to_string()
                            }
                            Reg8::R10B => {
                                "R10B".to_string()
                            }
                            Reg8::R11B => {
                                "R11B".to_string()
                            }
                            Reg8::R12B => {
                                "R12B".to_string()
                            }
                            Reg8::R13B => {
                                "R13B".to_string()
                            }
                            Reg8::R14B => {
                                "R14B".to_string()
                            }
                            Reg8::R15B => {
                                "R15B".to_string()
                            }
                        }
                    }
                }
            }
            OperandType::Mem(mem) => {
                match mem {
                    MemoryOperandType::MemTile => "MemTile".to_string(),
                    MemoryOperandType::MemStruct => "MemStruct".to_string(),
                    MemoryOperandType::Mem512 => "Mem512".to_string(),
                    MemoryOperandType::Mem384 => "Mem384".to_string(),
                    MemoryOperandType::Mem320 => "Mem320".to_string(),
                    MemoryOperandType::Mem256 => "Mem256".to_string(),
                    MemoryOperandType::Mem192 => "Mem192".to_string(),
                    MemoryOperandType::Mem128 => "Mem128".to_string(),
                    MemoryOperandType::Mem160 => "Mem160".to_string(),
                    MemoryOperandType::Mem80 => "Mem80".to_string(),
                    MemoryOperandType::Mem64 => "Mem64".to_string(),
                    MemoryOperandType::Mem48 => "Mem48".to_string(),
                    MemoryOperandType::Mem32 => "Mem32".to_string(),
                    MemoryOperandType::Mem16 => "Mem16".to_string(),
                    MemoryOperandType::Mem8 => "Mem8".to_string(),
                }
            }
            OperandType::Imm(imm) => {
                match imm {
                    Imm::Imm8 => {
                        "Imm8".to_string()
                    }
                    Imm::Imm16 => {
                        "Imm16".to_string()
                    }
                    Imm::Imm32 => {
                        "Imm32".to_string()
                    }
                    Imm::Imm64 => {
                        "Imm64".to_string()
                    }
                }
            }
            OperandType::ImmSpecific(imm) => {
                format!("Imm{imm}")
            }
            OperandType::Flags(_) => {
                "Flags".to_string()
            }
            OperandType::Agen(_) => {
                "Agen".to_string()
            }
            OperandType::Rel8 => {
                "Rel8".to_string()
            }
            OperandType::Rel16 => {
                "Rel16".to_string()
            }
            OperandType::Rel32 => {
                "Rel32".to_string()
            }
        }
    }
}


#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum MemoryOperandType {
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
