use std::collections::HashMap;
use std::num::NonZeroU8;
use std::str::FromStr;

use serde::{Deserialize, Serialize};
use uops_info::{Extension, InstructionElement, Root};

use crate::operand_index::OperandIndex;
use crate::operand_type::{FromRawError, OperandType};

#[derive(Debug, Serialize, Deserialize)]
pub struct InstructionEncoding {
    pub zeroing: bool,
    pub bcast: Option<NonZeroU8>,
    pub iform: String,
    pub mode_prefix_string: String,
    pub operands: HashMap<OperandIndex, OperandType>,
}

impl InstructionEncoding {
    pub fn new(raw: &uops_info::Instruction) -> Result<Self, FromRawError> {
        let uops_info::Instruction {
            operands,
            zeroing,
            string,
            iform,
            bcast,
            ..
        } = raw;
        let mut operands_res = HashMap::new();
        if let Some(operands) = operands {
            for operand in operands {
                if let InstructionElement::Operand {
                    idx,
                    r#type,
                    width,
                    xtype,
                    memory_prefix,
                    val,
                    vsib,
                    ..
                } = operand {
                    let operand_index = OperandIndex::from_str(idx)?;
                    let duplicate = operands_res.insert(operand_index,
                                                        OperandType::new(
                                                            r#type,
                                                            xtype.as_ref(),
                                                            val.as_ref(),
                                                            memory_prefix.as_ref(),
                                                            width.as_ref(),
                                                            vsib.as_ref()
                                                        )?).is_some();
                    if duplicate {
                        return Err(FromRawError::MultipleOperandsWithSameIndex);
                    }
                }
            }
        }
        Ok(Self {
            iform: iform.to_string(),
            zeroing: zeroing == &Some("1".to_string()),
            mode_prefix_string: string.split(" ").next().unwrap().to_string(),
            operands: operands_res,
            bcast: match bcast {
                None => None,
                Some(bcast) => {
                    match NonZeroU8::new(u8::from_str(bcast.as_str()).unwrap()) {
                        Some(x) => Some(x),
                        None => None,
                    }
                }
            }
        })
    }
}

#[derive(Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub struct InstructionName(pub String);

impl InstructionName {
    pub fn new(str: impl Into<String>) -> InstructionName {
        InstructionName(str.into().split(" ").last().unwrap().to_string())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Instruction {
    pub encodings: Vec<InstructionEncoding>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Instructions {
    pub instructions: HashMap<InstructionName, Instruction>,
}

impl Instructions {
    pub fn new(raw: Root) -> Result<Self, FromRawError> {
        let mut instructions_res = HashMap::new();
        let Root { date: _, extensions } = raw;
        for extension in extensions {
            let Extension { name: _, instructions } = extension;
            if let Some(instructions) = instructions {
                for instruction in instructions {
                    let instruction_name = InstructionName::new(&instruction.asm);
                    instructions_res
                        .entry(instruction_name)
                        .or_insert(Instruction { encodings: vec![] })
                        .encodings.push(InstructionEncoding::new(&instruction)?)
                }
            }
        }
        Ok(Self {
            instructions: instructions_res,
        })
    }
}

