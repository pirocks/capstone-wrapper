use bumpalo::Bump;
// use remill_semantics_parser::remill_semantics;
use xed_enum::*;

use crate::x86_machine::X86MachineState;

pub mod iform_aaa;
pub mod iform_adc;

impl <'arena> X86MachineState<'arena> {
    pub fn apply_instruction(&mut self,arena: &'arena Bump, instr: X86Instruction) {
        match instr {
            X86Instruction::AAA(aaa) => self.apply_iform_aaa(arena,aaa),
            X86Instruction::ADC(adc) => self.apply_iform_adc(arena,adc),
            _ => todo!(),
        }
    }
}
