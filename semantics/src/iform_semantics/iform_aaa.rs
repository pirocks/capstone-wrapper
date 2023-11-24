use bumpalo::Bump;
use xed_enum::AAA;

use crate::x86_machine::{X86MachineState};
use crate::x86_machine::semantics_builder::{ConstantBuilder, SemanticBuilder32, SemanticsBuilder, SemanticsBuilder32Ext};
use crate::x86_machine::values::{BoolValue, EmptyValue};

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

impl <'arena> X86MachineState<'arena> {
    //for making xed enum_defs xed has all the tables that are needed
    pub fn apply_iform_aaa(&mut self,arena: &'arena Bump, iform_aaa: AAA) {
        match iform_aaa {
            AAA::AAA {} => {
                let mut semantics = SemanticsBuilder::new(arena);
                let mut semantics = semantics.undefined_exception_if_64_bit(self);
                let condition = semantics.less(
                    semantics.constant(9),
                    self.al() & semantics.constant(0xF),
                ) | semantics.equal::<BoolValue<'arena>>(self.af(), semantics.constant(true));
                self.set_ax(arena,semantics.emit_conditional(
                    self,
                    condition,
                    |state, mut semantics| {
                        state.ax() + semantics.constant(0x106)
                    },
                    |state, mut semantics| {
                        *semantics.constant(0)
                    },
                ));

                self.set_af(arena,semantics.emit_conditional(
                    self,
                    condition,
                    |state, mut semantics| {
                        *semantics.constant(true)
                    },
                    |state, mut semantics| {
                        *semantics.constant(false)
                    },
                ));

                self.set_cf(arena,semantics.emit_conditional(
                    self,
                    condition,
                    |state, mut semantics| {
                        *semantics.constant(true)
                    },
                    |state, mut semantics| {
                        *semantics.constant(false)
                    },
                ));

                self.set_al(arena,arena.alloc(self.al() & semantics.constant(0xF)));
            }
        }
    }
}
