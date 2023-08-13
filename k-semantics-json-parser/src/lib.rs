use std::fs::File;
use std::io::BufReader;
use std::ops::Deref;
use crate::raw::{KExpression, KSentence, TopLevel};
use crate::untyped_semantics::{Expression, Rule};

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
}

pub fn recursive_operand_extract(operand_list: &KExpression, current_type: Option<RawOperandType>, raw_operands: &mut Vec<RawOperand>) {
    match operand_list {
        KExpression::KApply { label, variable, arity, args } => {
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
        KExpression::KVariable { name, originalName } => {
            raw_operands.push(RawOperand { raw_operand_type: current_type, name: name.to_string() })
        }
        _ => panic!()
    }
}

pub fn extract_operand_data_from_semantics(semantic_rule_decl: &[KExpression]) -> RuleOperandsData {
    let no_dots = &semantic_rule_decl[0];
    let meat = &semantic_rule_decl[1];
    let dots = &semantic_rule_decl[2];
    assert_eq!(no_dots, &empty_kapply("#noDots"));
    assert_eq!(dots, &empty_kapply("#dots"));
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
            };
        } else {
            panic!()
        }
    } else {
        panic!()
    }
    ;
    todo!()
}

pub struct ExpressionDiffData {}

pub fn extract_diff_expression_from_semantics(semantic_rule_decl: &[KExpression], operands: &RuleOperandsData) -> ExpressionDiffData {
    eprintln!("{}", serde_json::to_string_pretty(semantic_rule_decl).unwrap());
    todo!()
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

pub fn build_rule(operands_data: RuleOperandsData, expression_data: ExpressionDiffData) -> Rule {
    todo!()
}

pub fn extract_rule_from_semantics(semantics: TopLevel) -> Rule {
    for module in semantics.term.modules {
        if module.name == "ADCB-R8-R8".to_string() {
            for local_sentence in module.localSentences {
                if let KSentence::KRule { body, requires, ensures, att: _ } = &local_sentence {
                    if has_execinstr_label(&local_sentence, "execinstr") {
                        assert_token_is_true(requires);
                        assert_token_is_true(ensures);
                        let mut apply_args = extract_apply_args(body, "#cells");
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
    extract_rule_from_semantics(top_level);
    panic!()
}