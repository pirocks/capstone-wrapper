use std::str::FromStr;
use crate::k_expressions::KExpression;
use crate::k_to_raw::{extract_apply_args, extract_apply_label, RuleOperandsData};
use crate::raw::{RawExpression, RawToken, SemanticCastKind};

pub struct ExpressionDiffData {
    pub reg_state_entries: Vec<MapEntry>,
}

pub struct MapEntry {
    pub(crate) lhs: String,
    pub(crate) expr: RawExpression,
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
        KExpression::KVariable { name, originalName: _ } => {
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
        KExpression::KToken { token, .. } => {
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

pub(crate) fn empty_ksequence() -> KExpression {
    KExpression::KSequence { arity: 0, items: vec![] }
}

pub(crate) fn empty_kapply(label: impl Into<String>) -> KExpression {
    KExpression::KApply {
        label: label.into(),
        variable: false,
        arity: 0,
        args: vec![],
    }
}