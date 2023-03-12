extern crate proc_macro;

use proc_macro::TokenStream;
use std::io::Cursor;
use std::num::NonZeroU8;

use itertools::Itertools;

use wrapper_common::instructions::{InstructionEncoding, InstructionName, Instructions};
use wrapper_common::operand_index::OperandIndex;
use wrapper_common::operand_type::{Imm, MemoryOperandType, MemoryOperandTypeKind, OperandType, VectorRegisterKind};
use wrapper_common::registers::RegisterType;

#[proc_macro]
pub fn make_enums(_item: TokenStream) -> TokenStream {
    let Instructions { instructions } = get_instruction_metadata();
    let mut res = vec![];
    for (instruction_name, instruct) in instructions.iter() {
        let mut instruction_encoding_enum = "use wrapper_common::registers::*;\nuse wrapper_common::memory_operand::*;\n".to_string();
        instruction_encoding_enum.push_str("pub enum ");
        instruction_encoding_enum.push_str(instruction_name_to_rust_enum_name(&instruction_name).as_str());
        instruction_encoding_enum.push_str(" { ");
        instruction_encoding_enum.push_str(instruct.encodings.iter().map(|encoding| {
            let name = encoding_to_name(encoding);
            format!("{} {{ {} }}", name, encoding_to_member(encoding))
        }).join(",\n ").as_str());
        instruction_encoding_enum.push_str(" } ");
        let token_stream: TokenStream = instruction_encoding_enum.parse().unwrap();
        res.extend(token_stream);
    }
    let mut instruction_enum = "pub enum Instructions {".to_string();
    for (instruction_name, instruction) in instructions.iter() {
        let variant_name = instruction_name_to_rust_enum_name(instruction_name);
        instruction_enum.push_str(format!("{}({}),", variant_name, variant_name).as_str());
    }
    instruction_enum.push_str("}");
    let token_stream: TokenStream = instruction_enum.parse().unwrap();
    res.extend(token_stream);

    TokenStream::from_iter(res.into_iter())
}

fn encoding_to_member(enconding: &InstructionEncoding) -> String {
    let mut res = "".to_string();
    for (idx, operand_type) in enconding.operands.iter().sorted_by_key(|(idx, _)| **idx) {
        match operand_type {
            OperandType::Reg(reg) => {
                match reg {
                    RegisterType::AllMmx => {
                        res.push_str(format!("mmx_{}: RegMMX,\n", idx.0).as_str());
                    }
                    RegisterType::SomeXmm(_) |
                    RegisterType::AllXmm16 |
                    RegisterType::AllXmm32 => {
                        res.push_str(format!("xmm_{}: RegXMM,\n", idx.0).as_str());
                    }
                    RegisterType::SingleXmm(_) => {}
                    RegisterType::AllYmm16 |
                    RegisterType::AllYmm32 => {
                        res.push_str(format!("ymm_{}: RegYMM,\n", idx.0).as_str());
                    }
                    RegisterType::SomeZmm(_) |
                    RegisterType::AllZmm32 => {
                        res.push_str(format!("zmm_{}: RegZMM,\n", idx.0).as_str());
                    }
                    RegisterType::AllTmm => {
                        res.push_str(format!("tmm_{}: RegTMM,\n", idx.0).as_str());
                    }
                    RegisterType::AllMask |
                    RegisterType::SomeMask(_) => {
                        res.push_str(format!("mask_{}: RegMask,\n", idx.0).as_str());
                    }
                    RegisterType::AllGP64WithoutRIP => {
                        res.push_str(format!("gp64_{}: Reg64WithoutRIP,\n", idx.0).as_str());
                    }
                    RegisterType::AllGP64WithRIP => {
                        res.push_str(format!("gp64_{}: Reg64WithRIP,\n", idx.0).as_str());
                    }
                    RegisterType::SingleGP64(_) => {}
                    RegisterType::AllGP32WithoutRIP => {
                        res.push_str(format!("gp32_{}: Reg32WithoutRIP,\n", idx.0).as_str());
                    }
                    RegisterType::SomeGP32(_) |
                    RegisterType::AllGP32WithRIP => {
                        res.push_str(format!("gp32_{}: Reg32WithRIP,\n", idx.0).as_str());
                    }
                    RegisterType::SingleGP32(_) => {}
                    RegisterType::AllGP16WithoutRIP => {
                        res.push_str(format!("gp16_{}: Reg16WithoutRIP,\n", idx.0).as_str());
                    }
                    RegisterType::SomeGP16(_) |
                    RegisterType::AllGP16WithRIP => {
                        res.push_str(format!("gp16_{}: Reg16WithRIP,\n", idx.0).as_str());
                    }
                    RegisterType::SingleGP16(_) => {}
                    RegisterType::AllGP8 => {
                        res.push_str(format!("gp8_{}: Reg8,\n", idx.0).as_str());
                    }
                    RegisterType::SomeGP8(_) => {
                        res.push_str(format!("gp8_{}: Reg8,\n", idx.0).as_str());
                    }
                    RegisterType::SingleGP8(_) => {}
                    RegisterType::AllFloat => {
                        res.push_str(format!("st_i_{}: RegFloat,\n", idx.0).as_str());
                    }
                    RegisterType::SingleFloat(_) => {}
                    RegisterType::AllBnd => {
                        res.push_str(format!("bnd_{}: RegBnd,\n", idx.0).as_str());
                    }
                    RegisterType::SomeSegment(_) |
                    RegisterType::AllSegment => {
                        res.push_str(format!("bnd_{}: RegSegment,\n", idx.0).as_str());
                    }
                    RegisterType::SingleSegment(_) => {}
                    RegisterType::AllDebug => {
                        res.push_str(format!("dr_{}: RegDebug,\n", idx.0).as_str());
                    }
                    RegisterType::SomeControl(_) => {
                        res.push_str(format!("cr_{}: RegControl,\n", idx.0).as_str());
                    }
                    RegisterType::SomeControlExtra(_) => {
                        res.push_str(format!("cr_{}: RegControlExtra,\n", idx.0).as_str());
                    }
                    RegisterType::SingleSegmentBase(_) |
                    RegisterType::SingleFloatControl(_) |
                    RegisterType::SingleSpecial(_) |
                    RegisterType::SingleControl(_) => {}
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
                res.push_str(format!("mem_{}{}_{}: MemoryOperand,\n", kind_string, vsib, idx.0).as_str());
            }
            OperandType::Imm(imm) => {
                match imm {
                    Imm::Imm8 => {
                        res.push_str(format!("imm_{}: i8,\n", idx.0).as_str());
                    }
                    Imm::Imm16 => {
                        res.push_str(format!("imm_{}: i16,\n", idx.0).as_str());
                    }
                    Imm::Imm32 => {
                        res.push_str(format!("imm_{}: i32,\n", idx.0).as_str());
                    }
                    Imm::Imm64 => {
                        res.push_str(format!("imm_{}: i64,\n", idx.0).as_str());
                    }
                }
            }
            OperandType::ImmSpecific(_) => {}
            OperandType::Flags(_) => {}
            OperandType::Agen(_) => {}
            OperandType::Rel8 => {
                res.push_str(format!("rel8_{}: i8,\n", idx.0).as_str());
            }
            OperandType::Rel16 => {
                res.push_str(format!("rel16_{}: i16,\n", idx.0).as_str());
            }
            OperandType::Rel32 => {
                res.push_str(format!("rel32_{}: i32,\n", idx.0).as_str());
            }
        }
    }
    res
}

fn encoding_to_name(encoding: &InstructionEncoding) -> String {
    let mut operand_types = vec![];
    let InstructionEncoding { bcast, mode_prefix_string, operands } = encoding;
    let prefix = mode_prefix_string.to_string() + match bcast {
        None => {
            "".to_string()
        }
        Some(bcast) => {
            format!("BCast{}", bcast.get())
        }
    }.as_str();
    let name = prefix + if operands.len() == 0 {
        "NoOperand".to_string()
    } else {
        for i in 0..operands.len() {
            let operand = OperandIndex(NonZeroU8::new((i + 1) as u8).unwrap());
            operand_types.push(operands.get(&operand).unwrap());
        }
        operand_types.iter().map(|operand_type| operand_type.to_identifier_string()).join("_")
    }.as_str();
    name
}

fn instruction_name_to_rust_enum_name(instruction_name: &InstructionName) -> String {
    instruction_name.0.replace("{evex} ", "vex").replace("REPE ", "repe")
}

fn get_instruction_metadata() -> Instructions {
    bincode::deserialize_from(Cursor::new(include_bytes!("../../out.bin"))).unwrap()
}


