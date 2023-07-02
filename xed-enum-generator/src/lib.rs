use quote::{format_ident, quote};
use xed_wrapper::{FieldType, xed_data};


#[proc_macro]
pub fn top_level_instruction_enum(_: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let data  = xed_data();

    let mut instruction_names = vec![];
    let mut instruction_enum_names = vec![];

    for (top_level_name, _) in data {
        instruction_names.push(top_level_name.proc_macro_safe_name());
        instruction_enum_names.push(top_level_name.proc_macro_safe_name());
    }

    let res = quote! {
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
        pub enum X86Instruction{
            #(#instruction_names(#instruction_enum_names)),*
        }
    };
    proc_macro::TokenStream::from(res)
}

#[proc_macro]
pub fn instruction_enums(_: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let data  = xed_data();
    let mut enums = vec![];
    for (instruction_name, variants) in data{
        let mut instruction_variant_names1 = vec![];
        let mut variant_field_names = vec![];
        let mut variant_types = vec![];
        for (variant_name, variant_fields) in variants {
            instruction_variant_names1.push(variant_name.proc_macro_safe_name());
            variant_field_names.push(vec![]);
            variant_types.push(vec![]);
            for (variant_field_index, variant_type) in variant_fields{
                variant_field_names.last_mut().unwrap().push(format_ident!("operand_{}", *variant_field_index));
                variant_types.last_mut().unwrap().push(match variant_type {
                    FieldType::Mem(_) => format_ident!("MemoryOperand"),
                    FieldType::Reg(reg) => reg.type_to_rust_type(),
                    FieldType::Imm => format_ident!("i64"),
                    FieldType::RelBR => format_ident!("i64"),
                    FieldType::Ptr => format_ident!("i64"),
                    FieldType::AGen => format_ident!("MemoryOperand")
                })
            }
        }
        let instruction_enum_name = instruction_name.proc_macro_safe_name();
        enums.push(quote! {
            #[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
            pub enum #instruction_enum_name {
                #(#instruction_variant_names1{

                }),*
            }
        });
    }
    proc_macro::TokenStream::from(quote! {
        #(#enums)*
    })
}