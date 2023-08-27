use crate::k_expressions::KExpression;
use crate::k_to_raw::utils::{extract_apply_args, extract_apply_label};
use crate::raw::OperandIdx;


pub(crate) mod utils;
pub(crate) mod strip_dots;
pub mod extract_register_expression;


#[derive(Debug)]
pub enum RuleData {
    OperandsOnlyRule(RuleOperandsData),
    OperandsAndDefinition {
        rule_operands_data: RuleOperandsData,
        definition: (),
    },
}

#[derive(Debug)]
pub struct RuleOperandsData {
    pub(crate) raw_instruction_name: String,
    pub raw_operand_list: Vec<RawOperand>,
}

#[derive(Debug)]
pub enum RawOperandType {
    R8
}

#[derive(Debug)]
pub struct RawOperand {
    pub(crate) raw_operand_type: Option<RawOperandType>,
    pub(crate) name: String,
    pub(crate) op_idx: OperandIdx,
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

pub fn extract_rule_data_from_semantics(semantic_rule_decl: &[KExpression]) -> RuleData {
    let meat = &semantic_rule_decl[0];
    assert_eq!(semantic_rule_decl.len(), 1);
    if let KExpression::KRewrite { lhs, rhs } = meat {
        let operands_data = if let KExpression::KApply { label, args, .. } = lhs.as_ref() {
            assert_eq!(label.as_str(), "execinstr");
            assert_eq!(args.len(), 1);
            let args = extract_apply_args(&args[0], "___X86-SYNTAX");
            assert_eq!(args.len(), 2);
            let instruction_name_kapply = args.first().unwrap();
            let raw_instruction_name = extract_apply_label(instruction_name_kapply);
            let operand_list = args.last().unwrap();
            let mut raw_operand_list = vec![];
            recursive_operand_extract(operand_list, None, &mut raw_operand_list);
            RuleOperandsData {
                raw_instruction_name: raw_instruction_name.to_string(),
                raw_operand_list,
            }
        } else {
            panic!();
        };
        let rule_definition = match rhs.as_ref() {
            KExpression::KApply { .. } => todo!(),
            KExpression::KVariable { .. } => todo!(),
            KExpression::KToken { .. } => todo!(),
            KExpression::KRewrite { .. } => todo!(),
            KExpression::KSequence { .. } => todo!(),
        };
    }
    todo!()
}

