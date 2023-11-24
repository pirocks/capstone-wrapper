use std::ops::{Add, BitAnd, BitOr};

use crate::x86_machine::{QWordVariable, VariableMappings};

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum BoolValue<'arena> {
    True,
    False,
    CompareByte {
        compare_type: CompareType,
        left: &'arena ByteValue<'arena>,
        right: &'arena ByteValue<'arena>,
    },
    CompareBool {
        compare_type: CompareType,
        left: &'arena BoolValue<'arena>,
        right: &'arena BoolValue<'arena>,
    },
    Or { lhs: &'arena BoolValue<'arena>, rhs: &'arena BoolValue<'arena> },
    Conditional {
        condition: &'arena BoolValue<'arena>,
        true_case: &'arena BoolValue<'arena>,
        false_case: &'arena BoolValue<'arena>,
    },
}

impl<'arena> BoolValue<'arena> {
    pub(crate) fn make_concrete(&self, variable_mappings: &VariableMappings<'arena>) -> bool {
        match self {
            BoolValue::True => {
                true
            }
            BoolValue::False => {
                false
            }
            BoolValue::CompareByte { compare_type, left, right } => {
                match compare_type {
                    CompareType::Less => {
                        left.make_concrete(variable_mappings) < right.make_concrete(variable_mappings)
                    }
                    CompareType::More => {
                        left.make_concrete(variable_mappings) > right.make_concrete(variable_mappings)
                    }
                    CompareType::Equal => {
                        left.make_concrete(variable_mappings) == right.make_concrete(variable_mappings)
                    }
                }
            }
            BoolValue::Conditional { condition, true_case, false_case } => {
                if condition.make_concrete(variable_mappings) {
                    true_case.make_concrete(variable_mappings)
                } else {
                    false_case.make_concrete(variable_mappings)
                }
            }

            BoolValue::CompareBool { compare_type, left, right } => {
                match compare_type {
                    CompareType::Less => {
                        left.make_concrete(variable_mappings) < right.make_concrete(variable_mappings)
                    }
                    CompareType::More => {
                        left.make_concrete(variable_mappings) > right.make_concrete(variable_mappings)
                    }
                    CompareType::Equal => {
                        left.make_concrete(variable_mappings) == right.make_concrete(variable_mappings)
                    }
                }
            }
            BoolValue::Or { lhs, rhs } => {
                lhs.make_concrete(variable_mappings) || rhs.make_concrete(variable_mappings)
            }
        }
    }
}

impl<'arena> BitOr<&'arena BoolValue<'arena>> for &'arena BoolValue<'arena> {
    type Output = BoolValue<'arena>;

    fn bitor(self, rhs: &'arena BoolValue<'_>) -> Self::Output {
        BoolValue::Or {
            lhs: self,
            rhs,
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ByteValue<'arena> {
    Constant(u8),
    QExtractByte {
        to_extract: &'arena QWordValue<'arena>,
        byte_from_lower: u8
    },
    And {
        left: &'arena ByteValue<'arena>,
        right: &'arena ByteValue<'arena>,
    },
    Or {
        left: &'arena ByteValue<'arena>,
        right: &'arena ByteValue<'arena>,
    },
    IfElse {
        condition: &'arena BoolValue<'arena>,
        true_case: &'arena ByteValue<'arena>,
        false_case: &'arena ByteValue<'arena>,
    },
}


impl<'arena> ByteValue<'arena> {
    fn make_concrete(&self, variable_mappings: &VariableMappings<'arena>) -> u8 {
        match self {
            ByteValue::Constant(constant) => {
                *constant
            }
            ByteValue::QExtractByte{ to_extract, byte_from_lower } => {
                ((to_extract.make_concrete(variable_mappings) >> (byte_from_lower * 8)) & 0xFF) as u8
            }
            ByteValue::And { left, right } => {
                left.make_concrete(variable_mappings) & right.make_concrete(variable_mappings)
            }
            ByteValue::Or { left, right } => {
                left.make_concrete(variable_mappings) | right.make_concrete(variable_mappings)
            }
            ByteValue::IfElse { condition, true_case, false_case } => {
                if condition.make_concrete(variable_mappings) {
                    true_case.make_concrete(variable_mappings)
                } else {
                    false_case.make_concrete(variable_mappings)
                }
            }
        }
    }
}

impl<'arena> BitAnd for &'arena ByteValue<'arena> {
    type Output = ByteValue<'arena>;

    fn bitand(self, rhs: Self) -> Self::Output {
        ByteValue::And { left: self, right: rhs }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum WordValue<'arena> {
    Constant(u16),
    LowerBits(&'arena QWordValue<'arena>),
    And {
        left: &'arena WordValue<'arena>,
        right: &'arena WordValue<'arena>,
    },
    Or {
        left: &'arena WordValue<'arena>,
        right: &'arena WordValue<'arena>,
    },
    IfElse {
        condition: &'arena BoolValue<'arena>,
        true_case: &'arena WordValue<'arena>,
        false_case: &'arena WordValue<'arena>,
    },
    Add {
        left: &'arena WordValue<'arena>,
        right: &'arena WordValue<'arena>,
    },
    Conditional {
        condition: &'arena BoolValue<'arena>,
        true_case: &'arena WordValue<'arena>,
        false_case: &'arena WordValue<'arena>,
    },
}

impl<'arena> WordValue<'arena> {
    pub fn make_concrete(&self, variable_mappings: &VariableMappings<'arena>) -> u16 {
        match self {
            WordValue::Constant(constant) => {
                *constant
            }
            WordValue::LowerBits(qword) => {
                (qword.make_concrete(variable_mappings) & 0xFFFF) as u16
            }
            WordValue::And { left, right } => {
                left.make_concrete(variable_mappings) & right.make_concrete(variable_mappings)
            }
            WordValue::Or { left, right } => {
                left.make_concrete(variable_mappings) | right.make_concrete(variable_mappings)
            }
            WordValue::IfElse { condition, true_case, false_case } => {
                if condition.make_concrete(variable_mappings) {
                    true_case.make_concrete(variable_mappings)
                } else {
                    false_case.make_concrete(variable_mappings)
                }
            }
            WordValue::Add { left, right } => {
                left.make_concrete(variable_mappings).wrapping_add(right.make_concrete(variable_mappings))
            }
            WordValue::Conditional { condition, true_case, false_case } => {
                if condition.make_concrete(variable_mappings) {
                    true_case.make_concrete(variable_mappings)
                } else {
                    false_case.make_concrete(variable_mappings)
                }
            }
        }
    }
}

impl<'arena> Add<&'arena WordValue<'arena>> for &'arena WordValue<'arena> {
    type Output = WordValue<'arena>;

    fn add(self, rhs: &'arena WordValue<'arena>) -> Self::Output {
        WordValue::Add {
            left: self,
            right: rhs,
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum DWordValue<'arena> {
    Constant(u32),
    And {
        left: &'arena DWordValue<'arena>,
        right: &'arena DWordValue<'arena>,
    },
}

impl<'arena> DWordValue<'arena> {
    pub fn make_concrete(&self, variable_mappings: &VariableMappings<'arena>) -> u32 {
        match self {
            DWordValue::Constant(constant) => {
                *constant
            }
            DWordValue::And { left, right } => {
                left.make_concrete(variable_mappings) & right.make_concrete(variable_mappings)
            }
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum QWordValue<'arena> {
    Constant(u64),
    And {
        left: &'arena QWordValue<'arena>,
        right: &'arena QWordValue<'arena>,
    },
    Or {
        left: &'arena QWordValue<'arena>,
        right: &'arena QWordValue<'arena>,
    },
    WriteLowerBits {
        prev: &'arena QWordValue<'arena>,
        lower: &'arena WordValue<'arena>,
    },
    ZeroExtendWord(&'arena WordValue<'arena>),
    ZeroExtendByte(&'arena ByteValue<'arena>),
    Var(QWordVariable),
}

impl<'arena> QWordValue<'arena> {
    pub fn make_concrete(&self, variable_mappings: &VariableMappings<'arena>) -> u64 {
        match self {
            QWordValue::Constant(constant) => {
                *constant
            }
            QWordValue::And { left, right } => {
                left.make_concrete(variable_mappings) & right.make_concrete(variable_mappings)
            }
            QWordValue::Or { left, right } => {
                left.make_concrete(variable_mappings) | right.make_concrete(variable_mappings)
            }
            QWordValue::WriteLowerBits { prev, lower } => {
                let prev = prev.make_concrete(variable_mappings);
                let lower = lower.make_concrete(variable_mappings);
                (prev & 0xFFFF_FFFF_FFFF_0000) | ((lower as u64) & 0xFFFF)
            }
            QWordValue::ZeroExtendWord(w) => {
                w.make_concrete(variable_mappings) as u64
            }
            QWordValue::ZeroExtendByte(b) => {
                b.make_concrete(variable_mappings) as u64
            }
            QWordValue::Var(var) => {
                variable_mappings.qword_variables.get(var).unwrap().make_concrete(variable_mappings)
            }
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum CompareType {
    Less,
    More,
    Equal,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum EmptyValue<'arena> {
    Empty {},
    Conditional {
        condition: &'arena BoolValue<'arena>,
        true_case: &'arena Self,
        false_case: &'arena Self,
    },
}

pub trait Value<'arena>: Copy + Clone {
    fn conditional(conditional: &'arena BoolValue<'arena>, true_case: &'arena Self, false_case: &'arena Self) -> Self;
}

impl<'arena> Value<'arena> for EmptyValue<'arena> {
    fn conditional(condition: &'arena BoolValue<'arena>, true_case: &'arena Self, false_case: &'arena Self) -> Self {
        EmptyValue::Conditional {
            condition,
            true_case,
            false_case,
        }
    }
}

pub trait NumericValue<'arena>: Copy + Clone + Value<'arena> {
    fn compare_bool(&'arena self, compare_type: CompareType, other: &'arena Self) -> BoolValue<'arena>;
}

impl<'arena> Value<'arena> for BoolValue<'arena> {
    fn conditional(condition: &'arena BoolValue<'arena>, true_case: &'arena Self, false_case: &'arena Self) -> Self {
        BoolValue::Conditional {
            condition,
            true_case,
            false_case,
        }
    }
}

impl<'arena> NumericValue<'arena> for BoolValue<'arena> {
    fn compare_bool(&'arena self, compare_type: CompareType, other: &'arena Self) -> BoolValue<'arena> {
        BoolValue::CompareBool {
            compare_type,
            left: self,
            right: other,
        }
    }
}

impl<'arena> NumericValue<'arena> for ByteValue<'arena> {
    fn compare_bool(&'arena self, compare_type: CompareType, other: &'arena Self) -> BoolValue<'arena> {
        BoolValue::CompareByte {
            compare_type,
            left: self,
            right: other,
        }
    }
}

impl<'arena> Value<'arena> for ByteValue<'arena> {
    fn conditional(conditional: &'arena BoolValue<'arena>, true_case: &'arena Self, false_case: &'arena Self) -> Self {
        todo!()
    }
}

impl<'arena> NumericValue<'arena> for WordValue<'arena> {
    fn compare_bool(&'arena self, compare_type: CompareType, other: &'arena Self) -> BoolValue<'arena> {
        todo!()
    }
}

impl<'arena> Value<'arena> for WordValue<'arena> {
    fn conditional(condition: &'arena BoolValue<'arena>, true_case: &'arena Self, false_case: &'arena Self) -> Self {
        WordValue::Conditional {
            condition,
            true_case,
            false_case,
        }
    }
}

impl<'arena> NumericValue<'arena> for DWordValue<'arena> {
    fn compare_bool(&'arena self, compare_type: CompareType, other: &'arena Self) -> BoolValue<'arena> {
        todo!()
    }
}

impl<'arena> Value<'arena> for DWordValue<'arena> {
    fn conditional(conditional: &'arena BoolValue<'arena>, true_case: &'arena Self, false_case: &'arena Self) -> Self {
        todo!()
    }
}

impl<'arena> NumericValue<'arena> for QWordValue<'arena> {
    fn compare_bool(&'arena self, compare_type: CompareType, other: &'arena Self) -> BoolValue<'arena> {
        todo!()
    }
}


impl<'arena> Value<'arena> for QWordValue<'arena> {
    fn conditional(conditional: &'arena BoolValue<'arena>, true_case: &'arena Self, false_case: &'arena Self) -> Self {
        todo!()
    }
}