use std::collections::{HashMap, HashSet};
use std::ffi::CStr;
use std::sync::{Once, OnceLock};

use itertools::Itertools;
use proc_macro2::Ident;
use quote::format_ident;
use xed_sys::{xed_iclass_enum_t, xed_iclass_enum_t2str, xed_iform_enum_t, xed_iform_enum_t2str, xed_iform_to_iclass, xed_inst_iclass, xed_inst_iform_enum, xed_inst_noperands, xed_inst_operand, xed_inst_table_base, XED_MAX_INST_TABLE_NODES, xed_operand_enum_t, xed_operand_name, xed_operand_nonterminal_name, xed_operand_operand_visibility, xed_operand_type, xed_operand_width, XED_OPVIS_EXPLICIT, xed_tables_init};

use wrapper_common::operand_width::XedOperandWidth;
use wrapper_common::registers::{OperandWidth, RegisterType};

use crate::nonterminal_type::NonTerminalType;
use crate::operand_name::OperandName;
use crate::operand_type::XedOperandType;

pub mod nonterminal_type;
pub mod operand_name;
pub mod operand_register;
pub mod operand_type;
pub mod operands;
pub mod generate_example_values;

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

#[derive(Clone, Eq, PartialEq, Hash, Debug, Ord, PartialOrd)]
pub struct VariantName(pub String);

impl VariantName {
    pub fn new(input: impl AsRef<str>) -> Self {
        Self(input.as_ref().replace(" ", "_").to_uppercase())
    }

    pub fn proc_macro_safe_name(&self) -> Ident {
        format_ident!("{}", self.0.as_str())
    }

    pub fn add_width(&self, width: OperandWidth) -> Self {
        let mut name = self.0.to_string();
        name.push_str(width.to_string().as_str());
        VariantName(name)
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum FieldType {
    Mem(HashSet<OperandWidth>),
    Reg(RegisterType),
    Imm(HashSet<OperandWidth>),
    RelBR,
    Ptr,
    AGen,
}

#[derive(Clone, Debug)]
pub struct Field {
    pub field_type: FieldType,
    pub field_name: xed_operand_enum_t,
}

impl Field {
    pub fn field_available_widths(&self) -> HashSet<OperandWidth> {
        [match &self.field_type {
            FieldType::Mem(mem) => {
                return mem.clone();
            }
            FieldType::Reg(reg) => {
                return reg.widths();
            }
            FieldType::Imm(imm) => {
                return imm.clone();
            }
            FieldType::RelBR => {
                OperandWidth::_64
            }
            FieldType::Ptr => {
                OperandWidth::_64
            }
            FieldType::AGen => {
                OperandWidth::_64
            }
        }].into_iter().collect()
    }

    pub fn field_as_width_with_imm(&self, reg_width: OperandWidth, imm_width: OperandWidth) -> Field {
        let Self { field_type, field_name } = self;
        let field_type = match field_type {
            FieldType::Mem(inner) => {
                if inner.len() != 1 && inner.contains(&reg_width) {
                    FieldType::Mem([reg_width].into_iter().collect())
                } else {
                    field_type.clone()
                }
            }
            FieldType::Reg(inner) => {
                FieldType::Reg(inner.field_as_width(reg_width))
            }
            FieldType::Imm(inner) => {
                if inner.len() != 1 && inner.contains(&imm_width) {
                    FieldType::Imm([imm_width].into_iter().collect())
                } else {
                    todo!()
                }
            }
            FieldType::RelBR => FieldType::RelBR,
            FieldType::Ptr => FieldType::Ptr,
            FieldType::AGen => FieldType::AGen
        };
        Self { field_type, field_name: *field_name }
    }

    pub fn field_as_width(&self, width: OperandWidth) -> Self {
        let Self { field_type, field_name } = self;
        let field_type = match field_type {
            FieldType::Mem(inner) => {
                if inner.len() != 1 && inner.contains(&width) {
                    FieldType::Mem([width].into_iter().collect())
                } else {
                    field_type.clone()
                }
            }
            FieldType::Reg(inner) => {
                FieldType::Reg(inner.field_as_width(width))
            }
            FieldType::Imm(inner) => {
                if inner.len() != 1 && inner.contains(&width) {
                    FieldType::Imm([width].into_iter().collect())
                } else {
                    field_type.clone()
                }
            }
            FieldType::RelBR => FieldType::RelBR,
            FieldType::Ptr => FieldType::Ptr,
            FieldType::AGen => FieldType::AGen
        };
        Self { field_type, field_name: *field_name }
    }

    pub fn field_is_multi_width(&self) -> bool {
        match &self.field_type {
            FieldType::Mem(mem) => {
                if mem.len() == 1 {
                    return false;
                }
            }
            FieldType::Reg(reg) => {
                match reg {
                    RegisterType::Multiple(_) => {}
                    _ => return false
                }
            }
            FieldType::Imm(imm) => if imm.len() == 1 {
                return false;
            }
            FieldType::RelBR => return false,
            FieldType::Ptr => return false,
            FieldType::AGen => return false,
        }
        let split_len = self.clone().field_available_widths().len();
        split_len > 1
    }
}

#[derive(Clone, Debug)]
pub struct Variant {
    pub operands: HashMap<usize, Field>,
    pub iform: xed_iform_enum_t,
}

#[derive(Clone, Debug)]
pub struct TopLevelInstruction {
    pub iclass: xed_iclass_enum_t,
    pub variants: HashMap<xed_iform_enum_t,HashMap<VariantName, Vec<Variant>>>
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
                let variant_name = VariantName::new(
                    CStr::from_ptr(xed_iform_enum_t2str(iform_i))
                        .to_str()
                        .unwrap()
                        .to_string(),
                );
                let instruction_name = TopLevelInstructionName::new(
                    CStr::from_ptr(xed_iclass_enum_t2str(xed_iform_to_iclass(iform_i)))
                        .to_str()
                        .unwrap()
                        .to_string(),
                );
                let variants = &mut res
                    .entry(instruction_name)
                    .or_insert_with(|| TopLevelInstruction {
                        iclass: xed_inst_iclass(instruction_table_elem),
                        variants: Default::default(),
                    })
                    .variants
                    .entry(iform_i)
                    .or_default()
                    .entry(variant_name.clone())
                    .or_default();
                variants.push(Variant {
                    operands: Default::default(),
                    iform: iform_i,
                });
                let variant_fields: &mut HashMap<usize, Field> = &mut variants.last_mut().unwrap().operands;
                let number_of_operands = xed_inst_noperands(instruction_table_elem);
                let mut visible_field_i = 0;
                for operand_i in 0..number_of_operands {
                    let operand = xed_inst_operand(instruction_table_elem, operand_i);
                    let visibility = xed_operand_operand_visibility(operand);
                    if visibility == XED_OPVIS_EXPLICIT {
                        let operand_name = OperandName::new(xed_operand_name(operand));
                        let operand_width = XedOperandWidth::new(xed_operand_width(operand));
                        let operand_type = XedOperandType::new(xed_operand_type(operand));
                        let non_terminal_name =
                            NonTerminalType::new(xed_operand_nonterminal_name(operand));
                        let field_type = match operand_name {
                            OperandName::MEM0 => FieldType::Mem(OperandWidth::width_for_mem(operand_width.unwrap())),
                            OperandName::REG0
                            | OperandName::REG1
                            | OperandName::REG2
                            | OperandName::REG3 => {
                                if let XedOperandType::NT_LOOKUP_FN
                                | XedOperandType::NT_LOOKUP_FN2
                                | XedOperandType::NT_LOOKUP_FN4 = operand_type
                                {
                                    let non_terminal_name = non_terminal_name.unwrap();
                                    match non_terminal_name {
                                        NonTerminalType::X87 => {
                                            FieldType::Reg(RegisterType::AllFloat)
                                        }
                                        NonTerminalType::SEG | NonTerminalType::SEG_MOV => {
                                            FieldType::Reg(RegisterType::AllSegment)
                                        }
                                        NonTerminalType::GPR8_R
                                        | NonTerminalType::GPRV_R
                                        | NonTerminalType::GPRV_B
                                        | NonTerminalType::GPRV_SB
                                        | NonTerminalType::GPR8_B
                                        | NonTerminalType::GPR16_B
                                        | NonTerminalType::GPR8_SB
                                        | NonTerminalType::GPR16_R
                                        | NonTerminalType::GPRZ_B
                                        | NonTerminalType::GPRZ_R
                                        | NonTerminalType::GPR32_B
                                        | NonTerminalType::GPR64_B
                                        | NonTerminalType::GPR32_R
                                        | NonTerminalType::GPR64_R
                                        | NonTerminalType::GPRY_R
                                        | NonTerminalType::GPRY_B
                                        | NonTerminalType::VGPR32_R
                                        | NonTerminalType::VGPRY_R
                                        | NonTerminalType::VGPR32_B
                                        | NonTerminalType::VGPRY_B
                                        | NonTerminalType::VGPR32_N
                                        | NonTerminalType::VGPRY_N
                                        | NonTerminalType::VGPR64_R
                                        | NonTerminalType::VGPR64_N
                                        | NonTerminalType::VGPR64_B => {
                                            match operand_width.unwrap() {
                                                XedOperandWidth::B => {
                                                    FieldType::Reg(RegisterType::AllGP8)
                                                }
                                                XedOperandWidth::V => {
                                                    FieldType::Reg(RegisterType::Multiple([
                                                        RegisterType::AllGP64WithRIP,
                                                        RegisterType::AllGP32WithRIP,
                                                        RegisterType::AllGP16WithRIP,
                                                    ].into_iter().collect()))
                                                }
                                                XedOperandWidth::Z => {
                                                    FieldType::Reg(RegisterType::Multiple([
                                                        RegisterType::AllGP64WithRIP,
                                                        RegisterType::AllGP32WithRIP,
                                                        RegisterType::AllGP16WithRIP,
                                                    ].into_iter().collect()))
                                                }
                                                XedOperandWidth::Y => {
                                                    FieldType::Reg(RegisterType::Multiple([
                                                        RegisterType::AllGP64WithRIP,
                                                        RegisterType::AllGP32WithRIP,
                                                    ].into_iter().collect()))
                                                }
                                                XedOperandWidth::W => {
                                                    FieldType::Reg(RegisterType::AllGP16WithRIP)
                                                }
                                                XedOperandWidth::D => {
                                                    FieldType::Reg(RegisterType::AllGP32WithRIP)
                                                }
                                                XedOperandWidth::Q => {
                                                    FieldType::Reg(RegisterType::AllGP64WithRIP)
                                                }
                                                width => todo!("{width:?}"),
                                            }
                                        }
                                        NonTerminalType::A_GPR_R | NonTerminalType::A_GPR_B => {
                                            FieldType::Reg(RegisterType::Multiple([
                                                RegisterType::AllGP64WithRIP,
                                                RegisterType::AllGP32WithRIP,
                                                RegisterType::AllGP16WithRIP,
                                            ].into_iter().collect()))
                                        }
                                        NonTerminalType::MMX_B | NonTerminalType::MMX_R => {
                                            FieldType::Reg(RegisterType::AllMmx)
                                        }
                                        NonTerminalType::XMM_B
                                        | NonTerminalType::XMM_R
                                        | NonTerminalType::XMM_R3
                                        | NonTerminalType::XMM_N
                                        | NonTerminalType::XMM_N3
                                        | NonTerminalType::XMM_SE
                                        | NonTerminalType::XMM_B3 => {
                                            FieldType::Reg(RegisterType::AllXmm32)
                                        }
                                        NonTerminalType::YMM_B
                                        | NonTerminalType::YMM_R
                                        | NonTerminalType::YMM_N
                                        | NonTerminalType::YMM_SE
                                        | NonTerminalType::YMM_R3
                                        | NonTerminalType::YMM_N3
                                        | NonTerminalType::YMM_B3 => {
                                            FieldType::Reg(RegisterType::AllYmm32)
                                        }
                                        NonTerminalType::ZMM_R3
                                        | NonTerminalType::ZMM_N3
                                        | NonTerminalType::ZMM_B3 => {
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
                                        NonTerminalType::MASK_R
                                        | NonTerminalType::MASK_N
                                        | NonTerminalType::MASK_B
                                        | NonTerminalType::MASK1
                                        | NonTerminalType::MASKNOT0 => {
                                            FieldType::Reg(RegisterType::AllMask)
                                        }
                                        NonTerminalType::TMM_R
                                        | NonTerminalType::TMM_N
                                        | NonTerminalType::TMM_B => {
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
                                FieldType::Imm(OperandWidth::width_for_imm(operand_width.unwrap()))
                            }
                            OperandName::RELBR => FieldType::RelBR,
                            OperandName::PTR => FieldType::Ptr,
                            OperandName::AGEN => FieldType::AGen,
                            other => {
                                todo!("{other:?}")
                            }
                        };
                        variant_fields.insert(
                            visible_field_i,
                            Field {
                                field_type,
                                field_name: xed_operand_name(operand),
                            },
                        );
                        visible_field_i += 1;
                    }
                }
            }
        }
        spread_multi_width_variants(res)
    })
}

pub fn spread_multi_width_variants(before: HashMap<TopLevelInstructionName, TopLevelInstruction>) -> HashMap<TopLevelInstructionName, TopLevelInstruction> {
    let mut res = HashMap::new();
    for (name, top_level_instr) in before.iter().sorted_by_key(|(key, _)| key.0.clone()) {
        let mut variants: HashMap<xed_iform_enum_t, HashMap<VariantName, Vec<Variant>>> = HashMap::new();
        for (iform, variant) in top_level_instr.variants.iter().sorted_by_key(|(key, _)| **key) {
            for (variant_name, variants_in) in variant.iter().sorted_by_key(|(variant_name,_)|variant_name.clone()) {
                for variant in variants_in.iter() {
                    if variant.operands.iter().any(|(_, operand)| operand.field_is_multi_width()) {
                        fn operands_to_width(variant: &Variant, width: OperandWidth) -> Variant {
                            Variant {
                                operands: variant.operands.iter().map(|(i, field)| (*i, field.field_as_width(width))).collect(),
                                iform: variant.iform,
                            }
                        }
                        fn operands_to_width_many(variant: &Variant, width: Vec<OperandWidth>) -> Variant {
                            let mut width = width.into_iter();
                            Variant {
                                operands: variant.operands.iter().sorted_by_key(|(i, _)| **i).map(|(i, field)| {
                                    if let Some(width) = width.next() {
                                        (*i, field.field_as_width(width))
                                    } else {
                                        (*i, field.clone())
                                    }
                                }).collect(),
                                iform: variant.iform,
                            }
                        }
                        fn operands_to_width_imm(variant: &Variant, width: OperandWidth, imm_width: OperandWidth) -> Variant {
                            Variant {
                                operands: variant.operands.iter().map(|(i, field)| (*i, field.field_as_width_with_imm(width, imm_width))).collect(),
                                iform: variant.iform,
                            }
                        }

                        let variant_name_ref = variant_name.0.replace(name.0.as_str(), "");
                        let variant_name_ref = variant_name_ref.as_str();
                        if variant_name_ref == "_GPRV_GPRV" ||
                            variant_name_ref == "_GPRV_GPRV_01" ||
                            variant_name_ref == "_GPRV_GPRV_03" ||
                            variant_name_ref == "_GPRV_GPRV_21" ||
                            variant_name_ref == "_GPRV_GPRV_23" ||
                            variant_name_ref == "_GPRV" ||
                            variant_name_ref == "_GPRV_GPR8" ||
                            variant_name_ref == "_GPRV_MEMB" ||
                            variant_name_ref == "_GPRV_MEMZ" ||
                            variant_name_ref == "_MEMV_IMMB" ||
                            variant_name_ref == "_MEMV" ||
                            variant_name_ref == "_MEMV_CL" ||
                            variant_name_ref == "_MEMV_CL_D3R4" ||
                            variant_name_ref == "_MEMV_CL_D3R6" ||
                            variant_name_ref == "_MEMV_IMMB_C1R4" ||
                            variant_name_ref == "_MEMV_IMMB_C1R6" ||
                            variant_name_ref == "_MEMV_ONE" ||
                            variant_name_ref == "_MEMV_ONE_D1R4" ||
                            variant_name_ref == "_MEMV_ONE_D1R6" ||
                            variant_name_ref == "_GPRV_GPR16" ||
                            variant_name_ref == "_GPRA_MEM" ||
                            variant_name_ref == "_GPRV_FFR1" ||
                            variant_name_ref == "_GPRV_FFR0" ||
                            variant_name_ref == "_GPRV_40" ||
                            variant_name_ref == "_GPRV_CL" {
                            variants.entry(*iform).or_default().entry(variant_name.add_width(OperandWidth::_16)).or_default().push(operands_to_width(variant, OperandWidth::_16));
                            variants.entry(*iform).or_default().entry(variant_name.add_width(OperandWidth::_32)).or_default().push(operands_to_width(variant, OperandWidth::_32));
                            variants.entry(*iform).or_default().entry(variant_name.add_width(OperandWidth::_64)).or_default().push(operands_to_width(variant, OperandWidth::_64));
                        } else if variant_name_ref == "_GPRV_GPRV_11" ||
                            variant_name_ref == "_GPRV_GPRV_13" ||
                            variant_name_ref == "_GPRV_GPRV_39" ||
                            variant_name_ref == "_GPRV_GPRV_3B" ||
                            variant_name_ref == "_GPRV_GPRV_89" ||
                            variant_name_ref == "_GPRV_GPRV_8B" ||
                            variant_name_ref == "_GPRV_GPRV_IMMB" ||
                            variant_name_ref == "_GPRV_IMMB_C1R4" ||
                            variant_name_ref == "_GPRV_IMMB_C1R6" ||
                            variant_name_ref == "_GPRV_MEMV_IMMB" ||
                            variant_name_ref == "_GPRV_IMMB" ||
                            variant_name_ref == "_GPRV_MEMV" ||
                            variant_name_ref == "_GPRV_MEMW" ||
                            variant_name_ref == "_GPRV_MEMP2" ||
                            variant_name_ref == "_GPRV_AGEN" ||
                            variant_name_ref == "_GPRV_SEG" ||
                            variant_name_ref == "_GPRV_0F18R0" ||
                            variant_name_ref == "_GPRV_0F18R1" ||
                            variant_name_ref == "_GPRV_0F18R2" ||
                            variant_name_ref == "_GPRV_0F18R3" ||
                            variant_name_ref == "_GPRV_0F18R4" ||
                            variant_name_ref == "_GPRV_0F18R5" ||
                            variant_name_ref == "_GPRV_0F18R6" ||
                            variant_name_ref == "_GPRV_0F18R7" ||
                            variant_name_ref == "_GPRV_GPRV_0F0D" ||
                            variant_name_ref == "_GPRV_GPRV_0F19" ||
                            variant_name_ref == "_GPRV_GPRV_0F1A" ||
                            variant_name_ref == "_GPRV_GPRV_0F1B" ||
                            variant_name_ref == "_GPRV_GPRV_0F1C" ||
                            variant_name_ref == "_GPRV_GPRV_0F1D" ||
                            variant_name_ref == "_GPRV_GPRV_0F1E" ||
                            variant_name_ref == "_GPRV_GPRV_0F1F" ||
                            variant_name_ref == "_GPRV_GPRV_0F1F" ||
                            variant_name_ref == "_GPRV_MEMV_0F1A" ||
                            variant_name_ref == "_GPRV_MEM_0F1B" ||
                            variant_name_ref == "_MEMV_0F18R4" ||
                            variant_name_ref == "_MEMV_0F18R5" ||
                            variant_name_ref == "_MEMV_0F18R6" ||
                            variant_name_ref == "_MEMV_0F18R7" ||
                            variant_name_ref == "_MEMV_GPRV_0F19" ||
                            variant_name_ref == "_MEMV_GPRV_0F1C" ||
                            variant_name_ref == "_MEMV_GPRV_0F1D" ||
                            variant_name_ref == "_MEMV_GPRV_0F1E" ||
                            variant_name_ref == "_MEMV_GPRV_0F1F" ||
                            variant_name_ref == "_MEMV_ORAX" ||
                            variant_name_ref == "_ORAX_MEMV" ||
                            variant_name_ref == "_GPRV_GPRV_09" ||
                            variant_name_ref == "_GPRV_GPRV_0B" ||
                            variant_name_ref == "_GPRV_GPRV_31" ||
                            variant_name_ref == "_GPRV_GPRV_33" ||
                            variant_name_ref == "_GPRV_58" ||
                            variant_name_ref == "_GPRV_50" ||
                            variant_name_ref == "_GPRV_8F" ||
                            variant_name_ref == "_GPRV_ONE" ||
                            variant_name_ref == "_GPRV_FFR6" ||
                            variant_name_ref == "_GPRV_GPRV_19" ||
                            variant_name_ref == "_GPRV_GPRV_1B" ||
                            variant_name_ref == "_GPRV_CL_D3R4" ||
                            variant_name_ref == "_GPRV_CL_D3R6" ||
                            variant_name_ref == "_GPRV_ONE_D1R4" ||
                            variant_name_ref == "_GPRV_ONE_D1R6" ||
                            variant_name_ref == "_GPRV_GPRV_CL" ||
                            variant_name_ref == "_MEMV_GPRV_CL" ||
                            variant_name_ref == "_MEMV_GPRV_IMMB" ||
                            variant_name_ref == "_GPRV_GPRV_29" ||
                            variant_name_ref == "_GPRV_GPRV_2B" ||
                            variant_name_ref == "_GPRV_ORAX" ||
                            variant_name_ref == "_GPRA" ||
                            variant_name_ref == "_MEMV_GPRV" {
                            variants.entry(*iform).or_default().entry(variant_name.add_width(OperandWidth::_16)).or_default().push(operands_to_width(variant, OperandWidth::_16));
                            variants.entry(*iform).or_default().entry(variant_name.add_width(OperandWidth::_32)).or_default().push(operands_to_width(variant, OperandWidth::_32));
                            variants.entry(*iform).or_default().entry(variant_name.add_width(OperandWidth::_64)).or_default().push(operands_to_width(variant, OperandWidth::_64));
                        } else if variant_name_ref == "_GPRV_IMMZ" ||
                            variant_name_ref == "_MEMV_IMMZ" ||
                            variant_name_ref == "_MEMV_IMMZ_F7R0" ||
                            variant_name_ref == "_MEMV_IMMZ_F7R1" ||
                            variant_name_ref == "_GPRV_GPRV_IMMZ" ||
                            variant_name_ref == "_GPRV_MEMV_IMMZ" ||
                            variant_name_ref == "_GPRV_IMMZ_F7R0" ||
                            variant_name_ref == "_GPRV_IMMZ_F7R1" ||
                            variant_name_ref == "_ORAX_IMMZ" ||
                            variant_name_ref == "_IMMZ" ||
                            variant_name_ref == "_GPRV_MEMA16" ||
                            variant_name_ref == "_AX_IMMZ" ||
                            variant_name_ref == "_GPRV_MEMA32" {
                            variants.entry(*iform).or_default().entry(variant_name.add_width(OperandWidth::_16)).or_default().push(operands_to_width_imm(variant, OperandWidth::_16, OperandWidth::_16));
                            variants.entry(*iform).or_default().entry(variant_name.add_width(OperandWidth::_32)).or_default().push(operands_to_width_imm(variant, OperandWidth::_32, OperandWidth::_32));
                            variants.entry(*iform).or_default().entry(variant_name.add_width(OperandWidth::_64)).or_default().push(operands_to_width_imm(variant, OperandWidth::_64, OperandWidth::_32));
                        } else if variant_name_ref == "_VGPRYY_MEMY_IMMD" ||
                            variant_name_ref == "_VGPRYY_VGPRYY_IMMD" ||
                            variant_name_ref == "_GPRYY_GPR8B" ||
                            variant_name_ref == "_VGPRYY_MEMY" ||
                            variant_name_ref == "_GPRYY_MEMB" ||
                            variant_name_ref == "_VGPRYY" ||
                            variant_name_ref == "_GPRY" ||
                            variant_name_ref == "_GPRY" ||
                            variant_name_ref == "_MEMY" ||
                            variant_name_ref == "_GPRA_MEMU32" ||
                            variant_name_ref == "_VGPRYY_VGPRYY" {
                            variants.entry(*iform).or_default().entry(variant_name.add_width(OperandWidth::_32)).or_default().push(operands_to_width(variant, OperandWidth::_32));
                            variants.entry(*iform).or_default().entry(variant_name.add_width(OperandWidth::_64)).or_default().push(operands_to_width(variant, OperandWidth::_64));
                        } else if variant_name_ref == "_GPRYY_GPRV" || variant_name_ref == "_GPRYY_MEMV" {
                            variants.entry(*iform).or_default().entry(variant_name.add_width(OperandWidth::_32).add_width(OperandWidth::_32)).or_default().push(operands_to_width_many(variant, vec![OperandWidth::_32, OperandWidth::_32]));
                            variants.entry(*iform).or_default().entry(variant_name.add_width(OperandWidth::_32).add_width(OperandWidth::_16)).or_default().push(operands_to_width_many(variant, vec![OperandWidth::_32, OperandWidth::_16]));
                            variants.entry(*iform).or_default().entry(variant_name.add_width(OperandWidth::_64).add_width(OperandWidth::_64)).or_default().push(operands_to_width_many(variant, vec![OperandWidth::_64, OperandWidth::_64]));
                        } else if variant_name_ref == "_GPRV_48" || variant_name_ref == "_GPRZ_MEMP" {
                            variants.entry(*iform).or_default().entry(variant_name.add_width(OperandWidth::_16)).or_default().push(operands_to_width(variant, OperandWidth::_16));
                            variants.entry(*iform).or_default().entry(variant_name.add_width(OperandWidth::_32)).or_default().push(operands_to_width(variant, OperandWidth::_32));
                        } else if variant_name_ref == "_GPRV_GPRZ" {
                            variants.entry(*iform).or_default().entry(variant_name.add_width(OperandWidth::_16).add_width(OperandWidth::_16)).or_default().push(operands_to_width_many(variant, vec![OperandWidth::_16, OperandWidth::_16]));
                            variants.entry(*iform).or_default().entry(variant_name.add_width(OperandWidth::_32).add_width(OperandWidth::_32)).or_default().push(operands_to_width_many(variant, vec![OperandWidth::_32, OperandWidth::_32]));
                            variants.entry(*iform).or_default().entry(variant_name.add_width(OperandWidth::_64).add_width(OperandWidth::_32)).or_default().push(operands_to_width_many(variant, vec![OperandWidth::_64, OperandWidth::_32]));
                        } else if variant_name_ref == "_VGPRYY_MEMD_IMMD" || variant_name_ref == "_VGPRYY_VGPR32Y_IMMD" {
                            variants.entry(*iform).or_default().entry(variant_name.add_width(OperandWidth::_32).add_width(OperandWidth::_32)).or_default().push(operands_to_width_many(variant, vec![OperandWidth::_32, OperandWidth::_32]));
                            variants.entry(*iform).or_default().entry(variant_name.add_width(OperandWidth::_64).add_width(OperandWidth::_32)).or_default().push(operands_to_width_many(variant, vec![OperandWidth::_64, OperandWidth::_32]));
                        } else if variant_name_ref == "_GPRV_IMMV" {
                            variants.entry(*iform).or_default().entry(variant_name.add_width(OperandWidth::_16)).or_default().push(operands_to_width_imm(variant, OperandWidth::_16, OperandWidth::_16));
                            variants.entry(*iform).or_default().entry(variant_name.add_width(OperandWidth::_32)).or_default().push(operands_to_width_imm(variant, OperandWidth::_32, OperandWidth::_32));
                            variants.entry(*iform).or_default().entry(variant_name.add_width(OperandWidth::_64)).or_default().push(operands_to_width_imm(variant, OperandWidth::_64, OperandWidth::_64));
                        } else if variant_name_ref == "_XMMPD_MEMPD" ||
                            variant_name_ref == "_XMMPS_MEMPS" ||
                            variant_name_ref == "_XMMXUD_MEMXUD" ||
                            variant_name_ref == "_XMMPD_MEMPD_IMMB" ||
                            variant_name_ref == "_XMMPS_MEMPS_IMMB" ||
                            variant_name_ref == "_XMMPS_MEMPD" ||
                            variant_name_ref == "_XMMDQ_MEMPD" ||
                            variant_name_ref == "_XMMDQ_MEMPS" ||
                            variant_name_ref == "_MMXQ_MEMPD" ||
                            variant_name_ref == "_MEMPD_XMMPD" ||
                            variant_name_ref == "_MEMPS_XMMPS" ||
                            variant_name_ref == "_XMMXUQ_MEMXUQ" {
                            variants.entry(*iform).or_default().entry(variant_name.add_width(OperandWidth::_128)).or_default().push(operands_to_width(variant, OperandWidth::_128));
                        } else if variant_name_ref == "_MEMP2" {
                            variants.entry(*iform).or_default().entry(variant_name.add_width(OperandWidth::_16)).or_default().push(operands_to_width(variant, OperandWidth::_16));
                            variants.entry(*iform).or_default().entry(variant_name.add_width(OperandWidth::_32)).or_default().push(operands_to_width(variant, OperandWidth::_32));
                        } else if variant_name_ref == "_MEMS" {
                            variants.entry(*iform).or_default().entry(variant_name.add_width(OperandWidth::_32)).or_default().push(operands_to_width(variant, OperandWidth::_32));
                        } else if variant_name_ref == "_MEMS64" {
                            variants.entry(*iform).or_default().entry(variant_name.add_width(OperandWidth::_64)).or_default().push(operands_to_width(variant, OperandWidth::_64));
                        } else {
                            dbg!(variant_name);
                            dbg!(variant_name_ref);
                            todo!()
                        }
                    } else {
                        let variant_name = variant_name.clone();
                        variants.entry(*iform).or_default().entry(variant_name).or_default().push(variant.clone());
                    }
                }
            }
        }
        let top_level_instr_res = TopLevelInstruction { iclass: top_level_instr.iclass, variants };
        res.insert(name.clone(), top_level_instr_res.clone());
    }
    res
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
