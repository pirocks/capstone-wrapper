use std::collections::HashMap;

use wrapper_common::registers::Reg64WithRIP;

use crate::raw::OperandIdx;

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub enum RegisterOrParameter64 {
    Operand(OperandIdx),
    Register(Reg64WithRIP),
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct NewFlags {
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
    // pub parameters: Vec<OperandIdx>,
    pub new_general_register_values: HashMap<RegisterOrParameter64, TypedExpression64>,
    pub new_flags_value: NewFlags,
    //todo for now all stores happen after loads which may not be accurate.
    pub memory_values_diff: MemoryValuesDiff,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct MemoryValuesDiff {
    pub(crate) stores: Vec<TypedExpression64>
}


#[derive(Clone, Debug)]
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
    Concatenate568 {
        left: Box<TypedExpression56>,
        right: Box<TypedExpression8>,
    },
    OperandR64 { operand_idx: OperandIdx },
    R64 { reg: Reg64WithRIP },
    Load(Box<TypedExpression64>),
    Store {
//todo this doesn't belong here, since it returns nothing
        address: Box<TypedExpression64>,
        value: Box<TypedExpression64>,
    },
    Sub { left: Box<TypedExpression64>, right: Box<TypedExpression64> },
    Constant(i128),
}

impl TypedExpression64 {
    pub fn operand_replace(&self, op: OperandIdx, replace_with: &TypedExpression) -> TypedExpression64 {
        match self {
            TypedExpression64::OperandR8 { operand_idx } => {
                if operand_idx == &op {
                    todo!()/*replace_with.unwrap_8()*/
                } else {
                    self.clone()
                }
            }
            TypedExpression64::OperandR64 { operand_idx } => {
                if operand_idx == &op {
                    replace_with.clone().unwrap_64()
                } else {
                    self.clone()
                }
            }
            TypedExpression64::Concatenate568 { left, right } => {
                TypedExpression64::Concatenate568 {
                    left: Box::new(left.as_ref().operand_replace(op, replace_with)),
                    right: Box::new(right.as_ref().operand_replace(op, replace_with)),
                }
            }
            TypedExpression64::Load(addr) => {
                TypedExpression64::Load(Box::new(addr.as_ref().operand_replace(op, replace_with)))
            }
            TypedExpression64::Store { address, value } => {
                TypedExpression64::Store {
                    address: Box::new(address.as_ref().operand_replace(op, replace_with)),
                    value: Box::new(value.as_ref().operand_replace(op, replace_with)),
                }
            }
            TypedExpression64::Sub { left, right } => {
                TypedExpression64::Sub {
                    left: Box::new(left.as_ref().operand_replace(op, replace_with)),
                    right: Box::new(right.as_ref().operand_replace(op, replace_with)),
                }
            }
            TypedExpression64::R64 { reg } => {
                TypedExpression64::R64 { reg: *reg }
            }
            TypedExpression64::Constant(con) => {
                TypedExpression64::Constant(*con)
            }
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum TypedExpression56 {
    Extract64 {
        source: TypedExpression64,
        base: usize,
    }
}

impl TypedExpression56 {
    fn operand_replace(&self, op: OperandIdx, replace_with: &TypedExpression) -> TypedExpression56 {
        match self {
            TypedExpression56::Extract64 { source, base } => {
                let source = source.operand_replace(op, replace_with);
                TypedExpression56::Extract64 { source, base: *base }
            }
        }
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

impl TypedExpression8 {
    fn operand_replace(&self, op: OperandIdx, replace_with: &TypedExpression) -> TypedExpression8 {
        match self {
            TypedExpression8::Extract { source, base } => {
                todo!()
            }
            TypedExpression8::Extract9 { source, base } => {
                todo!()
            }
            TypedExpression8::Extract64 { source, base } => {
                todo!()
            }
            TypedExpression8::Constant(con) => TypedExpression8::Constant(*con)
        }
    }
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
        right: Box<TypedExpression1>,
    },
    Extract64 { source: Box<TypedExpression64>, base: usize },
    Xor { left: Box<TypedExpression1>, right: Box<TypedExpression1> },
    Equals8 { left: Box<TypedExpression8>, right: Box<TypedExpression8> },
    AndBool { left: Box<TypedExpression1>, right: Box<TypedExpression1> },
}
