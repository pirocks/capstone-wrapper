extern crate proc_macro;

use proc_macro::TokenStream;
use std::io::Cursor;
use std::num::NonZeroU8;

use itertools::Itertools;

use wrapper_common::instructions::{InstructionEncoding, Instructions};
use wrapper_common::operand_index::OperandIndex;

#[proc_macro]
pub fn make_enum(_item: TokenStream) -> TokenStream {
    let Instructions { instructions } = get_instruction_metadata();
    let mut res = vec![];
    for (instruction_name, instruct) in instructions {
        let mut enum_ = "".to_string();
        enum_.push_str("pub enum ");
        enum_.push_str(instruction_name.0.replace("{evex} ", "vex").replace("REPE ", "repe").as_str());
        enum_.push_str(" { ");
        enum_.push_str(instruct.encodings.iter().map(|encoding| {
            let mut operand_types = vec![];
            let InstructionEncoding { zeroing, bcast, iform, mode_prefix_string, operands } = encoding;
            let prefix = "".to_string() /*+ dbg!(iform.as_str())*/ + mode_prefix_string.as_str() + if *zeroing{
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
                dbg!(operand_types.iter().map(|operand_type| operand_type.to_identifier_string()).join("_"))
            }.as_str()
        }).join(",\n ").as_str());
        enum_.push_str(" } ");
        let token_stream: TokenStream = enum_.parse().unwrap();
        res.extend(token_stream);
    }
    TokenStream::from_iter(res.into_iter())
}

fn get_instruction_metadata() -> Instructions {
    bincode::deserialize_from(Cursor::new(include_bytes!("../../out.bin"))).unwrap()
}


