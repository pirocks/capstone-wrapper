use xed_enum::X86Instruction;
use crate::x86_machine::X86MachineState;

pub mod iform_aaa;
pub mod iform_adc{
    use xed_enum::ADC;
    use crate::x86_machine::X86MachineState;

    impl X86MachineState {
        pub fn apply_iform_adc(&mut self, adc: ADC) {
            match adc {
                ADC::ADC_GPRV_IMMZ { operand_0, operand_1 } => {

                }
                _ => todo!()
            }
        }
    }
}

impl X86MachineState {
    fn apply_instruction(&mut self, instr: X86Instruction) {
        match instr {
            X86Instruction::AAA(aaa) => self.apply_iform_aaa(aaa),
            X86Instruction::ADC(adc) => self.apply_iform_adc(adc),
            _ => todo!()
        }
    }
}