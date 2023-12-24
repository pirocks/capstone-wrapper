use std::collections::HashSet;
use std::ffi::c_uint;

use itertools::Itertools;
use proc_macro2::Ident;
use quote::{format_ident, quote};

use wrapper_common::registers::OperandWidth;
use xed_wrapper::{FieldType, Variant, xed_data};

//todo have a pattern expander which expands specific instances of MEMZ/IMMV

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

// fn imm_width_to_indent(width: XedOperandWidth) -> Ident {
//     match width {
//         XedOperandWidth::B => format_ident!("Imm8"),
//         XedOperandWidth::W => format_ident!("Imm16"),
//         XedOperandWidth::D => format_ident!("Imm32"),
//         XedOperandWidth::Z | XedOperandWidth::V => format_ident!("Immediate"),
//         width => todo!("{width:?}"),
//     }
// }

fn imm_width_to_indent(width: HashSet<OperandWidth>) -> Ident {
    if width.len() == 1 {
        match width.into_iter().next().unwrap() {
            OperandWidth::_8 => format_ident!("Imm8"),
            OperandWidth::_16 => format_ident!("Imm16"),
            OperandWidth::_32 => format_ident!("Imm32"),
            OperandWidth::_64 => format_ident!("Imm64"),
            /*OperandWidth::_ | XedOperandWidth::V => format_ident!("Immediate"),*/
            width => todo!("{width:?}"),
        }
    } else {
        todo!()
    }
}

#[proc_macro]
pub fn instruction_enums(_: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let data = xed_data();
    let mut enums = vec![];
    for (instruction_name, top_level_instruction) in data {
        let mut instruction_variant_names1 = vec![];
        let mut variant_field_names = vec![];
        let mut variant_types = vec![];
        for (_iform, variants) in top_level_instruction.variants.iter().sorted_by_key(|(iform, _)| *iform) {
            for (variant_name, Variant { operands, iform:_, effective_operand_width:_ }) in variants.iter().sorted_by_key(|(vn, _)| *vn) {
                instruction_variant_names1.push(variant_name.proc_macro_safe_name());
                variant_field_names.push(vec![]);
                variant_types.push(vec![]);
                for (variant_field_index, variant_type) in operands.iter().sorted_by_key(|(i, _)| *i) {
                    variant_field_names
                        .last_mut()
                        .unwrap()
                        .push(format_ident!("operand_{}", *variant_field_index));
                    variant_types
                        .last_mut()
                        .unwrap()
                        .push(match &variant_type.field_type {
                            FieldType::Mem(_) => format_ident!("MemoryOperands"),
                            FieldType::Reg(reg) => reg.type_to_rust_type(),
                            FieldType::Imm(imm) => imm_width_to_indent(imm.clone()),
                            FieldType::RelBR => format_ident!("RelativeBr"),
                            FieldType::Ptr => format_ident!("Immediate"),
                            FieldType::AGen => format_ident!("MemoryOperands"),
                        })
                }
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
        use wrapper_common::memory_operand::*;
        #(#enums)*
    })
}

#[proc_macro]
pub fn enum_from_xed(_: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let data = xed_data();
    let mut impls = vec![];
    let mut iclass_nums = vec![];
    let mut iclass_names1 = vec![];
    let mut iclass_names2 = vec![];
    for (instruction_name, top_level_instruction) in data {
        let instruction_enum_name = instruction_name.proc_macro_safe_name();
        iclass_nums.push(top_level_instruction.iclass);
        iclass_names1.push(instruction_enum_name.clone());
        iclass_names2.push(instruction_enum_name.clone());
        let mut closure_vec = vec![];
        let mut iform_nums = vec![];
        //todo regroup by iform.
        for (iform, variants_in) in top_level_instruction.variants.iter() {
            iform_nums.push(*iform);
            let mut closures = vec![];
            for (variant_name, variant_in) in variants_in.iter().sorted_by_key(|(vn, _)| *vn) {
                let iform_name = variant_name.proc_macro_safe_name();
                let mut variant_field_names = vec![];
                let mut fields = vec![];
                let Variant { operands, iform: _, effective_operand_width:_ } = variant_in;
                let mut second_immediate = false;
                let mut mem_idx = 0u32;
                for (operand_i, operand) in operands.iter().sorted_by_key(|(i, _)| *i) {
                    variant_field_names
                        .push(format_ident!("operand_{}", operand_i));
                    let operand_name = operand.field_name;
                    fields.push(match &operand.field_type {
                        FieldType::Mem(_) | FieldType::AGen => {
                            quote! {
                                let instr_template = xed_decoded_inst_inst(xed);
                                MemoryOperands::from_xed(xed, instr_template, #mem_idx)
                            }
                        }
                        FieldType::Reg(register_type) => {
                            let reg_type = register_type.type_to_rust_type();
                            if reg_type == format_ident!("GeneralReg") {
                                quote! {
                                    let reg = xed_decoded_inst_get_reg(xed,#operand_name);
                                    let width = xed_decoded_inst_get_operand_width(xed);
                                    #reg_type::try_new(reg, Some(width))?
                                }
                            } else {
                                quote! {
                                    let reg = xed_decoded_inst_get_reg(xed,#operand_name);
                                    #reg_type::try_new(reg)?
                                }
                            }
                        }
                        FieldType::Ptr => {
                            quote! {
                                Immediate::from_xed(xed, #second_immediate)
                            }
                        }
                        FieldType::Imm(width) => {
                            let type_ = imm_width_to_indent(width.clone());
                            quote! {
                                #type_::from_xed(xed, #second_immediate)
                            }
                        }
                        FieldType::RelBR => {
                            quote! {
                                RelativeBr::from_xed(xed)
                            }
                        }
                    });
                    if let FieldType::Mem(_) | FieldType::AGen = &operand.field_type {
                        mem_idx += 1;
                    }
                    if let FieldType::Ptr | FieldType::Imm(_) = &operand.field_type {
                        second_immediate = true;
                    }
                }
                closures.push(quote! {
                    (||{Some(Self::#iform_name {
                        #(#variant_field_names: { #fields }),*
                    })})
                });
            }
            closure_vec.push(quote! {
                vec![#(Box::new(#closures) as Box<dyn FnOnce() -> Option<Self>>),*].into_iter().find_map(|inner|inner()).unwrap()
            });
        }
        impls.push(quote! {
            impl #instruction_enum_name {
                pub fn from_xed(xed: *const xed_decoded_inst_t) -> Self {
                    unsafe {
                        let iform = xed_decoded_inst_get_iform_enum(xed);
                        match iform {
                            #(#iform_nums => {
                                #closure_vec
                            }),*,
                            _ => panic!()
                        }
                    }
                }
            }
        })
    }

    proc_macro::TokenStream::from(quote! {
        use xed_sys::{xed_decoded_inst_t,xed_decoded_inst_get_reg, xed_decoded_inst_get_iform_enum, xed_decoded_inst_inst, xed_decoded_inst_get_iclass, xed_iclass_enum_t2str, xed_decoded_inst_get_operand_width, xed_tables_init};
        use xed_wrapper::operands::*;
        use std::ffi::CStr;

        impl X86Instruction {
            pub fn from_xed(xed: *const xed_decoded_inst_t) -> Self {
                crate::START.call_once(||{
                    unsafe { xed_tables_init(); }
                });
                let iclass = unsafe { xed_decoded_inst_get_iclass(xed) };
                match iclass {
                    #(#iclass_nums => Self::#iclass_names1(#iclass_names2::from_xed(xed))),*,
                    other => panic!("{other} {}",unsafe { CStr::from_ptr(xed_iclass_enum_t2str(other)).to_str().unwrap() })
                }
            }
        }

        #(#impls)*
    })
}

#[proc_macro]
pub fn enum_to_xed(_: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let data = xed_data();
    let mut impls = vec![];
    let mut iclass_names1 = vec![];
    let mut iclass_names2 = vec![];
    for (instruction_name, top_level_instruction) in data {
        let instruction_enum_name = instruction_name.proc_macro_safe_name();
        iclass_names1.push(instruction_enum_name.clone());
        iclass_names2.push(instruction_enum_name.clone());
        let mut iform_names = vec![];
        let mut variant_field_names = vec![];
        let mut variant_constructors = vec![];
        let iclass = top_level_instruction.iclass;
        for (_iform, variants) in top_level_instruction.variants.iter() {
            for (variant_name, Variant { operands, iform: _, effective_operand_width }) in variants.iter() {
                iform_names.push(variant_name.proc_macro_safe_name());
                variant_field_names.push(vec![]);
                let mut variant_field_constructors = vec![];
                let mut second_immediate = false;
                let effective_operand_width = effective_operand_width;
                for (operand_i, operand) in operands.iter().sorted_by_key(|(i, _)| **i) {
                    let variant_ident = format_ident!("operand_{}", operand_i);
                    variant_field_names
                        .last_mut()
                        .unwrap()
                        .push(variant_ident.clone());
                    match &operand.field_type {
                        FieldType::Mem(mem) => {
                            assert_eq!(mem.len(), 1);
                            let mem_xed = mem.into_iter().next().unwrap().to_xed_width_bits() as c_uint;
                            variant_field_constructors.push(quote! {
                                if let MemoryOperands::SIBAddressing { segment, base, scale, index, disp, disp_width } = #variant_ident.clone(){
                                    xed_mem_gbisd(
                                        segment.as_ref().map(|seg|seg.to_xed()).unwrap_or(0),
                                        base.to_xed(),
                                        index.as_ref().map(|index|index.to_xed()).unwrap_or(0),
                                        scale.to_xed(),
                                        xed_disp(disp, disp_width),
                                        #mem_xed)
                                } else {
                                    todo!()
                                }
                            });
                        }
                        FieldType::Reg(_) => {
                            variant_field_constructors.push(quote! {
                            let reg = #variant_ident.to_xed();
                            xed_reg(reg)
                        });
                        }
                        FieldType::Imm(_) => {
                            if second_immediate {
                                variant_field_constructors.push(quote! {
                                xed_imm1(#variant_ident.to_xed_byte_identifier())
                            });
                            } else {
                                variant_field_constructors.push(quote! {
                                #variant_ident.to_xed()
                            });
                            }
                            second_immediate = true;
                        }
                        FieldType::RelBR => {
                            variant_field_constructors.push(quote! {
                            todo!()
                        });
                            second_immediate = true;
                        }
                        FieldType::Ptr => {
                            variant_field_constructors.push(quote! {
                            todo!()
                        });
                            second_immediate = true;
                        }
                        FieldType::AGen => {
                            variant_field_constructors.push(quote! {
                            todo!()
                        })
                        }
                    }
                }
                let length = operands.len();
                let xed_inst = format_ident!("xed_inst{length}");
                variant_constructors.push(quote! {
                    #xed_inst(encoder_inst.as_mut_ptr(), xed_state.assume_init_ref().clone(), #iclass,#effective_operand_width,  #( { #variant_field_constructors }),* );
                });
            }
        }
        impls.push(quote! {
            impl #instruction_enum_name {
                pub fn to_xed(&self, encode_context: &mut EncodeDecodeContext) -> xed_encoder_request_t {
                    crate::START.call_once(||{
                        unsafe { xed_tables_init(); }
                    });

                    let mut encoder_request: MaybeUninit<xed_encoder_request_t> = MaybeUninit::zeroed();
                    let mut encoder_inst: MaybeUninit<xed_encoder_instruction_t> = MaybeUninit::zeroed();
                    let mut xed_state: MaybeUninit<xed_state_t> = encode_context.xed_state.clone();

                    unsafe {
                        match self {
                            #(Self::#iform_names { #(#variant_field_names),* } => {
                                #variant_constructors
                            }),*,
                        }
                        xed_encoder_request_zero_set_mode(encoder_request.as_mut_ptr(), xed_state.as_ptr());
                        let convert_ok = xed_convert_to_encoder_request(encoder_request.as_mut_ptr(), encoder_inst.as_mut_ptr()) != 0;
                        if !convert_ok {
                            panic!()
                        }

                        encoder_request.assume_init()
                    }
                }
            }
        })
    }
    proc_macro::TokenStream::from(quote! {

        use xed_sys::{xed_mem_gbisd, xed_imm1, xed_imm0, xed_reg,xed_inst5, xed_inst4,xed_inst3, xed_inst2, xed_inst1, xed_inst0, xed_encoder_instruction_t, xed_convert_to_encoder_request, xed_encoder_request_zero_set_mode, xed_encoder_request_t, xed_disp};

        #(#impls)*

        impl X86Instruction {
            pub fn to_xed(&self, encode_context: &mut EncodeDecodeContext) -> xed_encoder_request_t {
                match self {
                    #(Self::#iclass_names1(inner) => inner.to_xed(encode_context)),*,
                    _ => panic!()
                }
            }
        }
    })
}


// #[proc_macro]
// pub fn arbitrary(_: proc_macro::TokenStream) -> proc_macro::TokenStream {
//     todo!()
// }