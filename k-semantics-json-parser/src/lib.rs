use std::fs::File;
use std::io::BufReader;

use crate::k_expressions::{KSentence, TopLevel};
use crate::k_to_raw::{assert_token_is_true, extract_apply_args, extract_diff_expression_from_semantics, extract_operand_data_from_semantics};
use crate::raw_to_typed::build_rule;
use crate::typed_semantics::{Rule};

pub mod k_expressions;
pub mod typed_semantics;
pub mod raw;
pub mod k_to_raw;
pub mod raw_to_typed;

pub fn extract_rule_from_semantics(semantics: TopLevel) -> Rule {
    for module in semantics.term.modules {
        if module.name == "ADCB-R8-R8".to_string() {
            for local_sentence in module.localSentences {
                if let KSentence::KRule { body, requires, ensures, att: _ } = &local_sentence {
                    if k_to_raw::has_execinstr_label(&local_sentence, "execinstr") {
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