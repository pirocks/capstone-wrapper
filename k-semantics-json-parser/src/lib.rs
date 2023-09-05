#![feature(assert_matches)]

use std::cell::OnceCell;
use crate::k_expressions::{KSentence, TopLevel};
use crate::k_to_raw::extract_register_expression::{extract_diff_expression_from_semantics, extract_expression};
use crate::k_to_raw::{extract_rule_data_from_k_rule, RuleAtom, OperandNames, RuleData};
use crate::k_to_raw::strip_dots::remove_dots_and_nodots;
use crate::k_to_raw::utils::{assert_token_is_true, extract_apply_args, extract_apply_label, has_execinstr_label, single_extract};
use crate::typed_semantics::Rule;

pub mod k_expressions;
pub mod typed_semantics;
pub mod raw;
pub mod k_to_raw;
pub mod raw_to_typed;

pub fn apply_k_atoms(atoms: Vec<RuleAtom>, primary_arg_definition: &mut OnceCell<OperandNames>, rules_datas: &mut Vec<RuleData>) {
    for atom in atoms {
        match atom {
            RuleAtom::RulesDecl(decl) => {
                let _ = primary_arg_definition.set(OperandNames::new(decl));
            }
            RuleAtom::MemoryLoadValueAndLoadFromMemory { mem_load_value_name, load_expression } => {
                todo!()
            }
            RuleAtom::LoadExpression { expr } => {
                extract_expression(&expr,primary_arg_definition.get().unwrap());
            }
            RuleAtom::MemLoadValue(expr) => {
                primary_arg_definition.get_mut().unwrap().sink_new_memory_operand(expr);
            }
            RuleAtom::StoreExpression { expr } => {
                let expression = extract_expression(&expr, primary_arg_definition.get().unwrap());
                rules_datas.push(RuleData::MemStoreAndNextDefinition { store_expression: expression });
            }
            RuleAtom::Expression(expr) => {
                let expression = extract_expression(&expr, primary_arg_definition.get().unwrap());
                rules_datas.push(RuleData::SideEffectingExpression { expression });
            }
        }
    }
}

pub fn extract_rule_from_semantics(semantics: TopLevel, name: impl Into<String>) -> Rule {
    let name = name.into();
    let mut rules_datas = vec![];
    let mut primary_arg_definition = OnceCell::new();
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
                        match extract_apply_label(body) {
                            "<k>" => {
                                let rule_expressions = extract_apply_args(body, "<k>");
                                let line = extract_rule_data_from_k_rule(single_extract(remove_dots_and_nodots(rule_expressions).as_ref()));
                                apply_k_atoms(line, &mut primary_arg_definition, &mut rules_datas);
                            }
                            "#cells" => {
                                let apply_args = extract_apply_args(body, "#cells");
                                assert_eq!(apply_args.len(), 2);
                                let k = apply_args.first().unwrap();
                                let reg_state = apply_args.last().unwrap();
                                let extracted_operands = extract_apply_args(&k, "<k>");
                                let line = extract_rule_data_from_k_rule(single_extract(remove_dots_and_nodots(extracted_operands).as_ref()));
                                apply_k_atoms(line, &mut primary_arg_definition, &mut rules_datas);
                                let extracted_diff = extract_apply_args(&reg_state, "<regstate>");
                                let diff_data = extract_diff_expression_from_semantics(extracted_diff, primary_arg_definition.get().unwrap());
                                rules_datas.push(RuleData::RegState { expression: diff_data })
                            }
                            s => todo!("{s}")
                        }
                    }
                }
            }
        }
    }
    dbg!(rules_datas);
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
        Ok(())
    }

    #[test]
    pub fn test_extract_callq_m64() -> anyhow::Result<()> {
        let top_level: TopLevel = serde_json::from_reader(BufReader::new(File::open("data/minimized-CALLQ-M64.json")?))?;
        let _rule = extract_rule_from_semantics(top_level, "CALLQ-M64");
        Ok(())
    }
}