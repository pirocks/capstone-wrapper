use serde::{Deserialize, Serialize};

use wrapper_common::registers::{Reg64WithRIP, RegXMM, RegYMM};

use crate::raw::OperandIdx;

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub enum RegisterOrParameter64 {
    Operand(OperandIdx),
    Register(Reg64WithRIP),
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub enum RegisterOrParameterXMM {
    Operand(OperandIdx),
    Register(RegXMM),
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
pub enum RuleElement {
    NewGeneralRegisterValue {
        register: RegisterOrParameter64,
        value: TypedExpression64,
    },
    NewVectorRegisterValue {
        register: RegisterOrParameterXMM,
        value: TypedExpression256,
    },
    NewFlagsValue {
        flag_cf: Option<TypedExpression1>,
        flag_pf: Option<TypedExpression1>,
        flag_af: Option<TypedExpression1>,
        flag_zf: Option<TypedExpression1>,
        flag_sf: Option<TypedExpression1>,
        flag_of: Option<TypedExpression1>,
    },
    Store {
        address: TypedExpression64,
        value: TypedExpression,
    },
    Load {
        op_idx: OperandIdx,
        address: TypedExpression64,
    },
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Rule {
    pub raw_name: String,
    // pub parameters: Vec<OperandIdx>,
    // pub new_general_register_values: HashMap<RegisterOrParameter64, TypedExpression64>,
    // pub new_flags_value: NewFlags,
    // pub memory_values_diff: MemoryValuesDiff,
    pub elements: Vec<RuleElement>,
}

impl Rule {
    pub fn new_general_register_value(
        &mut self,
        reg: RegisterOrParameter64,
        value: TypedExpression64,
    ) {
        self.elements.push(RuleElement::NewGeneralRegisterValue {
            register: reg,
            value,
        })
    }

    pub fn new_vector_register_value(
        &mut self,
        reg: RegisterOrParameterXMM,
        value: TypedExpression256,
    ) {
        self.elements.push(RuleElement::NewVectorRegisterValue {
            register: reg,
            value,
        })
    }

    pub fn new_flags_value(&mut self, flags: NewFlags) {
        self.elements.push(RuleElement::NewFlagsValue {
            flag_cf: flags.flag_cf,
            flag_pf: flags.flag_pf,
            flag_af: flags.flag_af,
            flag_zf: flags.flag_zf,
            flag_sf: flags.flag_sf,
            flag_of: flags.flag_of,
        })
    }

    pub fn new_load(&mut self, op_idx: OperandIdx, address: TypedExpression64) {
        self.elements.push(RuleElement::Load { op_idx, address })
    }

    pub fn new_store(&mut self, address: TypedExpression64, value: TypedExpression) {
        self.elements.push(RuleElement::Store { address, value })
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct MemoryValuesDiff {
    pub(crate) stores: Vec<TypedExpression64>,
}

pub struct ReplaceWith {
    pub _256: TypedExpression256,
    pub _128: TypedExpression128,
    pub _64: TypedExpression64,
    pub _56: TypedExpression56,
    pub _9: TypedExpression9,
    pub _8: TypedExpression8,
    pub _1: TypedExpression1,
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub enum TypedExpression {
    // _264(TypedExpression264),
    _256(TypedExpression256),
    // _248(TypedExpression248),
    // _240(TypedExpression240),
    // _232(TypedExpression232),
    // _224(TypedExpression224),
    // _216(TypedExpression216),
    // _208(TypedExpression208),
    // _200(TypedExpression200),
    // _192(TypedExpression192),
    // _184(TypedExpression184),
    // _176(TypedExpression176),
    // _168(TypedExpression168),
    // _160(TypedExpression160),
    // _152(TypedExpression152),
    // _144(TypedExpression144),
    // _136(TypedExpression136),
    _128(TypedExpression128),
    _120(TypedExpression120),
    _112(TypedExpression112),
    _104(TypedExpression104),
    _96(TypedExpression96),
    _88(TypedExpression88),
    _80(TypedExpression80),
    _72(TypedExpression72),
    _64(TypedExpression64),
    _56(TypedExpression56),
    _48(TypedExpression48),
    _40(TypedExpression40),
    _32(TypedExpression32),
    _24(TypedExpression24),
    _16(TypedExpression16),
    _9(TypedExpression9),
    _8(TypedExpression8),
    _1(TypedExpression1),
}

impl TypedExpression {
    pub(crate) fn operand_replace(
        &self,
        op: OperandIdx,
        replace_with: &ReplaceWith,
    ) -> TypedExpression {
        match self {
            TypedExpression::_64(_64) => {
                TypedExpression::_64(_64.operand_replace(op, replace_with))
            }
            TypedExpression::_56(_56) => {
                TypedExpression::_56(_56.operand_replace(op, replace_with))
            }
            TypedExpression::_9(_9) => {
                todo!()
            }
            TypedExpression::_8(_8) => {
                todo!()
            }
            TypedExpression::_1(_1) => {
                todo!()
            }
            TypedExpression::_128(_) => {
                todo!()
            }
            TypedExpression::_256(_) => {
                todo!()
            }
            TypedExpression::_16(_) => {
                todo!()
            }
            other => todo!("{other:?}"),
        }
    }

    pub fn size(&self) -> usize {
        match self {
            TypedExpression::_64(_) => 64,
            TypedExpression::_56(_) => 56,
            TypedExpression::_9(_) => 9,
            TypedExpression::_8(_) => 8,
            TypedExpression::_1(_) => 1,
            TypedExpression::_128(_) => 128,
            TypedExpression::_256(_) => 256,
            TypedExpression::_16(_) => 16,
            TypedExpression::_24(_) => 24,
            TypedExpression::_32(_) => 32,
            TypedExpression::_48(_) => 48,
            TypedExpression::_72(_) => 72,
            TypedExpression::_120(_) => 120,
            TypedExpression::_112(_) => 112,
            TypedExpression::_104(_) => 104,
            TypedExpression::_96(_) => 96,
            TypedExpression::_88(_) => 88,
            TypedExpression::_80(_) => 80,
            TypedExpression::_40(_) => 40,
        }
    }

    pub fn unwrap_1(self) -> TypedExpression1 {
        match self {
            TypedExpression::_1(inner) => inner,
            _ => panic!("{:?}", self),
        }
    }

    pub fn unwrap_8(self) -> TypedExpression8 {
        match self {
            TypedExpression::_8(inner) => inner,
            _ => panic!(),
        }
    }

    pub fn unwrap_9(self) -> TypedExpression9 {
        match self {
            TypedExpression::_9(inner) => inner,
            _ => panic!(),
        }
    }

    pub fn unwrap_64(self) -> TypedExpression64 {
        match self {
            TypedExpression::_64(inner) => inner,
            _ => panic!("{:?}", self),
        }
    }

    pub fn unwrap_128(self) -> TypedExpression128 {
        match self {
            TypedExpression::_128(inner) => inner,
            _ => panic!(),
        }
    }

    pub fn unwrap_256(self) -> TypedExpression256 {
        // eprintln!("{}", serde_json::to_string_pretty(&self).unwrap());
        match self {
            TypedExpression::_256(inner) => inner,
            other => panic!("{other:?}"),
        }
    }
}

// #[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
// pub enum TypedExpression248 {
//     Concatenate2408 { left: Box<TypedExpression8>, right: Box<TypedExpression240> },
// }
//
// #[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
// pub enum TypedExpression240 {
//     Concatenate2328 { left: Box<TypedExpression8>, right: Box<TypedExpression232> },
// }
//
// #[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
// pub enum TypedExpression232 {
//     Concatenate2248 { left: Box<TypedExpression8>, right: Box<TypedExpression224> },
// }
//
// #[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
// pub enum TypedExpression224 {
//     Concatenate2168 { left: Box<TypedExpression8>, right: Box<TypedExpression216> },
// }
//
// #[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
// pub enum TypedExpression216 {
//     Concatenate2088 { left: Box<TypedExpression8>, right: Box<TypedExpression208> },
// }
//
// #[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
// pub enum TypedExpression208 {
//     Concatenate2008 { left: Box<TypedExpression8>, right: Box<TypedExpression200> },
// }
//
// #[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
// pub enum TypedExpression200 {
//     Concatenate1928 { left: Box<TypedExpression8>, right: Box<TypedExpression192> },
// }
//
// #[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
// pub enum TypedExpression192 {
//     Concatenate1848 { left: Box<TypedExpression8>, right: Box<TypedExpression184> },
// }
//
// #[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
// pub enum TypedExpression184 {
//     Concatenate1768 { left: Box<TypedExpression8>, right: Box<TypedExpression176> },
// }
//
// #[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
// pub enum TypedExpression176 {
//     Concatenate1688 { left: Box<TypedExpression8>, right: Box<TypedExpression168> },
// }
//
// #[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
// pub enum TypedExpression168 {
//     Concatenate1608 { left: Box<TypedExpression8>, right: Box<TypedExpression160> },
// }
//
// #[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
// pub enum TypedExpression160 {
//     Concatenate1528 { left: Box<TypedExpression8>, right: Box<TypedExpression152> },
// }
//
// #[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
// pub enum TypedExpression152 {
//     Concatenate1448 { left: Box<TypedExpression8>, right: Box<TypedExpression144> },
// }
//
// #[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
// pub enum TypedExpression144 {
//     Concatenate1368 { left: Box<TypedExpression8>, right: Box<TypedExpression136> },
// }
//
// #[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
// pub enum TypedExpression136 {
//     Concatenate1288 { left: Box<TypedExpression8>, right: Box<TypedExpression128> },
// }

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub enum TypedExpression120 {
    Concatenate1128 {
        left: Box<TypedExpression8>,
        right: Box<TypedExpression112>,
    },
}

impl TypedExpression120 {
    pub fn operand_replace(&self, op: OperandIdx, replace_with: &ReplaceWith) -> Self {
        match self {
            Self::Concatenate1128 { left, right } => Self::Concatenate1128 {
                left: Box::new(left.operand_replace(op, replace_with)),
                right: Box::new(right.operand_replace(op, replace_with)),
            },
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub enum TypedExpression112 {
    Concatenate1048 {
        left: Box<TypedExpression8>,
        right: Box<TypedExpression104>,
    },
}

impl TypedExpression112 {
    pub fn operand_replace(&self, op: OperandIdx, replace_with: &ReplaceWith) -> Self {
        match self {
            Self::Concatenate1048 { left, right } => Self::Concatenate1048 {
                left: Box::new(left.operand_replace(op, replace_with)),
                right: Box::new(right.operand_replace(op, replace_with)),
            },
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub enum TypedExpression104 {
    Concatenate968 {
        left: Box<TypedExpression8>,
        right: Box<TypedExpression96>,
    },
}

impl TypedExpression104 {
    pub fn operand_replace(&self, op: OperandIdx, replace_with: &ReplaceWith) -> Self {
        match self {
            Self::Concatenate968 { left, right } => Self::Concatenate968 {
                left: Box::new(left.operand_replace(op, replace_with)),
                right: Box::new(right.operand_replace(op, replace_with)),
            },
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub enum TypedExpression96 {
    Concatenate888 {
        left: Box<TypedExpression8>,
        right: Box<TypedExpression88>,
    },
}

impl TypedExpression96 {
    pub fn operand_replace(&self, op: OperandIdx, replace_with: &ReplaceWith) -> Self {
        match self {
            Self::Concatenate888 { left, right } => Self::Concatenate888 {
                left: Box::new(left.operand_replace(op, replace_with)),
                right: Box::new(right.operand_replace(op, replace_with)),
            },
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub enum TypedExpression88 {
    Concatenate880 {
        left: Box<TypedExpression8>,
        right: Box<TypedExpression80>,
    },
}

impl TypedExpression88 {
    pub fn operand_replace(&self, op: OperandIdx, replace_with: &ReplaceWith) -> Self {
        match self {
            Self::Concatenate880 { left, right } => Self::Concatenate880 {
                left: Box::new(left.operand_replace(op, replace_with)),
                right: Box::new(right.operand_replace(op, replace_with)),
            },
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub enum TypedExpression80 {
    Concatenate872 {
        left: Box<TypedExpression8>,
        right: Box<TypedExpression72>,
    },
}

impl TypedExpression80 {
    pub fn operand_replace(&self, op: OperandIdx, replace_with: &ReplaceWith) -> Self {
        match self {
            Self::Concatenate872 { left, right } => Self::Concatenate872 {
                left: Box::new(left.operand_replace(op, replace_with)),
                right: Box::new(right.operand_replace(op, replace_with)),
            },
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub enum TypedExpression72 {
    Concatenate864 {
        left: Box<TypedExpression8>,
        right: Box<TypedExpression64>,
    },
}

impl TypedExpression72 {
    pub fn operand_replace(&self, op: OperandIdx, replace_with: &ReplaceWith) -> Self {
        match self {
            Self::Concatenate864 { left, right } => Self::Concatenate864 {
                left: Box::new(left.operand_replace(op, replace_with)),
                right: Box::new(right.operand_replace(op, replace_with)),
            },
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub enum TypedExpression64 {
    OperandR8 {
        operand_idx: OperandIdx,
    },
    Concatenate568 {
        left: Box<TypedExpression56>,
        right: Box<TypedExpression8>,
    },
    OperandR64 {
        operand_idx: OperandIdx,
    },
    R64 {
        reg: Reg64WithRIP,
    },
    Load(Box<TypedExpression64>),
    Store {
        //todo this doesn't belong here, since it returns nothing
        address: Box<TypedExpression64>,
        value: Box<TypedExpression64>,
    },
    Sub {
        left: Box<TypedExpression64>,
        right: Box<TypedExpression64>,
    },
    Constant(i128),
    Extract128 {
        source: Box<TypedExpression128>,
        base: usize,
    },
    Float2MInt {
        inner: Box<TypedExpressionF64>,
    },
    And {
        left: Box<TypedExpression64>,
        right: Box<TypedExpression64>,
    },
    Extract256 {
        source: Box<TypedExpression256>,
        base: usize,
    },
    Concatenate856 {
        left: Box<TypedExpression8>,
        right: Box<TypedExpression56>,
    },
}

impl TypedExpression64 {
    pub fn operand_replace(&self, op: OperandIdx, replace_with: &ReplaceWith) -> TypedExpression64 {
        match self {
            TypedExpression64::OperandR8 { operand_idx } => {
                if operand_idx == &op {
                    todo!() /*replace_with.unwrap_8()*/
                } else {
                    self.clone()
                }
            }
            TypedExpression64::OperandR64 { operand_idx } => {
                if operand_idx == &op {
                    replace_with._64.clone()
                } else {
                    self.clone()
                }
            }
            TypedExpression64::Concatenate568 { left, right } => {
                TypedExpression64::Concatenate568 {
                    left: Box::new(left.operand_replace(op, replace_with)),
                    right: Box::new(right.operand_replace(op, replace_with)),
                }
            }
            TypedExpression64::Load(addr) => {
                TypedExpression64::Load(Box::new(addr.operand_replace(op, replace_with)))
            }
            TypedExpression64::Store { address, value } => TypedExpression64::Store {
                address: Box::new(address.operand_replace(op, replace_with)),
                value: Box::new(value.operand_replace(op, replace_with)),
            },
            TypedExpression64::Sub { left, right } => TypedExpression64::Sub {
                left: Box::new(left.operand_replace(op, replace_with)),
                right: Box::new(right.operand_replace(op, replace_with)),
            },
            TypedExpression64::R64 { reg } => TypedExpression64::R64 { reg: *reg },
            TypedExpression64::Constant(con) => TypedExpression64::Constant(*con),
            TypedExpression64::Extract128 { source, base } => TypedExpression64::Extract128 {
                source: Box::new(source.operand_replace(op, replace_with)),
                base: *base,
            },
            TypedExpression64::Float2MInt { inner } => TypedExpression64::Float2MInt {
                inner: Box::new(inner.operand_replace(op, replace_with)),
            },
            TypedExpression64::And { .. } => {
                todo!()
            }
            TypedExpression64::Extract256 { .. } => {
                todo!()
            }
            TypedExpression64::Concatenate856 { left, right } => {
                TypedExpression64::Concatenate856 {
                    left: Box::new(left.operand_replace(op, replace_with)),
                    right: Box::new(right.operand_replace(op, replace_with)),
                }
            }
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub enum TypedExpression56 {
    Extract64 {
        source: TypedExpression64,
        base: usize,
    },
    Load(Box<TypedExpression64>),
    Concatenate848 {
        left: Box<TypedExpression8>,
        right: Box<TypedExpression48>,
    },
}

impl TypedExpression56 {
    fn operand_replace(&self, op: OperandIdx, replace_with: &ReplaceWith) -> TypedExpression56 {
        match self {
            TypedExpression56::Extract64 { source, base } => {
                let source = source.operand_replace(op, replace_with);
                TypedExpression56::Extract64 {
                    source,
                    base: *base,
                }
            }
            TypedExpression56::Load(_) => {
                todo!()
            }
            TypedExpression56::Concatenate848 { left, right } => {
                TypedExpression56::Concatenate848 {
                    left: Box::new(left.operand_replace(op, replace_with)),
                    right: Box::new(right.operand_replace(op, replace_with)),
                }
            }
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub enum TypedExpression48 {
    Concatenate840 {
        left: Box<TypedExpression8>,
        right: Box<TypedExpression40>,
    },
}

impl TypedExpression48 {
    pub fn operand_replace(&self, op: OperandIdx, replace_with: &ReplaceWith) -> Self {
        match self {
            Self::Concatenate840 { left, right } => Self::Concatenate840 {
                left: Box::new(left.operand_replace(op, replace_with)),
                right: Box::new(right.operand_replace(op, replace_with)),
            },
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub enum TypedExpression40 {
    Concatenate832 {
        left: Box<TypedExpression8>,
        right: Box<TypedExpression32>,
    },
}

impl TypedExpression40 {
    pub fn operand_replace(&self, op: OperandIdx, replace_with: &ReplaceWith) -> Self {
        match self {
            Self::Concatenate832 { left, right } => Self::Concatenate832 {
                left: Box::new(left.operand_replace(op, replace_with)),
                right: Box::new(right.operand_replace(op, replace_with)),
            },
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub enum TypedExpression32 {
    Concatenate824 {
        left: Box<TypedExpression8>,
        right: Box<TypedExpression24>,
    },
}

impl TypedExpression32 {
    pub fn operand_replace(&self, op: OperandIdx, replace_with: &ReplaceWith) -> Self {
        match self {
            Self::Concatenate824 { left, right } => Self::Concatenate824 {
                left: Box::new(left.operand_replace(op, replace_with)),
                right: Box::new(right.operand_replace(op, replace_with)),
            },
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub enum TypedExpression24 {
    Concatenate816 {
        left: Box<TypedExpression8>,
        right: Box<TypedExpression16>,
    },
}

impl TypedExpression24 {
    pub fn operand_replace(&self, op: OperandIdx, replace_with: &ReplaceWith) -> Self {
        match self {
            Self::Concatenate816 { left, right } => Self::Concatenate816 {
                left: Box::new(left.operand_replace(op, replace_with)),
                right: Box::new(right.operand_replace(op, replace_with)),
            },
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub enum TypedExpression16 {
    Concatenate88 {
        left: Box<TypedExpression8>,
        right: Box<TypedExpression8>,
    },
}

impl TypedExpression16 {
    pub fn operand_replace(&self, op: OperandIdx, replace_with: &ReplaceWith) -> Self {
        match self {
            Self::Concatenate88 { left, right } => Self::Concatenate88 {
                left: Box::new(left.operand_replace(op, replace_with)),
                right: Box::new(right.operand_replace(op, replace_with)),
            },
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
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
    Load(Box<TypedExpression64>),
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub enum TypedExpression8 {
    Extract {
        source: TypedExpression64,
        base: usize,
    },
    Extract9 {
        source: TypedExpression9,
        base: usize,
    },
    Extract64 {
        source: TypedExpression64,
        base: usize,
    },
    Constant(i16),
    Store {
        address: Box<TypedExpression64>,
        value: Box<TypedExpression8>,
    },
    And {
        left: Box<TypedExpression8>,
        right: Box<TypedExpression8>,
    },
    OperandR8 {
        operand_idx: OperandIdx,
    },
    Load(Box<TypedExpression64>),
    OperandR1 {
        operand_idx: OperandIdx,
    },
    Extract128 {
        source: TypedExpression128,
        base: usize,
    },
    Extract256 {
        source: TypedExpression256,
        base: usize,
    },
    IfThenElse {
        condition: TypedExpression1,
        true_case: Box<TypedExpression8>,
        false_case: Box<TypedExpression8>,
    },
}

impl TypedExpression8 {
    pub(crate) fn operand_replace(
        &self,
        op: OperandIdx,
        replace_with: &ReplaceWith,
    ) -> TypedExpression8 {
        match self {
            TypedExpression8::Extract { .. } => {
                todo!()
            }
            TypedExpression8::Extract9 { .. } => {
                todo!()
            }
            TypedExpression8::Extract64 { source, base } => TypedExpression8::Extract64 {
                source: source.operand_replace(op, replace_with),
                base: *base,
            },
            TypedExpression8::Constant(con) => TypedExpression8::Constant(*con),
            TypedExpression8::Store { .. } => {
                todo!()
            }
            TypedExpression8::And { left, right } => TypedExpression8::And {
                left: Box::new(left.operand_replace(op, replace_with)),
                right: Box::new(right.operand_replace(op, replace_with)),
            },
            TypedExpression8::OperandR8 { operand_idx } => {
                if &op == operand_idx {
                    return replace_with._8.clone();
                }
                todo!()
            }
            TypedExpression8::Load(_) => {
                todo!()
            }
            TypedExpression8::OperandR1 { .. } => {
                todo!()
            }
            TypedExpression8::Extract128 { source, base } => TypedExpression8::Extract128 {
                source: source.operand_replace(op, replace_with),
                base: *base,
            },
            TypedExpression8::Extract256 { source, base } => TypedExpression8::Extract256 {
                source: source.operand_replace(op, replace_with),
                base: *base,
            },
            TypedExpression8::IfThenElse {
                condition,
                true_case,
                false_case,
            } => TypedExpression8::IfThenElse {
                condition: condition.operand_replace(op, replace_with),
                true_case: Box::new(true_case.operand_replace(op, replace_with)),
                false_case: Box::new(false_case.operand_replace(op, replace_with)),
            },
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
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
    Extract9 {
        source: Box<TypedExpression9>,
        base: usize,
    },
    Not(Box<TypedExpression1>),
    XorBool {
        left: Box<TypedExpression1>,
        right: Box<TypedExpression1>,
    },
    Extract64 {
        source: Box<TypedExpression64>,
        base: usize,
    },
    Xor {
        left: Box<TypedExpression1>,
        right: Box<TypedExpression1>,
    },
    Equals8 {
        left: Box<TypedExpression8>,
        right: Box<TypedExpression8>,
    },
    AndBool {
        left: Box<TypedExpression1>,
        right: Box<TypedExpression1>,
    },
    Load(Box<TypedExpression64>),
    And {
        left: Box<TypedExpression1>,
        right: Box<TypedExpression1>,
    },
    Undefined,
    OperandR1 {
        operand_idx: OperandIdx,
    },
    Extract8 {
        source: Box<TypedExpression8>,
        base: usize,
    },
    Extract256 {
        source: Box<TypedExpression256>,
        base: usize,
    },
}

impl TypedExpression1 {
    pub(crate) fn operand_replace(
        &self,
        op: OperandIdx,
        replace_with: &ReplaceWith,
    ) -> TypedExpression1 {
        match self {
            TypedExpression1::FlagCF => {
                todo!()
            }
            TypedExpression1::Constant(const_) => TypedExpression1::Constant(*const_),
            TypedExpression1::Equals1 { left, right } => TypedExpression1::Equals1 {
                left: Box::new(left.operand_replace(op, replace_with)),
                right: Box::new(right.operand_replace(op, replace_with)),
            },
            TypedExpression1::IfThenElse {
                condition,
                true_case,
                false_case,
            } => TypedExpression1::IfThenElse {
                condition: Box::new(condition.operand_replace(op, replace_with)),
                true_case: Box::new(true_case.operand_replace(op, replace_with)),
                false_case: Box::new(false_case.operand_replace(op, replace_with)),
            },
            TypedExpression1::Extract9 { .. } => {
                todo!()
            }
            TypedExpression1::Not(inner) => {
                TypedExpression1::Not(Box::new(inner.operand_replace(op, replace_with)))
            }
            TypedExpression1::XorBool { left, right } => TypedExpression1::XorBool {
                left: Box::new(left.operand_replace(op, replace_with)),
                right: Box::new(right.operand_replace(op, replace_with)),
            },
            TypedExpression1::Extract64 { source, base } => TypedExpression1::Extract64 {
                source: Box::new(source.operand_replace(op, replace_with)),
                base: *base,
            },
            TypedExpression1::Xor { .. } => {
                todo!()
            }
            TypedExpression1::Equals8 { left, right } => TypedExpression1::Equals8 {
                left: Box::new(left.operand_replace(op, replace_with)),
                right: Box::new(right.operand_replace(op, replace_with)),
            },
            TypedExpression1::AndBool { .. } => {
                todo!()
            }
            TypedExpression1::Load(_) => {
                todo!()
            }
            TypedExpression1::And { left, right } => TypedExpression1::And {
                left: Box::new(left.operand_replace(op, replace_with)),
                right: Box::new(right.operand_replace(op, replace_with)),
            },
            TypedExpression1::Undefined => TypedExpression1::Undefined,
            TypedExpression1::OperandR1 { operand_idx } => {
                if op == *operand_idx {
                    return replace_with._1.clone();
                }
                todo!()
            }
            TypedExpression1::Extract8 { source, base } => TypedExpression1::Extract8 {
                source: Box::new(source.operand_replace(op, replace_with)),
                base: *base,
            },
            TypedExpression1::Extract256 { source, base } => TypedExpression1::Extract256 {
                source: Box::new(source.operand_replace(op, replace_with)),
                base: *base,
            },
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub enum TypedExpression128 {
    Constant(i128),
    OperandR128 {
        operand_idx: OperandIdx,
    },
    Concatenate6464 {
        left: Box<TypedExpression64>,
        right: Box<TypedExpression64>,
    },
    Load(Box<TypedExpression64>),
    OperandR256 {
        operand_idx: OperandIdx,
    },
    Extract256 {
        source: Box<TypedExpression256>,
        base: usize,
    },
    Neg(Box<TypedExpression128>),
    And {
        left: Box<TypedExpression128>,
        right: Box<TypedExpression128>,
    },
    Concatenate1208 {
        left: Box<TypedExpression8>,
        right: Box<TypedExpression120>,
    },
}

impl TypedExpression128 {
    pub fn operand_replace(
        &self,
        op: OperandIdx,
        replace_with: &ReplaceWith,
    ) -> TypedExpression128 {
        match self {
            TypedExpression128::Constant(_) => {
                todo!()
            }
            TypedExpression128::OperandR128 { operand_idx } => {
                if op == *operand_idx {
                    return replace_with._128.clone();
                }
                todo!()
            }
            TypedExpression128::Concatenate6464 { .. } => {
                todo!()
            }
            TypedExpression128::Load(_) => {
                todo!()
            }
            TypedExpression128::OperandR256 { .. } => {
                todo!()
            }
            TypedExpression128::Extract256 { source, base } => TypedExpression128::Extract256 {
                source: Box::new(source.operand_replace(op, replace_with)),
                base: *base,
            },
            TypedExpression128::Neg(_) => {
                todo!()
            }
            TypedExpression128::And { .. } => {
                todo!()
            }
            TypedExpression128::Concatenate1208 { left, right } => {
                TypedExpression128::Concatenate1208 {
                    left: Box::new(left.operand_replace(op, replace_with)),
                    right: Box::new(right.operand_replace(op, replace_with)),
                }
            }
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub enum TypedExpressionF64 {
    FloatAdd {
        left: Box<TypedExpressionF64>,
        right: Box<TypedExpressionF64>,
    },
    FloatMul {
        left: Box<TypedExpressionF64>,
        right: Box<TypedExpressionF64>,
    },
    MInt2Float {
        from: TypedExpression64,
        range_end: i32,
        range_start: i32,
    },
}

impl TypedExpressionF64 {
    pub fn operand_replace(
        &self,
        op: OperandIdx,
        replace_with: &ReplaceWith,
    ) -> TypedExpressionF64 {
        match self {
            TypedExpressionF64::FloatAdd { left, right } => TypedExpressionF64::FloatAdd {
                left: Box::new(left.operand_replace(op, replace_with)),
                right: Box::new(right.operand_replace(op, replace_with)),
            },
            TypedExpressionF64::FloatMul { left, right } => TypedExpressionF64::FloatMul {
                left: Box::new(left.operand_replace(op, replace_with)),
                right: Box::new(right.operand_replace(op, replace_with)),
            },
            TypedExpressionF64::MInt2Float {
                from,
                range_end,
                range_start,
            } => TypedExpressionF64::MInt2Float {
                from: from.operand_replace(op, replace_with),
                range_end: *range_end,
                range_start: *range_start,
            },
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub enum TypedExpression256 {
    Concatenate128128 {
        left: Box<TypedExpression128>,
        right: Box<TypedExpression128>,
    },
    Load(Box<TypedExpression64>),
    OperandR256 {
        operand_idx: OperandIdx,
    },
    And {
        left: Box<TypedExpression256>,
        right: Box<TypedExpression256>,
    },
    R256 {
        reg: RegYMM,
    },
}

impl TypedExpression256 {
    pub fn operand_replace(
        &self,
        op: OperandIdx,
        replace_with: &ReplaceWith,
    ) -> TypedExpression256 {
        match self {
            TypedExpression256::Concatenate128128 { left, right } => {
                TypedExpression256::Concatenate128128 {
                    left: Box::new(left.operand_replace(op, replace_with)),
                    right: Box::new(right.operand_replace(op, replace_with)),
                }
            }
            TypedExpression256::Load(_) => {
                todo!()
            }
            TypedExpression256::OperandR256 { operand_idx } => {
                if op == *operand_idx {
                    return replace_with._256.clone();
                }
                return self.clone();
            }
            TypedExpression256::And { .. } => {
                todo!()
            }
            TypedExpression256::R256 { .. } => self.clone(),
        }
    }
}
