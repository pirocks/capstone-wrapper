use wrapper_common::memory_operand::GeneralReg;
use xed_enum::ADC;

use crate::x86_machine::{SemanticBuilderCommon, SemanticsBuilder, X86MachineState};

//can be a memory address or register
pub trait Writeable<T> {}

//can be a memory address or immediate
pub trait Readable<T> {}



pub fn adc_generic<T1, T2, D1: Writeable<T1>, S1: Readable<T1>, S2: Readable<T2>>(semantics: SemanticsBuilder, writeable1: D1, readable1: S1, readable2: S2) {
    todo!()
}

impl X86MachineState<'_> {
    pub fn apply_iform_adc(&mut self, adc: ADC) {
        let mut semantics = SemanticsBuilder::new();
        Self::apply_iform_adc_impl(semantics, adc);
    }

    fn apply_iform_adc_impl(semantics: SemanticsBuilder, adc: ADC) {
        match adc {
            ADC::ADC_AL_IMMB { operand_0 } => {
                adc_generic(semantics, semantics.al(), semantics.al(), operand_0);
            }
            ADC::ADC_ORAX_IMMZ { operand_0 } => {
                adc_generic(semantics, semantics.rax(), semantics.rax(), operand_0);
            }
            ADC::ADC_GPRV_IMMZ { operand_0, operand_1 } => {
                match operand_0 {
                    GeneralReg::Reg64(reg) => {
                        adc_generic(semantics, semantics.get_reg_64(reg), semantics.get_reg_64(reg), operand_1.unwrap_u64());
                    }
                    GeneralReg::Reg32(reg) => {
                        adc_generic(semantics, semantics.get_reg_32(reg), semantics.get_reg_32(reg), operand_1.unwrap_i32());
                    }
                    GeneralReg::Reg16(reg) => {
                        adc_generic(semantics, semantics.get_reg_16(reg), semantics.get_reg_16(reg), operand_1.unwrap_i16());
                    }
                    GeneralReg::Reg8(reg) => {
                        adc_generic(semantics, semantics.get_reg_8(reg), semantics.get_reg_8(reg), operand_1.unwrap_i8());
                    }
                }
            }
            ADC::ADC_GPRV_IMMB { operand_0, operand_1 } => {
                match operand_0 {
                    GeneralReg::Reg64(reg) => {
                        adc_generic(semantics, semantics.get_reg_64(reg), semantics.get_reg_64(reg), operand_1);
                    }
                    GeneralReg::Reg32(reg) => {
                        adc_generic(semantics, semantics.get_reg_32(reg), semantics.get_reg_32(reg), operand_1);
                    }
                    GeneralReg::Reg16(reg) => {
                        adc_generic(semantics, semantics.get_reg_16(reg), semantics.get_reg_16(reg), operand_1);
                    }
                    GeneralReg::Reg8(reg) => {
                        adc_generic(semantics, semantics.get_reg_8(reg), semantics.get_reg_8(reg), operand_1);
                    }
                }
            }
            ADC::ADC_MEMV_IMMZ { operand_0, operand_1 } => {
                adc_generic(semantics, operand_0, operand_0, operand_1);
            }
            ADC::ADC_MEMB_GPR8 { operand_0, operand_1 } => {
                adc_generic(semantics, operand_0, operand_0, operand_1);
            }
            ADC::ADC_GPRV_GPRV_11 { operand_0, operand_1 } => {
                adc_generic(semantics, operand_0, operand_0, operand_1);
            }
            ADC::ADC_GPR8_GPR8_10 { operand_0, operand_1 } => {
                todo!()
            }
            ADC::ADC_MEMB_IMMB_80R2 { operand_0, operand_1 } => {
                todo!()
            }
            ADC::ADC_GPR8_MEMB { .. } => {
                todo!()
            }
            ADC::ADC_GPRV_MEMV { .. } => {
                todo!()
            }
            ADC::ADC_GPR8_IMMB_82R2 { .. } => {
                todo!()
            }
            ADC::ADC_GPRV_GPRV_13 { .. } => {
                todo!()
            }
            ADC::ADC_GPR8_GPR8_12 { .. } => {
                todo!()
            }
            ADC::ADC_MEMV_IMMB { .. } => {
                todo!()
            }
            ADC::ADC_MEMV_GPRV { .. } => {
                todo!()
            }
            ADC::ADC_GPR8_IMMB_80R2 { .. } => {
                todo!()
            }
            ADC::ADC_MEMB_IMMB_82R2 { .. } => {
                todo!()
            }
            ADC::ADC_LOCK_MEMB_GPR8 { operand_0, operand_1 } => {
                Self::apply_iform_adc_impl(semantics, ADC::ADC_MEMB_GPR8 { operand_0, operand_1 })
            }
            ADC::ADC_LOCK_MEMB_IMMB_80R2 { .. } => {
                Self::apply_iform_adc_impl(semantics, ADC::ADC_MEMB_IMMB_80R2 { operand_0, operand_1 })
            }
            ADC::ADC_LOCK_MEMV_IMMB { operand_0, operand_1 } => {
                Self::apply_iform_adc_impl(semantics, ADC::ADC_MEMV_IMMB { operand_0, operand_1 })
            }
            ADC::ADC_LOCK_MEMB_IMMB_82R2 { operand_0, operand_1 } => {
                Self::apply_iform_adc_impl(semantics, ADC::ADC_MEMB_IMMB_82R2 { operand_0, operand_1 })
            }
            ADC::ADC_LOCK_MEMV_IMMZ { operand_0, operand_1 } => {
                Self::apply_iform_adc_impl(semantics, ADC::ADC_MEMV_IMMZ { operand_0, operand_1 })
            }
            ADC::ADC_LOCK_MEMV_GPRV { operand_0, operand_1 } => {
                Self::apply_iform_adc_impl(semantics, ADC::ADC_MEMV_GPRV { operand_0, operand_1 })
            }
        }
    }
}
