use std::collections::HashMap;
use wrapper_common::registers::Reg64WithRIP;
use crate::raw::OperandIdx;

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub enum RegisterOrParameter64 {
    Parameter(OperandIdx),
    Register(Reg64WithRIP)
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct NewFlags{
    pub(crate) flag_cf: Option<TypedExpression1>,
    pub(crate) flag_pf: Option<TypedExpression1>,
    pub(crate) flag_af: Option<TypedExpression1>,
    pub(crate) flag_zf: Option<TypedExpression1>,
    pub(crate) flag_sf: Option<TypedExpression1>,
    pub(crate) flag_of: Option<TypedExpression1>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Rule {
    pub raw_name: String,
    pub parameters: Vec<OperandIdx>,
    pub new_general_register_values: HashMap<RegisterOrParameter64, TypedExpression64>,
    pub new_flags_value: NewFlags,
    pub memory_values_diff: MemoryValuesDiff,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct MemoryValuesDiff {}


#[derive(Debug)]
pub enum TypedExpression {
    _64(TypedExpression64),
    _56(TypedExpression56),
    _9(TypedExpression9),
    _8(TypedExpression8),
    _1(TypedExpression1),
}

impl TypedExpression {
    pub fn size(&self) -> usize {
        match self {
            TypedExpression::_64(_) => 64,
            TypedExpression::_56(_) => 56,
            TypedExpression::_9(_) => 9,
            TypedExpression::_8(_) => 8,
            TypedExpression::_1(_) => 1,
        }
    }

    pub fn unwrap_1(self) -> TypedExpression1 {
        match self {
            TypedExpression::_1(inner) => inner,
            _ => panic!()
        }
    }

    pub fn unwrap_8(self) -> TypedExpression8 {
        match self {
            TypedExpression::_8(inner) => inner,
            _ => panic!()
        }
    }

    pub fn unwrap_9(self) -> TypedExpression9 {
        match self {
            TypedExpression::_9(inner) => inner,
            _ => panic!()
        }
    }

    pub fn unwrap_64(self) -> TypedExpression64 {
        match self {
            TypedExpression::_64(inner) => inner,
            _ => panic!()
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum TypedExpression64 {
    OperandR8 {
        operand_idx: OperandIdx
    },
    Concatenate568{
        left: Box<TypedExpression56>,
        right: Box<TypedExpression8>
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum TypedExpression56 {
    Extract64 {
        source: TypedExpression64,
        base: usize,
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum TypedExpression9 {
    Concatenate18 {
        left: TypedExpression1,
        right: Box<TypedExpression8>,
    },
    Equals {
        left: Box<TypedExpression9>,
        right: Box<TypedExpression9>,
    },
    IfThenElse {
        condition: TypedExpression1,
        true_case: Box<TypedExpression9>,
        false_case: Box<TypedExpression9>,
    },
    Constant(i16),
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum TypedExpression8 {
    Extract { source: TypedExpression64, base: usize },
    Extract9 { source: TypedExpression9, base: usize },
    Extract64 { source: TypedExpression64, base: usize },
    Constant(i16),
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum TypedExpression1 {
    FlagCF,
    Constant(bool),
    Equals1 {
        left: Box<TypedExpression1>,
        right: Box<TypedExpression1>,
    },
    IfThenElse {
        condition: Box<TypedExpression1>,
        true_case: Box<TypedExpression1>,
        false_case: Box<TypedExpression1>,
    },
    Extract9 { source: Box<TypedExpression9>, base: usize },
    Not(Box<TypedExpression1>),
    XorBool {
        left: Box<TypedExpression1>,
        right: Box<TypedExpression1>
    },
    Extract64 { source: Box<TypedExpression64>, base: usize },
    Xor { left: Box<TypedExpression1>, right: Box<TypedExpression1> },
    Equals8 { left: Box<TypedExpression8>, right: Box<TypedExpression8> },
    AndBool { left: Box<TypedExpression1>, right: Box<TypedExpression1> },
}
