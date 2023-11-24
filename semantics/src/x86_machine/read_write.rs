use wrapper_common::registers::{Reg16WithRIP, Reg32WithRIP, Reg64WithRIP, Reg8};
use xed_wrapper::operands::{Imm16, Imm32, Imm8, MemoryOperands};
use crate::x86_machine::semantics_builder::{SemanticsBuilder, SemanticsBuilder64Ext};
use crate::x86_machine::values::{ByteValue, DWordValue, NumericValue, QWordValue, WordValue};
use crate::x86_machine::X86MachineState;

pub trait Writeable<'arena,T : NumericValue<'arena>> {
    //todo refactor such that SemanticsBuilder and x86 state are passed in, and the semantics builder only builds semantics while the state has the value accessor functions
    fn emit_write(&self, state: &mut X86MachineState<'arena>, semantics: &SemanticsBuilder<'arena>) -> T;
}

//can be a memory address or immediate
pub trait Readable<'arena, T: NumericValue<'arena>> {
    fn emit_read(&self, state: &X86MachineState<'arena>, semantics: &SemanticsBuilder<'arena>) -> T;
}


impl <'arena> Readable<'arena,ByteValue<'arena>> for Imm8 {
    fn emit_read(&self, state: &X86MachineState<'arena>, semantics: &SemanticsBuilder<'arena>) -> ByteValue<'arena> {
        semantics.constant(self.0)
    }
}

impl <'arena> Readable<'arena,WordValue<'arena>> for Imm16 {
    fn emit_read(&self, state: &X86MachineState<'arena>, semantics: &SemanticsBuilder<'arena>) -> WordValue<'arena> {
        semantics.constant(self.0)
    }
}

impl <'arena> Readable<'arena,DWordValue<'arena>> for Imm32 {
    fn emit_read(&self, state: &X86MachineState<'arena>, semantics: &SemanticsBuilder<'arena>) -> DWordValue<'arena> {
        semantics.constant(self.0)
    }
}

impl <'arena, T: NumericValue<'arena>> Readable<'arena,T> for T{
    fn emit_read(&self, state: &X86MachineState<'arena>, semantics: &SemanticsBuilder<'arena>) -> T {
        self.clone()
    }
}

impl <'arena, T: NumericValue<'arena>> Readable<'arena,T> for MemoryOperands{
    fn emit_read(&self, state: &X86MachineState<'arena>, semantics: &SemanticsBuilder<'arena>) -> T {
        todo!()
    }
}

impl <'arena, T: NumericValue<'arena>> Writeable<'arena,T> for MemoryOperands{
    fn emit_write(&self, state: &mut X86MachineState<'arena>, semantics: &SemanticsBuilder<'arena>) -> T {
        todo!()
    }
}

impl <'arena> Readable<'arena,QWordValue<'arena>> for Reg64WithRIP{
    fn emit_read(&self, state: &X86MachineState<'arena>, semantics: &SemanticsBuilder<'arena>) -> QWordValue<'arena> {
        todo!()
    }
}

impl <'arena> Writeable<'arena,QWordValue<'arena>> for Reg64WithRIP{
    fn emit_write(&self, state: &mut X86MachineState<'arena>, semantics: &SemanticsBuilder<'arena>) -> QWordValue<'arena> {
        todo!()
    }
}

impl <'arena> Readable<'arena,DWordValue<'arena>> for Reg32WithRIP {
    fn emit_read(&self, state: &X86MachineState<'arena>, semantics: &SemanticsBuilder<'arena>) -> DWordValue<'arena> {
        todo!()
    }
}

impl <'arena> Writeable<'arena,DWordValue<'arena>> for Reg32WithRIP {
    fn emit_write(&self, state: &mut X86MachineState<'arena>, semantics: &SemanticsBuilder<'arena>) -> DWordValue<'arena> {
        todo!()
    }
}

impl <'arena> Readable<'arena,WordValue<'arena>> for Reg16WithRIP {
    fn emit_read(&self, state: &X86MachineState<'arena>, semantics: &SemanticsBuilder<'arena>) -> WordValue<'arena> {
        todo!()
    }
}

impl <'arena> Writeable<'arena,WordValue<'arena>> for Reg16WithRIP {
    fn emit_write(&self, state: &mut X86MachineState<'arena>, semantics: &SemanticsBuilder<'arena>) -> WordValue<'arena> {
        todo!()
    }
}

impl <'arena> Readable<'arena,ByteValue<'arena>> for Reg8 {
    fn emit_read(&self, state: &X86MachineState<'arena>, semantics: &SemanticsBuilder<'arena>) -> ByteValue<'arena> {
        *state.get_reg_8(self)
    }
}

impl <'arena> Writeable<'arena,ByteValue<'arena>> for Reg8 {
    fn emit_write(&self, state: &mut X86MachineState<'arena>, semantics: &SemanticsBuilder<'arena>) -> ByteValue<'arena> {
        todo!()
    }
}