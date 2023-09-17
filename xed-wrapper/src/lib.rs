use std::collections::HashMap;
use std::ffi::{CStr};
use std::sync::{Once, OnceLock};

use proc_macro2::Ident;
use quote::format_ident;
use xed_sys::{xed_iclass_enum_t, xed_iform_enum_t, xed_iform_enum_t2str, xed_iform_to_iclass_string_intel, xed_inst_iclass, xed_inst_iform_enum, xed_inst_noperands, xed_inst_operand, xed_inst_table_base, XED_MAX_INST_TABLE_NODES, xed_operand_enum_t, xed_operand_name, xed_operand_nonterminal_name, xed_operand_operand_visibility, xed_operand_type, xed_operand_width, XED_OPVIS_EXPLICIT, xed_tables_init};

use wrapper_common::registers::RegisterType;

use crate::nonterminal_type::NonTerminalType;
use crate::operand_name::OperandName;
use crate::operand_type::XedOperandType;
use crate::operand_width::OperandWidth;

pub mod operand_name;
pub mod operand_width;
pub mod operand_register;
pub mod operand_type;
pub mod nonterminal_type;
pub mod operands;


#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct TopLevelInstructionName(pub String);

impl TopLevelInstructionName {
    pub fn new(input: impl AsRef<str>) -> Self {
        Self(input.as_ref().replace(" ", "_").to_uppercase())
    }

    pub fn proc_macro_safe_name(&self) -> Ident {
        format_ident!("{}", self.0.as_str())
    }
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct VariantName(pub String);

impl VariantName {
    pub fn new(input: impl AsRef<str>) -> Self {
        Self(input.as_ref().replace(" ", "_").to_uppercase())
    }

    pub fn proc_macro_safe_name(&self) -> Ident {
        format_ident!("{}", self.0.as_str())
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum FieldType {
    Mem(OperandWidth),
    Reg(RegisterType),
    Imm,
    RelBR,
    Ptr,
    AGen,
}

#[derive(Debug)]
pub struct Field {
    pub field_type: FieldType,
    pub field_name: xed_operand_enum_t,
}

#[derive(Debug)]
pub struct Variant{
    pub operands: HashMap<usize, Field>,
    pub iform: xed_iform_enum_t
}

#[derive(Debug)]
pub struct TopLevelInstruction{
    pub iclass: xed_iclass_enum_t,
    pub variants: HashMap<VariantName, Variant>
}

static START: Once = Once::new();

static DATA: OnceLock<HashMap<TopLevelInstructionName, TopLevelInstruction>> = OnceLock::new();

pub fn xed_data() -> &'static HashMap<TopLevelInstructionName, TopLevelInstruction> {
    DATA.get_or_init(|| {
        let mut res: HashMap<TopLevelInstructionName, TopLevelInstruction> = HashMap::new();
        unsafe {
            START.call_once(|| {
                xed_tables_init();
            });
            for i in 0..XED_MAX_INST_TABLE_NODES {
                let instruction_table_elem = xed_inst_table_base().add(i as usize);
                let iform_i = xed_inst_iform_enum(instruction_table_elem);
                let variant_name = VariantName::new(CStr::from_ptr(xed_iform_enum_t2str(iform_i)).to_str().unwrap().to_string());
                let instruction_name = TopLevelInstructionName::new(CStr::from_ptr(xed_iform_to_iclass_string_intel(iform_i)).to_str().unwrap().to_string());
                let variant_fields: &mut HashMap<usize, Field> = &mut res.entry(instruction_name)
                    .or_insert_with(||TopLevelInstruction{ iclass: xed_inst_iclass(instruction_table_elem), variants: Default::default() })
                    .variants
                    .entry(variant_name)
                    .or_insert_with(||Variant{ operands: Default::default(), iform: iform_i })
                    .operands;
                let number_of_operands = xed_inst_noperands(instruction_table_elem);
                let mut visible_field_i = 0;
                for operand_i in 0..number_of_operands {
                    let operand = xed_inst_operand(instruction_table_elem, operand_i);
                    let visibility = xed_operand_operand_visibility(operand);
                    if visibility == XED_OPVIS_EXPLICIT {
                        let operand_name = OperandName::new(xed_operand_name(operand));
                        let operand_width = OperandWidth::new(xed_operand_width(operand));
                        let operand_type = XedOperandType::new(xed_operand_type(operand));
                        let non_terminal_name = NonTerminalType::new(xed_operand_nonterminal_name(operand));
                        let field_type = match operand_name {
                            OperandName::MEM0 => {
                                FieldType::Mem(operand_width.unwrap())
                            }
                            OperandName::REG0 | OperandName::REG1 | OperandName::REG2 | OperandName::REG3 => {
                                if let XedOperandType::NT_LOOKUP_FN | XedOperandType::NT_LOOKUP_FN2 | XedOperandType::NT_LOOKUP_FN4 = operand_type {
                                    let non_terminal_name = non_terminal_name.unwrap();
                                    match non_terminal_name {
                                        NonTerminalType::X87 => {
                                            FieldType::Reg(RegisterType::AllFloat)
                                        }
                                        NonTerminalType::SEG | NonTerminalType::SEG_MOV => {
                                            FieldType::Reg(RegisterType::AllSegment)
                                        }
                                        NonTerminalType::GPR8_R | NonTerminalType::GPRV_R | NonTerminalType::GPRV_B | NonTerminalType::GPRV_SB
                                        | NonTerminalType::GPR8_B | NonTerminalType::GPR16_B | NonTerminalType::GPR8_SB | NonTerminalType::GPR16_R
                                        | NonTerminalType::GPRZ_B | NonTerminalType::GPRZ_R | NonTerminalType::GPR32_B | NonTerminalType::GPR64_B
                                        | NonTerminalType::GPR32_R | NonTerminalType::GPR64_R | NonTerminalType::GPRY_R | NonTerminalType::GPRY_B
                                        | NonTerminalType::VGPR32_R | NonTerminalType::VGPRY_R | NonTerminalType::VGPR32_B | NonTerminalType::VGPRY_B
                                        | NonTerminalType::VGPR32_N | NonTerminalType::VGPRY_N | NonTerminalType::VGPR64_R | NonTerminalType::VGPR64_N
                                        | NonTerminalType::VGPR64_B => {
                                            match operand_width.unwrap() {
                                                OperandWidth::B => FieldType::Reg(RegisterType::AllGP8),
                                                OperandWidth::V => FieldType::Reg(RegisterType::Multiple(vec![RegisterType::AllGP64WithRIP, RegisterType::AllGP32WithRIP, RegisterType::AllGP16WithRIP, RegisterType::AllGP8])),
                                                OperandWidth::Z => FieldType::Reg(RegisterType::Multiple(vec![RegisterType::AllGP64WithRIP, RegisterType::AllGP32WithRIP, RegisterType::AllGP16WithRIP])),
                                                OperandWidth::Y => FieldType::Reg(RegisterType::Multiple(vec![RegisterType::AllGP64WithRIP, RegisterType::AllGP32WithRIP])),
                                                OperandWidth::W => FieldType::Reg(RegisterType::AllGP16WithRIP),
                                                OperandWidth::D => FieldType::Reg(RegisterType::AllGP32WithRIP),
                                                OperandWidth::Q => FieldType::Reg(RegisterType::AllGP64WithRIP),
                                                width => todo!("{width:?}")
                                            }
                                        }
                                        NonTerminalType::A_GPR_R | NonTerminalType::A_GPR_B => {
                                            FieldType::Reg(RegisterType::Multiple(vec![RegisterType::AllGP64WithRIP, RegisterType::AllGP32WithRIP, RegisterType::AllGP16WithRIP]))
                                        }
                                        NonTerminalType::MMX_B | NonTerminalType::MMX_R => {
                                            FieldType::Reg(RegisterType::AllMmx)
                                        }
                                        NonTerminalType::XMM_B | NonTerminalType::XMM_R | NonTerminalType::XMM_R3 | NonTerminalType::XMM_N | NonTerminalType::XMM_N3
                                        | NonTerminalType::XMM_SE | NonTerminalType::XMM_B3 => {
                                            FieldType::Reg(RegisterType::AllXmm32)
                                        }
                                        NonTerminalType::YMM_B | NonTerminalType::YMM_R | NonTerminalType::YMM_N | NonTerminalType::YMM_SE | NonTerminalType::YMM_R3
                                        | NonTerminalType::YMM_N3 | NonTerminalType::YMM_B3 => {
                                            FieldType::Reg(RegisterType::AllYmm32)
                                        }
                                        NonTerminalType::ZMM_R3 | NonTerminalType::ZMM_N3 | NonTerminalType::ZMM_B3 => {
                                            FieldType::Reg(RegisterType::AllZmm32)
                                        }
                                        NonTerminalType::CR_R => {
                                            FieldType::Reg(RegisterType::AllControl)
                                        }
                                        NonTerminalType::DR_R => {
                                            FieldType::Reg(RegisterType::AllDebug)
                                        }
                                        NonTerminalType::BND_R | NonTerminalType::BND_B => {
                                            FieldType::Reg(RegisterType::AllBnd)
                                        }
                                        NonTerminalType::MASK_R | NonTerminalType::MASK_N | NonTerminalType::MASK_B | NonTerminalType::MASK1 | NonTerminalType::MASKNOT0 => {
                                            FieldType::Reg(RegisterType::AllMask)
                                        }
                                        NonTerminalType::TMM_R | NonTerminalType::TMM_N | NonTerminalType::TMM_B => {
                                            FieldType::Reg(RegisterType::AllTmm)
                                        }
                                        non_terminal_name => {
                                            todo!("{non_terminal_name:?}")
                                        }
                                    }
                                } else {
                                    todo!()
                                }
                            }
                            OperandName::IMM0 | OperandName::IMM1 => {
                                FieldType::Imm
                            }
                            OperandName::RELBR => {
                                FieldType::RelBR
                            }
                            OperandName::PTR => {
                                FieldType::Ptr
                            }
                            OperandName::AGEN => {
                                FieldType::AGen
                            }
                            other => {
                                todo!("{other:?}")
                            }
                        };
                        variant_fields.insert(visible_field_i, Field { field_type, field_name: xed_operand_name(operand) });
                        visible_field_i += 1;
                    }
                }
            }
        }
        res
    })
}


#[cfg(test)]
mod tests {
    use crate::xed_data;

    #[test]
    fn print_all_iforms() {
        // unsafe {
        //     for i in 1..xed_iform_enum_t_last() {
        //         let cstr = CStr::from_ptr(xed_iform_enum_t2str(i));
        //         dbg!(CStr::from_ptr(xed_iform_to_iclass_string_intel(i)).to_str().unwrap());
        //         dbg!(cstr.to_str().unwrap());
        //     }
        // }
        let data = xed_data();
        dbg!(data);
        todo!()
    }
}

pub mod temp;