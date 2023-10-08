use itertools::Itertools;
use quote::{format_ident, quote};

use crate::function_def_to_intermediate::{RemillSemanticsExpression, RemillSemanticsParsed, RemillSemanticsStatement};

pub fn to_rust(semantics: &RemillSemanticsParsed) -> Option<proc_macro2::TokenStream> {
    let RemillSemanticsParsed { name, template_params: _, params, statements } = semantics;

    let name = format_ident!("{name}");
    let mut statement_tokens = vec![];
    for statement in statements {
        statement_tokens.push(statement_to_rust(statement));
    }
    let params = params.iter().map(|param| {
        let param = format_ident!("{param}");
        quote! {
            #param: Input
        }
    }).collect_vec();

    let statement_tokens: proc_macro2::TokenStream = statement_tokens.into_iter().collect::<Option<_>>()?;
    let stream = quote! {
        pub fn #name(#(#params,)*) -> MemoryRes {
            #statement_tokens
        }
    };
    Some(stream)
}


pub fn statement_to_rust(statement: &RemillSemanticsStatement) -> Option<proc_macro2::TokenStream> {
    Some(match statement {
        RemillSemanticsStatement::VarDecl { expression, name } => {
            let name = format_ident!("{name}");
            let expression: proc_macro2::TokenStream = expression_to_rust(expression)?;
            quote! {
                let #name = (#expression);
            }
        }
        RemillSemanticsStatement::VarAssign { expression, variable } => {
            let variable = format_ident!("{variable}");
            let expression: proc_macro2::TokenStream = expression_to_rust(expression)?;
            quote! {
                #variable = #expression;
            }
        }
        RemillSemanticsStatement::CompoundStatement { statements } => {
            let mut statement_tokens = vec![];
            for statement in statements {
                statement_tokens.push(statement_to_rust(statement));
            }
            let statement_tokens: proc_macro2::TokenStream = statement_tokens.into_iter().collect::<Option<_>>()?;
            quote! {
                #statement_tokens
            }
        }
        RemillSemanticsStatement::CallStatement { expression } => {
            let expression: proc_macro2::TokenStream = expression_to_rust(expression)?;
            quote! {
                #expression;
            }
        }
        RemillSemanticsStatement::Return { expression } => {
            if let Some(expression) = expression.as_ref() {
                let expression: proc_macro2::TokenStream = expression_to_rust(expression)?;
                quote! {
                    return #expression;
                }
            } else {
                quote! {
                    return;
                }
            }
        }
        RemillSemanticsStatement::Inc { inner } => {
            let inner = expression_to_rust(inner)?;
            quote! {
                #inner += 1;
            }
        }
        RemillSemanticsStatement::LValueAssign { lvalue, rhs } => {
            let lvalue = expression_to_rust(lvalue)?;
            let rhs = expression_to_rust(rhs)?;
            quote! {
                #lvalue = #rhs;
            }
        }
        RemillSemanticsStatement::ForStatement {
            init_decl,
            condition,
            increment,
            for_statements
        } => {
            let init_statements: proc_macro2::TokenStream = init_decl.iter().map(|init| statement_to_rust(init)).collect::<Option<_>>()?;
            let condition = expression_to_rust(condition)?;
            let for_statements: proc_macro2::TokenStream = for_statements.iter().map(|init| statement_to_rust(init)).collect::<Option<_>>()?;
            let increment: proc_macro2::TokenStream = increment.iter().map(|init| statement_to_rust(init)).collect::<Option<_>>()?;
            quote! {
                #init_statements;
                while(#condition){
                    #for_statements;
                    #increment;
                };
            }
        }
        RemillSemanticsStatement::IfElse { condition, if_body, else_body } => {
            let condition = expression_to_rust(condition)?;
            let if_tokens: proc_macro2::TokenStream = if_body.iter().map(|statement| statement_to_rust(statement)).collect::<Option<_>>()?;
            let else_tokens: proc_macro2::TokenStream = else_body.iter().map(|statement| statement_to_rust(statement)).collect::<Option<_>>()?;
            quote! {
                if(#condition) {
                    #if_tokens
                } else {
                    #else_tokens
                }
            }
        }
        RemillSemanticsStatement::If { condition, if_body } => {
            let condition = expression_to_rust(condition)?;
            let statement_tokens: proc_macro2::TokenStream = if_body.iter().map(|statement| statement_to_rust(statement)).collect::<Option<_>>()?;
            quote! {
                if(#condition) {
                    #statement_tokens
                }
            }
        }
        RemillSemanticsStatement::VarMinusEqual { expression, variable_lvalue } => {
            let variable_lvalue = expression_to_rust(variable_lvalue)?;
            let expression = expression_to_rust(expression)?;
            quote! {
                #variable_lvalue -= #expression;
            }
        }
        RemillSemanticsStatement::VarPlusEqual { expression, variable_lvalue } => {
            let variable_lvalue = expression_to_rust(variable_lvalue)?;
            let expression = expression_to_rust(expression)?;
            quote! {
                #variable_lvalue += #expression;
            }
        }
        RemillSemanticsStatement::SwitchStatement { .. } => {
            return None
        }
        RemillSemanticsStatement::CaseStatement { .. } => {
            todo!()
        }
        RemillSemanticsStatement::DefaultStatement { .. } => {
            todo!()
        }
        RemillSemanticsStatement::BreakStatement { .. } => {
            todo!()
        }
        RemillSemanticsStatement::WhileStatement { condition, statements } => {
            /*let condition = expression_to_rust(condition)?;
            let statement_tokens: proc_macro2::TokenStream = statements.iter().map(|statement| statement_to_rust(statement)).collect::<Option<_>>()?;
            quote! {
                while(#condition) {
                    #statement_tokens
                };
            }*/
            return None
        }
        RemillSemanticsStatement::DoWhile { condition, statements } => {
            /*let condition = expression_to_rust(condition)?;
            let statement_tokens: proc_macro2::TokenStream = statements.iter().map(|statement| statement_to_rust(statement)).collect::<Option<_>>()?;
            quote! {
                while {
                    #statement_tokens;
                    #condition
                }{};
            }*/
            return None
        }
        RemillSemanticsStatement::VarMulEqual { expression, variable_lvalue } => {
            let variable_lvalue = expression_to_rust(variable_lvalue)?;
            let expression = expression_to_rust(expression)?;
            quote! {
                #variable_lvalue *= #expression;
            }
        }
        RemillSemanticsStatement::VarOrEqual { expression, variable_lvalue } => {
            let variable_lvalue = expression_to_rust(variable_lvalue)?;
            let expression = expression_to_rust(expression)?;
            quote! {
                #variable_lvalue |= #expression;
            }
        }
        RemillSemanticsStatement::ExprStatement { expression } => {
            let expression = expression_to_rust(expression)?;
            quote! {
                #expression;
            }
        }
        RemillSemanticsStatement::VarDivEqual { expression, variable_lvalue } => {
            let variable_lvalue = expression_to_rust(variable_lvalue)?;
            let expression = expression_to_rust(expression)?;
            quote! {
                #variable_lvalue /= #expression;
            }
        }
    })
}

fn expression_to_rust(expr: &RemillSemanticsExpression) -> Option<proc_macro2::TokenStream> {
    Some(match expr {
        RemillSemanticsExpression::Call { function_name, args } => {
            let mut function_name = function_name.replace("\"", "_quote_");
            if function_name == "" {
                function_name = "empty".to_string();
            }
            let function_name = format_ident!("{function_name}");
            let args = args.iter().map(|arg| expression_to_rust(arg)).collect::<Option<Vec<_>>>()?;
            quote!((#function_name(#(#args),*)))
        }
        RemillSemanticsExpression::VariableRef { name } => {
            let name = format_ident!("{name}");
            quote!(#name)
        }
        RemillSemanticsExpression::And { left, right } => {
            let left = expression_to_rust(left)?;
            let right = expression_to_rust(right)?;
            quote!(#left & #right)
        }
        RemillSemanticsExpression::EnumConstantRef { name } => {
            let name = format_ident!("{name}");
            quote!(#name)
        }
        RemillSemanticsExpression::IntegerInt { value } => {
            quote!(#value)
        }
        RemillSemanticsExpression::IntegerUInt { value } => {
            quote!(#value)
        }
        RemillSemanticsExpression::LessThan { left, right } => {
            let left = expression_to_rust(left)?;
            let right = expression_to_rust(right)?;
            quote!(#left < #right)
        }
        RemillSemanticsExpression::GreaterThanEq { left, right } => {
            let left = expression_to_rust(left)?;
            let right = expression_to_rust(right)?;
            quote!(#left >= #right)
        }
        RemillSemanticsExpression::LessThanEq { left, right } => {
            let left = expression_to_rust(left)?;
            let right = expression_to_rust(right)?;
            quote!(#left <= #right)
        }
        RemillSemanticsExpression::GreaterThan { left, right } => {
            let left = expression_to_rust(left)?;
            let right = expression_to_rust(right)?;
            quote!(#left >= #right)
        }
        RemillSemanticsExpression::DotMember { inner, name } => {
            let inner = expression_to_rust(inner.as_ref())?;
            let name = format_ident!("{name}");
            quote!(#inner.#name)
        }
        RemillSemanticsExpression::ArrayIndex { array, index } => {
            let array = expression_to_rust(array.as_ref())?;
            let index = expression_to_rust(index.as_ref())?;
            quote!(#array[#index])
        }
        RemillSemanticsExpression::LongUInt { value } => {
            quote!(#value)
        }
        RemillSemanticsExpression::Div { left, right } => {
            let left = expression_to_rust(left.as_ref())?;
            let right = expression_to_rust(right.as_ref())?;
            quote!(#left / #right)
        }
        RemillSemanticsExpression::LongLongUInt { value } => {
            quote!(#value)
        }
        RemillSemanticsExpression::DefaultInitFloatArray { len } => {
            quote!([0f32;#len])
        }
        RemillSemanticsExpression::RightShift { left, right } => {
            let left = expression_to_rust(left.as_ref())?;
            let right = expression_to_rust(right.as_ref())?;
            quote!(#left >> #right)
        }
        RemillSemanticsExpression::LeftShift { left, right } => {
            let left = expression_to_rust(left.as_ref())?;
            let right = expression_to_rust(right.as_ref())?;
            quote!(#left << #right)
        }
        RemillSemanticsExpression::Eq { left, right } => {
            let left = expression_to_rust(left.as_ref())?;
            let right = expression_to_rust(right.as_ref())?;
            quote!(#left == #right)
        }
        RemillSemanticsExpression::Bool { value } => {
            quote!(#value)
        }
        RemillSemanticsExpression::Mul { left, right } => {
            let left = expression_to_rust(left.as_ref())?;
            let right = expression_to_rust(right.as_ref())?;
            quote!(#left * #right)
        }
        RemillSemanticsExpression::Add { left, right } => {
            let left = expression_to_rust(left.as_ref())?;
            let right = expression_to_rust(right.as_ref())?;
            quote!(#left + #right)
        }
        RemillSemanticsExpression::Sub { left, right } => {
            let left = expression_to_rust(left.as_ref())?;
            let right = expression_to_rust(right.as_ref())?;
            quote!(#left - #right)
        }
        RemillSemanticsExpression::Sizeof { arg_type } => {
            let type_ = match arg_type.qual_type.as_str() {
                "uint8_t" => quote!(u8),
                "uint16_t" => quote!(u16),
                "uint32_t" => quote!(u32),
                "uint64_t" => quote!(u64),
                "unsigned short" => quote!(std::ffi::c_ushort),
                "unsigned long" => quote!(std::ffi::c_ulong),
                "union bcd_digit_pair_t[9]" => quote!([bcd_digit_pair_t;9]),
                "In<unsigned long>" => quote!(std::ffi::c_ulong),
                other => todo!("{other:?}")
            };
            quote!(std::mem::size_of::<#type_>())
        }
        RemillSemanticsExpression::Neg { inner } => {
            let inner = expression_to_rust(inner)?;
            quote!((-(#inner)))
        }
        RemillSemanticsExpression::Dec { inner } => {
            let inner = expression_to_rust(inner)?;
            quote!(((#inner) -= 1))
        }
        RemillSemanticsExpression::BoolNeg { inner } => {
            let inner = expression_to_rust(inner)?;
            quote!(!(#inner))
        }
        RemillSemanticsExpression::BoolAnd { left, right } => {
            let left = expression_to_rust(left.as_ref())?;
            let right = expression_to_rust(right.as_ref())?;
            quote!(#left && #right)
        }
        RemillSemanticsExpression::DefaultInitUint32Array { len } => {
            quote!([0u32;#len])
        }
        RemillSemanticsExpression::BitNeg { inner } => {
            let inner = expression_to_rust(inner)?;
            quote!(!(#inner))
        }
        RemillSemanticsExpression::FloatLiteral { value } => {
            let ident = format!("{value}f32");
            quote!(#ident)
        }
        RemillSemanticsExpression::UnknownConsructExpr => {
            return None
        }
        RemillSemanticsExpression::DefaultInitUint16Array { len } => {
            quote!([0u16;#len])
        }
        RemillSemanticsExpression::DefaultInitUint8Array { len } => {
            quote!([0u8;#len])
        }
        RemillSemanticsExpression::DefaultDoubleArray { len } => {
            quote!([0f64;#len])
        }
        RemillSemanticsExpression::U32 { value } => {
            let ident = format!("{value}u32");
            quote!(#ident)
        }
        RemillSemanticsExpression::DoubleLiteral { value } => {
            let ident = format!("{value}f64");
            quote!(#ident)
        }
        RemillSemanticsExpression::Mod { left, right } => {
            let left = expression_to_rust(left.as_ref())?;
            let right = expression_to_rust(right.as_ref())?;
            quote!(#left % #right)
        }
        RemillSemanticsExpression::BitOr { left, right } => {
            let left = expression_to_rust(left.as_ref())?;
            let right = expression_to_rust(right.as_ref())?;
            quote!(#left | #right)
        }
        RemillSemanticsExpression::NotEq { left, right } => {
            let left = expression_to_rust(left.as_ref())?;
            let right = expression_to_rust(right.as_ref())?;
            quote!(#left != #right)
        }
        RemillSemanticsExpression::FunctionRef { name } => {
            let name = format_ident!("{name}");
            quote!(#name)
        }
        RemillSemanticsExpression::DefaultInitUnionArray { len } => {
            let len = *len;
            quote!([_;#len])
        }
        RemillSemanticsExpression::BoolOr { left, right } => {
            let left = expression_to_rust(left.as_ref())?;
            let right = expression_to_rust(right.as_ref())?;
            quote!(#left || #right)
        }
        RemillSemanticsExpression::Lambda { statements } => {
            let statement_tokens: proc_macro2::TokenStream = statements.iter().map(|statement| statement_to_rust(statement)).collect::<Option<_>>()?;
            quote! (
                (||{
                    #statement_tokens
                })
            )
        }
        RemillSemanticsExpression::Conditional { condition, true_case, false_case } => {
            let condition = expression_to_rust(condition)?;
            let if_tokens: proc_macro2::TokenStream = expression_to_rust(true_case)?;
            let else_tokens: proc_macro2::TokenStream = expression_to_rust(false_case)?;
            quote!(
                (if(#condition) {
                    #if_tokens
                } else {
                    #else_tokens
                })
            )
        }
        RemillSemanticsExpression::Address { inner } => {
            let inner = expression_to_rust(inner)?;
            quote! (&(#inner))
        }
    })
}
