#![feature(assert_matches)]

use std::cell::OnceCell;

use itertools::Itertools;

use wrapper_common::operand_type::OperandType;
use wrapper_common::registers::{Reg64WithRIP, RegisterType};

use crate::k_expressions::{KSentence, TopLevel};
use crate::k_to_raw::{extract_rule_data_from_k_rule, OperandNames, RawOperandType, RuleAtom, RuleData, RuleOperandsData};
use crate::k_to_raw::extract_register_expression::{extract_diff_expression_from_semantics, extract_expression, MapEntry, MapEntryKind};
use crate::k_to_raw::strip_dots::remove_dots_and_nodots;
use crate::k_to_raw::utils::{assert_token_is_true, extract_apply_args, extract_apply_label, has_execinstr_label, single_extract};
use crate::raw::{OperandIdx, RawExpression};
use crate::raw_to_typed::expr_to_typed_expr;
use crate::typed_semantics::{RegisterOrParameter64, RegisterOrParameterXMM, Rule, RuleElement, TypedExpression, TypedExpression64};

pub mod k_expressions;
pub mod typed_semantics;
pub mod raw;
pub mod k_to_raw;
pub mod raw_to_typed;

pub fn apply_k_atoms(atoms: Vec<RuleAtom>, primary_arg_definition: &mut OnceCell<OperandNames>, rules_datas: &mut Vec<RuleData>) {
    for atom in atoms {
        match atom {
            RuleAtom::RulesDecl(decl) => {
                let _ = primary_arg_definition.set(OperandNames::new(decl.clone()));
                rules_datas.push(RuleData::DefinitionOnly(decl));
            }
            RuleAtom::MemoryLoadValueAndLoadFromMemory { .. } => {
                todo!()
            }
            RuleAtom::LoadExpression { expr } => {
                rules_datas.push(RuleData::MemLoadAndNextDefinition { load_expression: extract_expression(&expr, primary_arg_definition.get().unwrap()) });
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

pub struct InstructionDescriptor {
    operands: Vec<OperandType>,
    name: String,
}

pub fn extract_rule_from_semantics(semantics: TopLevel, instruction_desc: &InstructionDescriptor) -> Rule {
    let name = instruction_desc.name.to_string();
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
    build_rule(name, rules_datas, instruction_desc)
}


pub fn build_rule(name: impl Into<String>, rule_datas: Vec<RuleData>, instruction_desc: &InstructionDescriptor) -> Rule {
    let mut rule = Rule {
        raw_name: name.into(),
        elements: vec![],
    };
    let mut operand_types = vec![];
    let mut pending_loads = vec![];
    let mut pending_stores = vec![];
    let mut pending_memory_op_idx = 0;
    for rule_data in rule_datas {
        match rule_data {
            RuleData::DefinitionOnly(RuleOperandsData { raw_instruction_name: _, raw_operand_list }) => {
                operand_types = vec![None; raw_operand_list.len()];
                assert_eq!(raw_operand_list.len(), instruction_desc.operands.len());
                for (i, raw_operand) in raw_operand_list.into_iter().enumerate() {
                    operand_types[i] = raw_operand.raw_operand_type.clone();
                }
            }
            RuleData::MemLoadAndNextDefinition { load_expression } => {
                if let RawExpression::LoadFromMemory { offset, size } = load_expression {
                    let offset = offset.as_ref();
                    let size = size.as_ref();
                    if let RawExpression::ConstantInt(size) = size {
                        let load = match size {
                            64 => TypedExpression::_64(TypedExpression64::Load(Box::new(expr_to_typed_expr(offset, &RegisterType::AllGP64WithRIP).unwrap_64()))),
                            size => todo!("{size}")
                        };
                        pending_memory_op_idx += instruction_desc.operands.as_slice()[pending_memory_op_idx..].iter().find_position(|op| {
                            if let OperandType::Mem(mem) = op {
                                mem.load
                            } else {
                                false
                            }
                        }).unwrap().0;
                        pending_loads.push((OperandIdx(pending_memory_op_idx as u8), load));
                    } else {
                        panic!()
                    }
                }
            }
            RuleData::MemStoreAndNextDefinition { store_expression } => {
                if let RawExpression::StoreFromMemory { value, address, size } = store_expression {
                    let value = value.as_ref();
                    let address = address.as_ref();
                    let size = size.as_ref();
                    if let RawExpression::ConstantInt(size) = size {
                        let store = match size {
                            64 => TypedExpression::_64(TypedExpression64::Store {
                                address: Box::new(expr_to_typed_expr(address, &RegisterType::AllGP64WithRIP).unwrap_64()),
                                value: Box::new(expr_to_typed_expr(value, &RegisterType::AllGP64WithRIP).unwrap_64()),
                            }),
                            size => todo!("{size}")
                        };
                        pending_stores.push(store);
                    }
                }
            }
            RuleData::RegState { expression } => {
                for MapEntry { kind, expr } in expression.reg_state_entries {
                    match kind {
                        MapEntryKind::Op(op_idx) => {
                            match operand_types[op_idx.0 as usize].as_ref().unwrap() {
                                RawOperandType::R8 => {
                                    todo!()
                                }
                                RawOperandType::Mem => {
                                    todo!()
                                }
                                RawOperandType::XMM => {
                                    rule.new_vector_register_value(RegisterOrParameterXMM::Operand(op_idx), expr_to_typed_expr(&expr, &RegisterType::AllXmm32).unwrap_256())
                                }
                                RawOperandType::R64 => {
                                    rule.new_general_register_value(RegisterOrParameter64::Operand(op_idx), expr_to_typed_expr(&expr, &RegisterType::AllGP64WithRIP).unwrap_64());
                                }
                            }
                        }
                        MapEntryKind::Flag(_) => {
                            todo!()
                        }
                        MapEntryKind::Reg64(reg64) => {
                            rule.new_general_register_value(RegisterOrParameter64::Register(reg64), expr_to_typed_expr(&expr, &RegisterType::AllGP64WithRIP).unwrap_64());
                        }
                    }
                }
            }
            RuleData::SideEffectingExpression { expression } => {
                if let RawExpression::DecRSPInBytes { inner } = expression {
                    let inner = expr_to_typed_expr(inner.as_ref(), &RegisterType::AllGP64WithRIP);
                    rule.new_general_register_value(RegisterOrParameter64::Register(Reg64WithRIP::RSP), TypedExpression64::Sub {
                        left: Box::new(TypedExpression64::R64 { reg: Reg64WithRIP::RSP }),
                        right: Box::new(inner.unwrap_64()),
                    });
                } else { todo!() }
            }
        }
    }
    for rule_element in rule.elements.iter_mut() {
        let mut values_to_operand_replace_256 = vec![];
        let mut values_to_operand_replace_64 = vec![];
        let mut values_to_operand_replace_1 = vec![];
        let mut values_to_operand_replace_untyped = vec![];
        match rule_element {
            RuleElement::NewGeneralRegisterValue { register: _, value } => {
                values_to_operand_replace_64.push(value);
            }
            RuleElement::NewFlagsValue { flag_cf, flag_pf, flag_af, flag_zf, flag_sf, flag_of } => {
                if let Some(flag_cf) = flag_cf {
                    values_to_operand_replace_1.push(flag_cf);
                }
                if let Some(flag_pf) = flag_pf {
                    values_to_operand_replace_1.push(flag_pf);
                }
                if let Some(flag_af) = flag_af {
                    values_to_operand_replace_1.push(flag_af);
                }
                if let Some(flag_zf) = flag_zf {
                    values_to_operand_replace_1.push(flag_zf);
                }
                if let Some(flag_sf) = flag_sf {
                    values_to_operand_replace_1.push(flag_sf);
                }
                if let Some(flag_of) = flag_of {
                    values_to_operand_replace_1.push(flag_of);
                }
            }
            RuleElement::Store { address, value } => {
                values_to_operand_replace_64.push(address);
                values_to_operand_replace_untyped.push(value);
            }
            RuleElement::NewVectorRegisterValue { register: _, value } => {
                values_to_operand_replace_256.push(value);
            }
        }

        for (pending_load_op_idx, typed_expr) in pending_loads.iter() {
            for value in values_to_operand_replace_64.iter_mut() {
                **value = value.operand_replace(*pending_load_op_idx, typed_expr);
            }
            for value in values_to_operand_replace_1.iter_mut() {
                **value = value.operand_replace(*pending_load_op_idx, typed_expr);
            }
            for value in values_to_operand_replace_untyped.iter_mut() {
                **value = value.operand_replace(*pending_load_op_idx, typed_expr);
            }
            for value in values_to_operand_replace_256.iter_mut() {
                **value = value.operand_replace(*pending_load_op_idx, typed_expr);
            }
        }
    }
    rule
}

#[cfg(test)]
pub mod test;