use std::fs::File;
use std::io::BufReader;
use crate::raw::{KExpression, KSentence, TopLevel};

#[test]
pub fn parse_output() -> anyhow::Result<()>{
    let _top_level : TopLevel = serde_json::from_reader(BufReader::new(File::open("/home/francis/x86-semantics-workspace/X86-64-semantics/formatted.json")?))?;
    Ok(())
}

fn has_execinstr_label_expr(expr: &KExpression) -> bool{
    match expr {
        KExpression::KApply { label, args, .. } => {
            if label.as_str() == "execinstr" {
                return true
            }
            for arg in args {
                if has_execinstr_label_expr(arg){
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
        KExpression::KRewrite { lhs, rhs } => has_execinstr_label_expr(lhs) || has_execinstr_label_expr(rhs),
        KExpression::KSequence {  items, .. } => {
            for x in items {
                if has_execinstr_label_expr(x){
                    return true
                }
            }
            false
        }
    }
}

fn has_execinstr_label(sentence: &KSentence) -> bool{
    match sentence {
        KSentence::KProduction { .. } => {
            false
        }
        KSentence::KModuleComment { .. } => false,
        KSentence::KSyntaxSort { .. } => false,
        KSentence::KRule { body, requires, ensures, .. } => {
            has_execinstr_label_expr(body) || has_execinstr_label_expr(requires) || has_execinstr_label_expr(ensures)
        }
        KSentence::KSyntaxAssociativity { .. } => false,
        KSentence::KSyntaxPriority { .. } => false,
        KSentence::KContext { body, requires, .. } => {
            has_execinstr_label_expr(body) || has_execinstr_label_expr(requires)
        }
        KSentence::KBubble { .. } => false
    }
}

#[test]
pub fn find_adcb() -> anyhow::Result<()>{
    let top_level : TopLevel = serde_json::from_reader(BufReader::new(File::open("/home/francis/x86-semantics-workspace/X86-64-semantics/formatted_parsed.json")?))?;
    for m in top_level.term.modules {
        if m.name == "ADCB-R8-R8".to_string(){
            for sentence in &m.localSentences {
                if has_execinstr_label(sentence){
                    println!("{}",serde_json::to_string_pretty(&sentence)?);
                }
            }
        }
    }
    panic!()
}