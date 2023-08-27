use crate::k_expressions::{KSentence, TopLevel};
use crate::k_to_raw::{extract_rule_data_from_semantics, RuleData};
use crate::k_to_raw::extract_register_expression::extract_diff_expression_from_semantics;
use crate::k_to_raw::strip_dots::remove_dots_and_nodots;
use crate::k_to_raw::utils::{assert_token_is_true, extract_apply_args, extract_apply_label, has_execinstr_label};
use crate::raw_to_typed::build_rule;
use crate::typed_semantics::{Rule};

pub mod k_expressions;
pub mod typed_semantics;
pub mod raw;
pub mod k_to_raw;
pub mod raw_to_typed;

pub fn extract_rule_from_semantics(semantics: TopLevel, name: impl Into<String>) -> Rule {
    let name = name.into();
    for module in semantics.term.modules {
        if module.name == name.as_str() {
            for local_sentence in module.localSentences {
                if let KSentence::KRule { body, requires, ensures, att: _ } = &local_sentence {
                    if has_execinstr_label(&local_sentence, "execinstr") {
                        assert_token_is_true(requires);
                        assert_token_is_true(ensures);
                        // possible situations here:
                        // rule defined in terms of other rules,
                        // we want any rules definitions in terms of other rules and regstate rules
                        match extract_apply_label(body){
                            "<k>" => {
                                let rule_expressions = extract_apply_args(body, "<k>");
                                let rule_operands_data = extract_rule_data_from_semantics(remove_dots_and_nodots(rule_expressions).as_ref());
                                match rule_operands_data {
                                    RuleData::OperandsOnlyRule(_) => {}
                                    RuleData::OperandsAndDefinition { .. } => {}
                                }
                                dbg!(&rule_operands_data);
                                todo!()
                            }
                            "#cells" => {
                                let apply_args = extract_apply_args(body, "#cells");
                                assert_eq!(apply_args.len(), 2);
                                let k = apply_args.first().unwrap();
                                let reg_state = apply_args.last().unwrap();
                                let extracted_operands = extract_apply_args(&k, "<k>");
                                let rule_operands_data = match extract_rule_data_from_semantics(remove_dots_and_nodots(extracted_operands).as_ref()) {
                                    RuleData::OperandsOnlyRule(rule) => rule,
                                    RuleData::OperandsAndDefinition { .. } => todo!()
                                };
                                let extracted_diff = extract_apply_args(&reg_state, "<regstate>");
                                let diff_data = extract_diff_expression_from_semantics(extracted_diff, &rule_operands_data);
                                return build_rule(rule_operands_data, diff_data);
                            }
                            s => todo!("{s}")
                        }

                    }
                }
            }
        }
    }
    panic!()
}

#[cfg(test)]
pub mod test {
    use std::fs::File;
    use std::io::BufReader;
    use crate::extract_rule_from_semantics;
    use crate::k_expressions::TopLevel;

    #[test]
    pub fn test_minimized() -> anyhow::Result<()> {
        let top_level: TopLevel = serde_json::from_reader(BufReader::new(File::open("data/formatted_parsed.json")?))?;
        let full_res = extract_rule_from_semantics(top_level, "ADCB-R8-R8");
        let top_level: TopLevel = serde_json::from_reader(BufReader::new(File::open("data/minimized.json")?))?;
        let minimized_res = extract_rule_from_semantics(top_level, "ADCB-R8-R8");
        assert_eq!(full_res, minimized_res);
        panic!();
        Ok(())
    }

    #[test]
    pub fn test_extract_callq_m64() -> anyhow::Result<()> {
        let top_level: TopLevel = serde_json::from_reader(BufReader::new(File::open("data/minimized-CALLQ-M64.json")?))?;
        let _rule = extract_rule_from_semantics(top_level, "CALLQ-M64");
        Ok(())
    }
}