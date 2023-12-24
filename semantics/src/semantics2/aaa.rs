use wrapper_common::registers::{Reg16WithRIP, Reg32WithRIP, Reg64WithRIP, Reg8};
use xed_enum::{AAA, ADC};

use crate::semantics2::arena::Arena;
use crate::semantics2::builder::SemanticsBuilder;
use crate::semantics2::expression::Expression;
use crate::semantics2::read_write::{Readable, Writeable};
use crate::semantics2::semantic_steps::InstructionSemanticsStep;
use crate::x86_machine::semantics_builder::FlagTag;
use crate::x86_machine::values::NumericValue;

///Operation
//            IF 64-Bit Mode
//               THEN
//                   #UD;
//               ELSE
//                   IF ((AL AND 0FH) > 9) or (AF = 1)
//                       THEN
//                            AX := AX + 106H;
//                            AF := 1;
//                            CF := 1;
//                       ELSE
//                            AF := 0;
//                            CF := 0;
//                   FI;
//                   AL := AL AND 0FH;
//            FI;
//


pub fn apply_iform_aaa<'arena>(arena: Arena<'arena>, aaa: AAA) -> Vec<InstructionSemanticsStep<'arena>> {
    match aaa {
        AAA::AAA {} => {
            let mut s = SemanticsBuilder::new(arena);
            s.undefined_exception_if_64_bit();
            let condition = s.bitor(s.less(
                s.constant(9u8),
                s.bitand(s.al(), s.constant(0xFu8)),
            ), s.equal(s.af(), s.constant(true)));
            s.emit_conditional(
                condition,
                |s| {
                    s.set_ax(s.add(s.ax(), s.constant(0x106u16)));
                    s.set_af(s.constant(true));
                    s.set_cf(s.constant(true));
                },
                |s| {
                    s.set_ax(s.constant(0u16));
                    s.set_af(s.constant(false));
                    s.set_cf(s.constant(false));
                },
            );
            s.set_al(s.bitand(s.al(), s.constant(0xFu8)));
            s.finalize()
        }
    }
}

