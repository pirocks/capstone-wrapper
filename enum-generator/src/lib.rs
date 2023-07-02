extern crate proc_macro;

use proc_macro::TokenStream;
use std::collections::HashMap;
use std::io::Cursor;
use std::num::NonZeroU8;

use string_concat_utils::concat_camel_case;
use wrapper_common::instructions::{Instruction, InstructionEncoding, InstructionName, Instructions};
use wrapper_common::operand_index::OperandIndex;
use wrapper_common::operand_type::{Imm, MemoryOperandType, MemoryOperandTypeKind, OperandType, VectorRegisterKind};
use wrapper_common::registers::RegisterType;

#[derive(Debug)]
struct VariantNamePretty(String);

impl VariantNamePretty {
    pub fn new(encoding: &InstructionEncoding) -> Self {
        let mut operand_types = vec![];
        let InstructionEncoding { bcast, mode_prefix_string, operands } = encoding;
        let mut variant_name_components = vec![mode_prefix_string.to_string()];
        if let Some(bcast) = bcast {
            variant_name_components.push(format!("BCast{}", bcast.get()));
        };

        if operands.len() == 0 {
            variant_name_components.push("NoOperand".to_string());
        } else {
            for i in 0..operands.len() {
                let operand = OperandIndex(NonZeroU8::new((i + 1) as u8).unwrap());
                operand_types.push(operands.get(&operand).unwrap());
            }
            for operand_type in operand_types {
                variant_name_components.push(operand_type.to_identifier_string());
            }
        }
        VariantNamePretty(concat_camel_case(variant_name_components))
    }
}

struct InstructionNamePretty(String);

impl InstructionNamePretty {
    pub fn new(instruction_name: &InstructionName) -> Self {
        InstructionNamePretty(instruction_name.0.replace("{evex} ", "vex").replace("REPE ", "repe"))
    }
}

#[derive(Debug)]
struct VariantEncodingGeneratorData {
    bcast: Option<NonZeroU8>,
    mode_prefix_string: String,
    operands: HashMap<OperandIndex, OperandType>,
    operand_names: HashMap<OperandIndex, String>,
    operand_type_names: HashMap<OperandIndex, String>,
}

impl VariantEncodingGeneratorData {
    fn operand_to_name(idx: &OperandIndex, op_type: &OperandType) -> Option<String> {
        Some(match op_type {
            OperandType::Reg(reg) => {
                match reg {
                    RegisterType::AllMmx => {
                        format!("mmx_{}", idx.0)
                    }
                    RegisterType::SomeXmm(_) |
                    RegisterType::AllXmm16 |
                    RegisterType::AllXmm32 => {
                        format!("xmm_{}", idx.0)
                    }
                    RegisterType::AllYmm16 |
                    RegisterType::AllYmm32 => {
                        format!("ymm_{}", idx.0)
                    }
                    RegisterType::SomeZmm(_) |
                    RegisterType::AllZmm32 => {
                        format!("zmm_{}", idx.0)
                    }
                    RegisterType::AllTmm => {
                        format!("tmm_{}", idx.0)
                    }
                    RegisterType::AllMask |
                    RegisterType::SomeMask(_) => {
                        format!("mask_{}", idx.0)
                    }
                    RegisterType::AllGP64WithoutRIP => {
                        format!("gp64_{}", idx.0)
                    }
                    RegisterType::AllGP64WithRIP => {
                        format!("gp64_{}", idx.0)
                    }
                    RegisterType::AllGP32WithoutRIP => {
                        format!("gp32_{}", idx.0)
                    }
                    RegisterType::SomeGP32(_) |
                    RegisterType::AllGP32WithRIP => {
                        format!("gp32_{}", idx.0)
                    }
                    RegisterType::AllGP16WithoutRIP => {
                        format!("gp16_{}", idx.0)
                    }
                    RegisterType::SomeGP16(_) |
                    RegisterType::AllGP16WithRIP => {
                        format!("gp16_{}", idx.0)
                    }
                    RegisterType::AllGP8 => {
                        format!("gp8_{}", idx.0)
                    }
                    RegisterType::SomeGP8(_) => {
                        format!("gp8_{}", idx.0)
                    }
                    RegisterType::AllFloat => {
                        format!("st_i_{}", idx.0)
                    }
                    RegisterType::AllBnd => {
                        format!("bnd_{}", idx.0)
                    }
                    RegisterType::SomeSegment(_) |
                    RegisterType::AllSegment => {
                        format!("bnd_{}", idx.0)
                    }
                    RegisterType::AllDebug => {
                        format!("dr_{}", idx.0)
                    }
                    RegisterType::SomeControl(_) => {
                        format!("cr_{}", idx.0)
                    }
                    RegisterType::SomeControlExtra(_) => {
                        format!("cr_{}", idx.0)
                    }
                    RegisterType::SingleGP64(_) |
                    RegisterType::SingleSegment(_) |
                    RegisterType::SingleFloat(_) |
                    RegisterType::SingleGP16(_) |
                    RegisterType::SingleGP8(_) |
                    RegisterType::SingleGP32(_) |
                    RegisterType::SingleXmm(_) |
                    RegisterType::SingleSegmentBase(_) |
                    RegisterType::SingleFloatControl(_) |
                    RegisterType::SingleSpecial(_) |
                    RegisterType::SingleControl(_) => {
                        return None;
                    }
                    RegisterType::AllControl |
                    RegisterType::Multiple(_) => {
                        todo!()
                    }
                }
            }
            OperandType::Mem(mem) => {
                let MemoryOperandType { vsib, kind } = mem;
                let vsib = match vsib {
                    None => "",
                    Some(VectorRegisterKind::XMM) => "_vsib_xmm",
                    Some(VectorRegisterKind::YMM) => "_vsib_ymm",
                    Some(VectorRegisterKind::ZMM) => "_vsib_zmm",
                };
                let kind_string = match kind {
                    MemoryOperandTypeKind::MemTile => "MemTile",
                    MemoryOperandTypeKind::MemStruct => "MemStruct",
                    MemoryOperandTypeKind::Mem512 => "Mem512",
                    MemoryOperandTypeKind::Mem384 => "Mem384",
                    MemoryOperandTypeKind::Mem320 => "Mem320",
                    MemoryOperandTypeKind::Mem256 => "Mem256",
                    MemoryOperandTypeKind::Mem192 => "Mem192",
                    MemoryOperandTypeKind::Mem128 => "Mem128",
                    MemoryOperandTypeKind::Mem160 => "Mem160",
                    MemoryOperandTypeKind::Mem80 => "Mem80",
                    MemoryOperandTypeKind::Mem64 => "Mem64",
                    MemoryOperandTypeKind::Mem48 => "Mem48",
                    MemoryOperandTypeKind::Mem32 => "Mem32",
                    MemoryOperandTypeKind::Mem16 => "Mem16",
                    MemoryOperandTypeKind::Mem8 => "Mem8",
                };
                format!("mem_{}{}_{}", kind_string, vsib, idx.0)
            }
            OperandType::Imm(imm) => {
                match imm {
                    Imm::Imm8 => {
                        format!("imm_{}", idx.0)
                    }
                    Imm::Imm16 => {
                        format!("imm_{}", idx.0)
                    }
                    Imm::Imm32 => {
                        format!("imm_{}", idx.0)
                    }
                    Imm::Imm64 => {
                        format!("imm_{}", idx.0)
                    }
                }
            }
            OperandType::ImmSpecific(_) |
            OperandType::Flags(_) |
            OperandType::Agen(_) => {
                return None;
            }
            OperandType::Rel8 => {
                format!("rel8_{}", idx.0)
            }
            OperandType::Rel16 => {
                format!("rel16_{}", idx.0)
            }
            OperandType::Rel32 => {
                format!("rel32_{}", idx.0)
            }
        })
    }

    fn operand_to_type_name(op_type: &OperandType) -> Option<String> {
        Some(match op_type {
            OperandType::Reg(reg) => {
                match reg {
                    RegisterType::AllMmx => {
                        "RegMMX"
                    }
                    RegisterType::SomeXmm(_) |
                    RegisterType::AllXmm16 |
                    RegisterType::AllXmm32 => {
                        "RegXMM"
                    }
                    RegisterType::AllYmm16 |
                    RegisterType::AllYmm32 => {
                        "RegYMM"
                    }
                    RegisterType::SomeZmm(_) |
                    RegisterType::AllZmm32 => {
                        "RegZMM"
                    }
                    RegisterType::AllTmm => {
                        "RegTMM"
                    }
                    RegisterType::AllMask |
                    RegisterType::SomeMask(_) => {
                        "RegMask"
                    }
                    RegisterType::AllGP64WithoutRIP => {
                        "Reg64WithoutRIP"
                    }
                    RegisterType::AllGP64WithRIP => {
                        "Reg64WithRIP"
                    }
                    RegisterType::AllGP32WithoutRIP => {
                        "Reg32WithoutRIP"
                    }
                    RegisterType::SomeGP32(_) |
                    RegisterType::AllGP32WithRIP => {
                        "Reg32WithRIP"
                    }
                    RegisterType::AllGP16WithoutRIP => {
                        "Reg16WithoutRIP"
                    }
                    RegisterType::SomeGP16(_) |
                    RegisterType::AllGP16WithRIP => {
                        "Reg16WithRIP"
                    }
                    RegisterType::AllGP8 => {
                        "Reg8"
                    }
                    RegisterType::SomeGP8(_) => {
                        "Reg8"
                    }
                    RegisterType::AllFloat => {
                        "RegFloat"
                    }
                    RegisterType::AllBnd => {
                        "RegBnd"
                    }
                    RegisterType::SomeSegment(_) |
                    RegisterType::AllSegment => {
                        "RegSegment"
                    }
                    RegisterType::AllDebug => {
                        "RegDebug"
                    }
                    RegisterType::SomeControl(_) => {
                        "RegControl"
                    }
                    RegisterType::SomeControlExtra(_) => {
                        "RegControlExtra"
                    }
                    RegisterType::SingleGP64(_) |
                    RegisterType::SingleSegment(_) |
                    RegisterType::SingleFloat(_) |
                    RegisterType::SingleGP16(_) |
                    RegisterType::SingleGP8(_) |
                    RegisterType::SingleGP32(_) |
                    RegisterType::SingleXmm(_) |
                    RegisterType::SingleSegmentBase(_) |
                    RegisterType::SingleFloatControl(_) |
                    RegisterType::SingleSpecial(_) |
                    RegisterType::SingleControl(_) => {
                        return None;
                    }
                    RegisterType::AllControl |
                    RegisterType::Multiple(_) => {
                        todo!()
                    }
                }
            }
            OperandType::Mem(_) => {
                "MemoryOperand"
            }
            OperandType::Imm(imm) => {
                match imm {
                    Imm::Imm8 => {
                        "i8"
                    }
                    Imm::Imm16 => {
                        "i16"
                    }
                    Imm::Imm32 => {
                        "i32"
                    }
                    Imm::Imm64 => {
                        "i64"
                    }
                }
            }
            OperandType::ImmSpecific(_) |
            OperandType::Flags(_) |
            OperandType::Agen(_) => {
                return None;
            }
            OperandType::Rel8 => {
                "i8"
            }
            OperandType::Rel16 => {
                "i16"
            }
            OperandType::Rel32 => {
                "i32"
            }
        }.to_string())
    }

    pub fn new(encoding_data: &InstructionEncoding) -> Self {
        let InstructionEncoding { bcast, mode_prefix_string, operands } = encoding_data;
        Self {
            bcast: bcast.clone(),
            mode_prefix_string: mode_prefix_string.to_string(),
            operands: operands.clone(),
            operand_names: operands.iter().flat_map(|(idx, op_type)| {
                Some((*idx, Self::operand_to_name(idx, op_type)?))
            }).collect(),
            operand_type_names: operands.iter().flat_map(|(idx, op_type)| {
                Some((*idx, Self::operand_to_type_name(op_type)?))
            }).collect(),
        }
    }
}

#[derive(Debug)]
struct VariantGeneratorData {
    variant_name: VariantNamePretty,
    encoding: VariantEncodingGeneratorData,
}

struct Generator {
    instruction_names: Vec<(InstructionNamePretty, Vec<VariantGeneratorData>)>,
}

impl Generator {
    pub fn new(instructions: HashMap<InstructionName, Instruction>) -> Self {
        let mut instruction_names = vec![];
        for (instruction_name, instruct) in instructions.iter() {
            let instruct_name_pretty = InstructionNamePretty::new(&instruction_name);
            let mut variant_data = vec![];
            for encoding in instruct.encodings.iter() {
                let variant_name_pretty = VariantNamePretty::new(&encoding);
                variant_data.push(VariantGeneratorData {
                    variant_name: variant_name_pretty,
                    encoding: VariantEncodingGeneratorData::new(encoding),
                });
            }
            instruction_names.push((instruct_name_pretty, variant_data));
        }
        Self {
            instruction_names,
        }
    }
}

fn generate_single_instruction_enum() -> Vec<TokenStream> {
    todo!()
}

fn generate_from_encoding() -> Vec<TokenStream> {
    todo!()
}


#[proc_macro]
pub fn make_enums(_: TokenStream) -> TokenStream {
    let Instructions { instructions } = get_instruction_metadata();
    let mut res = vec![];
    let generator = Generator::new(instructions);
    for (instruction_name, variants) in generator.instruction_names.iter() {
        let mut instruction_encoding_enum = "pub enum ".to_string();
        instruction_encoding_enum.push_str(instruction_name.0.as_str());
        instruction_encoding_enum.push_str(" { ");
        for variant_data in variants.iter() {
            let variant_name = &variant_data.variant_name;
            instruction_encoding_enum.push_str(variant_name.0.as_str());
            instruction_encoding_enum.push_str(" { ");
            for (idx, name) in variant_data.encoding.operand_names.iter() {
                let type_string = variant_data.encoding.operand_type_names.get(idx).unwrap();
                instruction_encoding_enum.push_str(format!("{} : {}", name.as_str(), type_string.as_str()).as_str());
                instruction_encoding_enum.push_str(" , ");
            }
            instruction_encoding_enum.push_str(" } ");
            instruction_encoding_enum.push_str(", ");
        }
        instruction_encoding_enum.push_str(" } ");
        let token_stream: TokenStream = instruction_encoding_enum.parse().unwrap();
        res.extend(token_stream);
    }
    let mut instruction_enum = "pub enum Instructions { ".to_string();
    for (instruction_name, _) in generator.instruction_names.iter() {
        instruction_enum.push_str(format!("{}({}),", instruction_name.0.as_str(), instruction_name.0.as_str()).as_str());
    }
    instruction_enum.push_str(" }");
    let token_stream: TokenStream = instruction_enum.parse().unwrap();
    res.extend(token_stream);
    TokenStream::from_iter(res.into_iter())
}

fn unsupported_instruction(s: &str) -> bool {
    match s {
        "VSQRTPH" => true,
        "VRSQRTSH" => true,
        "REP_STOSB" => true,
        "REPE_SCASD" => true,
        "VADDSH" => true,
        "SEAMRET" => true,
        "REP_INSD" => true,
        "MCOMMIT" => true,
        "XSUSLDTRK" => true,
        "AOR" => true,
        "VCVTUQQ2PH" => true,
        "AESENC128KL" => true,
        "RET_NEAR" => true,
        "REPE_CMPSQ" => true,
        "VFMADD213PH" => true,
        "PSMASH" => true,
        "REP_XSHA256" => true,
        "AESDEC128KL" => true,
        "VMULPH" => true,
        "INVLPGB" => true,
        "VCVTSH2SS" => true,
        "VFNMADD231PH" => true,
        "ADC_LOCK" => true,
        "AESDECWIDE256KL" => true,
        "VFMADDSUB213PH" => true,
        "AXOR" => true,
        "VPDPBSSDS" => true,
        "VPCMPESTRM64" => true,
        "VFMADDSUB132PH" => true,
        "VCVTUW2PH" => true,
        "CMPPXADD" => true,
        "VRCPSH" => true,
        "VCVTTSH2SI" => true,
        "VFNMADD231SH" => true,
        "PREFETCHIT0" => true,
        "AND_LOCK" => true,
        "TESTUI" => true,
        "PCMPESTRM64" => true,
        "VFNMSUB213SH" => true,
        "CMOVNBE" => true,
        "CMPXCHG_LOCK" => true,
        "VFPCLASSPH" => true,
        "REPE_SCASW" => true,
        "VGETMANTSH" => true,
        "VMINSH" => true,
        "VCVTPH2UDQ" => true,
        "STUI" => true,
        "VFNMSUB213PH" => true,
        "VPDPBSSD" => true,
        "VCVTTSH2USI" => true,
        "REP_MOVSW" => true,
        "VMAXSH" => true,
        "VCVTPS2PHX" => true,
        "VPEXTRW_C5" => true,
        "BTR_LOCK" => true,
        "CMPNOXADD" => true,
        "LDTILECFG" => true,
        "VFMSUB231PH" => true,
        "VRNDSCALEPH" => true,
        "CMOVNL" => true,
        "CMPNBEXADD" => true,
        "REPNE_SCASQ" => true,
        "VCVTPH2UW" => true,
        "VCVTPH2PSX" => true,
        "VFNMADD132PH" => true,
        "CMPSD_XMM" => true,
        "REP_XCRYPTOFB" => true,
        "FADDP" => true,
        "JNLE" => true,
        "VCVTNEEBF162PS" => true,
        "REP_OUTSW" => true,
        "VFMSUB132SH" => true,
        "VFNMSUB132SH" => true,
        "CMPNZXADD" => true,
        "VCVTPH2UQQ" => true,
        "VGETEXPSH" => true,
        "SENDUIPI" => true,
        "VFMADDCSH" => true,
        "VFNMSUB132PH" => true,
        "VRCPPH" => true,
        "VCOMISH" => true,
        "REP_OUTSD" => true,
        "SETNZ" => true,
        "TILERELEASE" => true,
        "VADDPH" => true,
        "CMOVZ" => true,
        "CMPXCHG8B_LOCK" => true,
        "VP2INTERSECTQ" => true,
        "VFMADD231SH" => true,
        "VFMSUB132PH" => true,
        "SETNLE" => true,
        "VFMADD132SH" => true,
        "VSUBPH" => true,
        "REP_STOSD" => true,
        "TDPBUSD" => true,
        "VGETEXPPH" => true,
        "VFNMADD213PH" => true,
        "TLBSYNC" => true,
        "VSUBSH" => true,
        "VCVTNEPS2BF16" => true,
        "PREFETCH_EXCLUSIVE" => true,
        "CALL_FAR" => true,
        "REP_XSHA1" => true,
        "REP_MOVSB" => true,
        "REP_LODSQ" => true,
        "VMAXPH" => true,
        "VDPBF16PS" => true,
        "REPE_SCASQ" => true,
        "REP_INSW" => true,
        "REPNE_CMPSD" => true,
        "RDMSRLIST" => true,
        "VCVTNEOPH2PS" => true,
        "DEC_LOCK" => true,
        "VCVTUDQ2PH" => true,
        "VFPCLASSSH" => true,
        "REPE_SCASB" => true,
        "PREFETCHIT1" => true,
        "VREDUCEPH" => true,
        "VPCMPESTRI64" => true,
        "UIRET" => true,
        "VFCMADDCSH" => true,
        "AESDEC256KL" => true,
        "REP_STOSQ" => true,
        "AESDECWIDE128KL" => true,
        "VFMADD213SH" => true,
        "VPDPBSUD" => true,
        "VFMSUB213SH" => true,
        "VCVTNEOBF162PS" => true,
        "MOV_DR" => true,
        "VBCSTNESH2PS" => true,
        "VCVTSH2SI" => true,
        "VDIVSH" => true,
        "BTS_LOCK" => true,
        "VFMADD132PH" => true,
        "JNBE" => true,
        "NEG_LOCK" => true,
        "FWAIT" => true,
        "TDPBF16PS" => true,
        "CMPNSXADD" => true,
        "CMPNLXADD" => true,
        "SETNB" => true,
        "VGETMANTPH" => true,
        "REP_XCRYPTCTR" => true,
        "RDPRU" => true,
        "VFNMADD132SH" => true,
        "XRESLDTRK" => true,
        "VPDPBUUD" => true,
        "REP_INSB" => true,
        "VFMSUBADD231PH" => true,
        "CMPZXADD" => true,
        "VCMPSH" => true,
        "REPNE_CMPSQ" => true,
        "REPE_CMPSD" => true,
        "HRESET" => true,
        "FUCOMIP" => true,
        "VFNMSUB231SH" => true,
        "VRNDSCALESH" => true,
        "VCVTSD2SH" => true,
        "AESENCWIDE128KL" => true,
        "CMPNLEXADD" => true,
        "VFMSUB213PH" => true,
        "VP2INTERSECTD" => true,
        "REP_LODSD" => true,
        "CMPXCHG16B_LOCK" => true,
        "AAND" => true,
        "AESENC256KL" => true,
        "REPNE_SCASD" => true,
        "INC_LOCK" => true,
        "VCVTSI2SH" => true,
        "VCVTNEEPH2PS" => true,
        "ADD_LOCK" => true,
        "VPDPBSUDS" => true,
        "VCVTTPH2W" => true,
        "REP_XCRYPTCBC" => true,
        "VCVTSH2USI" => true,
        "SEAMCALL" => true,
        "VUCOMISH" => true,
        "PEXTRW_SSE4" => true,
        "WRMSRNS" => true,
        "VFCMULCSH" => true,
        "VFMSUB231SH" => true,
        "WRMSRLIST" => true,
        "BEXTR_XOP" => true,
        "VCVTPH2PD" => true,
        "CMOVNZ" => true,
        "REP_MOVSQ" => true,
        "BTC_LOCK" => true,
        "VFMULCPH" => true,
        "CLUI" => true,
        "VFMSUBADD132PH" => true,
        "CMPLEXADD" => true,
        "VCVTPH2DQ" => true,
        "VMOVSH" => true,
        "VFCMADDCPH" => true,
        "STTILECFG" => true,
        "VCVTTPH2UDQ" => true,
        "TDPBSSD" => true,
        "REPNE_CMPSW" => true,
        "VSCALEFSH" => true,
        "AESENCWIDE256KL" => true,
        "FCOMIP" => true,
        "CMPOXADD" => true,
        "VCVTPH2W" => true,
        "REPE_CMPSB" => true,
        "VPDPBUUDS" => true,
        "REP_LODSB" => true,
        "SETNBE" => true,
        "REPE_CMPSW" => true,
        "MOVSD_XMM" => true,
        "VCMPPH" => true,
        "REP_XSTORE" => true,
        "VBCSTNEBF162PS" => true,
        "VCVTPH2QQ" => true,
        "JNZ" => true,
        "JNL" => true,
        "VFNMADD213SH" => true,
        "LOADIWKEY" => true,
        "REPNE_SCASB" => true,
        "VSQRTSH" => true,
        "VCVTTPH2QQ" => true,
        "TDCALL" => true,
        "VFNMSUB231PH" => true,
        "TILESTORED" => true,
        "VFMSUBADD213PH" => true,
        "CMPNBXADD" => true,
        "VMINPH" => true,
        "CMPBXADD" => true,
        "SERIALIZE" => true,
        "SYSRET64" => true,
        "JZ" => true,
        "XLAT" => true,
        "SETZ" => true,
        "TDPBSUD" => true,
        "VFCMULCPH" => true,
        "JMP_FAR" => true,
        "CMOVNB" => true,
        "CALL_NEAR" => true,
        "XADD_LOCK" => true,
        "VMULSH" => true,
        "TILEZERO" => true,
        "REPNE_CMPSB" => true,
        "CMOVNLE" => true,
        "REP_OUTSB" => true,
        "REP_XCRYPTCFB" => true,
        "NOT_LOCK" => true,
        "VDIVPH" => true,
        "REP_MOVSD" => true,
        "PCMPESTRI64" => true,
        "REP_STOSW" => true,
        "TILELOADD" => true,
        "VREDUCESH" => true,
        "XOR_LOCK" => true,
        "VFMADDSUB231PH" => true,
        "CMPBEXADD" => true,
        "OR_LOCK" => true,
        "VCVTTPH2UQQ" => true,
        "REP_LODSW" => true,
        "VMOVW" => true,
        "VCVTPD2PH" => true,
        "JNB" => true,
        "SEAMOPS" => true,
        "SBB_LOCK" => true,
        "CMPNPXADD" => true,
        "ENQCMD" => true,
        "VCVTQQ2PH" => true,
        "VCVTTPH2DQ" => true,
        "RMPUPDATE" => true,
        "SETNL" => true,
        "REP_XCRYPTECB" => true,
        "MOV_CR" => true,
        "ENQCMDS" => true,
        "VCVTSH2SD" => true,
        "VCVTUSI2SH" => true,
        "REPNE_SCASW" => true,
        "VCVTW2PH" => true,
        "TILELOADDT1" => true,
        "VRSQRTPH" => true,
        "VCVTNE2PS2BF16" => true,
        "VFMULCSH" => true,
        "VSCALEFPH" => true,
        "RET_FAR" => true,
        "VFMADDCPH" => true,
        "VCVTDQ2PH" => true,
        "SUB_LOCK" => true,
        "PVALIDATE" => true,
        "VCVTSS2SH" => true,
        "VCVTTPH2UW" => true,
        "AADD" => true,
        "TDPBUUD" => true,
        "ENCODEKEY128" => true,
        "TDPFP16PS" => true,
        "RMPADJUST" => true,
        "CMPSD" => true,
        "ENCODEKEY256" => true,
        "VFMADD231PH" => true,
        "VFMADD231PD" => true,
        "CMPPD" => true,
        "CMPSXADD" => true,
        "CMPLXADD" => true,
        _ => false
    }
}

fn convert_instruction_name_to_capstone_name(s: &str) -> &str {
    match s {
        other => other
    }
}

#[proc_macro]
pub fn top_level_make_from_detail(_: TokenStream) -> TokenStream {
    let Instructions { instructions } = get_instruction_metadata();
    let mut res = vec![];
    let mut code = "use capstone::arch::x86::X86Insn;".to_string();
    code.push_str("impl Instructions { ");
    code.push_str(" pub fn from_detail(ins_code: X86Insn, detail: &X86InsnDetail) -> Self { ");
    code.push_str(" match ins_code { ");
    let generator = Generator::new(instructions);
    for (instruction_name, _) in generator.instruction_names.iter() {
        if unsupported_instruction(instruction_name.0.as_str()) {
            continue;
        }
        //todo using xed instead of capstone should avoid this
        code.push_str(format!("capstone::arch::x86::X86Insn::X86_INS_{} => Instructions::{}({}::from_detail(detail)),", convert_instruction_name_to_capstone_name(instruction_name.0.as_str()), instruction_name.0.as_str(), instruction_name.0.as_str()).as_str());
    }
    code.push_str(format!("_ => panic!(\"unhandled instruction\"),").as_str());
    code.push_str(" } ");
    code.push_str(" } ");
    code.push_str(" } ");
    let token_stream: TokenStream = code.parse().unwrap();
    res.extend(token_stream);
    TokenStream::from_iter(res.into_iter())
}

#[proc_macro]
pub fn make_from_detail(_: TokenStream) -> TokenStream {
    let Instructions { instructions } = get_instruction_metadata();
    let mut res = vec![];
    let generator = Generator::new(instructions);
    for (instruction_name, variants) in generator.instruction_names.iter() {
        let mut instruction_encoding_enum = "impl ".to_string();
        instruction_encoding_enum.push_str(instruction_name.0.as_str());
        instruction_encoding_enum.push_str(" { ");
        instruction_encoding_enum.push_str(" pub fn from_detail(detail: &X86InsnDetail) -> Self {");
        instruction_encoding_enum.push_str(" let operands: Vec<X86Operand> = dbg!(detail.operands().collect_vec()); ");
        instruction_encoding_enum.push_str(" let operands = operands.iter().map(|operand|Operand::from_capstone(operand)).collect_vec(); ");
        instruction_encoding_enum.push_str(" dbg!(&operands); ");
        for (variant_i, variant_data) in variants.iter().enumerate() {
            let encoding = &variant_data.encoding;
            instruction_encoding_enum.push_str(format!("let is_this_variant_{variant_i} = ").as_str());
            for (index, operand_type) in encoding.operands.iter() {
                instruction_encoding_enum.push_str(format!("operands.len() >= {} && {}.is_of_type(&operands[{}]) && ",
                                                           index.0.get(),
                                                           operand_type.to_declaration_string(),
                                                           index.0.get() - 1)
                    .as_str());
            }
            instruction_encoding_enum.push_str(" true;");
        }
        for (variant_i, variant_data) in variants.iter().enumerate() {
            instruction_encoding_enum.push_str(format!("if is_this_variant_{variant_i}").as_str());
            instruction_encoding_enum.push_str("{");
            instruction_encoding_enum.push_str(format!("return Self::{}{{", variant_data.variant_name.0.as_str()).as_str());
            for (idx, name) in variant_data.encoding.operand_names.iter() {
                let type_string = variant_data.encoding.operand_type_names.get(idx).unwrap();
                let unwrap_name = type_string.to_ascii_lowercase();
                instruction_encoding_enum.push_str(format!("{} : operands[{}].unwrap_{}()", name.as_str(), idx.0.get() - 1, unwrap_name).as_str());
                instruction_encoding_enum.push_str(" , ");
            }
            instruction_encoding_enum.push_str("};");
            instruction_encoding_enum.push_str("}");
        }
        instruction_encoding_enum.push_str(format!("panic!(\"Not any variant: {}\")", instruction_name.0.as_str()).as_str());
        instruction_encoding_enum.push_str(" } ");
        instruction_encoding_enum.push_str(" } ");
        let token_stream: TokenStream = instruction_encoding_enum.parse().unwrap();
        res.extend(token_stream);
    }
    TokenStream::from_iter(res.into_iter())
}


fn get_instruction_metadata() -> Instructions {
    bincode::deserialize_from(Cursor::new(include_bytes!("../../out.bin"))).unwrap()
}


