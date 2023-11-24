use bumpalo::Bump;

use wrapper_common::registers::{Reg16WithRIP, Reg32WithRIP, Reg64WithRIP, Reg8};

use crate::x86_machine::accessors::{DWordAccessor, QWordAccessor, WordAccessor};
use crate::x86_machine::values::{BoolValue, ByteValue, CompareType, DWordValue, NumericValue, QWordValue, Value, WordValue};
use crate::x86_machine::{X86MachineState, X86Mode};

pub trait ArenaRef<'arena, T> {
    fn arena_ref(self, arena: &'arena Bump) -> &'arena T;
}

impl <'arena, T> ArenaRef<'arena, T> for &'arena T {
    fn arena_ref(self, _arena: &'arena Bump) -> &'arena T {
        self
    }
}
impl<'arena, T> ArenaRef<'arena, T> for T {
    fn arena_ref(self, arena: &'arena Bump) -> &'arena T {
        arena.alloc(self)
    }
}

pub struct SemanticsBuilder<'arena> {
    arena: &'arena Bump,
}

impl<'l, 'arena> SemanticsBuilder<'arena> {
    pub fn new(arena: &'arena Bump) -> Self {
        Self {
            arena,
        }
    }

    pub fn undefined_exception_if_64_bit(self, state: &X86MachineState<'arena>) -> SemanticsBuilder32Impl<'arena> {
        match state.mode {
            X86Mode::Real => {
                SemanticsBuilder32Impl { arena: state.arena }
            }
            X86Mode::Protected => {
                todo!()
            }
            X86Mode::_64Bit => {
                todo!()
            }
        }

    }

    //64 bit registers:

    pub(crate) fn rax(&self) -> QWordValue {
        todo!()
    }

    pub(crate) fn rax_accessor(&self) -> QWordAccessor {
        QWordAccessor {
            reg: Reg64WithRIP::RAX,
        }
    }

    pub(crate) fn get_reg_64(&self, reg: Reg64WithRIP) -> QWordValue {
        todo!()
    }
}


impl<'arena> SemanticBuilder16<'arena> for SemanticsBuilder<'arena> {
    fn arena(&self) -> &'arena Bump {
        self.arena
    }
}

impl<'arena> SemanticBuilder32<'arena> for SemanticsBuilder<'arena> {}

impl<'arena> SemanticsBuilder64Ext<'arena> for SemanticsBuilder<'arena> {}

pub struct SemanticsBuilder32Impl<'arena> {
    arena: &'arena Bump,
}

impl<'arena> SemanticsBuilder32Impl<'arena> {
    pub fn emit_conditional<T: Value<'arena> + 'arena>(
        &self,
        state: &'_ X86MachineState<'arena>,
        condition: BoolValue<'arena>,
        true_case: impl FnOnce(&'_ X86MachineState<'arena>,&SemanticsBuilder32Impl<'arena>) -> T + 'arena,
        false_case: impl FnOnce(&'_ X86MachineState<'arena>,&SemanticsBuilder32Impl<'arena>) -> T + 'arena,
    ) -> T {
        let arena = self.arena;
        T::conditional(self.arena.alloc(condition),arena.alloc(true_case(state,self)), arena.alloc(false_case(state,self)))
    }
}

impl<'arena> SemanticBuilder16<'arena> for SemanticsBuilder32Impl<'arena> {
    fn arena(&self) -> &'arena Bump {
        self.arena
    }
}

impl<'arena> SemanticBuilder32<'arena> for SemanticsBuilder32Impl<'arena> {}

impl<'arena> SemanticsBuilder32Ext<'arena> for SemanticsBuilder32Impl<'arena> {}

pub trait SemanticBuilder16<'arena> {

    fn arena(&self) -> &'arena Bump;
    fn constant_16(&self, num_in: u16) -> WordValue<'arena> {
        todo!()
    }


    fn ax_accessor(&self) -> WordAccessor {
        WordAccessor {
            reg: Reg16WithRIP::AX,
        }
    }
}

pub trait SemanticBuilder32<'arena>: SemanticBuilder16<'arena> {

    // 32 bit regs:

    fn eax_accessor(&self) -> DWordAccessor {
        DWordAccessor {
            reg: Reg32WithRIP::EAX,
        }
    }

    // generic regs:

    fn get_reg_8(&self, reg: Reg8) -> ByteValue<'arena> {
        todo!()
    }

    fn get_reg_16(&self, reg: Reg16WithRIP) -> WordValue<'arena> {
        todo!()
    }

    fn get_reg_32(&self, reg: Reg32WithRIP) -> DWordValue<'arena> {
        todo!()
    }

    //comparison

    fn less<T: NumericValue<'arena> + 'arena>(&self, left: impl ArenaRef<'arena, T>, right: impl ArenaRef<'arena, T>) -> &'arena BoolValue<'arena> {
        let left:&'arena T = left.arena_ref(self.arena());
        let right:&'arena T = right.arena_ref(self.arena());
        self.arena().alloc(left.compare_bool(CompareType::Less, right))
    }

    fn equal<T: NumericValue<'arena> + 'arena>(&self, left: impl ArenaRef<'arena, T>, right: impl ArenaRef<'arena, T>) -> &'arena BoolValue<'arena> {
        let left:&'arena T = left.arena_ref(self.arena());
        let right:&'arena T = right.arena_ref(self.arena());
        self.arena().alloc(left.compare_bool(CompareType::Equal, right))
    }

    // bitwise logic
    fn bitwise_and<T: NumericValue<'arena>>(&self, left: impl ArenaRef<'arena, T>, right: impl ArenaRef<'arena, T>) -> T {
        todo!()
    }

    fn bitwise_xor<T: NumericValue<'arena>>(&self, left: impl ArenaRef<'arena, T>, right: impl ArenaRef<'arena, T>) -> T {
        todo!()
    }
}

pub trait ConstantBuilder<'arena, NumIn, NumOut: NumericValue<'arena>> {
    fn constant(&self, num_in: NumIn) -> &'arena NumOut;
}

impl <'arena> ConstantBuilder<'arena, bool,BoolValue<'arena>> for SemanticsBuilder32Impl<'arena>{
    fn constant(&self, num_in: bool) -> &'arena BoolValue<'arena> {
        self.arena.alloc(match num_in {
            true => BoolValue::True,
            false => BoolValue::False,
        })
    }
}

impl <'arena> ConstantBuilder<'arena, u8,ByteValue<'arena>> for SemanticsBuilder32Impl<'arena>{
    fn constant(&self, num_in: u8) -> &'arena ByteValue<'arena> {
        self.arena.alloc(ByteValue::Constant(num_in))
    }
}

impl <'arena> ConstantBuilder<'arena, u16,WordValue<'arena>> for SemanticsBuilder32Impl<'arena>{
    fn constant(&self, num_in: u16) -> &'arena WordValue<'arena> {
        self.arena.alloc(WordValue::Constant(num_in))
    }
}

impl <'arena> ConstantBuilder<'arena, u32,DWordValue<'arena>> for SemanticsBuilder32Impl<'arena>{
    fn constant(&self, num_in: u32) -> &'arena DWordValue<'arena> {
        todo!()
    }
}


pub trait SemanticsBuilder32Ext<'arena> {
    /*fn constant<NumIn, NumOut: NumericValue<'arena>>(&self, num_in: NumIn) -> NumOut {
        todo!()
    }*/
}

pub trait SemanticsBuilder64Ext<'arena>: SemanticBuilder32<'arena> {
    fn unsigned_add<T: NumericValue<'arena>>(&self, lhs: T, rhs: T) -> T {
        todo!()
    }

    fn constant<NumIn, NumOut: NumericValue<'arena>>(&self, num_in: NumIn) -> NumOut {
        todo!()
    }


    //more generic manipulation


    fn emit_zext_to<To, From>(&self, from: From) -> To {
        todo!()
    }


    //flag computation


    //[[gnu::const]] [[gnu::always_inline]] inline static bool
    // ParityFlag(uint8_t r0) {
    //   return !__builtin_parity(static_cast<unsigned>(r0));
    // }
    fn parity_flag<T: NumericValue<'arena>>(&self, state: &X86MachineState<'arena>, arena: &'arena Bump, r0: T) -> &'arena BoolValue<'arena> {
        todo!()
    }


    fn aux_carry_flag<T: NumericValue<'arena>>(&self, state: &X86MachineState<'arena>, arena: &'arena Bump, lhs: T, rhs: T, res: T) -> &'arena BoolValue<'arena> {
        //template <typename T>
        // [[gnu::const]] [[gnu::always_inline]] inline static bool
        // AuxCarryFlag(T lhs, T rhs, T res) {
        //   return ((res ^ lhs ^ rhs) & T(0x10));
        // }
        let this_gets_implicit_cast_to_bool = self.bitwise_and(
            self.bitwise_xor(
                self.bitwise_xor(res, lhs),
                rhs,
            ),
            self.constant::<_,T>(0x10),
        );
        todo!()
    }

    fn zero_flag<T>(&self, state: &X86MachineState<'arena>, arena: &'arena Bump, res: T, lhs: T, rhs: T) -> &'arena BoolValue<'arena> {
        // template <typename T, typename S1, typename S2>
        //     [[gnu::const]] [[gnu::always_inline]] inline static bool
        // NotZeroFlag(T res, S1 lhs, S2 rhs) {
        //     return !__remill_flag_computation_zero(T(0) == res, lhs, rhs, res);
        // }
        todo!()
    }

    fn sign_flag<T>(&self, state: &X86MachineState<'arena>, arena: &'arena Bump, res: T, lhs: T, rhs: T) -> BoolValue<'arena> {
        // template <typename T, typename S1, typename S2>
        //     [[gnu::const]] [[gnu::always_inline]] inline static bool SignFlag(T res, S1 lhs,
        //                                                                       S2 rhs) {
        //     return __remill_flag_computation_sign(0 > Signed(res), lhs, rhs, res);
        // }
        todo!()
    }

    fn overflow_flag<T>(&self, state: &X86MachineState<'arena>, arena: &'arena Bump, overflow_tag: FlagTag, res: T, lhs: T, rhs: T) -> &'arena BoolValue<'arena> {
        todo!()
    }
    fn write_flags_inc_dec<T: NumericValue<'arena>>(&self, state: &mut X86MachineState<'arena>, arena: &'arena Bump, overflow_tag: FlagTag, lhs: T, rhs: T, res: T) {
        //template <typename Tag, typename T>
        // [[gnu::always_inline]] inline static void WriteFlagsIncDec(State &state, T lhs,
        //                                                            T rhs, T res) {
        //   state.aflag.pf = ParityFlag(res);
        //   state.aflag.af = AuxCarryFlag(lhs, rhs, res);
        //   state.aflag.zf = ZeroFlag(res, lhs, rhs);
        //   state.aflag.sf = SignFlag(res, lhs, rhs);
        //   state.aflag.of = Overflow<Tag>::Flag(lhs, rhs, res);
        // }
        state.set_pf(arena, self.parity_flag(state, arena, res));
        state.set_af(arena, self.aux_carry_flag(state, arena, lhs, rhs, res));
        state.set_zf(arena, self.zero_flag(state, arena, res, lhs, rhs));
        state.set_sf(arena, self.sign_flag(state, arena, res, lhs, rhs));
        state.set_of(arena, self.overflow_flag(state, arena, overflow_tag, res, lhs, rhs));
    }

    //template <typename TagT, typename T>
    // [[gnu::always_inline]] inline static bool CarryFlag(T a, T b, T ab, T c,
    //                                                     T abc) {
    //   static_assert(std::is_unsigned<T>::value,
    //                 "Invalid specialization of `CarryFlag` for addition.");
    //   return Carry<TagT>::Flag(a, b, ab) || Carry<TagT>::Flag(ab, c, abc);
    // }
    fn carry_flag<T>(&self, carry_tag: FlagTag, a: T, b: T, ab: T, c: T, abc: T) -> &'arena BoolValue<'arena> {
        todo!()
    }
}

pub enum FlagTag {
    Add,
    Sub,
    Mul,
}
