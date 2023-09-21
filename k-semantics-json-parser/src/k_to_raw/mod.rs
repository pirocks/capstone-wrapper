use std::assert_matches::assert_matches;
use std::collections::HashMap;

use itertools::Itertools;

use wrapper_common::operand_type::OperandType;

use crate::InstructionDescriptor;
use crate::k_expressions::KExpression;
use crate::k_to_raw::extract_register_expression::ExpressionDiffData;
use crate::k_to_raw::utils::{extract_apply_args, extract_apply_label};
use crate::raw::{OperandIdx, RawExpression};

pub mod utils;
pub(crate) mod strip_dots;
pub mod extract_register_expression;


#[derive(Debug)]
pub enum InstructionDefinition {
    Definition(RawExpression),
    ReferToOtherRule {},
}


#[derive(Debug)]
pub enum OperandKind {
    Mem,
    Imm,
    Reg,
}

pub struct OperandNames {
    kinds: Vec<OperandKind>,
    operands_original: HashMap<String, OperandIdx>,
    memory_operand_rename_index: usize,
    register_operand_rename_index: usize,
    imm_operand_rename_index: usize,
    operands_renamed: HashMap<String, OperandIdx>,
}

impl OperandNames {
    pub fn new(desc: &InstructionDescriptor) -> Self {
        Self {
            kinds: desc.operands.iter().map(|op| match op {
                OperandType::Mem(_) => OperandKind::Mem,
                OperandType::Reg(_) => OperandKind::Reg,
                OperandType::Imm(_) => OperandKind::Imm,
                OperandType::ImmSpecific(_) => OperandKind::Imm,
                OperandType::Flags(_) => todo!(),
                OperandType::Agen(_) => todo!(),
                OperandType::Rel8 => OperandKind::Imm,
                OperandType::Rel16 => OperandKind::Imm,
                OperandType::Rel32 => OperandKind::Imm,
            }).collect(),
            operands_original: HashMap::new(),
            memory_operand_rename_index: 0,
            register_operand_rename_index: 0,
            imm_operand_rename_index: 0,
            operands_renamed: Default::default(),
        }
    }

    pub fn name_lookup(&self, name: impl Into<String>) -> OperandIdx {
        self.try_name_lookup(name.into()).unwrap()
    }

    pub fn try_name_lookup(&self, name: impl Into<String>) -> Option<OperandIdx> {
        let name = name.into();
        match self.operands_renamed.get(&name) {
            Some(x) => Some(*x),
            None => {
                match self.operands_original.get(&name).cloned() {
                    None => {
                        match name.as_str() {
                            "R1" => {
                                assert_matches!(&self.kinds[0],OperandKind::Reg);
                                return Some(OperandIdx(0));
                            }
                            "R2" => {
                                assert_matches!(&self.kinds[1],OperandKind::Reg);
                                return Some(OperandIdx(1));
                            }
                            "R3" => {
                                assert_matches!(&self.kinds[2],OperandKind::Reg);
                                return Some(OperandIdx(2));
                            }
                            "Imm8" => {
                                assert_eq!(self.kinds.iter().filter(|k| matches!(k, OperandKind::Imm)).count(), 1);
                                return Some(OperandIdx(self.kinds.iter().find_position(|k| matches!(k, OperandKind::Mem)).unwrap().0 as u8));
                            }
                            "MemOff" => {
                                assert_eq!(self.kinds.iter().filter(|k| matches!(k, OperandKind::Mem)).count(), 1);
                                return Some(OperandIdx(self.kinds.iter().find_position(|k| matches!(k, OperandKind::Mem)).unwrap().0 as u8));
                            }
                            "CF" | "PF" | "AF" | "ZF" | "SF" | "OF" => {
                                return None;
                            }
                            "RAX" | "RIP" => {
                                return None;
                            }
                            name => {
                                todo!("{name}")
                            }
                        }
                    }
                    Some(op_idx) => {
                        return Some(op_idx);
                    }
                }
            }
        }
    }

    pub fn sink_new_memory_operand(&mut self, new_memory_name: impl Into<String>) {
        let new_memory_name = new_memory_name.into();
        let op_idx = self.memory_operand_rename_index + self.kinds[self.memory_operand_rename_index..].iter().find_position(|x| matches!(x,OperandKind::Mem)).unwrap().0;
        self.operands_renamed.insert(new_memory_name, OperandIdx(op_idx as u8));
        self.memory_operand_rename_index = op_idx;
    }
}

#[derive(Debug)]
pub enum RuleData {
    DefinitionOnly(RuleOperandsData),
    MemLoadAndNextDefinition {
        load_expression: RawExpression,
    },
    MemStoreAndNextDefinition {
        store_expression: RawExpression,
    },
    RegState {
        expression: ExpressionDiffData
    },
    SideEffectingExpression {
        expression: RawExpression
    },
}

#[derive(Clone, Debug)]
pub struct RuleOperandsData {
    pub raw_instruction_name: String,
    pub raw_operand_list: Vec<RawOperand>,
}

#[derive(Clone, Debug)]
pub enum RawOperandType {
    R8,
    Mem,
    XMM,
    R64,
}

#[derive(Clone, Debug)]
pub struct RawOperand {
    pub(crate) raw_operand_type: Option<RawOperandType>,
    pub(crate) name: String,
    pub op_idx: OperandIdx,
}

pub fn recursive_operand_extract(operand_list: &KExpression, current_type: Option<RawOperandType>, raw_operands: &mut Vec<RawOperand>) {
    match operand_list {
        KExpression::KApply { label, args, .. } => {
            if label.as_str() == "#SemanticCastToR8" {
                assert_eq!(args.len(), 1);
                return recursive_operand_extract(&args[0], Some(RawOperandType::R8), raw_operands);
            } else if label.as_str() == "operandlist" {
                for arg in args {
                    assert!(current_type.is_none());
                    recursive_operand_extract(arg, None, raw_operands);
                }
                return;
            } else if label.as_str() == ".List{\"operandlist\"}" {
                return;
            } else if label.as_str() == "memOffset" {
                return recursive_operand_extract(&args[0], Some(RawOperandType::Mem), raw_operands);
            } else if label.as_str() == "#SemanticCastToMInt" {
                assert_matches!(current_type, Some(RawOperandType::Mem));
                return recursive_operand_extract(&args[0], current_type, raw_operands);
            } else if label.as_str() == "#SemanticCastToXmm" {
                return recursive_operand_extract(&args[0], Some(RawOperandType::XMM), raw_operands);
            } else if label.as_str() == "#SemanticCastToR64" {
                return recursive_operand_extract(&args[0], Some(RawOperandType::R64), raw_operands);
            } else if label.as_str() == "#SemanticCastToMemOffset" {
                assert_eq!(args.len(), 1);
                return recursive_operand_extract(&args[0], current_type, raw_operands);
            }
            dbg!(label);
            dbg!(args);
            todo!()
        }
        KExpression::KVariable { name, .. } => {
            raw_operands.push(RawOperand { raw_operand_type: current_type, name: name.to_string(), op_idx: OperandIdx(raw_operands.len() as u8) })
        }
        _ => panic!()
    }
}

pub struct LoadExpression {}

#[derive(Debug)]
pub enum RuleAtom {
    RulesDecl(RuleOperandsData),
    LoadExpression {
        expr: KExpression
    },
    MemLoadValue(String),
    StoreExpression {
        expr: KExpression
    },
    MemoryLoadValueAndLoadFromMemory {
        mem_load_value_name: String,
        load_expression: Option<KExpression>,
    },
    Expression(KExpression),
}

pub fn extract_rule_data_from_k_rule(semantic_rule_decl: &KExpression) -> Vec<RuleAtom> {
    let mut res = vec![];
    match semantic_rule_decl {
        KExpression::KRewrite { lhs, rhs } => {
            match lhs.as_ref() {
                KExpression::KApply { label:_, args:_, .. } => {
                    // assert_eq!(label.as_str(), "execinstr");
                    // assert_eq!(args.len(), 1);
                    // let args = extract_apply_args(&args[0], "___X86-SYNTAX");
                    // assert_eq!(args.len(), 2);
                    // let instruction_name_kapply = args.first().unwrap();
                    // let raw_instruction_name = extract_apply_label(instruction_name_kapply);
                    // let operand_list = args.last().unwrap();
                    // let mut raw_operand_list = vec![];
                    // recursive_operand_extract(operand_list, None, &mut raw_operand_list);
                    // res.push(RuleAtom::RulesDecl(RuleOperandsData {
                    //     raw_instruction_name: raw_instruction_name.to_string(),
                    //     raw_operand_list,
                    // }));
                }
                KExpression::KSequence { items, .. } => {
                    assert_eq!(items.len(), 2);
                    assert_eq!(extract_apply_label(&items[0]), "#SemanticCastToMemLoadValue");
                    assert_eq!(extract_apply_label(&items[1]), "execinstr");
                    let without_semantic_cast = &extract_apply_args(&items[0], "#SemanticCastToMemLoadValue")[0];
                    assert_eq!(extract_apply_label(without_semantic_cast), "memLoadValue");
                    let without_mem_load_value = &extract_apply_args(without_semantic_cast, "memLoadValue")[0];
                    let variable = &extract_apply_args(without_mem_load_value, "#SemanticCastToMInt")[0];
                    if let KExpression::KVariable { name, originalName: _ } = variable {
                        res.push(RuleAtom::MemLoadValue(name.to_string()));
                    } else { todo!() }
                }
                _ => {
                    dbg!(lhs);
                    panic!();
                }
            }
            match rhs.as_ref() {
                KExpression::KVariable { .. } => todo!(),
                KExpression::KToken { .. } => todo!(),
                KExpression::KRewrite { lhs, rhs } => {
                    dbg!(lhs);
                    dbg!(rhs);
                    todo!()
                }
                KExpression::KApply { label, .. } => {
                    if label.as_str() == "storeToMemory" {
                        res.push(RuleAtom::StoreExpression { expr: rhs.as_ref().clone() })
                    } else {
                        todo!()
                    }
                }
                KExpression::KSequence { items, .. } => {
                    if items.len() != 0 {
                        assert_eq!(items.len(), 2);

                        if let KExpression::KApply { label, .. } = &items[0] {
                            if label.as_str() == "loadFromMemory" {
                                res.push(RuleAtom::LoadExpression { expr: items[0].clone() });
                            } else if label.as_str() == "storeToMemory" {
                                res.push(RuleAtom::StoreExpression { expr: items[0].clone() })
                            } else {
                                todo!("{label}")
                            }
                        } else {
                            todo!("{:?}", &items[0])
                        };
                        if let KExpression::KApply { label, .. } = &items[1] {
                            if label.as_str() == "execinstr" {
                                // skip rexec for now
                            } else {
                                res.push(RuleAtom::Expression(items[1].clone()));
                            }
                        } else { todo!() }
                    }
                }
            };
        }
        _ => {}
    }
    res
}

