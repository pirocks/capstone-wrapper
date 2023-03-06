extern crate proc_macro;

use proc_macro::TokenStream;
use std::io::Cursor;
use std::num::NonZeroU8;

use itertools::Itertools;

use wrapper_common::instructions::{InstructionEncoding, InstructionName, Instructions};
use wrapper_common::operand_index::OperandIndex;

#[proc_macro]
pub fn make_enums(_item: TokenStream) -> TokenStream {
    let Instructions { instructions } = get_instruction_metadata();
    let mut res = vec![];
    for (instruction_name, instruct) in instructions.iter() {
        let mut instruction_encoding_enum = "".to_string();
        instruction_encoding_enum.push_str("pub enum ");
        instruction_encoding_enum.push_str(instruction_name_to_rust_enum_name(&instruction_name).as_str());
        instruction_encoding_enum.push_str(" { ");
        instruction_encoding_enum.push_str(instruct.encodings.iter().map(|encoding| {
            let mut operand_types = vec![];
            let InstructionEncoding { zeroing, bcast, iform:_, mode_prefix_string, operands } = encoding;
            let prefix = mode_prefix_string.to_string() + if *zeroing{
                "Zeroing"
            }else {
                ""
            } + match bcast {
                None => {
                    "".to_string()
                }
                Some(bcast) => {
                    format!("BCast{}",bcast.get())
                }
            }.as_str();
            prefix + if operands.len() == 0 {
                "NoOperand".to_string()
            } else {
                for i in 0..operands.len() {
                    let operand = OperandIndex(NonZeroU8::new((i + 1) as u8).unwrap());
                    operand_types.push(operands.get(&operand).unwrap());
                }
                operand_types.iter().map(|operand_type| operand_type.to_identifier_string()).join("_")
            }.as_str()
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

fn instruction_name_to_rust_enum_name(instruction_name: &InstructionName) -> String {
    instruction_name.0.replace("{evex} ", "vex").replace("REPE ", "repe")
}

fn get_instruction_metadata() -> Instructions {
    bincode::deserialize_from(Cursor::new(include_bytes!("../../out.bin"))).unwrap()
}


