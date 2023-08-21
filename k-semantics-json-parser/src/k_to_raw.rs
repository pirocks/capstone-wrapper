use std::ops::Deref;
use std::str::FromStr;
use crate::k_expressions::{KExpression, KSentence};
use crate::raw::{OperandIdx, RawExpression, RawToken, SemanticCastKind};

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

pub fn has_execinstr_label(sentence: &KSentence, label: &str) -> bool {
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
    pub(crate) raw_instruction_name: String,
    pub raw_operand_list: Vec<RawOperand>,
}

#[derive(Debug)]
pub enum RawOperandType {
    R8
}

#[derive(Debug)]
pub struct RawOperand {
    pub(crate) raw_operand_type: Option<RawOperandType>,
    pub(crate) name: String,
    pub(crate) op_idx: OperandIdx,
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
