use itertools::Itertools;
use quote::{format_ident, quote};

use xed_wrapper::{FieldType, Variant, xed_data};

#[proc_macro]
pub fn top_level_instruction_enum(_: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let data = xed_data();

    let mut instruction_names = vec![];
    let mut instruction_enum_names = vec![];

    for (top_level_name, _) in data {
        instruction_names.push(top_level_name.proc_macro_safe_name());
        instruction_enum_names.push(top_level_name.proc_macro_safe_name());
    }

    let res = quote! {
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
        #[derive(enum_visitor::EnumVisitor)]
        pub enum X86Instruction{
            #(#instruction_names(#instruction_enum_names)),*
        }
    };
    proc_macro::TokenStream::from(res)
}

#[proc_macro]
pub fn instruction_enums(_: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let data = xed_data();
    let mut enums = vec![];
    for (instruction_name, top_level_instruction) in data {
        let mut instruction_variant_names1 = vec![];
        let mut variant_field_names = vec![];
        let mut variant_types = vec![];
        for (variant_name, variant_fields) in top_level_instruction.variants.iter() {
            instruction_variant_names1.push(variant_name.proc_macro_safe_name());
            variant_field_names.push(vec![]);
            variant_types.push(vec![]);
            for (variant_field_index, variant_type) in variant_fields.operands.iter().sorted_by_key(|(i, _)| *i) {
                variant_field_names.last_mut().unwrap().push(format_ident!("operand_{}", *variant_field_index));
                variant_types.last_mut().unwrap().push(match &variant_type.field_type {
                    FieldType::Mem(_) => format_ident!("MemoryOperands"),
                    FieldType::Reg(reg) => reg.type_to_rust_type(),
                    FieldType::Imm => format_ident!("Immediate"),
                    FieldType::RelBR => format_ident!("RelativeBr"),
                    FieldType::Ptr => format_ident!("Immediate"),
                    FieldType::AGen => format_ident!("MemoryOperands")
                })
            }
        }
        let instruction_enum_name = instruction_name.proc_macro_safe_name();
        enums.push(quote! {
            #[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
            #[derive(enum_visitor::EnumVisitor)]
            pub enum #instruction_enum_name {
                #(#instruction_variant_names1{
                    #(#variant_field_names: #variant_types),*
                }),*
            }
        });
    }
    proc_macro::TokenStream::from(quote! {
        use wrapper_common::registers::*;
        use xed_wrapper::operands::*;
        #(#enums)*
    })
}

#[proc_macro]
pub fn enum_from_xed(_: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let data = xed_data();
    let mut impls = vec![];
    for (instruction_name, top_level_instruction) in data {
        let instruction_enum_name = instruction_name.proc_macro_safe_name();
        let mut iform_nums = vec![];
        let mut iform_names = vec![];
        let mut variant_field_names = vec![];
        let mut variants = vec![];
        let mut mem_idx = 0u32;
        let mut second_immediate = false;
        for (variant_name, Variant { operands, iform }) in top_level_instruction.variants.iter() {
            variant_field_names.push(vec![]);
            variants.push(vec![]);
            iform_nums.push(iform);
            iform_names.push(variant_name.proc_macro_safe_name());
            for (operand_i, operand) in operands {
                variant_field_names.last_mut().unwrap().push(format_ident!("operand_{}", operand_i));
                let current_variants = variants.last_mut().unwrap();
                let operand_name = operand.field_name;
                match &operand.field_type {
                    FieldType::Mem(_) | FieldType::AGen => {
                        mem_idx += 1;
                        current_variants.push(quote! {
                            let instr_template = xed_decoded_inst_inst(xed);
                            MemoryOperands::from_xed(xed, instr_template, #mem_idx)
                        });
                    }
                    FieldType::Reg(register_type) => {
                        let reg_type = register_type.type_to_rust_type();
                        current_variants.push(quote! {
                                    let reg = xed_decoded_inst_get_reg(xed,#operand_name);
                                    #reg_type::try_new(reg).unwrap()
                                });
                    }
                    FieldType::Imm | FieldType::Ptr => {
                        current_variants.push(quote! {
                            Immediate::from_xed(xed, #second_immediate)
                        });
                        second_immediate = true;
                    }
                    FieldType::RelBR => {
                        current_variants.push(quote! {
                            RelativeBr::from_xed(xed)
                        });
                    }
                }
            }
        }
        impls.push(quote! {
            impl #instruction_enum_name {
                pub fn from_xed(xed: *const xed_decoded_inst_t) -> Self {
                    unsafe {
                        let iform = xed_decoded_inst_get_iform_enum(xed);
                        match iform {
                            #(#iform_nums => Self::#iform_names {
                                #(#variant_field_names: { #variants }),*
                            }),*,
                            _ => panic!()
                        }
                    }
                }
            }
        })
    }

    proc_macro::TokenStream::from(quote! {
        use xed_sys::{xed_decoded_inst_t,xed_decoded_inst_get_reg, xed_decoded_inst_get_iform_enum, xed_decoded_inst_inst};
        use xed_wrapper::operands::*;

        #(#impls)*
    })
}