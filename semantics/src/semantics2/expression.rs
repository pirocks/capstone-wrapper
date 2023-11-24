use wrapper_common::memory_operand::GeneralReg;

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
pub enum Expression<'arena> {
    GetReg {
        reg: GeneralReg
    },
    GetFlag {
        flag: Flag,
    },
    Constant {
        value: &'arena Value<'arena>,
    },
    BitWise {
        op: BitWiseOp,
        left: &'arena Expression<'arena>,
        right: &'arena Expression<'arena>,
    },
    Compare {
        op: ComparisonOp,
        signedness: Signedness,
        left: &'arena Expression<'arena>,
        right: &'arena Expression<'arena>,
    },
    Add {
        signedness: Signedness,
        left: &'arena Expression<'arena>,
        right: &'arena Expression<'arena>,
    },
    Mul {
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
}

impl<'arena> Expression<'arena> {
    pub fn width(&self) -> usize {
        match self {
            Expression::Constant { value } => value.width(),
            Expression::BitWise { left, right, .. } => left.width().max(right.width()),
            Expression::Add { left, right, .. } => left.width().max(right.width()),
            Expression::Mul { left, right, .. } => left.width().max(right.width()),
            Expression::Extract { value, low, high } => high - low + 1,
            Expression::Concat { left, right } => left.width() + right.width(),
            // Expression::Variable { name } => .inner.get(name).unwrap().width(),
            Expression::Conditional { condition, true_value, false_value } => {
                assert_eq!(false_value.width(), true_value.width());
                true_value.width()
            }
            Expression::Compare { op, signedness, left, right } => {
                assert_eq!(left.width(), right.width());
                1
            }
            Expression::GetReg { reg } => {
                reg.bit_width()
            }
            Expression::GetFlag { flag } => {
                1
            }
            Expression::ZeroExtend { value, len } => {
                *len
            }
        }
    }
}
