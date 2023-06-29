use crate::x86_machine::X86MachineState;


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


// xed tables
pub enum IFormAAA{

}

impl X86MachineState{
    //for making xed enum_defs xed has all the tables that are needed
    pub fn apply_iform_aaa(&mut self, iform_aaa: ){

    }

}
