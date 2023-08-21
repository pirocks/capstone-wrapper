use std::fs::File;
use std::io::BufReader;
use std::ops::Deref;
use std::str::FromStr;

use crate::raw::{KExpression, KSentence, TopLevel};
use crate::untyped_semantics::{MemoryValuesDiff, NewFlags, OperandIdx, RawExpression, RawToken, RegisterOrParameter64, Rule, SemanticCastKind, TypedExpression, TypedExpression1, TypedExpression56, TypedExpression64, TypedExpression8, TypedExpression9};

pub mod raw;
pub mod untyped_semantics;


fn has_a_label_expr(expr: &KExpression, label: &str) -> bool {
    match expr {
        KExpression::KApply { label, args, .. } => {
            if label.as_str() == "execinstr" {
                return true;
            }
            for arg in args {
                if has_a_label_expr(arg, label) {
                    return true;
                }
            }
            false
        }
        KExpression::KVariable { .. } => {
            false
        }
        KExpression::KToken { .. } => {
            false
        }
        KExpression::KRewrite { lhs, rhs } => has_a_label_expr(lhs, label) || has_a_label_expr(rhs, label),
        KExpression::KSequence { items, .. } => {
            for x in items {
                if has_a_label_expr(x, label) {
                    return true;
                }
            }
            false
        }
    }
}

fn has_execinstr_label(sentence: &KSentence, label: &str) -> bool {
    match sentence {
        KSentence::KProduction { .. } => {
            false
        }
        KSentence::KModuleComment { .. } => false,
        KSentence::KSyntaxSort { .. } => false,
        KSentence::KRule { body, requires, ensures, .. } => {
            has_a_label_expr(body, label) || has_a_label_expr(requires, label) || has_a_label_expr(ensures, label)
        }
        KSentence::KSyntaxAssociativity { .. } => false,
        KSentence::KSyntaxPriority { .. } => false,
        KSentence::KContext { body, requires, .. } => {
            has_a_label_expr(body, label) || has_a_label_expr(requires, label)
        }
        KSentence::KBubble { .. } => false
    }
}


pub fn assert_token_is_true(expr: &KExpression) {
    match expr {
        KExpression::KToken { sort, token } => {
            assert_eq!(sort.as_str(), "Bool");
            assert_eq!(token.as_str(), "true");
        }
        KExpression::KApply { .. } |
        KExpression::KVariable { .. } |
        KExpression::KRewrite { .. } |
        KExpression::KSequence { .. } => panic!()
    }
}

pub fn extract_apply_args<'l>(expr: &'l KExpression, expected_label: &str) -> &'l [KExpression] {
    match expr {
        KExpression::KApply { label, variable: _, arity: _, args } => {
            assert_eq!(label.as_str(), expected_label);
            args
        }
        _ => panic!()
    }
}

pub fn extract_apply_label(expr: &KExpression) -> &str {
    match expr {
        KExpression::KApply { label, .. } => {
            label.as_str()
        }
        _ => panic!()
    }
}

#[derive(Debug)]
pub struct RuleOperandsData {
    raw_instruction_name: String,
    pub raw_operand_list: Vec<RawOperand>,
}

#[derive(Debug)]
pub enum RawOperandType {
    R8
}

#[derive(Debug)]
pub struct RawOperand {
    raw_operand_type: Option<RawOperandType>,
    name: String,
    op_idx: OperandIdx,
}

pub fn recursive_operand_extract(operand_list: &KExpression, current_type: Option<RawOperandType>, raw_operands: &mut Vec<RawOperand>) {
    match operand_list {
        KExpression::KApply { label, args, .. } => {
            if label.as_str() == "#SemanticCastToR8" {
                assert_eq!(args.len(), 1);
                return recursive_operand_extract(&args[0], Some(RawOperandType::R8), raw_operands);
            } else if label.as_str() == "operandlist" {
                for arg in args {
                    assert!(current_type.is_none());
                    recursive_operand_extract(arg, None, raw_operands);
                }
                return;
            } else if label.as_str() == ".List{\"operandlist\"}" {
                return;
            }
            todo!()
        }
        KExpression::KVariable { name, .. } => {
            raw_operands.push(RawOperand { raw_operand_type: current_type, name: name.to_string(), op_idx: OperandIdx(raw_operands.len() as u8) })
        }
        _ => panic!()
    }
}

pub fn extract_operand_data_from_semantics(semantic_rule_decl: &[KExpression]) -> RuleOperandsData {
    let meat = &semantic_rule_decl[1];
    assert_eq!(&semantic_rule_decl[0], &empty_kapply("#noDots"));
    assert_eq!(&semantic_rule_decl[2], &empty_kapply("#dots"));
    if let KExpression::KRewrite { lhs, rhs } = meat {
        assert_eq!(rhs.deref(), &empty_ksequence());
        if let KExpression::KApply { label, args, .. } = lhs.as_ref() {
            assert_eq!(label.as_str(), "execinstr");
            assert_eq!(args.len(), 1);
            let args = extract_apply_args(&args[0], "___X86-SYNTAX");
            assert_eq!(args.len(), 2);
            let instruction_name_kapply = args.first().unwrap();
            let raw_instruction_name = extract_apply_label(instruction_name_kapply);
            let operand_list = args.last().unwrap();
            let mut raw_operand_list = vec![];
            recursive_operand_extract(operand_list, None, &mut raw_operand_list);
            return RuleOperandsData {
                raw_instruction_name: raw_instruction_name.to_string(),
                raw_operand_list,
            }
        }
    }
    todo!()
}

pub struct ExpressionDiffData {
    pub reg_state_entries: Vec<MapEntry>,
}

pub struct MapEntry {
    lhs: String,
    expr: RawExpression,
}

fn extract_expression(expr: &KExpression, operands: &RuleOperandsData) -> RawExpression {
    match expr {
        KExpression::KApply { label, args, .. } => {
            if label.as_str() == "#ifMInt_#then_#else_#fi_MINT-WRAPPER-SYNTAX" {
                assert_eq!(args.len(), 3);
                return RawExpression::IfElse {
                    condition: Box::new(extract_expression(&args[0], operands)),
                    true_case: Box::new(extract_expression(&args[1], operands)),
                    false_case: Box::new(extract_expression(&args[2], operands)),
                };
            } else if label.as_str() == "_andBool_" {
                assert_eq!(args.len(), 2);
                return RawExpression::AndBool {
                    left: Box::new(extract_expression(&args[0], operands)),
                    right: Box::new(extract_expression(&args[1], operands)),
                };
            } else if label.as_str() == "_==Bool_" {
                assert_eq!(args.len(), 2);
                return RawExpression::EqualsBool {
                    left: Box::new(extract_expression(&args[0], operands)),
                    right: Box::new(extract_expression(&args[1], operands)),
                };
            } else if label.as_str() == "eqMInt" {
                assert_eq!(args.len(), 2);
                return RawExpression::Equals {
                    left: Box::new(extract_expression(&args[0], operands)),
                    right: Box::new(extract_expression(&args[1], operands)),
                };
            } else if label.as_str() == "extractMInt" {
                assert_eq!(args.len(), 3);
                return RawExpression::Extract {
                    from: Box::new(extract_expression(&args[0], operands)),
                    range_start: Box::new(extract_expression(&args[1], operands)),
                    range_end: Box::new(extract_expression(&args[2], operands)),
                };
            } else if label.as_str() == "getParentValue" {
                assert_eq!(args.len(), 2);
                return RawExpression::GetParentValue {
                    lookup: Box::new(extract_expression(&args[0], operands)),
                    map: Box::new(extract_expression(&args[1], operands)),
                };
            } else if label.as_str() == "#SemanticCastToR8" {
                assert_eq!(args.len(), 1);
                return RawExpression::SemanticCast {
                    kind: SemanticCastKind::R8,
                    inner: Box::new(extract_expression(&args[0], operands)),
                };
            } else if label.as_str() == "#SemanticCastToMap" {
                assert_eq!(args.len(), 1);
                return RawExpression::SemanticCast {
                    kind: SemanticCastKind::Map,
                    inner: Box::new(extract_expression(&args[0], operands)),
                };
            } else if label.as_str() == "mi" {
                assert_eq!(args.len(), 2);
                return RawExpression::MI {
                    len: Box::new(extract_expression(&args[0], operands)),
                    val: Box::new(extract_expression(&args[1], operands)),
                };
            } else if label.as_str() == "notBool_" {
                assert_eq!(args.len(), 1);
                return RawExpression::NotBool {
                    inner: Box::new(extract_expression(&args[0], operands)),
                };
            } else if label.as_str() == "addMInt" {
                return RawExpression::Add {
                    left: Box::new(extract_expression(&args[0], operands)),
                    right: Box::new(extract_expression(&args[1], operands)),
                };
            } else if label.as_str() == "getFlag" {
                assert_eq!(args.len(), 2);
                return RawExpression::GetFlag {
                    lookup: Box::new(extract_expression(&args[0], operands)),
                    map: Box::new(extract_expression(&args[1], operands)),
                };
            } else if label.as_str() == "concatenateMInt" {
                assert_eq!(args.len(), 2);
                return RawExpression::Concatenate {
                    left: Box::new(extract_expression(&args[0], operands)),
                    right: Box::new(extract_expression(&args[1], operands)),
                };
            } else if label.as_str() == "xorMInt" {
                assert_eq!(args.len(), 2);
                return RawExpression::Xor {
                    left: Box::new(extract_expression(&args[0], operands)),
                    right: Box::new(extract_expression(&args[1], operands)),
                };
            } else if label.as_str() == "_xorBool_" {
                assert_eq!(args.len(), 2);
                return RawExpression::XorBool {
                    left: Box::new(extract_expression(&args[0], operands)),
                    right: Box::new(extract_expression(&args[1], operands)),
                };
            } else {
                todo!("{}", label);
            }
        }
        KExpression::KVariable { name, originalName:_ } => {
            if name.as_str() == "RSMap" {
                return RawExpression::RSMap;
            }
            return RawExpression::Op(operands.raw_operand_list.iter().find(|operand| &operand.name == name).unwrap().op_idx);
        }
        KExpression::KToken { sort, token } => {
            if sort.as_str() == "Int" {
                return RawExpression::ConstantInt(i128::from_str(token).unwrap());
            } else if sort.as_str() == "String" {
                return RawExpression::Token(match token.as_str() {
                    "\"CF\"" => RawToken::CF,
                    token => todo!("{token}")
                });
            }
            todo!("{sort}")
        }
        _ => panic!()
    }
}

fn handle_map_entry(expr: &KExpression, operands: &RuleOperandsData) -> MapEntry {
    let args = extract_apply_args(expr, "_|->_");
    let lhs = &args[0];
    let rhs = &args[1];
    let lhs = match lhs {
        KExpression::KToken {  token, ..  } => {
            token.as_str().strip_prefix("\"").unwrap().strip_suffix("\"").unwrap().to_string()
        }
        KExpression::KApply { label, args, .. } => {
            assert_eq!(label.as_str(), "convToRegKeys");
            if let KExpression::KVariable { name, .. } = &extract_apply_args(&args[0], "#SemanticCastToR8")[0] {
                name.to_string()
            } else {
                panic!()
            }
        }
        _ => {
            eprintln!("{}", serde_json::to_string_pretty(lhs).unwrap());
            panic!()
        }
    };
    let expr = extract_expression(rhs, operands);
    MapEntry { lhs, expr }
}

pub fn recursively_extract_map_entries(expr: &KExpression, operands: &RuleOperandsData, entries: &mut Vec<MapEntry>) {
    let args = extract_apply_args(expr, "_Map_");
    if args.len() == 1 {
        todo!()
    } else if args.len() == 2 {
        let first = &args[0];
        let second = &args[1];
        let first_label = extract_apply_label(first);
        if first_label == "_|->_" {
            entries.push(handle_map_entry(first, operands));
        } else {
            recursively_extract_map_entries(first, operands, entries);
        }
        entries.push(handle_map_entry(second, operands));
    }
}

pub fn extract_diff_expression_from_semantics(semantic_rule_decl: &[KExpression], operands: &RuleOperandsData) -> ExpressionDiffData {
    assert_eq!(&semantic_rule_decl[0], &empty_kapply("#noDots"));
    assert_eq!(&semantic_rule_decl[2], &empty_kapply("#noDots"));
    assert_eq!(semantic_rule_decl.len(), 3);
    let meat = &semantic_rule_decl[1];
    match meat {
        KExpression::KRewrite { lhs, rhs } => {
            //lhs maps result to map
            let semantic_cast_to_map = KExpression::KApply {
                label: "#SemanticCastToMap".to_string(),
                variable: false,
                arity: 1,
                args: vec![KExpression::KVariable { name: "RSMap".to_string(), originalName: "RSMap".to_string() }],
            };
            assert_eq!(lhs.as_ref(), &semantic_cast_to_map);
            //rhs calls updateMap
            let update_map_args = extract_apply_args(rhs.as_ref(), "updateMap");
            assert_eq!(lhs.as_ref(), &update_map_args[0]);
            let update_map_body = &update_map_args[1];
            let mut reg_state_entries = vec![];
            recursively_extract_map_entries(update_map_body, operands, &mut reg_state_entries);
            ExpressionDiffData {
                reg_state_entries
            }
        }
        _ => panic!()
    }
}

fn empty_ksequence() -> KExpression {
    KExpression::KSequence { arity: 0, items: vec![] }
}

fn empty_kapply(label: impl Into<String>) -> KExpression {
    KExpression::KApply {
        label: label.into(),
        variable: false,
        arity: 0,
        args: vec![],
    }
}

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
                        rule.new_general_register_values.insert(RegisterOrParameter64::Parameter(op_idx),typed_expr.unwrap_64());
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

pub fn extract_rule_from_semantics(semantics: TopLevel) -> Rule {
    for module in semantics.term.modules {
        if module.name == "ADCB-R8-R8".to_string() {
            for local_sentence in module.localSentences {
                if let KSentence::KRule { body, requires, ensures, att: _ } = &local_sentence {
                    if has_execinstr_label(&local_sentence, "execinstr") {
                        assert_token_is_true(requires);
                        assert_token_is_true(ensures);
                        let apply_args = extract_apply_args(body, "#cells");
                        assert_eq!(apply_args.len(), 2);
                        let k = apply_args.first().unwrap();
                        let reg_state = apply_args.last().unwrap();
                        let extracted_operands = extract_apply_args(&k, "<k>");
                        let rule_operands_data = extract_operand_data_from_semantics(extracted_operands);
                        let extracted_diff = extract_apply_args(&reg_state, "<regstate>");
                        let diff_data = extract_diff_expression_from_semantics(extracted_diff, &rule_operands_data);
                        return build_rule(rule_operands_data, diff_data);
                    }
                }
            }
        }
    }
    panic!()
}

#[test]
pub fn test_semantics() -> anyhow::Result<()> {
    let top_level: TopLevel = serde_json::from_reader(BufReader::new(File::open("data/formatted_parsed.json")?))?;
    let _res = extract_rule_from_semantics(top_level);
    Ok(())
}