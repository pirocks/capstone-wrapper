#![feature(assert_matches)]

use itertools::Itertools;

use wrapper_common::operand_type::OperandType;
use wrapper_common::registers::{Reg64WithRIP, RegisterType};

use crate::k_expressions::{KSentence, TopLevel};
use crate::k_to_raw::{extract_rule_data_from_k_rule, OperandNames, RuleAtom, RuleData, RuleOperandsData};
use crate::k_to_raw::extract_register_expression::{extract_diff_expression_from_semantics, extract_expression, Flag, MapEntry, MapEntryKind};
use crate::k_to_raw::strip_dots::remove_dots_and_nodots;
use crate::k_to_raw::utils::{assert_token_is_true, extract_apply_args, extract_apply_label, has_execinstr_label, single_extract};
use crate::raw::{OperandIdx, RawExpression};
use crate::raw_to_typed::{expr_to_typed_expr, ExpressionType};
use crate::typed_semantics::{NewFlags, RegisterOrParameter64, RegisterOrParameterXMM, ReplaceWith, Rule, RuleElement, TypedExpression, TypedExpression1, TypedExpression128, TypedExpression256, TypedExpression56, TypedExpression64, TypedExpression8, TypedExpression9};

pub mod k_expressions;
pub mod typed_semantics;
pub mod raw;
pub mod k_to_raw;
pub mod raw_to_typed;

pub fn apply_k_atoms(atoms: Vec<RuleAtom>, primary_arg_definition: &mut OperandNames, rules_datas: &mut Vec<RuleData>) {
    for atom in atoms {
        match atom {
            RuleAtom::RulesDecl(decl) => {
                rules_datas.push(RuleData::DefinitionOnly(decl));
            }
            RuleAtom::MemoryLoadValueAndLoadFromMemory { .. } => {
                todo!()
            }
            RuleAtom::LoadExpression { expr } => {
                rules_datas.push(RuleData::MemLoadAndNextDefinition { load_expression: extract_expression(&expr, primary_arg_definition) });
            }
            RuleAtom::MemLoadValue(expr) => {
                primary_arg_definition.sink_new_memory_operand(expr);
            }
            RuleAtom::StoreExpression { expr } => {
                let expression = extract_expression(&expr, primary_arg_definition);
                rules_datas.push(RuleData::MemStoreAndNextDefinition { store_expression: expression });
            }
            RuleAtom::Expression(expr) => {
                let expression = extract_expression(&expr, primary_arg_definition);
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
    let mut primary_arg_definition = OperandNames::new(instruction_desc);
    for module in semantics.term.modules {
        if module.name == name.as_str() {
            for local_sentence in module.localSentences.iter().rev() {
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
                                let diff_data = extract_diff_expression_from_semantics(extracted_diff, &mut primary_arg_definition);
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
    let mut pending_memory_op_idx = 0;
    for rule_data in rule_datas {
        match rule_data {
            RuleData::DefinitionOnly(RuleOperandsData { .. }) => {}
            RuleData::MemLoadAndNextDefinition { load_expression } => {
                if let RawExpression::LoadFromMemory { offset, size } = load_expression {
                    let offset = offset.as_ref();
                    let size = size.as_ref();
                    if let RawExpression::ConstantInt(_size) = size {
                        let address = expr_to_typed_expr(offset, Some(&ExpressionType::_64), instruction_desc).unwrap_64();
                        pending_memory_op_idx += instruction_desc.operands.as_slice()[pending_memory_op_idx..].iter().find_position(|op| {
                            if let OperandType::Mem(mem) = op {
                                mem.load
                            } else {
                                false
                            }
                        }).unwrap().0;
                        rule.new_load(OperandIdx(pending_memory_op_idx as u8), address);
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
                    let address_out;
                    let value_out;
                    if let RawExpression::ConstantInt(size) = size {
                        match size {
                            64 => {
                                address_out = expr_to_typed_expr(address, Some(&ExpressionType::_64), instruction_desc).unwrap_64();
                                value_out = expr_to_typed_expr(value, Some(&ExpressionType::_64), instruction_desc)
                            }
                            8 => {
                                address_out = expr_to_typed_expr(address, Some(&ExpressionType::_64), instruction_desc).unwrap_64();
                                value_out = expr_to_typed_expr(value, Some(&ExpressionType::_8), instruction_desc)
                            }
                            size => todo!("{size}")
                        };
                        rule.new_store(address_out, value_out);
                    }
                }
            }
            RuleData::RegState { expression } => {
                for MapEntry { kind, expr } in expression.reg_state_entries {
                    match kind {
                        MapEntryKind::Op(op_idx) => {
                            match &instruction_desc.operands[op_idx.0 as usize] {
                                OperandType::Mem(_) => {
                                    todo!()
                                }
                                OperandType::Reg(reg_) => {
                                    match reg_ {
                                        RegisterType::AllGP64WithoutRIP |
                                        RegisterType::AllGP64WithRIP |
                                        RegisterType::SingleGP64(_) => {
                                            rule.new_general_register_value(RegisterOrParameter64::Operand(op_idx), expr_to_typed_expr(&expr, Some(&ExpressionType::_64), instruction_desc).unwrap_64())
                                        }
                                        RegisterType::AllXmm32 | RegisterType::AllXmm16 | RegisterType::SingleXmm(_) => {
                                            rule.new_vector_register_value(RegisterOrParameterXMM::Operand(op_idx), expr_to_typed_expr(&expr, Some(&ExpressionType::_256), instruction_desc).unwrap_256())
                                        }
                                        _ => {
                                            todo!()
                                        }
                                    }

                                }
                                OperandType::Imm(_) |
                                OperandType::ImmSpecific(_) |
                                OperandType::Flags(_) |
                                OperandType::Agen(_) |
                                OperandType::Rel8 |
                                OperandType::Rel16 |
                                OperandType::Rel32 => {
                                    todo!()
                                }
                            }
                        }
                        MapEntryKind::Flag(flag) => {
                            match flag {
                                Flag::CF => {
                                    rule.new_flags_value(NewFlags {
                                        flag_cf: Some(expr_to_typed_expr(&expr, Some(&ExpressionType::_1), instruction_desc).unwrap_1()),
                                        flag_pf: None,
                                        flag_af: None,
                                        flag_zf: None,
                                        flag_sf: None,
                                        flag_of: None,
                                    });
                                }
                                Flag::PF => {
                                    rule.new_flags_value(NewFlags {
                                        flag_cf: None,
                                        flag_pf: Some(expr_to_typed_expr(&expr, Some(&ExpressionType::_1), instruction_desc).unwrap_1()),
                                        flag_af: None,
                                        flag_zf: None,
                                        flag_sf: None,
                                        flag_of: None,
                                    });
                                }
                                Flag::ZF => {
                                    rule.new_flags_value(NewFlags {
                                        flag_cf: None,
                                        flag_pf: None,
                                        flag_af: None,
                                        flag_zf: Some(expr_to_typed_expr(&expr, Some(&ExpressionType::_1), instruction_desc).unwrap_1()),
                                        flag_sf: None,
                                        flag_of: None,
                                    });
                                }
                                Flag::SF => {
                                    rule.new_flags_value(NewFlags {
                                        flag_cf: None,
                                        flag_pf: None,
                                        flag_af: None,
                                        flag_zf: None,
                                        flag_sf: Some(expr_to_typed_expr(&expr, Some(&ExpressionType::_1), instruction_desc).unwrap_1()),
                                        flag_of: None,
                                    });
                                }
                                Flag::OF => {
                                    rule.new_flags_value(NewFlags {
                                        flag_cf: None,
                                        flag_pf: None,
                                        flag_af: None,
                                        flag_zf: None,
                                        flag_sf: None,
                                        flag_of: Some(expr_to_typed_expr(&expr, Some(&ExpressionType::_1), instruction_desc).unwrap_1()),
                                    });
                                }
                            }
                        }
                        MapEntryKind::Reg64(reg64) => {
                            rule.new_general_register_value(RegisterOrParameter64::Register(reg64), expr_to_typed_expr(&expr, Some(&ExpressionType::_64), instruction_desc).unwrap_64());
                        }
                    }
                }
            }
            RuleData::SideEffectingExpression { expression } => {
                if let RawExpression::DecRSPInBytes { inner } = expression {
                    let inner = expr_to_typed_expr(inner.as_ref(), Some(&ExpressionType::_64), instruction_desc);
                    rule.new_general_register_value(RegisterOrParameter64::Register(Reg64WithRIP::RSP), TypedExpression64::Sub {
                        left: Box::new(TypedExpression64::R64 { reg: Reg64WithRIP::RSP }),
                        right: Box::new(inner.unwrap_64()),
                    });
                } else { todo!() }
            }
        }
    }
    let mut pending_loads = vec![];
    for rule_element in rule.elements.iter_mut() {
        let mut values_to_operand_replace_256 = vec![];
        let mut values_to_operand_replace_64 = vec![];
        let mut values_to_operand_replace_8 = vec![];
        let mut values_to_operand_replace_1 = vec![];
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
                if let TypedExpression64::OperandR8 {..} | TypedExpression64::OperandR64 {..}   = address{

                } else {
                    values_to_operand_replace_64.push(address);
                }
                match value {
                    TypedExpression::_256(_) => {
                        todo!()
                    }
                    TypedExpression::_128(_) => {
                        todo!()
                    }
                    TypedExpression::_64(_64) => {
                        values_to_operand_replace_64.push(_64);
                    }
                    TypedExpression::_56(_) => {
                        todo!()
                    }
                    TypedExpression::_9(_) => {
                        todo!()
                    }
                    TypedExpression::_8(_8) => {
                        values_to_operand_replace_8.push(_8);
                    }
                    TypedExpression::_1(_) => {
                        todo!()
                    }
                    TypedExpression::_16(_) => {
                        todo!()
                    }
                    TypedExpression::_24(_) => {
                        todo!()
                    }
                    other => todo!("{other:?}")
                }
            }
            RuleElement::NewVectorRegisterValue { register: _, value } => {
                values_to_operand_replace_256.push(value);
            }
            RuleElement::Load { op_idx, address } => {
                pending_loads.push((*op_idx,address));
            }
        }


        for (pending_load_op_idx, address) in pending_loads.iter() {
            let replace_with = ReplaceWith{
                _256: TypedExpression256::Load(Box::new((*address).clone())),
                _128: TypedExpression128::Load(Box::new((*address).clone())),
                _64: TypedExpression64::Load(Box::new((*address).clone())),
                _56: TypedExpression56::Load(Box::new((*address).clone())),
                _9: TypedExpression9::Load(Box::new((*address).clone())),
                _8: TypedExpression8::Load(Box::new((*address).clone())),
                _1: TypedExpression1::Load(Box::new((*address).clone())),
            };
            for value in values_to_operand_replace_1.iter_mut() {
                **value = value.operand_replace(*pending_load_op_idx, &replace_with);
            }
            for value in values_to_operand_replace_8.iter_mut() {
                **value = value.operand_replace(*pending_load_op_idx, &replace_with);
            }
            for value in values_to_operand_replace_64.iter_mut() {
                **value = value.operand_replace(*pending_load_op_idx, &replace_with);
            }
            for value in values_to_operand_replace_256.iter_mut() {
                **value = value.operand_replace(*pending_load_op_idx, &replace_with);
            }
        }
    }
    rule
}

#[cfg(test)]
pub mod test;