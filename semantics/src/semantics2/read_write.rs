use wrapper_common::memory_operand::GeneralReg;
use wrapper_common::registers::{Reg16WithRIP, Reg32WithRIP, Reg64WithRIP, Reg8, RegXMM, RegYMM, RegZMM};
use xed_wrapper::operands::{Imm16, Imm32, Imm8, MemoryOperands};
use crate::semantics2;

use crate::semantics2::builder::SemanticsBuilder;
use crate::semantics2::expression::Expression;
use crate::semantics2::state::X86MachineState64;

pub trait Writeable<'arena> {
    fn write(&self, semantics: &mut SemanticsBuilder<'arena>, expr: &'arena Expression<'arena>);
}

pub trait Readable<'arena> {
    fn read(&self, semantics: &SemanticsBuilder<'arena>) -> &'arena Expression<'arena>;
}


impl<'arena> Readable<'arena> for Reg8 {
    fn read(&self, semantics: &SemanticsBuilder<'arena>) -> &'arena Expression<'arena> {
        semantics.get_reg_8(self.clone())
    }
}

impl<'arena> Writeable<'arena> for Reg8 {
    fn write(&self, semantics: &mut SemanticsBuilder<'arena>, expr: &'arena Expression<'arena>) {
        semantics.set_reg_8(self.clone(), expr);
    }
}

impl<'arena> Readable<'arena> for Reg16WithRIP {
    fn read(&self, semantics: &SemanticsBuilder<'arena>) -> &'arena Expression<'arena> {
        todo!()
    }
}

impl<'arena> Writeable<'arena> for Reg16WithRIP {
    fn write(&self, semantics: &mut SemanticsBuilder<'arena>, expr: &'arena Expression<'arena>) {
        todo!()
    }
}

impl<'arena> Readable<'arena> for Reg32WithRIP {
    fn read(&self, semantics: &SemanticsBuilder<'arena>) -> &'arena Expression<'arena> {
        todo!()
    }
}

impl<'arena> Writeable<'arena> for Reg32WithRIP {
    fn write(&self, semantics: &mut SemanticsBuilder<'arena>, expr: &'arena Expression<'arena>) {
        todo!()
    }
}

impl<'arena> Readable<'arena> for Reg64WithRIP {
    fn read(&self, semantics: &SemanticsBuilder<'arena>) -> &'arena Expression<'arena> {
        todo!()
    }
}

impl<'arena> Writeable<'arena> for Reg64WithRIP {
    fn write(&self, semantics: &mut SemanticsBuilder<'arena>, expr: &'arena Expression<'arena>) {
        todo!()
    }
}

impl <'arena> Readable<'arena> for Imm8 {
    fn read(&self, s: &SemanticsBuilder<'arena>) -> &'arena Expression<'arena> {
        s.constant(self.0 as u8)
    }
}

impl <'arena> Readable<'arena> for Imm16 {
    fn read(&self, s: &SemanticsBuilder<'arena>) -> &'arena Expression<'arena> {
        s.constant(self.0 as u16)
    }
}

impl <'arena> Readable<'arena> for Imm32 {
    fn read(&self, s: &SemanticsBuilder<'arena>) -> &'arena Expression<'arena> {
        s.constant(self.0 as u32)
    }
}

impl <'arena> Readable<'arena> for MemoryOperands{
    fn read(&self, semantics: &SemanticsBuilder<'arena>) -> &'arena Expression<'arena> {
        todo!()
    }
}

impl <'arena> Writeable<'arena> for MemoryOperands {
    fn write(&self, semantics: &mut SemanticsBuilder<'arena>, expr: &'arena Expression<'arena>) {
        todo!()
    }
}

impl <'arena> Readable<'arena> for RegXMM {
    fn read(&self, semantics: &SemanticsBuilder<'arena>) -> &'arena Expression<'arena> {
        todo!()
    }
}

impl <'arena> Writeable<'arena> for RegXMM {
    fn write(&self, semantics: &mut SemanticsBuilder<'arena>, expr: &'arena Expression<'arena>) {
        todo!()
    }
}

impl <'arena> Readable<'arena> for RegYMM {
    fn read(&self, semantics: &SemanticsBuilder<'arena>) -> &'arena Expression<'arena> {
        todo!()
    }
}

impl <'arena> Writeable<'arena> for RegYMM {
    fn write(&self, semantics: &mut SemanticsBuilder<'arena>, expr: &'arena Expression<'arena>) {
        todo!()
    }
}

impl <'arena> Readable<'arena> for RegZMM {
    fn read(&self, semantics: &SemanticsBuilder<'arena>) -> &'arena Expression<'arena> {
        todo!()
    }
}

impl <'arena> Writeable<'arena> for RegZMM {
    fn write(&self, semantics: &mut SemanticsBuilder<'arena>, expr: &'arena Expression<'arena>) {
        todo!()
    }
}
