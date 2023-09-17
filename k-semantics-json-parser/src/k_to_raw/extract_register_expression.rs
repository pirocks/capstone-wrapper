use std::str::FromStr;
use wrapper_common::registers::Reg64WithRIP;

use crate::k_expressions::KExpression;
use crate::k_to_raw::{extract_apply_args, extract_apply_label, OperandNames};
use crate::raw::{OperandIdx, RawExpression, RawToken, SemanticCastKind};

#[derive(Debug)]
pub struct ExpressionDiffData {
    pub reg_state_entries: Vec<MapEntry>,
}

#[derive(Debug)]
pub enum Flag {
    CF,
    PF,
    ZF,
    SF,
    OF,
}

#[derive(Debug)]

pub enum MapEntryKind {
    Op(OperandIdx),
    Flag(Flag),
    Reg64(Reg64WithRIP),
}

#[derive(Debug)]
pub struct MapEntry {
    pub(crate) kind: MapEntryKind,
    pub(crate) expr: RawExpression,
}

pub fn extract_expression(expr: &KExpression, operands: &OperandNames) -> RawExpression {
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
            } else if label.as_str() == "getRegisterValue" {
                assert_eq!(args.len(), 2);
                return RawExpression::GetRegisterValue {
                    lookup: Box::new(extract_expression(&args[0], operands)),
                    map: Box::new(extract_expression(&args[1], operands)),
                };
            } else if label.as_str() == "Map:lookup" {
                assert_eq!(args.len(), 2);
                return RawExpression::MapLookup {
                    map: Box::new(extract_expression(&args[0], operands)),
                    lookup: Box::new(extract_expression(&args[1], operands)),
                };
            } else if label.as_str() == "#SemanticCastToR8" {
                assert_eq!(args.len(), 1);
                return RawExpression::SemanticCast {
                    kind: SemanticCastKind::R8,
                    inner: Box::new(extract_expression(&args[0], operands)),
                };
            } else if label.as_str() == "#SemanticCastToRh" {
                assert_eq!(args.len(), 1);
                return RawExpression::SemanticCast {
                    kind: SemanticCastKind::RH,
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
            } else if label.as_str() == "decRSPInBytes" {
                assert_eq!(args.len(), 1);
                return RawExpression::DecRSPInBytes {
                    inner: Box::new(extract_expression(&args[0], operands)),
                };
            } else if label.as_str() == "project:MInt" {
                assert_eq!(args.len(), 1);
                return RawExpression::ProjectMInt {
                    inner: Box::new(extract_expression(&args[0], operands)),
                };
            }else if label.as_str() == "negMInt" {
                assert_eq!(args.len(), 1);
                return RawExpression::Neg {
                    inner: Box::new(extract_expression(&args[0], operands)),
                };
            } else if label.as_str() == "addMInt" {
                return RawExpression::Add {
                    left: Box::new(extract_expression(&args[0], operands)),
                    right: Box::new(extract_expression(&args[1], operands)),
                };
            } else if label.as_str() == "andMInt" {
                return RawExpression::And {
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
            } else if label.as_str() == "subMInt" {
                assert_eq!(args.len(), 2);
                return RawExpression::SubMInt {
                    left: Box::new(extract_expression(&args[0], operands)),
                    right: Box::new(extract_expression(&args[1], operands)),
                };
            } else if label.as_str() == "_xorBool_" {
                assert_eq!(args.len(), 2);
                return RawExpression::XorBool {
                    left: Box::new(extract_expression(&args[0], operands)),
                    right: Box::new(extract_expression(&args[1], operands)),
                };
            } else if label.as_str() == "loadFromMemory" {
                assert_eq!(args.len(), 2);
                return RawExpression::LoadFromMemory {
                    offset: Box::new(extract_expression(&args[0], operands)),
                    size: Box::new(extract_expression(&args[1], operands)),
                };
            } else if label.as_str() == "storeToMemory" {
                assert_eq!(args.len(), 3);
                return RawExpression::StoreFromMemory {
                    value: Box::new(extract_expression(&args[0], operands)),
                    address: Box::new(extract_expression(&args[1], operands)),
                    size: Box::new(extract_expression(&args[2], operands)),
                };
            } else if label.as_str() == "#SemanticCastToMInt" {
                return RawExpression::SemanticCast {
                    kind: SemanticCastKind::MInt,
                    inner: Box::new(extract_expression(&args[0], operands)),
                };
            } else if label.as_str() == "#SemanticCastToXmm" {
                return RawExpression::SemanticCast {
                    kind: SemanticCastKind::Xmm,
                    inner: Box::new(extract_expression(&args[0], operands)),
                };
            } else if label.as_str() == "#SemanticCastToR64" {
                return RawExpression::SemanticCast {
                    kind: SemanticCastKind::R64,
                    inner: Box::new(extract_expression(&args[0], operands)),
                };
            } else if label.as_str() == "%rsp_X86-SYNTAX" {
                return RawExpression::Token(RawToken::RSP);
            } else if label.as_str() == "undefMInt_MINT-WRAPPER-SYNTAX" {
                return RawExpression::Undefined;
            } else if label.as_str() == "_(_,_,_)_MINT-WRAPPER-SYNTAX" {
                let token = match &args[0] {
                    KExpression::KToken { sort, token } => {
                        assert_eq!(sort.as_str(), "UIFTerOperation");
                        token.to_string()
                    }
                    _ => panic!()
                };
                let mut res_args = vec![];
                for arg in &args[1..] {
                    res_args.push(extract_expression(arg, operands));
                }
                return RawExpression::FunctionCall {
                    token,
                    args: res_args,
                };
            } else {
                dbg!(args);
                todo!("{}", label);
            }
        }
        KExpression::KVariable { name, originalName: _ } => {
            if name.as_str() == "RSMap" {
                return RawExpression::RSMap;
            }
            return RawExpression::Op(operands.name_lookup(name));
        }
        KExpression::KToken { sort, token } => {
            if sort.as_str() == "Int" {
                return RawExpression::ConstantInt(i128::from_str(token).unwrap());
            } else if sort.as_str() == "String" {
                return RawExpression::Token(match token.as_str() {
                    "\"CF\"" => RawToken::CF,
                    "\"RIP\"" => RawToken::RIP,
                    token => todo!("{token}")
                });
            }
            todo!("{sort}")
        }
        _ => panic!()
    }
}

fn handle_map_entry_kind(str: impl Into<String>, operands: &OperandNames) -> MapEntryKind {
    let str = str.into();
    match operands.try_name_lookup(&str) {
        None => {
            match str.as_str() {
                "RIP" => MapEntryKind::Reg64(Reg64WithRIP::RIP),
                "CF" => MapEntryKind::Flag(Flag::CF),
                "PF" => MapEntryKind::Flag(Flag::PF),
                "AF" => MapEntryKind::Flag(Flag::PF),
                "ZF" => MapEntryKind::Flag(Flag::ZF),
                "SF" => MapEntryKind::Flag(Flag::SF),
                "OF" => MapEntryKind::Flag(Flag::OF),
                other => todo!("{other}")
            }
        }
        Some(op_idx) => {
            MapEntryKind::Op(op_idx)
        }
    }
}

fn handle_map_entry(expr: &KExpression, operands: &OperandNames) -> MapEntry {
    let args = extract_apply_args(expr, "_|->_");
    let lhs = &args[0];
    let rhs = &args[1];
    let kind = match lhs {
        KExpression::KToken { token, .. } => {
            let token_string = token.as_str().strip_prefix("\"").unwrap().strip_suffix("\"").unwrap().to_string();
            handle_map_entry_kind(token_string, operands)
        }
        KExpression::KApply { label, args, .. } => {
            assert_eq!(label.as_str(), "convToRegKeys");
            let label = extract_apply_label(&args[0]);
            if let KExpression::KVariable { name, .. } = &extract_apply_args(&args[0], label)[0] {
                handle_map_entry_kind(name, operands)
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
    MapEntry { kind, expr }
}

pub fn recursively_extract_map_entries(expr: &KExpression, operands: &OperandNames, entries: &mut Vec<MapEntry>) {
    let first_label = extract_apply_label(expr);
    if first_label == "_|->_" {
        entries.push(handle_map_entry(expr, operands));
        return;
    }
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

pub fn extract_diff_expression_from_semantics(semantic_rule_decl: &[KExpression], operands: &OperandNames) -> ExpressionDiffData {
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
        KExpression::KApply { label, args, .. } => {
            if label.as_str() == "#SemanticCastToMap" {
                if args.as_slice() == &[KExpression::KVariable { name: "RSMap".to_string(), originalName: "RSMap".to_string() }]{
                    return ExpressionDiffData{ reg_state_entries: vec![] }
                }
            }
            todo!()
        }
        other => todo!("{other:?}")
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
