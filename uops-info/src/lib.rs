use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum InstructionElement {
    #[serde(rename = "operand")]
    Operand {
        idx: String,
        r: Option<String>,
        w: Option<String>,
        r#type: String,
        width: Option<String>,
        xtype: Option<String>,
        name: Option<String>,
        #[serde(rename = "VSIB")]
        vsib: Option<String>,
        #[serde(rename = "memory-prefix")]
        memory_prefix: Option<String>,
        #[serde(rename = "$value")]
        val: Option<String>,
    },
    #[serde(rename = "architecture")]
    Architecture {},
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Instruction {
    pub asm: String,
    pub category: String,
    pub cpl: String,
    pub extension: String,
    pub iclass: String,
    pub iform: String,
    #[serde(rename = "isa-set")]
    pub isa_set: String,
    pub string: String,
    pub url: Option<String>,
    pub summary: Option<String>,
    #[serde(rename = "url-ref")]
    pub url_ref: Option<String>,
    pub vex: Option<String>,
    pub evex: Option<String>,
    pub mxcsr: Option<String>,
    pub mask: Option<String>,
    pub zeroing: Option<String>,
    pub roundc: Option<String>,
    pub sae: Option<String>,
    pub bcast: Option<String>,
    pub immzero: Option<String>,
    pub high8: Option<String>,
    pub rm: Option<String>,
    pub eosz: Option<String>,
    pub locked: Option<String>,
    pub agen: Option<String>,
    pub rep: Option<String>,
    #[serde(rename = "$value")]
    pub operands: Option<Vec<InstructionElement>>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Extension {
    pub name: String,
    #[serde(rename = "$value")]
    pub instructions: Option<Vec<Instruction>>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Root {
    pub date: String,
    #[serde(rename = "$value")]
    pub extensions: Vec<Extension>,
}
