use xed_enum::X86Instruction;

use crate::semantics2::aaa::apply_iform_aaa;
use crate::semantics2::aad::apply_iform_aad;
use crate::semantics2::adc::apply_iform_adc;
use crate::semantics2::adcx::apply_iform_adcx;
use crate::semantics2::add::apply_iform_add;
use crate::semantics2::addpd::apply_iform_addpd;
use crate::semantics2::arena::Arena;
use crate::semantics2::semantic_steps::InstructionSemanticsStep;
use crate::semantics2::vaddpd::apply_iform_vaddpd;

pub mod arena;
pub mod variables;
pub mod value;
pub mod expression;
pub mod state;
pub mod read_write;
pub mod semantic_steps;
pub mod builder;
pub mod aaa;
pub mod aad;
pub mod adc;
pub mod adcx;

pub mod add;
pub mod addpd;
pub mod vaddpd;

pub mod num_traits;

pub fn apply_instruction(arena: Arena, instr: X86Instruction) -> Vec<InstructionSemanticsStep> {
    match instr {
        X86Instruction::AAA(a) => apply_iform_aaa(arena, a),
        X86Instruction::AAD(a) => apply_iform_aad(arena, a),
        X86Instruction::ADC(a) => apply_iform_adc(arena, a),
        X86Instruction::ADCX(a) => apply_iform_adcx(arena, a),
        X86Instruction::ADD(a) => apply_iform_add(arena, a),
        X86Instruction::ADDPD(a) => apply_iform_addpd(arena, a),
        X86Instruction::VADDPD(a) => apply_iform_vaddpd(arena, a),
        _ => todo!()
    }
}

#[cfg(test)]
pub mod test;