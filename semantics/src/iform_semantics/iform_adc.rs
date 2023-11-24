use bumpalo::Bump;
use wrapper_common::registers::{Reg16WithRIP, Reg32WithRIP, Reg64WithRIP, Reg8};
use xed_enum::ADC;

use crate::x86_machine::read_write::{Readable, Writeable};
use crate::x86_machine::semantics_builder::{FlagTag, SemanticsBuilder, SemanticsBuilder64Ext};
use crate::x86_machine::values::{ByteValue, DWordValue, NumericValue, QWordValue, WordValue};
use crate::x86_machine::X86MachineState;

//can be a memory address or register


fn adc_generic<'arena, T1: NumericValue<'arena>, T2: NumericValue<'arena>, D1: Writeable<'arena,T1>, S1: Readable<'arena,T1>, S2: Readable<'arena,T2>>(
    state: &mut X86MachineState<'arena>,
    arena: &'arena Bump,
    writeable1: D1,
    readable1: S1,
    readable2: S2,
) {
//template <typename D, typename S1, typename S2>
// DEF_SEM(ADC, D dst, S1 src1, S2 src2) {
//   auto lhs = Read(src1);
//   auto rhs = Read(src2);
//   auto carry = ZExtTo<S1>(Unsigned(Read(FLAG_CF)));
//   auto sum = UAdd(lhs, rhs);
//   auto res = UAdd(sum, carry);
//   WriteZExt(dst, res);
//   Write(FLAG_CF, CarryFlag<tag_add>(lhs, rhs, sum, carry, res));
//   WriteFlagsIncDec<tag_add>(state, lhs, rhs, res);
//   return memory;
// }

    let semantics = SemanticsBuilder::new(arena);
    // lhs is bigger than rhs
    let lhs = readable1.emit_read(state,&semantics);
    let rhs: T1 = semantics.emit_zext_to(readable2.emit_read(state,&semantics));
    let carry = semantics.emit_zext_to(state.cf());
    let sum = semantics.unsigned_add(lhs, rhs);
    let res = semantics.unsigned_add(sum, carry);
    writeable1.emit_write(state, semantics.emit_zext_to(res));
    state.set_cf(arena,semantics.carry_flag(FlagTag::Add, lhs, rhs, sum, carry, res));
    semantics.write_flags_inc_dec(state,arena, FlagTag::Add, lhs, rhs, res);
}

impl <'arena> X86MachineState<'arena> {
    pub fn apply_iform_adc(&mut self,arena: &'arena Bump, adc: ADC) {
        self.apply_iform_adc_impl(arena,adc);
    }

    fn apply_iform_adc_impl(&mut self,arena: &'arena Bump, adc: ADC) {
        match adc {
            ADC::ADC_AL_IMMB { operand_0 } => {
                adc_generic(self,arena, Reg8::AL, Reg8::AL, operand_0);
            }
            ADC::ADC_MEMB_GPR8 {
                operand_0,
                operand_1,
            } => {
                adc_generic::<ByteValue<'_>, _, _, _, _>(self,arena, operand_0, operand_0, operand_1);
            }
            ADC::ADC_GPR8_GPR8_10 {
                operand_0,
                operand_1,
            } => {
                adc_generic(self,arena, operand_0, operand_0, operand_1);
            }
            ADC::ADC_MEMB_IMMB_80R2 {
                operand_0,
                operand_1,
            } => {
                adc_generic::<ByteValue<'_>, _, _, _, _>(self,arena, operand_0, operand_0, operand_1);
            }
            ADC::ADC_GPR8_MEMB { operand_0, operand_1 } => {
                adc_generic::<_, ByteValue<'_>, _, _, _>(self,arena, operand_0, operand_0, operand_1);
            }
            ADC::ADC_GPR8_IMMB_82R2 { operand_0, operand_1 } => {
                adc_generic(self,arena, operand_0, operand_0, operand_1);
            }
            ADC::ADC_GPR8_GPR8_12 { operand_0, operand_1 } => {
                adc_generic(self,arena, operand_0, operand_0, operand_1);
            }
            ADC::ADC_GPR8_IMMB_80R2 { operand_0, operand_1 } => {
                adc_generic(self,arena, operand_0, operand_0, operand_1);
            }
            ADC::ADC_MEMB_IMMB_82R2 { operand_0, operand_1 } => {
                adc_generic::<ByteValue<'_>, _, _, _, _>(self,arena, operand_0, operand_0, operand_1);
            }
            ADC::ADC_GPRV_GPRV_11_16 { operand_0, operand_1 } => {
                adc_generic(self,arena, operand_0, operand_0, operand_1);
            }
            ADC::ADC_GPRV_GPRV_11_32 { operand_0, operand_1 } => {
                adc_generic(self,arena, operand_0, operand_0, operand_1);
            }
            ADC::ADC_GPRV_GPRV_11_64 { operand_0, operand_1 } => {
                adc_generic(self,arena, operand_0, operand_0, operand_1);
            }
            ADC::ADC_GPRV_GPRV_13_16 { operand_0, operand_1 } => {
                adc_generic(self,arena, operand_0, operand_0, operand_1);
            }
            ADC::ADC_GPRV_GPRV_13_32 { operand_0, operand_1 } => {
                adc_generic(self,arena, operand_0, operand_0, operand_1);
            }
            ADC::ADC_GPRV_GPRV_13_64 { operand_0, operand_1 } => {
                adc_generic(self,arena, operand_0, operand_0, operand_1);
            }
            ADC::ADC_GPRV_IMMB_16 { operand_0, operand_1 } => {
                adc_generic(self,arena, operand_0, operand_0, operand_1);
            }
            ADC::ADC_GPRV_IMMB_32 { operand_0, operand_1 } => {
                adc_generic(self,arena, operand_0, operand_0, operand_1);
            }
            ADC::ADC_GPRV_IMMB_64 { operand_0, operand_1 } => {
                adc_generic(self,arena, operand_0, operand_0, operand_1);
            }
            ADC::ADC_GPRV_IMMZ_16 { operand_0, operand_1 } => {
                adc_generic(self,arena, operand_0, operand_0, operand_1);
            }
            ADC::ADC_GPRV_IMMZ_32 { operand_0, operand_1 } => {
                adc_generic(self,arena, operand_0, operand_0, operand_1);
            }
            ADC::ADC_GPRV_IMMZ_64 { operand_0, operand_1 } => {
                adc_generic(self,arena, operand_0, operand_0, operand_1);
            }
            ADC::ADC_GPRV_MEMV_16 { operand_0, operand_1 } => {
                adc_generic::<_, WordValue<'_>, _, _, _>(self,arena, operand_0, operand_0, operand_1);
            }
            ADC::ADC_GPRV_MEMV_32 { operand_0, operand_1 } => {
                adc_generic::<_, DWordValue<'_>, _, _, _>(self,arena, operand_0, operand_0, operand_1);
            }
            ADC::ADC_GPRV_MEMV_64 { operand_0, operand_1 } => {
                adc_generic::<_, QWordValue<'_>, _, _, _>(self,arena, operand_0, operand_0, operand_1);
            }
            ADC::ADC_MEMV_GPRV_16 { operand_0, operand_1 } => {
                adc_generic::<WordValue<'_>, _, _, _, _>(self,arena, operand_0, operand_0, operand_1);
            }
            ADC::ADC_MEMV_GPRV_32 { operand_0, operand_1 } => {
                adc_generic::<DWordValue<'_>, _, _, _, _>(self,arena, operand_0, operand_0, operand_1);
            }
            ADC::ADC_MEMV_GPRV_64 { operand_0, operand_1 } => {
                adc_generic::<QWordValue<'_>, _, _, _, _>(self,arena, operand_0, operand_0, operand_1);
            }
            ADC::ADC_MEMV_IMMB_16 { operand_0, operand_1 } => {
                adc_generic::<WordValue<'_>, _, _, _, _>(self,arena, operand_0, operand_0, operand_1);
            }
            ADC::ADC_MEMV_IMMB_32 { operand_0, operand_1 } => {
                adc_generic::<DWordValue<'_>, _, _, _, _>(self,arena, operand_0, operand_0, operand_1);
            }
            ADC::ADC_MEMV_IMMB_64 { operand_0, operand_1 } => {
                adc_generic::<QWordValue<'_>, _, _, _, _>(self,arena, operand_0, operand_0, operand_1);
            }
            ADC::ADC_MEMV_IMMZ_16 { operand_0, operand_1 } => {
                adc_generic::<WordValue<'_>, _, _, _, _>(self,arena, operand_0, operand_0, operand_1);
            }
            ADC::ADC_MEMV_IMMZ_32 { operand_0, operand_1 } => {
                adc_generic::<DWordValue<'_>, _, _, _, _>(self,arena, operand_0, operand_0, operand_1);
            }
            ADC::ADC_MEMV_IMMZ_64 { operand_0, operand_1 } => {
                adc_generic::<QWordValue<'_>, _, _, _, _>(self,arena, operand_0, operand_0, operand_1);
            }
            ADC::ADC_ORAX_IMMZ_16 { operand_0 } => {
                adc_generic(self,arena, Reg16WithRIP::AX, Reg16WithRIP::AX, operand_0);
            }
            ADC::ADC_ORAX_IMMZ_32 { operand_0 } => {
                adc_generic(self,arena, Reg32WithRIP::EAX, Reg32WithRIP::EAX, operand_0);
            }
            ADC::ADC_ORAX_IMMZ_64 { operand_0 } => {
                adc_generic(self,arena, Reg64WithRIP::RAX, Reg64WithRIP::RAX, operand_0);
            }
        }
    }
}
