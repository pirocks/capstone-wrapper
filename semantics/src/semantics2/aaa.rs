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


pub fn apply_iform_aaa<'arena>(arena: Arena<'arena>, iform_aaa: AAA) -> Vec<InstructionSemanticsStep<'arena>> {
    match iform_aaa {
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


impl <'arena> SemanticsBuilder<'arena> {
    //[[gnu::const]] [[gnu::always_inline]] inline static bool
    // ParityFlag(uint8_t r0) {
    //   return !__builtin_parity(static_cast<unsigned>(r0));
    // }
    fn parity_flag(&self, r0: &'arena Expression<'arena>) -> &'arena Expression<'arena> {
        todo!()
    }


    fn aux_carry_flag(&self, lhs: &'arena Expression<'arena>, rhs: &'arena Expression<'arena>, res: &'arena Expression<'arena>) -> &'arena Expression<'arena> {
        //template <typename T>
        // [[gnu::const]] [[gnu::always_inline]] inline static bool
        // AuxCarryFlag(T lhs, T rhs, T res) {
        //   return ((res ^ lhs ^ rhs) & T(0x10));
        // }
        let this_gets_implicit_cast_to_bool = self.bitand(
            self.bitxor(
                self.bitxor(res, lhs),
                rhs,
            ),
            self.constant(0x10u8),
        );
        todo!()
    }

    fn zero_flag(&self, res: &'arena Expression<'arena>, lhs: &'arena Expression<'arena>, rhs: &'arena Expression<'arena>) -> &'arena Expression<'arena> {
        // template <typename T, typename S1, typename S2>
        //     [[gnu::const]] [[gnu::always_inline]] inline static bool
        // NotZeroFlag(T res, S1 lhs, S2 rhs) {
        //     return !__remill_flag_computation_zero(T(0) == res, lhs, rhs, res);
        // }
        todo!()
    }

    fn sign_flag(&self, res: &'arena Expression<'arena>, lhs: &'arena Expression<'arena>, rhs: &'arena Expression<'arena>) -> &'arena Expression<'arena> {
        // template <typename T, typename S1, typename S2>
        //     [[gnu::const]] [[gnu::always_inline]] inline static bool SignFlag(T res, S1 lhs,
        //                                                                       S2 rhs) {
        //     return __remill_flag_computation_sign(0 > Signed(res), lhs, rhs, res);
        // }
        todo!()
    }

    fn overflow_flag(&self, overflow_tag: FlagTag, res: &'arena Expression<'arena>, lhs: &'arena Expression<'arena>, rhs: &'arena Expression<'arena>) -> &'arena Expression<'arena> {
        todo!()
    }

    //template <typename TagT, typename T>
    // [[gnu::always_inline]] inline static bool CarryFlag(T a, T b, T ab, T c,
    //                                                     T abc) {
    //   static_assert(std::is_unsigned<T>::value,
    //                 "Invalid specialization of `CarryFlag` for addition.");
    //   return Carry<TagT>::Flag(a, b, ab) || Carry<TagT>::Flag(ab, c, abc);
    // }
    pub fn carry_flag(&self, flag_tag: FlagTag, a: &'arena Expression<'arena>, b: &'arena Expression<'arena>, ab: &'arena Expression<'arena>, c: &'arena Expression<'arena>, abc: &'arena Expression<'arena>) -> &'arena Expression<'arena> {
        todo!()
    }

    fn write_flags_inc_dec(&mut self, overflow_tag: FlagTag, lhs: &'arena Expression<'arena>, rhs: &'arena Expression<'arena>, res: &'arena Expression<'arena>) {
        //template <typename Tag, typename T>
        // [[gnu::always_inline]] inline static void WriteFlagsIncDec(State &state, T lhs,
        //                                                            T rhs, T res) {
        //   state.aflag.pf = ParityFlag(res);
        //   state.aflag.af = AuxCarryFlag(lhs, rhs, res);
        //   state.aflag.zf = ZeroFlag(res, lhs, rhs);
        //   state.aflag.sf = SignFlag(res, lhs, rhs);
        //   state.aflag.of = Overflow<Tag>::Flag(lhs, rhs, res);
        // }
        self.set_pf(self.parity_flag(res));
        self.set_af(self.aux_carry_flag(lhs, rhs, res));
        self.set_zf(self.zero_flag(res, lhs, rhs));
        self.set_sf(self.sign_flag(res, lhs, rhs));
        self.set_of(self.overflow_flag(overflow_tag, res, lhs, rhs));
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
    s.set_cf(s.carry_flag(FlagTag::Add, lhs, rhs, sum, carry, res));
    s.write_flags_inc_dec(FlagTag::Add, lhs, rhs, res);
    s.finalize()
}

pub fn apply_iform_adc<'arena>(arena: Arena<'arena>, adc: ADC) {
    apply_iform_adc_impl(arena, adc);
}

fn apply_iform_adc_impl<'arena>(arena: Arena<'arena>, adc: ADC) {
    match adc {
        ADC::ADC_AL_IMMB { operand_0 } => {
            adc_generic(arena, Reg8::AL, Reg8::AL, operand_0);
        }
        ADC::ADC_MEMB_GPR8 {
            operand_0,
            operand_1,
        } => {
            adc_generic(arena, operand_0, operand_0, operand_1);
        }
        ADC::ADC_GPR8_GPR8_10 {
            operand_0,
            operand_1,
        } => {
            adc_generic(arena, operand_0, operand_0, operand_1);
        }
        ADC::ADC_MEMB_IMMB_80R2 {
            operand_0,
            operand_1,
        } => {
            adc_generic(arena, operand_0, operand_0, operand_1);
        }
        ADC::ADC_GPR8_MEMB { operand_0, operand_1 } => {
            adc_generic(arena, operand_0, operand_0, operand_1);
        }
        ADC::ADC_GPR8_IMMB_82R2 { operand_0, operand_1 } => {
            adc_generic(arena, operand_0, operand_0, operand_1);
        }
        ADC::ADC_GPR8_GPR8_12 { operand_0, operand_1 } => {
            adc_generic(arena, operand_0, operand_0, operand_1);
        }
        ADC::ADC_GPR8_IMMB_80R2 { operand_0, operand_1 } => {
            adc_generic(arena, operand_0, operand_0, operand_1);
        }
        ADC::ADC_MEMB_IMMB_82R2 { operand_0, operand_1 } => {
            adc_generic(arena, operand_0, operand_0, operand_1);
        }
        ADC::ADC_GPRV_GPRV_11_16 { operand_0, operand_1 } => {
            adc_generic(arena, operand_0, operand_0, operand_1);
        }
        ADC::ADC_GPRV_GPRV_11_32 { operand_0, operand_1 } => {
            adc_generic(arena, operand_0, operand_0, operand_1);
        }
        ADC::ADC_GPRV_GPRV_11_64 { operand_0, operand_1 } => {
            adc_generic(arena, operand_0, operand_0, operand_1);
        }
        ADC::ADC_GPRV_GPRV_13_16 { operand_0, operand_1 } => {
            adc_generic(arena, operand_0, operand_0, operand_1);
        }
        ADC::ADC_GPRV_GPRV_13_32 { operand_0, operand_1 } => {
            adc_generic(arena, operand_0, operand_0, operand_1);
        }
        ADC::ADC_GPRV_GPRV_13_64 { operand_0, operand_1 } => {
            adc_generic(arena, operand_0, operand_0, operand_1);
        }
        ADC::ADC_GPRV_IMMB_16 { operand_0, operand_1 } => {
            adc_generic(arena, operand_0, operand_0, operand_1);
        }
        ADC::ADC_GPRV_IMMB_32 { operand_0, operand_1 } => {
            adc_generic(arena, operand_0, operand_0, operand_1);
        }
        ADC::ADC_GPRV_IMMB_64 { operand_0, operand_1 } => {
            adc_generic(arena, operand_0, operand_0, operand_1);
        }
        ADC::ADC_GPRV_IMMZ_16 { operand_0, operand_1 } => {
            adc_generic(arena, operand_0, operand_0, operand_1);
        }
        ADC::ADC_GPRV_IMMZ_32 { operand_0, operand_1 } => {
            adc_generic(arena, operand_0, operand_0, operand_1);
        }
        ADC::ADC_GPRV_IMMZ_64 { operand_0, operand_1 } => {
            adc_generic(arena, operand_0, operand_0, operand_1);
        }
        ADC::ADC_GPRV_MEMV_16 { operand_0, operand_1 } => {
            adc_generic(arena, operand_0, operand_0, operand_1);
        }
        ADC::ADC_GPRV_MEMV_32 { operand_0, operand_1 } => {
            adc_generic(arena, operand_0, operand_0, operand_1);
        }
        ADC::ADC_GPRV_MEMV_64 { operand_0, operand_1 } => {
            adc_generic(arena, operand_0, operand_0, operand_1);
        }
        ADC::ADC_MEMV_GPRV_16 { operand_0, operand_1 } => {
            adc_generic(arena, operand_0, operand_0, operand_1);
        }
        ADC::ADC_MEMV_GPRV_32 { operand_0, operand_1 } => {
            adc_generic(arena, operand_0, operand_0, operand_1);
        }
        ADC::ADC_MEMV_GPRV_64 { operand_0, operand_1 } => {
            adc_generic(arena, operand_0, operand_0, operand_1);
        }
        ADC::ADC_MEMV_IMMB_16 { operand_0, operand_1 } => {
            adc_generic(arena, operand_0, operand_0, operand_1);
        }
        ADC::ADC_MEMV_IMMB_32 { operand_0, operand_1 } => {
            adc_generic(arena, operand_0, operand_0, operand_1);
        }
        ADC::ADC_MEMV_IMMB_64 { operand_0, operand_1 } => {
            adc_generic(arena, operand_0, operand_0, operand_1);
        }
        ADC::ADC_MEMV_IMMZ_16 { operand_0, operand_1 } => {
            adc_generic(arena, operand_0, operand_0, operand_1);
        }
        ADC::ADC_MEMV_IMMZ_32 { operand_0, operand_1 } => {
            adc_generic(arena, operand_0, operand_0, operand_1);
        }
        ADC::ADC_MEMV_IMMZ_64 { operand_0, operand_1 } => {
            adc_generic(arena, operand_0, operand_0, operand_1);
        }
        ADC::ADC_ORAX_IMMZ_16 { operand_0 } => {
            adc_generic(arena, Reg16WithRIP::AX, Reg16WithRIP::AX, operand_0);
        }
        ADC::ADC_ORAX_IMMZ_32 { operand_0 } => {
            adc_generic(arena, Reg32WithRIP::EAX, Reg32WithRIP::EAX, operand_0);
        }
        ADC::ADC_ORAX_IMMZ_64 { operand_0 } => {
            adc_generic(arena, Reg64WithRIP::RAX, Reg64WithRIP::RAX, operand_0);
        }
    }
}



