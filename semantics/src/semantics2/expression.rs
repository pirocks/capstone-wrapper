use wrapper_common::memory_operand::GeneralReg;
use crate::semantics2::arena::Arena;
use crate::semantics2::state::ConcreteX86MachineState64;

use crate::semantics2::value::Value;

#[derive(Copy, Clone)]
pub enum Signedness {
    Signed,
    Unsigned,
}

#[derive(Copy, Clone)]
pub enum BitWiseOp {
    And,
    Or,
    Xor,
}

#[derive(Copy, Clone)]
pub enum ComparisonOp {
    Less,
    LessOrEqual,
    Equal,
    GreaterOrEqual,
    Greater,
}

#[derive(Copy, Clone)]
pub enum Flag {
    CF,
    PF,
    AF,
    ZF,
    SF,
    OF,
}

#[derive(Copy, Clone)]
pub enum ArithmeticOp {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Copy, Clone)]
pub enum Expression<'arena> {
    GetReg {
        reg: GeneralReg,
        at_index: usize
    },
    GetFlag {
        flag: Flag,
        at_index: usize
    },
    Constant {
        value: &'arena Value<'arena>,
    },
    BitWise {
        op: BitWiseOp,
        left: &'arena Expression<'arena>,
        right: &'arena Expression<'arena>,
    },
    IntCompare {
        op: ComparisonOp,
        signedness: Signedness,
        left: &'arena Expression<'arena>,
        right: &'arena Expression<'arena>,
    },
    IntArithmetic {
        op: ArithmeticOp,
        signedness: Signedness,
        left: &'arena Expression<'arena>,
        right: &'arena Expression<'arena>,
    },
    Extract {
        value: &'arena Expression<'arena>,
        low: usize,
        high: usize,
    },
    Concat {
        left: &'arena Expression<'arena>,
        right: &'arena Expression<'arena>,
    },
    // Variable {
    //     name: VariableName,
    // },
    Conditional {
        condition: &'arena Expression<'arena>,
        true_value: &'arena Expression<'arena>,
        false_value: &'arena Expression<'arena>,
    },
    ZeroExtend {
        value: &'arena Expression<'arena>,
        len: usize,
    },
    LowerBits {
        value: &'arena Expression<'arena>,
        len: usize,
    },
    UpperBits {
        value: &'arena Expression<'arena>,
        len: usize,
    },
    ChangeRange { value: &'arena Expression<'arena>, range_start_inclusive: usize, range_end_exclusive: usize, new_value: &'arena Expression<'arena> },
    FAdd { left: &'arena Expression<'arena>, right: &'arena Expression<'arena> },
}

impl<'arena> Expression<'arena> {
    pub fn width(&self) -> usize {
        match self {
            Expression::Constant { value } => value.width(),
            Expression::BitWise { left, right, .. } => left.width().max(right.width()),
            Expression::Extract { value, low, high } => high - low + 1,
            Expression::Concat { left, right } => left.width() + right.width(),
            // Expression::Variable { name } => .inner.get(name).unwrap().width(),
            Expression::Conditional { condition, true_value, false_value } => {
                assert_eq!(false_value.width(), true_value.width());
                true_value.width()
            }
            Expression::IntCompare { op, signedness, left, right } => {
                assert_eq!(left.width(), right.width());
                1
            }
            Expression::GetReg { reg, at_index:_ } => {
                reg.bit_width()
            }
            Expression::GetFlag { .. } => {
                1
            }
            Expression::ZeroExtend { value, len } => {
                *len
            }
            Expression::LowerBits { value, len } => {
                *len
            }
            Expression::UpperBits { value, len } => {
                *len
            }
            Expression::ChangeRange { value, range_start_inclusive, range_end_exclusive, new_value } => {
                value.width()
            }
            Expression::IntArithmetic { op, signedness, left, right } => {
                todo!()
            }
            Expression::FAdd { left, right } => {
                todo!()
            }
        }
    }

    pub fn apply_concrete(&self, concrete: &ConcreteX86MachineState64) -> Value{
        match self {
            Expression::GetReg { reg, at_index } => {
                concrete.get_reg(*reg, *at_index)
            }
            Expression::GetFlag { .. } => {

            }
            Expression::Constant { .. } => {

            }
            Expression::BitWise { .. } => {

            }
            Expression::IntCompare { .. } => {

            }
            Expression::IntArithmetic { .. } => {

            }
            Expression::Extract { .. } => {

            }
            Expression::Concat { .. } => {

            }
            Expression::Conditional { .. } => {

            }
            Expression::ZeroExtend { .. } => {

            }
            Expression::LowerBits { .. } => {

            }
            Expression::UpperBits { .. } => {

            }
            Expression::ChangeRange { .. } => {

            }
            Expression::FAdd { .. } => {

            }
        }
    }
}

