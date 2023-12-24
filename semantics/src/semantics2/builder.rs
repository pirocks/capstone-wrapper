use bitvec::bitvec;
use wrapper_common::memory_operand::GeneralReg;

use wrapper_common::registers::{Reg16WithRIP, Reg64WithRIP, Reg8};

use crate::semantics2::arena::Arena;
use crate::semantics2::expression::{ArithmeticOp, BitWiseOp, ComparisonOp, Expression, Flag, Signedness};
use crate::semantics2::num_traits::IntegerWidth;
use crate::semantics2::semantic_steps::{InstructionSemanticsStep, ZeroUpper};
use crate::semantics2::value::Value;

pub struct SemanticsBuilder<'arena> {
    pub(crate) semantics: Vec<InstructionSemanticsStep<'arena>>,
    arena: Arena<'arena>,
}

impl<'arena> SemanticsBuilder<'arena> {
    pub fn new(arena: Arena<'arena>) -> Self {
        Self {
            semantics: vec![],
            arena,
        }
    }

    pub fn undefined_exception_if_64_bit(&mut self) {
        todo!()/*if state.mode == X86Mode::_64Bit {
            self.semantics.push(InstructionSemanticsStep::UndefinedException);
        }*/
    }

    pub fn set_cf(&mut self, value: &'arena Expression<'arena>) {
        self.semantics.push(InstructionSemanticsStep::SetFlag {
            flag: Flag::CF,
            value,
        })
    }

    pub fn cf(&self) -> &'arena Expression<'arena> {
        self.get_flag(Flag::CF)
    }

    pub fn set_af(&mut self, value: &'arena Expression<'arena>) {
        self.semantics.push(InstructionSemanticsStep::SetFlag {
            flag: Flag::AF,
            value,
        })
    }

    pub fn af(&self) -> &'arena Expression<'arena> {
        self.get_flag(Flag::AF)
    }

    pub fn set_zf(&mut self, value: &'arena Expression<'arena>) {
        self.semantics.push(InstructionSemanticsStep::SetFlag {
            flag: Flag::ZF,
            value,
        })
    }
    pub fn zf(&self) -> &'arena Expression<'arena> {
        self.get_flag(Flag::ZF)
    }

    pub fn set_pf(&mut self, value: &'arena Expression<'arena>) {
        self.semantics.push(InstructionSemanticsStep::SetFlag {
            flag: Flag::PF,
            value,
        })
    }

    pub fn pf(&self) -> &'arena Expression<'arena> {
        self.get_flag(Flag::PF)
    }

    pub fn set_sf(&mut self, value: &'arena Expression<'arena>) {
        self.semantics.push(InstructionSemanticsStep::SetFlag {
            flag: Flag::SF,
            value,
        })
    }

    pub fn sf(&self) -> &'arena Expression<'arena> {
        self.get_flag(Flag::SF)
    }

    pub fn set_of(&mut self, value: &'arena Expression<'arena>) {
        self.semantics.push(InstructionSemanticsStep::SetFlag {
            flag: Flag::OF,
            value,
        })
    }

    pub fn of(&self) -> &'arena Expression<'arena> {
        self.get_flag(Flag::OF)
    }


    pub fn get_flag(&self, flag: Flag) -> &'arena Expression<'arena> {
        self.arena.a(Expression::GetFlag {
            flag,
            at_index: self.semantics.len()
        })
    }

    //reg 8s:
    pub fn al(&self) -> &'arena Expression<'arena> {
        self.get_reg_8(Reg8::AL)
    }

    pub fn set_al(&mut self, value: &'arena Expression<'arena>) {
        self.set_reg_8(Reg8::AL, value)
    }

    pub fn ah(&self) -> &'arena Expression<'arena> {
        self.get_reg_8(Reg8::AH)
    }

    pub fn set_ah(&mut self, value: &'arena Expression<'arena>) {
        self.set_reg_8(Reg8::AH, value)
    }


    pub fn get_reg_8(&self, reg: Reg8) -> &'arena Expression<'arena> {
        self.arena.a(Expression::GetReg { reg: GeneralReg::Reg8(reg), at_index: self.semantics.len() })
    }

    pub fn set_reg_8(&mut self, reg: Reg8, value: &'arena Expression<'arena>) {
        self.semantics.push(InstructionSemanticsStep::SetRegister {
            zero_upper: ZeroUpper::NoZeroUpper,
            register: GeneralReg::Reg8(reg),
            value,
        })
    }

    pub fn ax(&self) -> &'arena Expression<'arena> {
        self.get_reg_16(Reg16WithRIP::AX)
    }

    pub fn set_ax(&mut self, value: &'arena Expression<'arena>) {
        self.set_reg_16(Reg16WithRIP::AX, value)
    }


    pub fn get_reg_16(&self, reg: Reg16WithRIP) -> &'arena Expression<'arena> {
        self.arena.a(Expression::GetReg { reg: GeneralReg::Reg16(reg), at_index: self.semantics.len() })
    }

    pub fn set_reg_16(&mut self, reg: Reg16WithRIP, value: &'arena Expression<'arena>) {
        self.semantics.push(InstructionSemanticsStep::SetRegister {
            zero_upper: ZeroUpper::NoZeroUpper,
            register: GeneralReg::Reg16(reg),
            value,
        })
    }


    pub fn zext_to(&self, value: &'arena Expression<'arena>, width: usize) -> &'arena Expression<'arena> {
        self.arena.a(Expression::ZeroExtend {
            value,
            len: width,
        })
    }

    pub fn extract(&self, value: &'arena Expression<'arena>, low: usize, high: usize) -> &'arena Expression<'arena> {
        self.arena.a(Expression::Extract {
            value,
            low,
            high,
        })
    }

    pub fn lower_bits(&self, value: &'arena Expression<'arena>, width: usize) -> &'arena Expression<'arena> {
        self.arena.a(Expression::LowerBits {
            value,
            len: width,
        })
    }

    pub fn upper_bits(&self, value: &'arena Expression<'arena>, width: usize) -> &'arena Expression<'arena> {
        self.arena.a(Expression::UpperBits {
            value,
            len: width,
        })
    }

    pub fn equal(&self, left: &'arena Expression<'arena>, right: &'arena Expression<'arena>) -> &'arena Expression<'arena> {
        self.arena.a(Expression::IntCompare {
            op: ComparisonOp::Equal,
            signedness: Signedness::Signed,
            left,
            right,
        })
    }
    pub fn less(&self, left: &'arena Expression<'arena>, right: &'arena Expression<'arena>) -> &'arena Expression<'arena> {
        self.arena.a(Expression::IntCompare {
            op: ComparisonOp::Less,
            signedness: Signedness::Signed,
            left,
            right,
        })
    }

    pub fn bitand(&self, left: &'arena Expression<'arena>, right: &'arena Expression<'arena>) -> &'arena Expression<'arena> {
        self.arena.a(Expression::BitWise {
            op: BitWiseOp::And,
            left,
            right,
        })
    }

    pub fn add(&self, left: &'arena Expression<'arena>, right: &'arena Expression<'arena>) -> &'arena Expression<'arena> {
        self.arena.a(Expression::IntArithmetic {
            op: ArithmeticOp::Add,
            signedness: Signedness::Signed,
            left,
            right,
        })
    }

    pub fn fadd(&self, left: &'arena Expression<'arena>, right: &'arena Expression<'arena>) -> &'arena Expression<'arena> {
        self.arena.a(Expression::FAdd {
            left,
            right,
        })
    }

    pub fn bitor(&self, left: &'arena Expression<'arena>, right: &'arena Expression<'arena>) -> &'arena Expression<'arena> {
        self.arena.a(Expression::BitWise {
            op: BitWiseOp::Or,
            left,
            right,
        })
    }

    pub fn bitxor(&self, left: &'arena Expression<'arena>, right: &'arena Expression<'arena>) -> &'arena Expression<'arena> {
        self.arena.a(Expression::BitWise {
            op: BitWiseOp::Xor,
            left,
            right,
        })
    }

    pub fn constant<T: IntegerWidth>(&self, value: T) -> &'arena Expression<'arena> {
        let value = self.arena.a(Value::new(bitvec![value.to_u64();T::width()]));
        self.arena.a(Expression::Constant {
            value,
        })
    }

    pub fn umul(&self, left: &'arena Expression<'arena>, right: &'arena Expression<'arena>) -> &'arena Expression<'arena> {
        self.arena.a(Expression::IntArithmetic {
            op: ArithmeticOp::Mul,
            signedness: Signedness::Unsigned,
            left,
            right,
        })
    }

    pub fn change(&self, value: &'arena Expression<'arena>, range_start_inclusive: usize, range_end_exclusive: usize, new_value: &'arena Expression<'arena>) -> &'arena Expression<'arena> {
        self.arena.a(Expression::ChangeRange {
            value,
            range_start_inclusive,
            range_end_exclusive,
            new_value,
        })
    }

    pub fn emit_conditional(
        &mut self,
        condition: &'arena Expression<'arena>,
        then: impl FnOnce(&mut SemanticsBuilder<'arena>),
        otherwise: impl FnOnce(&mut SemanticsBuilder<'arena>),
    ) {
        let mut then_builder = SemanticsBuilder::new(self.arena);
        then(&mut then_builder);
        let mut otherwise_builder = SemanticsBuilder::new(self.arena);
        otherwise(&mut otherwise_builder);
        self.semantics.push(InstructionSemanticsStep::Conditional {
            condition,
            true_semantics: then_builder.finalize(),
            false_semantics: otherwise_builder.finalize(),
        });
    }

    pub fn finalize(self) -> Vec<InstructionSemanticsStep<'arena>> {
        self.semantics
    }

    pub fn sync_uninterruptable(&mut self) {
        self.semantics.push(InstructionSemanticsStep::InstructionSyncPoint {
            interruptable: false,
        })
    }

    pub fn a(&self, expr: Expression<'arena>) -> &'arena Expression<'arena> {
        self.arena.a(expr)
    }
}
