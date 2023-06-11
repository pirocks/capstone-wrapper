extern crate proc_macro;

use proc_macro::{TokenStream};
use std::collections::HashMap;
use std::io::Cursor;
use std::num::NonZeroU8;

use string_concat_utils::concat_camel_case;
use wrapper_common::instructions::{Instruction, InstructionEncoding, InstructionName, Instructions};
use wrapper_common::operand_index::OperandIndex;
use wrapper_common::operand_type::{Imm, MemoryOperandType, MemoryOperandTypeKind, OperandType, VectorRegisterKind};
use wrapper_common::registers::RegisterType;

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
                        return None
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
                return None
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
                        return None
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
                return None
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
        instruction_encoding_enum.push_str(" let operands: Vec<X86Operand> = detail.operands().collect_vec(); ");
        instruction_encoding_enum.push_str(" let operands = operands.iter().map(|operand|Operand::from_capstone(operand)).collect_vec(); ");
        for (variant_i, variant_data) in variants.iter().enumerate() {
            let encoding = &variant_data.encoding;
            instruction_encoding_enum.push_str(format!("let is_this_variant_{variant_i} = ").as_str());
            for (index, operand_type) in encoding.operands.iter() {
                instruction_encoding_enum.push_str(format!("operands.len() >= {} && {}.is_of_type(&operands[{}]) && ",
                                                           index.0.get(),
                                                           operand_type.to_declaration_string(),
                                                           index.0.get())
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
                instruction_encoding_enum.push_str(format!("{} : operands[{}].unwrap_{}()", name.as_str(), idx.0, unwrap_name).as_str());
                instruction_encoding_enum.push_str(" , ");
            }
            instruction_encoding_enum.push_str("};");
            instruction_encoding_enum.push_str("}");
        }
        instruction_encoding_enum.push_str("panic!()");
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


