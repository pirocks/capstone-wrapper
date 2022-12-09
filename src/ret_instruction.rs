use capstone::arch::x86::{X86InsnDetail, X86OperandType};
use capstone::prelude::DetailsArchInsn;
use itertools::Itertools;

#[derive(Debug, Eq, PartialEq)]
pub struct RetInstruction{
    to_pop: Option<i16>
}

impl RetInstruction {
    pub fn from_details(detail: &X86InsnDetail) -> Self {
        let operands = detail.operands().collect_vec();
        if operands.is_empty(){
            return Self{
                to_pop: None
            }
        }
        match &operands[0].op_type {
            X86OperandType::Reg(_) => {
                todo!()
            }
            X86OperandType::Imm(imm) => {
                RetInstruction{
                    to_pop: Some((*imm).try_into().expect("malformed immediate from capstone"))
                }
            }
            X86OperandType::Mem(_) => {
                todo!()
            }
            X86OperandType::Invalid => {
                todo!()
            }
        }
    }
}