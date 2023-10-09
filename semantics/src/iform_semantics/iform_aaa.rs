use xed_enum::AAA;

use crate::x86_machine::{SemanticsBuilder, X86MachineState};

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

impl X86MachineState<'_> {
    //for making xed enum_defs xed has all the tables that are needed
    pub fn apply_iform_aaa(&mut self, iform_aaa: AAA) {
        match iform_aaa {
            AAA::AAA {} => {
                let mut semantics = SemanticsBuilder::new();
                let mut semantics = semantics.undefined_exception_if_64_bit();
                let condition = semantics.less(semantics.constant(9), semantics.al() & semantics.constant(0)) |
                    semantics.equal(semantics.af(), semantics.constant(true));
                semantics.emit_conditional(condition, |mut semantics| {
                    semantics.set_ax(semantics.ax() + semantics.constant(0x106));
                    semantics.set_af(semantics.constant(true));
                    semantics.set_cf(semantics.constant(true));
                }, |mut semantics| {
                    semantics.set_af(semantics.constant(false));
                    semantics.set_cf(semantics.constant(false));
                });
                semantics.set_al(semantics.al() & semantics.constant(0));//todo is this a zero?
            }
        }
    }
}
