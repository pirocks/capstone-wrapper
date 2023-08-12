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

#[derive(Serialize, Deserialize, Debug)]
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

#[cfg(test)]
pub mod test;

