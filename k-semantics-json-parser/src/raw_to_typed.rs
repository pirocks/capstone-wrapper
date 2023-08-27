use crate::k_to_raw::{RawOperandType, RuleOperandsData};
use crate::k_to_raw::extract_register_expression::ExpressionDiffData;
use crate::raw::{RawExpression, RawToken, SemanticCastKind};
use crate::typed_semantics::{MemoryValuesDiff, NewFlags, RegisterOrParameter64, Rule, TypedExpression, TypedExpression1, TypedExpression56, TypedExpression64, TypedExpression8, TypedExpression9};

pub fn expr_to_typed_expr(expr: &RawExpression) -> TypedExpression {
    match expr {
        RawExpression::Op(_) => {
            todo!()
        }
        RawExpression::IfElse { condition, true_case, false_case } => {
            let condition = expr_to_typed_expr(condition.as_ref()).unwrap_1();
            let true_case = expr_to_typed_expr(true_case.as_ref());
            let false_case = expr_to_typed_expr(false_case.as_ref());
            match true_case {
                TypedExpression::_64(_) => {
                    todo!()
                }
                TypedExpression::_56(_) => {
                    todo!()
                }
                TypedExpression::_9(true_case) => {
                    let false_case = false_case.unwrap_9();
                    TypedExpression::_9(TypedExpression9::IfThenElse {
                        condition,
                        true_case: Box::new(true_case),
                        false_case: Box::new(false_case),
                    })
                }
                TypedExpression::_8(_) => {
                    todo!()
                }
                TypedExpression::_1(true_case) => {
                    let false_case = false_case.unwrap_1();
                    TypedExpression::_1(TypedExpression1::IfThenElse {
                        condition: Box::new(condition),
                        true_case: Box::new(true_case),
                        false_case: Box::new(false_case),
                    })
                }
            }
        }
        RawExpression::AndBool { left, right } => {
            let left = expr_to_typed_expr(left.as_ref());
            let right = expr_to_typed_expr(right.as_ref());
            match left {
                TypedExpression::_64(_) => todo!(),
                TypedExpression::_56(_) => todo!(),
                TypedExpression::_9(_) => todo!(),
                TypedExpression::_8(_) => todo!(),
                TypedExpression::_1(left) => {
                    let right = right.unwrap_1();
                    TypedExpression::_1(TypedExpression1::AndBool {
                        left: Box::new(left),
                        right: Box::new(right),
                    })
                }
            }
        }
        RawExpression::EqualsBool { left, right } => {
            let left = expr_to_typed_expr(left.as_ref());
            let right = expr_to_typed_expr(right.as_ref());
            match left {
                TypedExpression::_64(_) => todo!(),
                TypedExpression::_56(_) => todo!(),
                TypedExpression::_9(_) => todo!(),
                TypedExpression::_8(_) => todo!(),
                TypedExpression::_1(left) => {
                    let right = right.unwrap_1();
                    TypedExpression::_1(TypedExpression1::Equals1 { left: Box::new(left), right: Box::new(right) })
                }
            }
        }
        RawExpression::Equals { left, right } => {
            let left = expr_to_typed_expr(left.as_ref());
            let right = expr_to_typed_expr(right.as_ref());
            match left {
                TypedExpression::_64(_) => todo!(),
                TypedExpression::_56(_) => todo!(),
                TypedExpression::_9(_) => todo!(),
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
            todo!("{len}")
        }
        RawExpression::Extract { from, range_start, range_end } => {
            let from = expr_to_typed_expr(from.as_ref());
            let range_start = extract_num(range_start);
            let range_end = extract_num(range_end);
            if (range_end - range_start) == 56 {
                TypedExpression::_56(TypedExpression56::Extract64 { source: from.unwrap_64(), base: range_start as usize })
            } else if (range_end - range_start) == 8 {
                match from {
                    TypedExpression::_64(inner) => {
                        TypedExpression::_8(TypedExpression8::Extract64 { source: inner, base: range_start as usize })
                    }
                    TypedExpression::_56(_) => todo!(),
                    TypedExpression::_9(inner) => {
                        TypedExpression::_8(TypedExpression8::Extract9 { source: inner, base: range_start as usize })
                    }
                    TypedExpression::_8(_) => todo!(),
                    TypedExpression::_1(_) => todo!(),
                }
            } else if (range_end - range_start) == 1 {
                match from {
                    TypedExpression::_64(inner) => {
                        TypedExpression::_1(TypedExpression1::Extract64 { source: Box::new(inner), base: range_start as usize })
                    }
                    TypedExpression::_56(_) => todo!(),
                    TypedExpression::_9(inner) => {
                        TypedExpression::_1(TypedExpression1::Extract9 { source: Box::new(inner), base: range_start as usize })
                    }
                    TypedExpression::_8(_) => todo!(),
                    TypedExpression::_1(_) => todo!(),
                }
            } else {
                dbg!(range_end - range_start);
                todo!()
            }
        }
        RawExpression::Concatenate { left, right } => {
            let left = expr_to_typed_expr(left.as_ref());
            let right = expr_to_typed_expr(right.as_ref());
            match left {
                TypedExpression::_64(_) => todo!(),
                TypedExpression::_56(_56) => {
                    match right {
                        TypedExpression::_64(_) => todo!(),
                        TypedExpression::_56(_) => todo!(),
                        TypedExpression::_9(_) => todo!(),
                        TypedExpression::_8(_8) => {
                            TypedExpression::_64(TypedExpression64::Concatenate568 { left: Box::new(_56), right: Box::new(_8) })
                        }
                        TypedExpression::_1(_) => todo!(),
                    }
                }
                TypedExpression::_9(_) => todo!(),
                TypedExpression::_8(_) => todo!(),
                TypedExpression::_1(_1) => {
                    match right {
                        TypedExpression::_64(_) => todo!(),
                        TypedExpression::_56(_) => todo!(),
                        TypedExpression::_9(_) => todo!(),
                        TypedExpression::_8(_8) => {
                            TypedExpression::_9(TypedExpression9::Concatenate18 { left: _1, right: Box::new(_8) })
                        }
                        TypedExpression::_1(_) => todo!(),
                    }
                }
            }
        }
        RawExpression::GetParentValue { lookup, map } => {
            handle_get_parent_value(lookup, map)
        }
        RawExpression::GetFlag { lookup, map } => {
            match (lookup.as_ref(), map.as_ref()) {
                (RawExpression::Token(RawToken::CF), RawExpression::SemanticCast { kind, inner }) => {
                    if let (SemanticCastKind::Map, RawExpression::RSMap) = (kind, inner.as_ref()) {
                        return TypedExpression::_1(TypedExpression1::FlagCF);
                    }
                }
                _ => todo!()
            }

            todo!()
        }
        RawExpression::SemanticCast { .. } => {
            todo!()
        }
        RawExpression::ConstantInt(_) => {
            todo!()
        }
        RawExpression::RSMap => {
            todo!()
        }
        RawExpression::NotBool { inner } => {
            let inner = expr_to_typed_expr(inner);
            TypedExpression::_1(TypedExpression1::Not(Box::new(inner.unwrap_1())))
        }
        RawExpression::Add { left, right } => {
            let left = expr_to_typed_expr(left.as_ref());
            let right = expr_to_typed_expr(right.as_ref());
            match left {
                TypedExpression::_64(_) => todo!(),
                TypedExpression::_56(_) => todo!(),
                TypedExpression::_9(left) => {
                    match right {
                        TypedExpression::_64(_) => todo!(),
                        TypedExpression::_56(_) => todo!(),
                        TypedExpression::_9(right) => {
                            TypedExpression::_9(TypedExpression9::Equals { left: Box::new(left), right: Box::new(right) })
                        }
                        TypedExpression::_8(_) => todo!(),
                        TypedExpression::_1(_) => todo!(),
                    }
                }
                TypedExpression::_8(_) => todo!(),
                TypedExpression::_1(_) => todo!(),
            }
        }
        RawExpression::Xor { left, right } => {
            let left = expr_to_typed_expr(left.as_ref());
            let right = expr_to_typed_expr(right.as_ref());
            match left {
                TypedExpression::_64(_) => todo!(),
                TypedExpression::_56(_) => todo!(),
                TypedExpression::_9(_) => todo!(),
                TypedExpression::_8(_) => todo!(),
                TypedExpression::_1(left) => match right {
                    TypedExpression::_64(_) => todo!(),
                    TypedExpression::_56(_) => todo!(),
                    TypedExpression::_9(_) => todo!(),
                    TypedExpression::_8(_) => todo!(),
                    TypedExpression::_1(right) => {
                        TypedExpression::_1(TypedExpression1::Xor { left: Box::new(left), right: Box::new(right) })
                    }
                }
            }
        }
        RawExpression::XorBool { left, right } => {
            let left = expr_to_typed_expr(left);
            let right = expr_to_typed_expr(right);
            match left {
                TypedExpression::_64(_) => todo!(),
                TypedExpression::_56(_) => todo!(),
                TypedExpression::_9(_) => todo!(),
                TypedExpression::_8(_) => todo!(),
                TypedExpression::_1(left) => {
                    match right {
                        TypedExpression::_64(_) => todo!(),
                        TypedExpression::_56(_) => todo!(),
                        TypedExpression::_9(_) => todo!(),
                        TypedExpression::_8(_) => todo!(),
                        TypedExpression::_1(right) => {
                            TypedExpression::_1(TypedExpression1::XorBool {
                                left: Box::new(left),
                                right: Box::new(right),
                            })
                        }
                    }
                }
            }
        }
        RawExpression::Token(_) => {
            todo!()
        }
    }
}

fn extract_num(num: &Box<RawExpression>) -> i128 {
    match num.as_ref() {
        RawExpression::ConstantInt(num) => *num,
        _ => panic!()
    }
}

fn handle_get_parent_value(lookup: &Box<RawExpression>, map: &Box<RawExpression>) -> TypedExpression {
    match (lookup.as_ref(), map.as_ref()) {
        (RawExpression::SemanticCast { kind: lookup_kind, inner: lookup_inner },
            RawExpression::SemanticCast { kind: map_kind, inner: map_inner }) => {
            if let (SemanticCastKind::Map, RawExpression::RSMap) = (map_kind, map_inner.as_ref()) {
                if let SemanticCastKind::R8 = lookup_kind {
                    if let RawExpression::Op(operand_idx) = lookup_inner.as_ref() {
                        return TypedExpression::_64(TypedExpression64::OperandR8 { operand_idx: *operand_idx });
                    }
                }
            } else {
                todo!()
            }
        }
        _ => todo!()
    }
    todo!()
}

pub fn build_rule(operands_data: RuleOperandsData, expression_data: ExpressionDiffData) -> Rule {
    let mut rule = Rule {
        raw_name: operands_data.raw_instruction_name.to_string(),
        parameters: vec![],
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
                    None => todo!()
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
