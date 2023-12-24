use wrapper_common::registers::{Reg16WithRIP, Reg32WithRIP, Reg64WithRIP, Reg8};
use xed_enum::ADC;
use crate::semantics2::arena::Arena;
use crate::semantics2::builder::SemanticsBuilder;
use crate::semantics2::expression::Expression;
use crate::semantics2::read_write::{Readable, Writeable};
use crate::semantics2::semantic_steps::InstructionSemanticsStep;
use crate::x86_machine::semantics_builder::FlagTag;

impl <'arena> SemanticsBuilder<'arena> {
    pub fn flag_calculate(&mut self, flag_tag: FlagTag, left: &'arena Expression<'arena>, right: &'arena Expression<'arena>) {
        //todo think about how to implement this.
        //CF (bit 0)Carry flag — Set if an arithmetic operation generates a carry or a borrow out of the most-
        // significant bit of the result; cleared otherwise. This flag indicates an overflow condition for
        // unsigned-integer arithmetic. It is also used in multiple-precision arithmetic.
        // PF (bit 2)Parity flag — Set if the least-significant byte of the result contains an even number of 1 bits;
        // cleared otherwise.
        // AF (bit 4)Auxiliary Carry flag — Set if an arithmetic operation generates a carry or a borrow out of bit
        // 3 of the result; cleared otherwise. This flag is used in binary-coded decimal (BCD) arithmetic.
        // ZF (bit 6)Zero flag — Set if the result is zero; cleared otherwise.
        // SF (bit 7)Sign flag — Set equal to the most-significant bit of the result, which is the sign bit of a signed
        // integer. (0 indicates a positive value and 1 indicates a negative value.)
        // OF (bit 11)Overflow flag — Set if the integer result is too large a positive number or too small a negative
        // number (excluding the sign-bit) to fit in the destination operand; cleared otherwise. This flag
        // indicates an overflow condition for signed-integer (two’s complement) arithmetic.
        self.semantics.push(InstructionSemanticsStep::CalculateFlags {
            flag_tag,
            left,
            right,
        })
    }
}


fn adc_generic<'arena, D1: Writeable<'arena>, S1: Readable<'arena>, S2: Readable<'arena>>(
    arena: Arena<'arena>,
    writeable1: D1,
    readable1: S1,
    readable2: S2,
) -> Vec<InstructionSemanticsStep<'arena>> {
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

    let mut s = SemanticsBuilder::new(arena);
    // lhs is bigger than rhs
    let lhs = readable1.read(&s);
    let readable_1_width = lhs.width();
    let rhs = s.zext_to(readable2.read(&s), readable_1_width);
    let carry = s.zext_to(s.cf(), readable_1_width);
    let sum = s.add(lhs, rhs);
    let res = s.add(sum, carry);
    let zero_extended = s.zext_to(res, readable_1_width);
    writeable1.write(&mut s, zero_extended);
    s.flag_calculate(FlagTag::Add, lhs, rhs);
    s.sync_uninterruptable();
    s.finalize()
}

pub fn apply_iform_adc(arena: Arena, adc: ADC) -> Vec<InstructionSemanticsStep> {
    apply_iform_adc_impl(arena, adc)
}

fn apply_iform_adc_impl(arena: Arena, adc: ADC) -> Vec<InstructionSemanticsStep> {
    match adc {
        ADC::ADC_AL_IMMB { operand_0 } => {
            adc_generic(arena, Reg8::AL, Reg8::AL, operand_0)
        }
        ADC::ADC_MEMB_GPR8 {
            operand_0,
            operand_1,
        } => {
            adc_generic(arena, operand_0, operand_0, operand_1)
        }
        ADC::ADC_GPR8_GPR8_10 {
            operand_0,
            operand_1,
        } => {
            adc_generic(arena, operand_0, operand_0, operand_1)
        }
        ADC::ADC_MEMB_IMMB_80R2 {
            operand_0,
            operand_1,
        } => {
            adc_generic(arena, operand_0, operand_0, operand_1)
        }
        ADC::ADC_GPR8_MEMB { operand_0, operand_1 } => {
            adc_generic(arena, operand_0, operand_0, operand_1)
        }
        ADC::ADC_GPR8_IMMB_82R2 { operand_0, operand_1 } => {
            adc_generic(arena, operand_0, operand_0, operand_1)
        }
        ADC::ADC_GPR8_GPR8_12 { operand_0, operand_1 } => {
            adc_generic(arena, operand_0, operand_0, operand_1)
        }
        ADC::ADC_GPR8_IMMB_80R2 { operand_0, operand_1 } => {
            adc_generic(arena, operand_0, operand_0, operand_1)
        }
        ADC::ADC_MEMB_IMMB_82R2 { operand_0, operand_1 } => {
            adc_generic(arena, operand_0, operand_0, operand_1)
        }
        ADC::ADC_GPRV_GPRV_11_16 { operand_0, operand_1 } => {
            adc_generic(arena, operand_0, operand_0, operand_1)
        }
        ADC::ADC_GPRV_GPRV_11_32 { operand_0, operand_1 } => {
            adc_generic(arena, operand_0, operand_0, operand_1)
        }
        ADC::ADC_GPRV_GPRV_11_64 { operand_0, operand_1 } => {
            adc_generic(arena, operand_0, operand_0, operand_1)
        }
        ADC::ADC_GPRV_GPRV_13_16 { operand_0, operand_1 } => {
            adc_generic(arena, operand_0, operand_0, operand_1)
        }
        ADC::ADC_GPRV_GPRV_13_32 { operand_0, operand_1 } => {
            adc_generic(arena, operand_0, operand_0, operand_1)
        }
        ADC::ADC_GPRV_GPRV_13_64 { operand_0, operand_1 } => {
            adc_generic(arena, operand_0, operand_0, operand_1)
        }
        ADC::ADC_GPRV_IMMB_16 { operand_0, operand_1 } => {
            adc_generic(arena, operand_0, operand_0, operand_1)
        }
        ADC::ADC_GPRV_IMMB_32 { operand_0, operand_1 } => {
            adc_generic(arena, operand_0, operand_0, operand_1)
        }
        ADC::ADC_GPRV_IMMB_64 { operand_0, operand_1 } => {
            adc_generic(arena, operand_0, operand_0, operand_1)
        }
        ADC::ADC_GPRV_IMMZ_16 { operand_0, operand_1 } => {
            adc_generic(arena, operand_0, operand_0, operand_1)
        }
        ADC::ADC_GPRV_IMMZ_32 { operand_0, operand_1 } => {
            adc_generic(arena, operand_0, operand_0, operand_1)
        }
        ADC::ADC_GPRV_IMMZ_64 { operand_0, operand_1 } => {
            adc_generic(arena, operand_0, operand_0, operand_1)
        }
        ADC::ADC_GPRV_MEMV_16 { operand_0, operand_1 } => {
            adc_generic(arena, operand_0, operand_0, operand_1)
        }
        ADC::ADC_GPRV_MEMV_32 { operand_0, operand_1 } => {
            adc_generic(arena, operand_0, operand_0, operand_1)
        }
        ADC::ADC_GPRV_MEMV_64 { operand_0, operand_1 } => {
            adc_generic(arena, operand_0, operand_0, operand_1)
        }
        ADC::ADC_MEMV_GPRV_16 { operand_0, operand_1 } => {
            adc_generic(arena, operand_0, operand_0, operand_1)
        }
        ADC::ADC_MEMV_GPRV_32 { operand_0, operand_1 } => {
            adc_generic(arena, operand_0, operand_0, operand_1)
        }
        ADC::ADC_MEMV_GPRV_64 { operand_0, operand_1 } => {
            adc_generic(arena, operand_0, operand_0, operand_1)
        }
        ADC::ADC_MEMV_IMMB_16 { operand_0, operand_1 } => {
            adc_generic(arena, operand_0, operand_0, operand_1)
        }
        ADC::ADC_MEMV_IMMB_32 { operand_0, operand_1 } => {
            adc_generic(arena, operand_0, operand_0, operand_1)
        }
        ADC::ADC_MEMV_IMMB_64 { operand_0, operand_1 } => {
            adc_generic(arena, operand_0, operand_0, operand_1)
        }
        ADC::ADC_MEMV_IMMZ_16 { operand_0, operand_1 } => {
            adc_generic(arena, operand_0, operand_0, operand_1)
        }
        ADC::ADC_MEMV_IMMZ_32 { operand_0, operand_1 } => {
            adc_generic(arena, operand_0, operand_0, operand_1)
        }
        ADC::ADC_MEMV_IMMZ_64 { operand_0, operand_1 } => {
            adc_generic(arena, operand_0, operand_0, operand_1)
        }
        ADC::ADC_ORAX_IMMZ_16 { operand_0 } => {
            adc_generic(arena, Reg16WithRIP::AX, Reg16WithRIP::AX, operand_0)
        }
        ADC::ADC_ORAX_IMMZ_32 { operand_0 } => {
            adc_generic(arena, Reg32WithRIP::EAX, Reg32WithRIP::EAX, operand_0)
        }
        ADC::ADC_ORAX_IMMZ_64 { operand_0 } => {
            adc_generic(arena, Reg64WithRIP::RAX, Reg64WithRIP::RAX, operand_0)
        }
    }
}




