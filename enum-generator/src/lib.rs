extern crate proc_macro;

use proc_macro::TokenStream;
use std::collections::HashMap;
use std::io::Cursor;
use std::num::NonZeroU8;

use string_concat_utils::concat_camel_case;

use wrapper_common::instructions::{Instruction, InstructionEncoding, InstructionName, Instructions};
use wrapper_common::operand_index::OperandIndex;

struct VariantNamePretty(String);

impl VariantNamePretty {
    pub fn new(encoding: &InstructionEncoding) -> Self {
        let mut operand_types = vec![];
        let InstructionEncoding { zeroing:_, bcast, iform: _, mode_prefix_string, operands } = encoding;
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

struct VariantPrettyNames {
    inner: Vec<VariantNamePretty>,
}

struct VariantGeneratorData{
    variant_name: VariantNamePretty,
    encoding: InstructionEncoding
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
                variant_data.push(VariantGeneratorData{
                    variant_name: variant_name_pretty,
                    encoding: encoding.clone(),
                });
            }
            instruction_names.push((instruct_name_pretty, variant_data));
        }
        Self{
            instruction_names,
        }
    }
}

fn generate_single_instruction_enum() -> Vec<TokenStream>{
    todo!()
}

fn generate_from_encoding() -> Vec<TokenStream> {
    todo!()
}


#[proc_macro]
pub fn make_enums(_item: TokenStream) -> TokenStream {
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
            instruction_encoding_enum.push_str(", ");
        }
        instruction_encoding_enum.push_str(" } ");
        let token_stream: TokenStream = instruction_encoding_enum.parse().unwrap();
        res.extend(token_stream);
    }
    let mut instruction_enum = "pub enum Instructions {".to_string();
    for (instruction_name, _) in generator.instruction_names.iter() {
        instruction_enum.push_str(format!("{}({}),", instruction_name.0.as_str(), instruction_name.0.as_str()).as_str());
    }
    instruction_enum.push_str("}");
    let token_stream: TokenStream = instruction_enum.parse().unwrap();
    res.extend(token_stream);
    TokenStream::from_iter(res.into_iter())
}

fn get_instruction_metadata() -> Instructions {
    bincode::deserialize_from(Cursor::new(include_bytes!("../../out.bin"))).unwrap()
}


