use wrapper_common::registers::{Reg16WithRIP, Reg32WithRIP, Reg64WithRIP, Reg8};
use crate::x86_machine::read_write::{Readable, Writeable};
use crate::x86_machine::X86MachineState;
use crate::x86_machine::semantics_builder::SemanticsBuilder;
use crate::x86_machine::values::{ByteValue, DWordValue, QWordValue, WordValue};

#[derive(Copy, Clone)]
pub struct QWordAccessor {
    pub(crate) reg: Reg64WithRIP,
}

impl <'arena> Readable<'arena,QWordValue<'arena>> for QWordAccessor {
    fn emit_read(&self, state: &X86MachineState<'arena>, semantics: &SemanticsBuilder<'arena>) -> QWordValue<'arena> {
        todo!()
    }
}

impl <'arena> Writeable<'arena,QWordValue<'arena>> for QWordAccessor {
    fn emit_write(&self, state: &mut X86MachineState<'arena>, semantics: &SemanticsBuilder<'arena>) -> QWordValue<'arena> {
        todo!()
    }
}

pub struct DWordAccessor {
    pub(crate) reg: Reg32WithRIP,
}

impl <'arena> Readable<'arena,DWordValue<'arena>> for DWordAccessor {
    fn emit_read(&self, state: &X86MachineState<'arena>, semantics: &SemanticsBuilder<'arena>) -> DWordValue<'arena> {
        todo!()
    }
}

impl <'arena> Writeable<'arena,DWordValue<'arena>> for DWordAccessor {
    fn emit_write(&self, state: &mut X86MachineState<'arena>, semantics: &SemanticsBuilder<'arena>) -> DWordValue<'arena> {
        todo!()
    }
}

pub struct WordAccessor {
    pub(crate) reg: Reg16WithRIP,
}

impl <'arena> Readable<'arena,WordValue<'arena>> for WordAccessor {
    fn emit_read(&self, state: &X86MachineState<'arena>, semantics: &SemanticsBuilder<'arena>) -> WordValue<'arena> {
        todo!()
    }
}

impl <'arena> Writeable<'arena,WordValue<'arena>> for WordAccessor {
    fn emit_write(&self, state: &mut X86MachineState<'arena>, semantics: &SemanticsBuilder<'arena>) -> WordValue<'arena> {
        todo!()
    }
}

pub struct ByteAccessor {
    pub(crate) reg: Reg8,
}

impl <'arena> Readable<'arena,ByteValue<'arena>> for ByteAccessor {
    fn emit_read(&self, state: &X86MachineState<'arena>, semantics: &SemanticsBuilder<'arena>) -> ByteValue<'arena> {
        todo!()
    }
}

impl <'arena> Writeable<'arena,ByteValue<'arena>> for ByteAccessor {
    fn emit_write(&self, state: &mut X86MachineState<'arena>, semantics: &SemanticsBuilder<'arena>) -> ByteValue<'arena> {
        todo!()
    }
}