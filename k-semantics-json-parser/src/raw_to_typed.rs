use wrapper_common::operand_type::{MemoryOperandTypeKind, OperandType};
use wrapper_common::registers::{Reg64WithRIP, RegYMM, RegisterType};

use crate::raw::{RawExpression, RawToken, SemanticCastKind};
use crate::typed_semantics::{
    TypedExpression, TypedExpression1, TypedExpression104, TypedExpression112, TypedExpression120,
    TypedExpression128, TypedExpression16, TypedExpression24, TypedExpression256,
    TypedExpression32, TypedExpression40, TypedExpression48, TypedExpression56, TypedExpression64,
    TypedExpression72, TypedExpression8, TypedExpression80, TypedExpression88, TypedExpression9,
    TypedExpression96, TypedExpressionF64,
};
use crate::InstructionDescriptor;

#[derive(Debug)]
pub enum ExpressionType {
    _1,
    _8,
    _9,
    _16,
    _64,
    _128,
    _256,
}

pub fn expr_to_typed_expr(
    expr: &RawExpression,
    expected_type: Option<&ExpressionType>,
    instruction_desc: &InstructionDescriptor,
) -> TypedExpression {
    match expr {
        RawExpression::Op(op_idx) => {
            match expected_type {
                Some(ExpressionType::_64) => TypedExpression::_64(TypedExpression64::OperandR64 {
                    operand_idx: *op_idx,
                }),
                Some(ExpressionType::_8) => TypedExpression::_8(TypedExpression8::OperandR8 {
                    operand_idx: *op_idx,
                }),
                Some(ExpressionType::_1) => TypedExpression::_1(TypedExpression1::OperandR1 {
                    operand_idx: *op_idx,
                }),
                Some(ExpressionType::_256) => {
                    TypedExpression::_256(TypedExpression256::OperandR256 {
                        operand_idx: *op_idx,
                    })
                }
                Some(ExpressionType::_9)
                | Some(ExpressionType::_16)
                | Some(ExpressionType::_128) => {
                    todo!()
                }
                None => {
                    let operand = &instruction_desc.operands[op_idx.0 as usize];
                    match operand {
                        OperandType::Reg(reg) => match reg {
                            RegisterType::AllGP64WithoutRIP
                            | RegisterType::AllGP64WithRIP
                            | RegisterType::SingleGP64(_) => {
                                TypedExpression::_64(TypedExpression64::OperandR64 {
                                    operand_idx: *op_idx,
                                })
                            }
                            reg => {
                                todo!("{reg:?}")
                            }
                        },
                        OperandType::Mem(mem) => {
                            //todo need to have a memory operand secition
                            match mem.kind {
                                MemoryOperandTypeKind::Mem8 => {
                                    TypedExpression::_8(TypedExpression8::OperandR8 {
                                        operand_idx: *op_idx,
                                    })
                                }
                                MemoryOperandTypeKind::Mem128 => {
                                    TypedExpression::_128(TypedExpression128::OperandR128 {
                                        operand_idx: *op_idx,
                                    })
                                }
                                kind => {
                                    todo!("{kind:?}")
                                }
                            }
                        }
                        _ => {
                            todo!("{operand:?}")
                        }
                    }
                }
            }
        }
        RawExpression::IfElse {
            condition,
            true_case,
            false_case,
        } => {
            let condition = expr_to_typed_expr(
                condition.as_ref(),
                Some(&ExpressionType::_1),
                instruction_desc,
            )
            .unwrap_1();
            let true_case = expr_to_typed_expr(true_case.as_ref(), expected_type, instruction_desc);
            let false_case =
                expr_to_typed_expr(false_case.as_ref(), expected_type, instruction_desc);
            match true_case {
                TypedExpression::_9(true_case) => {
                    let false_case = false_case.unwrap_9();
                    TypedExpression::_9(TypedExpression9::IfThenElse {
                        condition,
                        true_case: Box::new(true_case),
                        false_case: Box::new(false_case),
                    })
                }
                TypedExpression::_8(true_case) => {
                    let false_case = false_case.unwrap_8();
                    TypedExpression::_8(TypedExpression8::IfThenElse {
                        condition,
                        true_case: Box::new(true_case),
                        false_case: Box::new(false_case),
                    })
                }
                TypedExpression::_1(true_case) => {
                    let false_case = false_case.unwrap_1();
                    TypedExpression::_1(TypedExpression1::IfThenElse {
                        condition: Box::new(condition),
                        true_case: Box::new(true_case),
                        false_case: Box::new(false_case),
                    })
                }
                true_case => {
                    todo!("{true_case:?}")
                }
            }
        }
        RawExpression::AndBool { left, right } => {
            let left = expr_to_typed_expr(left.as_ref(), expected_type, instruction_desc);
            let right = expr_to_typed_expr(right.as_ref(), expected_type, instruction_desc);
            match left {
                TypedExpression::_1(left) => {
                    let right = right.unwrap_1();
                    TypedExpression::_1(TypedExpression1::AndBool {
                        left: Box::new(left),
                        right: Box::new(right),
                    })
                }
                _ => todo!(),
            }
        }
        RawExpression::EqualsBool { left, right } => {
            let left = expr_to_typed_expr(left.as_ref(), expected_type, instruction_desc);
            let right = expr_to_typed_expr(right.as_ref(), expected_type, instruction_desc);
            match left {
                TypedExpression::_1(left) => {
                    let right = right.unwrap_1();
                    TypedExpression::_1(TypedExpression1::Equals1 {
                        left: Box::new(left),
                        right: Box::new(right),
                    })
                }
                _ => todo!(),
            }
        }
        RawExpression::Equals { left, right } => {
            let left = expr_to_typed_expr(left.as_ref(), None, instruction_desc);
            let right = expr_to_typed_expr(right.as_ref(), None, instruction_desc);
            match left {
                TypedExpression::_8(left) => {
                    let right = right.unwrap_8();
                    TypedExpression::_1(TypedExpression1::Equals8 {
                        left: Box::new(left),
                        right: Box::new(right),
                    })
                }
                TypedExpression::_1(left) => {
                    let right = right.unwrap_1();
                    TypedExpression::_1(TypedExpression1::Equals1 {
                        left: Box::new(left),
                        right: Box::new(right),
                    })
                }
                _ => todo!(),
            }
        }
        RawExpression::MI { len, val } => {
            let len = extract_num(len);
            let val = extract_num(val);
            if len == 1 {
                return TypedExpression::_1(TypedExpression1::Constant(val != 0));
            }
            if len == 8 {
                return TypedExpression::_8(TypedExpression8::Constant(val as i16));
            }
            if len == 9 {
                return TypedExpression::_9(TypedExpression9::Constant(val as i16));
            }
            if len == 64 {
                return TypedExpression::_64(TypedExpression64::Constant(val));
            }
            if len == 128 {
                return TypedExpression::_128(TypedExpression128::Constant(val));
            }
            dbg!(val);
            todo!("{len}")
        }
        RawExpression::Extract {
            from,
            range_start,
            range_end,
        } => {
            let from = expr_to_typed_expr(from.as_ref(), None, instruction_desc);
            let range_start = extract_num(range_start);
            let range_end = extract_num(range_end);
            if (range_end - range_start) == 56 {
                TypedExpression::_56(TypedExpression56::Extract64 {
                    source: from.unwrap_64(),
                    base: range_start as usize,
                })
            } else if (range_end - range_start) == 8 {
                match from {
                    TypedExpression::_256(inner) => {
                        TypedExpression::_8(TypedExpression8::Extract256 {
                            source: inner,
                            base: range_start as usize,
                        })
                    }
                    TypedExpression::_128(inner) => {
                        TypedExpression::_8(TypedExpression8::Extract128 {
                            source: inner,
                            base: range_start as usize,
                        })
                    }
                    TypedExpression::_64(inner) => {
                        TypedExpression::_8(TypedExpression8::Extract64 {
                            source: inner,
                            base: range_start as usize,
                        })
                    }
                    TypedExpression::_9(inner) => TypedExpression::_8(TypedExpression8::Extract9 {
                        source: inner,
                        base: range_start as usize,
                    }),
                    from => todo!("{from:?}"),
                }
            } else if (range_end - range_start) == 1 {
                match from {
                    TypedExpression::_64(inner) => {
                        TypedExpression::_1(TypedExpression1::Extract64 {
                            source: Box::new(inner),
                            base: range_start as usize,
                        })
                    }
                    TypedExpression::_9(inner) => TypedExpression::_1(TypedExpression1::Extract9 {
                        source: Box::new(inner),
                        base: range_start as usize,
                    }),
                    TypedExpression::_8(inner) => TypedExpression::_1(TypedExpression1::Extract8 {
                        source: Box::new(inner),
                        base: range_start as usize,
                    }),
                    TypedExpression::_256(inner) => {
                        TypedExpression::_1(TypedExpression1::Extract256 {
                            source: Box::new(inner),
                            base: range_start as usize,
                        })
                    }
                    _ => todo!(),
                }
            } else if (range_end - range_start) == 64 {
                match from {
                    TypedExpression::_128(inner) => {
                        TypedExpression::_64(TypedExpression64::Extract128 {
                            source: Box::new(inner),
                            base: range_start as usize,
                        })
                    }
                    TypedExpression::_256(inner) => {
                        TypedExpression::_64(TypedExpression64::Extract256 {
                            source: Box::new(inner),
                            base: range_start as usize,
                        })
                    }
                    _ => todo!(),
                }
            } else if (range_end - range_start) == 128 {
                match from {
                    TypedExpression::_256(inner) => {
                        TypedExpression::_128(TypedExpression128::Extract256 {
                            source: Box::new(inner),
                            base: range_start as usize,
                        })
                    }
                    from => todo!("{from:?}"),
                }
            } else {
                todo!()
            }
        }
        RawExpression::Concatenate { left, right } => {
            let left = expr_to_typed_expr(left.as_ref(), None, instruction_desc);
            let right = expr_to_typed_expr(right.as_ref(), None, instruction_desc);
            match left {
                TypedExpression::_56(_56) => match right {
                    TypedExpression::_8(_8) => {
                        TypedExpression::_64(TypedExpression64::Concatenate568 {
                            left: Box::new(_56),
                            right: Box::new(_8),
                        })
                    }
                    _ => todo!(),
                },
                TypedExpression::_1(_1) => match right {
                    TypedExpression::_8(_8) => {
                        TypedExpression::_9(TypedExpression9::Concatenate18 {
                            left: _1,
                            right: Box::new(_8),
                        })
                    }
                    _ => todo!(),
                },
                TypedExpression::_8(_8_left) => match right {
                    TypedExpression::_8(_8_right) => {
                        TypedExpression::_16(TypedExpression16::Concatenate88 {
                            left: Box::new(_8_left),
                            right: Box::new(_8_right),
                        })
                    }
                    TypedExpression::_16(_16_right) => {
                        TypedExpression::_24(TypedExpression24::Concatenate816 {
                            left: Box::new(_8_left),
                            right: Box::new(_16_right),
                        })
                    }
                    TypedExpression::_24(_24_right) => {
                        TypedExpression::_32(TypedExpression32::Concatenate824 {
                            left: Box::new(_8_left),
                            right: Box::new(_24_right),
                        })
                    }
                    TypedExpression::_32(_32_right) => {
                        TypedExpression::_40(TypedExpression40::Concatenate832 {
                            left: Box::new(_8_left),
                            right: Box::new(_32_right),
                        })
                    }
                    TypedExpression::_40(_40_right) => {
                        TypedExpression::_48(TypedExpression48::Concatenate840 {
                            left: Box::new(_8_left),
                            right: Box::new(_40_right),
                        })
                    }
                    TypedExpression::_48(_48_right) => {
                        TypedExpression::_56(TypedExpression56::Concatenate848 {
                            left: Box::new(_8_left),
                            right: Box::new(_48_right),
                        })
                    }
                    TypedExpression::_56(_56_right) => {
                        TypedExpression::_64(TypedExpression64::Concatenate856 {
                            left: Box::new(_8_left),
                            right: Box::new(_56_right),
                        })
                    }
                    TypedExpression::_64(_64_right) => {
                        TypedExpression::_72(TypedExpression72::Concatenate864 {
                            left: Box::new(_8_left),
                            right: Box::new(_64_right),
                        })
                    }
                    TypedExpression::_72(_72_right) => {
                        TypedExpression::_80(TypedExpression80::Concatenate872 {
                            left: Box::new(_8_left),
                            right: Box::new(_72_right),
                        })
                    }
                    TypedExpression::_80(_80_right) => {
                        TypedExpression::_88(TypedExpression88::Concatenate880 {
                            left: Box::new(_8_left),
                            right: Box::new(_80_right),
                        })
                    }
                    TypedExpression::_88(_88_right) => {
                        TypedExpression::_96(TypedExpression96::Concatenate888 {
                            left: Box::new(_8_left),
                            right: Box::new(_88_right),
                        })
                    }
                    TypedExpression::_96(_96_right) => {
                        TypedExpression::_104(TypedExpression104::Concatenate968 {
                            left: Box::new(_8_left),
                            right: Box::new(_96_right),
                        })
                    }
                    TypedExpression::_104(_104_right) => {
                        TypedExpression::_112(TypedExpression112::Concatenate1048 {
                            left: Box::new(_8_left),
                            right: Box::new(_104_right),
                        })
                    }
                    TypedExpression::_112(_112_right) => {
                        TypedExpression::_120(TypedExpression120::Concatenate1128 {
                            left: Box::new(_8_left),
                            right: Box::new(_112_right),
                        })
                    }
                    TypedExpression::_120(_120_right) => {
                        TypedExpression::_128(TypedExpression128::Concatenate1208 {
                            left: Box::new(_8_left),
                            right: Box::new(_120_right),
                        })
                    }
                    right => todo!("{right:?}"),
                },
                TypedExpression::_64(left) => match right {
                    TypedExpression::_128(_) => {
                        todo!()
                    }
                    TypedExpression::_64(right) => {
                        TypedExpression::_128(TypedExpression128::Concatenate6464 {
                            left: Box::new(left),
                            right: Box::new(right),
                        })
                    }
                    TypedExpression::_56(_) => {
                        todo!()
                    }
                    TypedExpression::_9(_) => {
                        todo!()
                    }
                    TypedExpression::_8(_) => {
                        todo!()
                    }
                    TypedExpression::_1(_) => {
                        todo!()
                    }
                    TypedExpression::_256(_) => {
                        todo!()
                    }
                    TypedExpression::_16(_) => {
                        todo!()
                    }
                    other => todo!("{other:?}"),
                },
                TypedExpression::_128(left) => match right {
                    TypedExpression::_256(_) => {
                        todo!()
                    }
                    TypedExpression::_128(right) => {
                        TypedExpression::_256(TypedExpression256::Concatenate128128 {
                            left: Box::new(left),
                            right: Box::new(right),
                        })
                    }
                    TypedExpression::_64(_) => {
                        todo!()
                    }
                    TypedExpression::_56(_) => {
                        todo!()
                    }
                    TypedExpression::_9(_) => {
                        todo!()
                    }
                    TypedExpression::_8(_) => {
                        todo!()
                    }
                    TypedExpression::_1(_) => {
                        todo!()
                    }
                    TypedExpression::_16(_) => {
                        todo!()
                    }
                    other => todo!("{other:?}"),
                },
                _ => todo!("{left:?}"),
            }
        }
        RawExpression::GetParentValue { lookup, map } => handle_get_parent_value(lookup, map),
        RawExpression::GetFlag { lookup, map } => {
            match (lookup.as_ref(), map.as_ref()) {
                (
                    RawExpression::Token(RawToken::CF),
                    RawExpression::SemanticCast { kind, inner },
                ) => {
                    if let (SemanticCastKind::Map, RawExpression::RSMap) = (kind, inner.as_ref()) {
                        return TypedExpression::_1(TypedExpression1::FlagCF);
                    }
                }
                _ => todo!(),
            }

            todo!()
        }
        RawExpression::SemanticCast { kind, inner } => match kind {
            SemanticCastKind::R8 => {
                todo!()
            }
            SemanticCastKind::Map => {
                todo!()
            }
            SemanticCastKind::MInt => {
                expr_to_typed_expr(inner.as_ref(), expected_type, instruction_desc)
            }
            SemanticCastKind::Xmm => {
                todo!()
            }
            SemanticCastKind::R64 => {
                todo!()
            }
            SemanticCastKind::RH => {
                todo!()
            }
            SemanticCastKind::Imm => {
                todo!()
            }
        },
        RawExpression::ConstantInt(const_) => match expected_type {
            Some(ExpressionType::_64) => TypedExpression::_64(TypedExpression64::Constant(*const_)),
            expected_type => todo!("{expected_type:?}"),
        },
        RawExpression::RSMap => {
            todo!()
        }
        RawExpression::NotBool { inner } => {
            let inner = expr_to_typed_expr(inner, expected_type, instruction_desc);
            TypedExpression::_1(TypedExpression1::Not(Box::new(inner.unwrap_1())))
        }
        RawExpression::Add { left, right } => {
            let left = expr_to_typed_expr(left.as_ref(), expected_type, instruction_desc);
            let right = expr_to_typed_expr(right.as_ref(), expected_type, instruction_desc);
            match left {
                TypedExpression::_9(left) => match right {
                    TypedExpression::_9(right) => TypedExpression::_9(TypedExpression9::Equals {
                        left: Box::new(left),
                        right: Box::new(right),
                    }),
                    _ => todo!(),
                },
                _ => todo!(),
            }
        }
        RawExpression::Xor { left, right } => {
            let left = expr_to_typed_expr(left.as_ref(), expected_type, instruction_desc);
            let right = expr_to_typed_expr(right.as_ref(), expected_type, instruction_desc);
            match left {
                TypedExpression::_1(left) => match right {
                    TypedExpression::_1(right) => TypedExpression::_1(TypedExpression1::Xor {
                        left: Box::new(left),
                        right: Box::new(right),
                    }),
                    _ => todo!(),
                },
                _ => todo!(),
            }
        }
        RawExpression::XorBool { left, right } => {
            let left = expr_to_typed_expr(left, expected_type, instruction_desc);
            let right = expr_to_typed_expr(right, expected_type, instruction_desc);
            match left {
                TypedExpression::_1(left) => match right {
                    TypedExpression::_1(right) => TypedExpression::_1(TypedExpression1::XorBool {
                        left: Box::new(left),
                        right: Box::new(right),
                    }),
                    _ => todo!(),
                },
                _ => todo!(),
            }
        }
        RawExpression::Token(_) => {
            todo!()
        }
        RawExpression::LoadFromMemory { .. } => {
            todo!()
        }
        RawExpression::StoreFromMemory { .. } => {
            todo!()
        }
        RawExpression::ProjectMInt { inner } => {
            let inner = inner.as_ref();
            expr_to_typed_expr(inner, expected_type, instruction_desc)
        }
        RawExpression::SubMInt { left, right } => match expected_type {
            Some(ExpressionType::_64) => {
                let left = Box::new(
                    expr_to_typed_expr(left.as_ref(), expected_type, instruction_desc).unwrap_64(),
                );
                let right = Box::new(
                    expr_to_typed_expr(right.as_ref(), expected_type, instruction_desc).unwrap_64(),
                );
                TypedExpression::_64(TypedExpression64::Sub { left, right })
            }
            _ => {
                todo!()
            }
        },
        RawExpression::MapLookup { lookup, map: _ } => {
            if let RawExpression::Token(raw_token) = lookup.as_ref() {
                TypedExpression::_64(TypedExpression64::R64 {
                    reg: raw_token_to_reg64(raw_token),
                })
            } else {
                panic!()
            }
        }
        RawExpression::GetRegisterValue { lookup, map: _ } => {
            if let RawExpression::Token(raw_token) = lookup.as_ref() {
                TypedExpression::_64(TypedExpression64::R64 {
                    reg: raw_token_to_reg64(raw_token),
                })
            } else {
                panic!()
            }
        }
        RawExpression::DecRSPInBytes { .. } => {
            todo!()
        }
        RawExpression::FunctionCall { token, args } => {
            match token.as_str() {
                "vfmadd213_double" => {
                    //rule vfmadd213_double(MI1:MInt, MI2:MInt, MI3:MInt) =>
                    //     Float2MInt((MInt2Float(MI2, 53, 11) *Float MInt2Float(MI1, 53, 11)) +Float MInt2Float(MI3, 53, 11), 64)
                    let mi1 =
                        expr_to_typed_expr(&args[0], Some(&ExpressionType::_64), instruction_desc)
                            .unwrap_64();
                    let mi2 =
                        expr_to_typed_expr(&args[1], Some(&ExpressionType::_64), instruction_desc)
                            .unwrap_64();
                    let mi3 =
                        expr_to_typed_expr(&args[2], Some(&ExpressionType::_64), instruction_desc)
                            .unwrap_64();
                    TypedExpression::_64(TypedExpression64::Float2MInt {
                        inner: Box::new(TypedExpressionF64::FloatAdd {
                            left: Box::new(TypedExpressionF64::FloatMul {
                                left: Box::new(TypedExpressionF64::MInt2Float {
                                    from: mi2,
                                    range_end: 53,
                                    range_start: 11,
                                }),
                                right: Box::new(TypedExpressionF64::MInt2Float {
                                    from: mi1,
                                    range_end: 53,
                                    range_start: 11,
                                }),
                            }),
                            right: Box::new(TypedExpressionF64::MInt2Float {
                                from: mi3,
                                range_end: 53,
                                range_start: 11,
                            }),
                        }),
                    })
                }
                _ => todo!(),
            }
        }
        RawExpression::And { left, right } => {
            let left = expr_to_typed_expr(left.as_ref(), expected_type, instruction_desc);
            let right = expr_to_typed_expr(right.as_ref(), expected_type, instruction_desc);
            match expected_type {
                Some(ExpressionType::_8) => TypedExpression::_8(TypedExpression8::And {
                    left: Box::new(left.unwrap_8()),
                    right: Box::new(right.unwrap_8()),
                }),
                Some(ExpressionType::_1) => TypedExpression::_1(TypedExpression1::And {
                    left: Box::new(left.unwrap_1()),
                    right: Box::new(right.unwrap_1()),
                }),
                Some(ExpressionType::_256) => TypedExpression::_256(TypedExpression256::And {
                    left: Box::new(left.unwrap_256()),
                    right: Box::new(right.unwrap_256()),
                }),
                None => match left {
                    TypedExpression::_256(_) => {
                        todo!()
                    }
                    TypedExpression::_128(_) => TypedExpression::_128(TypedExpression128::And {
                        left: Box::new(left.unwrap_128()),
                        right: Box::new(right.unwrap_128()),
                    }),
                    TypedExpression::_64(_) => TypedExpression::_64(TypedExpression64::And {
                        left: Box::new(left.unwrap_64()),
                        right: Box::new(right.unwrap_64()),
                    }),
                    TypedExpression::_56(_) => {
                        todo!()
                    }
                    TypedExpression::_9(_) => {
                        todo!()
                    }
                    TypedExpression::_8(_) => TypedExpression::_8(TypedExpression8::And {
                        left: Box::new(left.unwrap_8()),
                        right: Box::new(right.unwrap_8()),
                    }),
                    TypedExpression::_1(_) => TypedExpression::_1(TypedExpression1::And {
                        left: Box::new(left.unwrap_1()),
                        right: Box::new(right.unwrap_1()),
                    }),
                    TypedExpression::_16(_) => {
                        todo!()
                    }
                    other => todo!("{other:?}"),
                },
                expected => todo!("{expected:?}"),
            }
        }
        RawExpression::Undefined => match expected_type {
            Some(ExpressionType::_64) => {
                todo!()
            }
            None => {
                todo!()
            }
            Some(ExpressionType::_1) => TypedExpression::_1(TypedExpression1::Undefined),
            Some(ExpressionType::_8) => {
                todo!()
            }
            Some(ExpressionType::_9) => {
                todo!()
            }
            Some(ExpressionType::_128) => {
                todo!()
            }
            Some(ExpressionType::_256) => {
                todo!()
            }
            Some(ExpressionType::_16) => {
                todo!()
            }
        },
        RawExpression::Neg { inner } => {
            let inner = expr_to_typed_expr(inner.as_ref(), expected_type, instruction_desc);
            match inner {
                TypedExpression::_256(_) => {
                    todo!()
                }
                TypedExpression::_128(_128) => {
                    TypedExpression::_128(TypedExpression128::Neg(Box::new(_128)))
                }
                TypedExpression::_64(_) => {
                    todo!()
                }
                TypedExpression::_56(_) => {
                    todo!()
                }
                TypedExpression::_9(_) => {
                    todo!()
                }
                TypedExpression::_8(_) => {
                    todo!()
                }
                TypedExpression::_1(_) => {
                    todo!()
                }
                TypedExpression::_16(_) => {
                    todo!()
                }
                other => todo!("{other:?}"),
            }
        }
        RawExpression::LShr { left, right } => {
            let left = expr_to_typed_expr(left.as_ref(), None, instruction_desc);
            let right = expr_to_typed_expr(right.as_ref(), None, instruction_desc);
            dbg!(left);
            dbg!(right);
            todo!()
        }
        RawExpression::UnsignedPortion { inner } => {
            let inner = expr_to_typed_expr(inner.as_ref(), expected_type, instruction_desc);
            dbg!(inner);
            todo!()
        }
        RawExpression::ShiftLeft { left, right } => {
            let left = expr_to_typed_expr(left.as_ref(), None, instruction_desc);
            let right = expr_to_typed_expr(right.as_ref(), None, instruction_desc);
            dbg!(left);
            dbg!(right);
            todo!()
        }
        RawExpression::HandleImmediateWithSignExtend {
            imm,
            length,
            extend_to_length,
        } => {
            todo!()
        }
    }
}

fn raw_token_to_reg64(raw_token: &RawToken) -> Reg64WithRIP {
    match raw_token {
        RawToken::CF => panic!(),
        RawToken::RIP => Reg64WithRIP::RIP,
        RawToken::RSP => Reg64WithRIP::RSP,
        RawToken::YMM0 => panic!(),
        RawToken::RAX => Reg64WithRIP::RAX,
    }
}

fn extract_num(num: &Box<RawExpression>) -> i128 {
    match num.as_ref() {
        RawExpression::ConstantInt(num) => *num,
        _ => panic!(),
    }
}

fn handle_get_parent_value(
    lookup: &Box<RawExpression>,
    map: &Box<RawExpression>,
) -> TypedExpression {
    match (lookup.as_ref(), map.as_ref()) {
        (
            RawExpression::SemanticCast {
                kind: lookup_kind,
                inner: lookup_inner,
            },
            RawExpression::SemanticCast {
                kind: map_kind,
                inner: map_inner,
            },
        ) => {
            if let (SemanticCastKind::Map, RawExpression::RSMap) = (map_kind, map_inner.as_ref()) {
                match lookup_kind {
                    SemanticCastKind::R8 => {
                        if let RawExpression::Op(operand_idx) = lookup_inner.as_ref() {
                            return TypedExpression::_64(TypedExpression64::OperandR8 {
                                operand_idx: *operand_idx,
                            });
                        }
                    }
                    SemanticCastKind::RH => {
                        if let RawExpression::Op(operand_idx) = lookup_inner.as_ref() {
                            return TypedExpression::_64(TypedExpression64::OperandR8 {
                                operand_idx: *operand_idx,
                            });
                        }
                    }
                    SemanticCastKind::R64 => {
                        if let RawExpression::Op(operand_idx) = lookup_inner.as_ref() {
                            return TypedExpression::_64(TypedExpression64::OperandR64 {
                                operand_idx: *operand_idx,
                            });
                        }
                    }
                    SemanticCastKind::Xmm => {
                        if let RawExpression::Op(operand_idx) = lookup_inner.as_ref() {
                            return TypedExpression::_256(TypedExpression256::OperandR256 {
                                operand_idx: *operand_idx,
                            });
                        }
                    }
                    lookup_kind => todo!("{lookup_kind:?}"),
                }
            } else {
                todo!()
            }
        }
        (RawExpression::Token(RawToken::YMM0), _) => {
            return TypedExpression::_256(TypedExpression256::R256 { reg: RegYMM::YMM0 });
        }
        (RawExpression::Token(RawToken::RAX), _) => {
            return TypedExpression::_64(TypedExpression64::R64 {
                reg: Reg64WithRIP::RAX,
            });
        }
        other => todo!("{other:?}"),
    }
    todo!()
}

/*pub fn build_rule(operands_data: RuleOperandsData, expression_data: ExpressionDiffData) -> Rule {
    let mut rule = Rule {
        raw_name: operands_data.raw_instruction_name.to_string(),
        new_general_register_values: Default::default(),
        new_flags_value: NewFlags {
            flag_cf: None,
            flag_pf: None,
            flag_af: None,
            flag_zf: None,
            flag_sf: None,
            flag_of: None,
        },
        memory_values_diff: MemoryValuesDiff {},
    };
    let flag_values = &mut rule.new_flags_value;
    for entry in expression_data.reg_state_entries {
        match entry.lhs.as_str() {
            "R2" => {
                let operand_type = operands_data.raw_operand_list.iter().find(|raw_operand| raw_operand.name.as_str() == "R2").unwrap();
                let op_idx = operand_type.op_idx;
                match operand_type.raw_operand_type.as_ref() {
                    Some(RawOperandType::R8) => {
                        rule.parameters.push(op_idx);
                        let typed_expr = expr_to_typed_expr(&entry.expr);
                        rule.new_general_register_values.insert(RegisterOrParameter64::Parameter(op_idx), typed_expr.unwrap_64());
                    }
                    Some(RawOperandType::Mem) => {
                        rule.parameters.push(op_idx);
                        todo!()
                    }
                    Some(RawOperandType::XMM) => {
                        todo!()
                    }
                    Some(RawOperandType::R64) => {
                        todo!()
                    }
                    None => todo!(),
                }
            }
            "CF" => {
                flag_values.flag_cf = Some(expr_to_typed_expr(&entry.expr).unwrap_1());
            }
            "PF" => {
                flag_values.flag_pf = Some(expr_to_typed_expr(&entry.expr).unwrap_1());
            }
            "AF" => {
                flag_values.flag_af = Some(expr_to_typed_expr(&entry.expr).unwrap_1());
            }
            "ZF" => {
                flag_values.flag_zf = Some(expr_to_typed_expr(&entry.expr).unwrap_1());
            }
            "SF" => {
                flag_values.flag_sf = Some(expr_to_typed_expr(&entry.expr).unwrap_1());
            }
            "OF" => {
                flag_values.flag_of = Some(expr_to_typed_expr(&entry.expr).unwrap_1());
            }
            lhs => todo!("{lhs}")
        };
    }
    rule
}
*/
