use std::fs::File;
use std::io::BufReader;
use crate::raw::{KExpression, KSentence, TopLevel};

pub mod raw;
pub mod untyped_semantics;


fn has_a_label_expr(expr: &KExpression, label: &str) -> bool{
    match expr {
        KExpression::KApply { label, args, .. } => {
            if label.as_str() == "execinstr" {
                return true
            }
            for arg in args {
                if has_a_label_expr(arg,label){
                    return true
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
        KExpression::KRewrite { lhs, rhs } => has_a_label_expr(lhs,label) || has_a_label_expr(rhs,label),
        KExpression::KSequence {  items, .. } => {
            for x in items {
                if has_a_label_expr(x,label){
                    return true
                }
            }
            false
        }
    }
}

fn has_execinstr_label(sentence: &KSentence, label: &str) -> bool{
    match sentence {
        KSentence::KProduction { .. } => {
            false
        }
        KSentence::KModuleComment { .. } => false,
        KSentence::KSyntaxSort { .. } => false,
        KSentence::KRule { body, requires, ensures, .. } => {
            has_a_label_expr(body, label) || has_a_label_expr(requires,label) || has_a_label_expr(ensures,label)
        }
        KSentence::KSyntaxAssociativity { .. } => false,
        KSentence::KSyntaxPriority { .. } => false,
        KSentence::KContext { body, requires, .. } => {
            has_a_label_expr(body,label) || has_a_label_expr(requires,label)
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

pub fn extract_apply_args(expr: &KExpression, expected_label: &str) -> Vec<KExpression> {
    match expr{
        KExpression::KApply { label, variable:_, arity:_, args } => {
            assert_eq!(label.as_str(), expected_label);
            args.clone()
        }
        _ => panic!()
    }
}

pub fn extract_expression_from_semantics(semantics: TopLevel){
    for module in semantics.term.modules {
        if module.name == "ADCB-R8-R8".to_string(){
            for local_sentence in module.localSentences{
                if let KSentence::KRule { body, requires, ensures, att:_ } = &local_sentence {
                    if has_execinstr_label(&local_sentence, "execinstr") {
                        assert_token_is_true(requires);
                        assert_token_is_true(ensures);
                        let mut apply_args = extract_apply_args(body, "#cells");
                        assert_eq!(apply_args.len(), 2);
                        let reg_state = apply_args.pop().unwrap();
                        let k = apply_args.pop().unwrap();
                        extract_apply_args(&k, "<k>");
                        extract_apply_args(&reg_state, "<regstate>");
                    }
                }
            }
        }
    }
}

#[test]
pub fn test_semantics() -> anyhow::Result<()> {
    let top_level : TopLevel = serde_json::from_reader(BufReader::new(File::open("data/formatted_parsed.json")?))?;
    extract_expression_from_semantics(top_level);
    panic!()
}