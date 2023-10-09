use proc_macro::TokenStream;

use quote::{format_ident, quote, ToTokens};
use syn::Data;

#[proc_macro_derive(EnumVisitor)]
pub fn derive_enum_visitor(_item: TokenStream) -> TokenStream {
    let enum_def: syn::DeriveInput = syn::parse(_item).unwrap();
    let mut visit_functions: Vec<proc_macro2::TokenStream> = vec![];
    let mut match_cases: Vec<proc_macro2::TokenStream> = vec![];
    if let Data::Enum(enum_) = enum_def.data {
        for variant in enum_.variants {
            let mut fields = vec![];
            let mut field_names = vec![];
            let mut single = false;
            for (i,field) in variant.fields.into_iter().enumerate() {
                let type_tokens = field.ty.to_token_stream();
                if let Some(ident) = &field.ident {
                    fields.push(quote! {
                        #ident : #type_tokens
                    });
                } else {
                    // let number_ident = format_ident!("{}",i);
                    assert_eq!(i, 0);
                    fields.push(quote! {
                        _0 : #type_tokens
                    });
                }
                if let Some(ident) = &field.ident {
                    field_names.push(quote! {
                        #ident
                    });
                } else {
                    assert_eq!(i, 0);
                    // let number_ident = format_ident!("{}",i);
                    field_names.push(quote! {
                        _0
                    });
                    single ^= true;
                }
            }
            let visit_ident = format_ident!("visit_{}", variant.ident);
            visit_functions.push(quote! {
                fn #visit_ident(&self, #(#fields),*) {}
            });
            let visit_ident = format_ident!("visit_{}", variant.ident);
            let variant_name = format_ident!("{}", variant.ident);
            let field_names_clone = field_names.clone();
            let enum_name = format_ident!("{}", enum_def.ident);
            if single {
                match_cases.push(quote! {
                    #enum_name::#variant_name( #(#field_names)* ) => self.#visit_ident(#(#field_names_clone),*),
                });
            }else {
                match_cases.push(quote! {
                    #enum_name::#variant_name { #(#field_names),* } => self.#visit_ident(#(#field_names_clone),*),
                });
            }
        }
        let enum_visitor_name = format_ident!("{}Visitor", enum_def.ident);
        let enum_name = format_ident!("{}", enum_def.ident);
        let stream: proc_macro2::TokenStream = quote! {
            pub trait #enum_visitor_name {
                fn visit(&self, enum_: #enum_name) {
                    match enum_ {
                        #(#match_cases)*
                    }
                }

                #(#visit_functions)*
            }
        };
        return proc_macro::TokenStream::from(stream);
    }else {
        panic!("Not an Enum");
    }
}

