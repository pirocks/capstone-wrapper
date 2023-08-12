use xed_enum::AAA;

use crate::x86_machine::{ByteValue, QWordValue, WordValue, X86MachineState, X86Mode};

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

impl X86MachineState {
    //for making xed enum_defs xed has all the tables that are needed
    pub fn apply_iform_aaa(&mut self, iform_aaa: AAA) {
        match iform_aaa {
            AAA::AAA {} => {
                match self.mode {
                    X86Mode::Real | X86Mode::Protected => {

                        let original_rax = self.a(self.rax);
                        let original_ax = self.a(WordValue::LowerBits(original_rax));
                        let original_al = self.a(ByteValue::LowerBits(original_rax));
                        let x106 = self.a(WordValue::Constant(0x106));
                        let nine = self.a(WordValue::Constant(9));
                        let one = self.a(WordValue::Constant(1));
                        let lower = ;
                        self.a(QWordValue::WriteLowerBits { prev: original_rax, lower:  });
                        todo!()
                    }
                    X86Mode::_64Bit => {
                        self.undefined_instruction_exception();
                    }
                }
            }
        }
    }
}
