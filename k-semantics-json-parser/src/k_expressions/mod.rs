#![allow(non_snake_case)]


use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct TopLevel {
    pub version: usize,
    pub term: KDefinition,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct KDefinition {
    pub mainModule: String,
    pub modules: Vec<KFlatModule>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct KFlatModule {
    pub name: String,
    pub imports: Vec<String>,
    pub localSentences: Vec<KSentence>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "node")]
pub enum KSort {
    KSort {
        name: String
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "node")]
pub enum KProductionItems {
    KTerminal {
        value: String
    },
    KNonTerminal {
        // value: Option<String>,
        sort: KSort
    },
    KRegexTerminal {
        precedeRegex: String,
        followRegex: String,
        regex: String,
    },
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
#[serde(tag = "node")]
pub enum KExpression {
    KApply {
        label: String,
        variable: bool,
        arity: usize,
        args: Vec<KExpression>,
    },
    KVariable {
        name: String,
        originalName: String,
    },
    KToken {
        sort: String,
        token: String,
    },
    KRewrite {
        lhs: Box<KExpression>,
        rhs: Box<KExpression>,
    },
    KSequence{
        arity: usize,
        items: Vec<KExpression>
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Att {
    pub function: Option<String>,
    pub predicate: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "node")]
#[serde(deny_unknown_fields)]
pub enum KAtt {
    KAtt {
        att: HashMap<String, String>
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Assoc{
    Left,
    Right,
    NonAssoc
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "node")]
#[serde(deny_unknown_fields)]
pub enum KSentence {
    KProduction {
        params: Vec<KSort>,
        sort: KSort,
        productionItems: Vec<KProductionItems>,
        klabel: Option<String>,
        att: KAtt,
    },
    KModuleComment {
        comment: String,
        att: KAtt,
    },
    KSyntaxSort {
        sort: KSort,
        att: KAtt
    },
    KRule {
        body: KExpression,
        requires: KExpression,
        ensures: KExpression,
        att: KAtt
    },
    KSyntaxAssociativity {
        assoc: Assoc,
        tags: Vec<String>,
        att: KAtt

    },
    KSyntaxPriority {
        priorities: Vec<Vec<String>>,
        att: KAtt,
    },
    KContext {
        body: KExpression,
        requires: KExpression,
        att: KAtt
    },
    KBubble {
        sentenceType: String,
        contents: String,
        att: KAtt
    },
}

pub fn has_execinstr_label_expr(expr: &KExpression) -> bool{
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

pub fn has_execinstr_label(sentence: &KSentence) -> bool{
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


#[cfg(test)]
pub mod test;

