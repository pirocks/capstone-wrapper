use std::ffi::{c_int, c_uint, c_ulong, c_ulonglong};
use std::str::FromStr;
use std::vec;

use crate::clang_json_defs::ASTType;
use crate::unneeded_data_stripped::ASTNodeCleanedUp;

fn expression(inner: &ASTNodeCleanedUp) -> RemillSemanticsExpression {
    match inner {
        ASTNodeCleanedUp::CallExpr { inner, .. } => {
            let function = &inner[0];
            let mut args = vec![];
            for arg in &inner[1..] {
                args.push(expression(arg));
            }
            match function {
                ASTNodeCleanedUp::ImplicitCastExpr { inner, .. } => {
                    let function_name = match &inner[0] {
                        ASTNodeCleanedUp::DeclRefExpr { referenced_decl, .. } => {
                            match referenced_decl.as_ref() {
                                ASTNodeCleanedUp::FunctionDecl { name, .. } => {
                                    name.to_string()
                                }
                                other => todo!("{other:?}")
                            }
                        }
                        other => todo!("{other:?}")
                    };
                    return RemillSemanticsExpression::Call { function_name, args };
                }
                ASTNodeCleanedUp::SubstNonTypeTemplateParmExpr { inner, .. } => {
                    assert_eq!(inner.len(), 2);
                    match &inner[1] {
                        ASTNodeCleanedUp::DeclRefExpr { referenced_decl, .. } => {
                            match referenced_decl.as_ref() {
                                ASTNodeCleanedUp::FunctionDecl { name, .. } => {
                                    return RemillSemanticsExpression::Call { function_name: name.to_string(), args };
                                }
                                other => todo!("{other:?}")
                            }
                        }
                        ASTNodeCleanedUp::UnaryOperator { opcode, inner, .. } => {
                            assert_eq!(opcode.as_str(), "&");
                            let inner = &inner[0];
                            match inner {
                                ASTNodeCleanedUp::DeclRefExpr { referenced_decl, .. } => {
                                    match referenced_decl.as_ref() {
                                        ASTNodeCleanedUp::FunctionDecl { name, .. } => {
                                            return RemillSemanticsExpression::Call { function_name: name.to_string(), args };
                                        }
                                        other => todo!("{other:?}")
                                    }
                                }
                                other => todo!("{other:?}")
                            }
                        }
                        other => todo!("{other:?}")
                    }
                }
                other => todo!("{other:?}")
            }
        }
        ASTNodeCleanedUp::VarDecl { name, .. } => {
            return RemillSemanticsExpression::VariableRef { name: name.as_ref().unwrap().to_string() };
        }
        ASTNodeCleanedUp::ParmVarDecl { name, .. } => {
            return RemillSemanticsExpression::VariableRef { name: name.as_ref().unwrap().to_string() };
        }
        ASTNodeCleanedUp::ImplicitCastExpr { inner, .. } => {
            assert_eq!(inner.len(), 1);
            let inner = &inner[0];
            return expression(inner);
        }
        ASTNodeCleanedUp::DeclRefExpr { referenced_decl, .. } => {
            match referenced_decl.as_ref() {
                ASTNodeCleanedUp::EnumConstantDecl { name, .. } => {
                    return RemillSemanticsExpression::EnumConstantRef { name: name.to_string() };
                }
                ASTNodeCleanedUp::ParmVarDecl { .. } | ASTNodeCleanedUp::VarDecl { .. } => {
                    return expression(referenced_decl.as_ref());
                }
                ASTNodeCleanedUp::FunctionDecl { name, .. } => {
                    return RemillSemanticsExpression::FunctionRef { name: name.to_string() };
                }
                other => todo!("{other:?}")
            }
        }
        ASTNodeCleanedUp::CXXStaticCastExpr { inner, .. } => {
            assert_eq!(inner.len(), 1);
            return expression(&inner[0]);
        }
        ASTNodeCleanedUp::BinaryOperator { inner, opcode, .. } => {
            assert_eq!(inner.len(), 2);
            let left = Box::new(expression(&inner[0]));
            let right = Box::new(expression(&inner[1]));
            match opcode.as_str() {
                "&" => RemillSemanticsExpression::And { left, right },
                "<" => RemillSemanticsExpression::LessThan { left, right },
                ">" => RemillSemanticsExpression::GreaterThan { left, right },
                "<=" => RemillSemanticsExpression::LessThanEq { left, right },
                ">=" => RemillSemanticsExpression::GreaterThanEq { left, right },
                "/" => RemillSemanticsExpression::Div { left, right },
                ">>" => RemillSemanticsExpression::RightShift { left, right },
                "<<" => RemillSemanticsExpression::LeftShift { left, right },
                "==" => RemillSemanticsExpression::Eq { left, right },
                "!=" => RemillSemanticsExpression::NotEq { left, right },
                "*" => RemillSemanticsExpression::Mul { left, right },
                "+" => RemillSemanticsExpression::Add { left, right },
                "-" => RemillSemanticsExpression::Sub { left, right },
                "&&" => RemillSemanticsExpression::BoolAnd { left, right },
                "||" => RemillSemanticsExpression::BoolOr { left, right },
                "%" => RemillSemanticsExpression::Mod { left, right },
                "|" => RemillSemanticsExpression::BitOr { left, right },
                other => todo!("{other:?}")
            }
        }
        ASTNodeCleanedUp::ConstantExpr { value, type_, .. } => {
            integer_literal(value, type_)
        }
        ASTNodeCleanedUp::IntegerLiteral { value, type_, .. } => {
            integer_literal(value, type_)
        }
        ASTNodeCleanedUp::ExprWithCleanups { inner, .. } => {
            let mut res = vec![];
            for inner in inner {
                res.push(expression(inner));
            }
            assert_eq!(res.len(), 1);
            return res.into_iter().next().unwrap();
        }
        ASTNodeCleanedUp::MaterializeTemporaryExpr { inner, .. } => {
            assert_eq!(inner.len(), 1);
            let inner = &inner[0];
            expression(inner)
        }
        ASTNodeCleanedUp::ArraySubscriptExpr { inner, .. } => {
            assert_eq!(inner.len(), 2);
            let array = Box::new(expression(&inner[0]));
            let index = Box::new(expression(&inner[1]));
            RemillSemanticsExpression::ArrayIndex { array, index }
        }
        ASTNodeCleanedUp::MemberExpr { inner, name, .. } => {
            assert_eq!(inner.len(), 1);
            let inner = Box::new(expression(&inner[0]));
            let name = name.to_string();
            RemillSemanticsExpression::DotMember { inner, name }
        }
        ASTNodeCleanedUp::InitListExpr { field, inner, .. } => {
            if let Some(field) = field.as_ref() {
                match field.as_ref() {
                    ASTNodeCleanedUp::FieldDecl { name, .. } => {
                        return RemillSemanticsExpression::VariableRef { name: name.as_ref().unwrap().to_string() };
                    }
                    other => todo!("{other:?}")
                }
            } else {
                // assert_eq!(inner.as_ref().unwrap().len(), 1);
                let inner = &inner.as_ref().unwrap()[0];
                match inner {
                    ASTNodeCleanedUp::InitListExpr { type_, array_filler, .. } => {
                        if array_filler.is_some() {
                            match type_.qual_type.as_str() {
                                "union bcd_digit_pair_t[9]" => {
                                    return RemillSemanticsExpression::DefaultInitUnionArray {
                                        len: 9usize
                                    };
                                }
                                other => todo!("{other:?}")
                            }
                        } else {
                            todo!()
                        }
                    }
                    ASTNodeCleanedUp::ImplicitValueInitExpr { type_, .. } => {
                        match type_.qual_type.as_str() {
                            "float[4]" => {
                                return RemillSemanticsExpression::DefaultInitFloatArray {
                                    len: 4usize
                                };
                            }
                            "uint32_t[2]" => {
                                return RemillSemanticsExpression::DefaultInitUint32Array {
                                    len: 2usize
                                };
                            }
                            "uint8_t[8]" => {
                                return RemillSemanticsExpression::DefaultInitUint8Array {
                                    len: 8usize
                                };
                            }
                            "uint8_t[16]" => {
                                return RemillSemanticsExpression::DefaultInitUint8Array {
                                    len: 16usize
                                };
                            }
                            "uint8_t[32]" => {
                                return RemillSemanticsExpression::DefaultInitUint8Array {
                                    len: 32usize
                                };
                            }
                            "double[2]" => {
                                return RemillSemanticsExpression::DefaultDoubleArray {
                                    len: 2usize
                                };
                            }
                            other => todo!("{other:?}")
                        }
                    }
                    ASTNodeCleanedUp::UnaryOperator {.. } | ASTNodeCleanedUp::BinaryOperator { .. } => {
                        return expression(&inner);
                    }
                    other => todo!("{other:?}")
                }
            }
        }
        ASTNodeCleanedUp::SubstNonTypeTemplateParmExpr { inner, .. } => {
            assert_eq!(inner.len(), 2);
            match &inner[1] {
                ASTNodeCleanedUp::IntegerLiteral { .. } => {
                    return expression(&inner[1]);
                }
                ASTNodeCleanedUp::CStyleCastExpr { inner, .. } => {
                    assert_eq!(inner.len(), 1);
                    return expression(&inner[0]);
                }
                other => todo!("{other:?}")
            }
        }
        ASTNodeCleanedUp::CXXConstructExpr { inner: inner1, .. } => {
            if let Some(inner) = inner1.as_ref() {
                assert_eq!(inner.len(), 1);
                let inner = &inner[0];
                return expression(inner);
            }
            return RemillSemanticsExpression::UnknownConsructExpr;
        }
        ASTNodeCleanedUp::ParenExpr { inner, .. } => {
            assert_eq!(inner.len(), 1);
            let inner = &inner[0];
            return expression(inner);
        }
        ASTNodeCleanedUp::UserDefinedLiteral { inner, .. } => {
            let inner = &inner[1];
            match inner {
                ASTNodeCleanedUp::IntegerLiteral { .. } => {
                    return expression(inner);
                }
                other => todo!("{other:?}")
            }
        }
        ASTNodeCleanedUp::CXXFunctionalCastExpr { inner, .. } => {
            assert_eq!(inner.len(), 1);
            expression(&inner[0])
        }
        ASTNodeCleanedUp::CXXBoolLiteralExpr { value, .. } => {
            return RemillSemanticsExpression::Bool { value: *value };
        }
        ASTNodeCleanedUp::UnaryExprOrTypeTraitExpr { name, arg_type, .. } => {
            match name.as_str() {
                "sizeof" => return RemillSemanticsExpression::Sizeof { arg_type: arg_type.clone() },
                other => todo!("{other:?}")
            }
        }
        ASTNodeCleanedUp::UnaryOperator { opcode, inner, .. } => {
            assert_eq!(inner.len(), 1);
            let inner = Box::new(expression(&inner[0]));
            match opcode.as_str() {
                "-" => { RemillSemanticsExpression::Neg { inner } }
                "--" => { RemillSemanticsExpression::Dec { inner } }
                "!" => { RemillSemanticsExpression::BoolNeg { inner } }
                "~" => { RemillSemanticsExpression::BitNeg { inner } }
                "&" => { RemillSemanticsExpression::Address { inner } }
                other => todo!("{other:?}")
            }
        }
        ASTNodeCleanedUp::FloatingLiteral { value, type_, .. } => {
            match type_.qual_type.as_str() {
                "float" => {
                    RemillSemanticsExpression::FloatLiteral { value: f32::from_str(value.as_str()).unwrap() }
                }
                "double" => {
                    RemillSemanticsExpression::DoubleLiteral { value: f64::from_str(value.as_str()).unwrap() }
                }
                other => todo!("{other:?}")
            }
        }
        ASTNodeCleanedUp::CXXReinterpretCastExpr { inner, .. } => {
            assert_eq!(inner.len(), 1);
            expression(&inner[0])
        }
        ASTNodeCleanedUp::CXXMemberCallExpr { inner, .. } => {
            assert_eq!(inner.len(), 1);
            expression(&inner[0])
        }
        ASTNodeCleanedUp::LambdaExpr { inner, .. } => {
            dbg!(inner);
            assert_eq!(inner.len(), 2);
            let _record_decl = &inner[0];
            let compound_statement = &inner[1];
            return RemillSemanticsExpression::Lambda { statments: statement(compound_statement) };
        }
        ASTNodeCleanedUp::ConditionalOperator { inner, .. } => {
            let condition = Box::new(expression(&inner[0]));
            let true_case = Box::new(expression(&inner[1]));
            let false_case = Box::new(expression(&inner[2]));
            return RemillSemanticsExpression::Conditional { condition, true_case, false_case }
        }
        other => todo!("{other:?}")
    }
}

fn integer_literal(value: &String, type_: &ASTType) -> RemillSemanticsExpression {
    match type_.qual_type.as_ref() {
        "unsigned int" => {
            let value = c_uint::from_str(value.as_str()).unwrap();
            RemillSemanticsExpression::IntegerUInt { value }
        }
        "int" => {
            let value = c_int::from_str(value.as_str()).unwrap();
            RemillSemanticsExpression::IntegerInt { value }
        }
        "unsigned long" => {
            let value = c_ulong::from_str(value.as_str()).unwrap();
            RemillSemanticsExpression::LongUInt { value }
        }
        "unsigned long long" => {
            let value = c_ulonglong::from_str(value.as_str()).unwrap();
            RemillSemanticsExpression::LongLongUInt { value }
        }
        "uint32_t" => {
            let value = u32::from_str(value.as_str()).unwrap();
            RemillSemanticsExpression::U32 { value }
        }
        other => todo!("{other:?}")
    }
}


#[derive(Debug)]
pub enum RemillSemanticsExpression {
    Call { function_name: String, args: Vec<RemillSemanticsExpression> },
    VariableRef { name: String },
    And { left: Box<RemillSemanticsExpression>, right: Box<RemillSemanticsExpression> },
    EnumConstantRef { name: String },
    IntegerInt { value: i32 },
    IntegerUInt { value: u32 },
    LessThan { left: Box<RemillSemanticsExpression>, right: Box<RemillSemanticsExpression> },
    DotMember { inner: Box<RemillSemanticsExpression>, name: String },
    ArrayIndex { array: Box<RemillSemanticsExpression>, index: Box<RemillSemanticsExpression> },
    LongUInt { value: u64 },
    Div { left: Box<RemillSemanticsExpression>, right: Box<RemillSemanticsExpression> },
    LongLongUInt { value: u64 },
    DefaultInitFloatArray { len: usize },
    RightShift { left: Box<RemillSemanticsExpression>, right: Box<RemillSemanticsExpression> },
    LeftShift { left: Box<RemillSemanticsExpression>, right: Box<RemillSemanticsExpression> },
    Eq { left: Box<RemillSemanticsExpression>, right: Box<RemillSemanticsExpression> },
    Bool { value: bool },
    Mul { left: Box<RemillSemanticsExpression>, right: Box<RemillSemanticsExpression> },
    Add { left: Box<RemillSemanticsExpression>, right: Box<RemillSemanticsExpression> },
    Sub { left: Box<RemillSemanticsExpression>, right: Box<RemillSemanticsExpression> },
    Sizeof { arg_type: Option<ASTType> },
    LessThanEq { left: Box<RemillSemanticsExpression>, right: Box<RemillSemanticsExpression> },
    Neg { inner: Box<RemillSemanticsExpression> },
    Dec { inner: Box<RemillSemanticsExpression> },
    GreaterThan { left: Box<RemillSemanticsExpression>, right: Box<RemillSemanticsExpression> },
    BoolNeg { inner: Box<RemillSemanticsExpression> },
    BoolAnd { left: Box<RemillSemanticsExpression>, right: Box<RemillSemanticsExpression> },
    DefaultInitUint32Array { len: usize },
    BitNeg { inner: Box<RemillSemanticsExpression> },
    GreaterThanEq { left: Box<RemillSemanticsExpression>, right: Box<RemillSemanticsExpression> },
    FloatLiteral { value: f32 },
    UnknownConsructExpr,
    DefaultInitUint16Array { len: usize },
    DefaultInitUint8Array { len: usize },
    DefaultDoubleArray { len: usize },
    U32 { value: u32 },
    DoubleLiteral { value: f64 },
    Mod { left: Box<RemillSemanticsExpression>, right: Box<RemillSemanticsExpression> },
    BitOr { left: Box<RemillSemanticsExpression>, right: Box<RemillSemanticsExpression> },
    FunctionRef { name: String },
    DefaultInitUnionArray { len: usize },
    BoolOr { left: Box<RemillSemanticsExpression>, right: Box<RemillSemanticsExpression> },
    NotEq { left: Box<RemillSemanticsExpression>, right: Box<RemillSemanticsExpression> },
    Lambda { statments: Vec<RemillSemanticsStatement> },
    Conditional { condition: Box<RemillSemanticsExpression>, true_case: Box<RemillSemanticsExpression>, false_case: Box<RemillSemanticsExpression> },
    Address { inner: Box<RemillSemanticsExpression> },
}

#[derive(Debug)]
pub enum RemillSemanticsStatement {
    VarDecl { expression: RemillSemanticsExpression, name: String },
    VarAssign { expression: RemillSemanticsExpression, variable: String },
    CompoundStatement { statements: Vec<RemillSemanticsStatement> },
    CallStatement { expression: RemillSemanticsExpression },
    Return { expression: Option<RemillSemanticsExpression> },
    Inc { inner: RemillSemanticsExpression },
    LValueAssign { lvalue: RemillSemanticsExpression, rhs: RemillSemanticsExpression },
    ForStatement { init_decl: Vec<RemillSemanticsStatement>, condition: RemillSemanticsExpression, increment: Vec<RemillSemanticsStatement>, for_statements: Vec<RemillSemanticsStatement> },
    IfElse { condition: RemillSemanticsExpression, if_body: Vec<RemillSemanticsStatement>, else_body: Vec<RemillSemanticsStatement> },
    If { condition: RemillSemanticsExpression, if_body: Vec<RemillSemanticsStatement> },
    VarMinusEqual { expression: RemillSemanticsExpression, variable_lvalue: RemillSemanticsExpression },
    VarPlusEqual { expression: RemillSemanticsExpression, variable_lvalue: RemillSemanticsExpression },
    SwitchStatement { expression: RemillSemanticsExpression, statements: Vec<RemillSemanticsStatement> },
    CaseStatement { case: RemillSemanticsExpression },
    DefaultStatement {},
    BreakStatement {},
    WhileStatement { condition: RemillSemanticsExpression, statements: Vec<RemillSemanticsStatement> },
    DoWhile { condition: RemillSemanticsExpression, statements: Vec<RemillSemanticsStatement> },
    VarMulEqual { expression: RemillSemanticsExpression, variable_lvalue: RemillSemanticsExpression },
    VarOrEqual { expression: RemillSemanticsExpression, variable_lvalue: RemillSemanticsExpression },
    ExprStatement { expression: RemillSemanticsExpression },
    VarDivEqual { expression: RemillSemanticsExpression, variable_lvalue: RemillSemanticsExpression },
}

#[derive(Debug)]
pub struct RemillSemanticsParsed {
    name: String,
    template_params: Vec<String>,
    params: Vec<String>,
    statements: Vec<RemillSemanticsStatement>,
}

#[derive(Debug)]
pub struct RemillSemanticsParsedUninit {
    name: Option<String>,
    template_params: Vec<String>,
    params: Vec<String>,
    statements: Vec<RemillSemanticsStatement>,
}

impl RemillSemanticsParsedUninit {
    pub fn to_init(self) -> RemillSemanticsParsed {
        let RemillSemanticsParsedUninit { name, template_params, params, statements } = self;
        dbg!(&name);
        assert!(statements.len() > 1);
        RemillSemanticsParsed {
            name: name.unwrap(),
            template_params,
            params,
            statements,
        }
    }
}

pub fn function_def_to_rust(ast: &ASTNodeCleanedUp) -> Option<RemillSemanticsParsed> {
    let mut remill_semantics_parsed = RemillSemanticsParsedUninit { name: None, template_params: vec![], params: vec![], statements: vec![] };
    match ast {
        ASTNodeCleanedUp::FunctionDecl { inner, name, .. } => {
            if name.as_str() == "HandleUnsupported" ||
                name.as_str() == "DoNothing" ||
                name.as_str() == "DoCLFLUSH_MEMmprefetch" ||
                name.as_str() == "DoCPUID" ||
                name.as_str() == "DoNothingWithParam" ||
                name.as_str() == "NOP_IMPL" ||
                name.as_str() == "PREFETCH" ||
                name.as_str() == "XABORT" ||
                name.as_str() == "DoRDTSC" ||
                name.as_str() == "DoRDTSCP"
            {
                return None;
            }
            remill_semantics_parsed.name = Some(name.to_string());
            let inner = inner.as_ref().unwrap();
            for inner in inner {
                match inner {
                    ASTNodeCleanedUp::TemplateArgument { type_, value, decl, is_pack, .. } => {
                        if decl.is_some() || is_pack.is_some() {} else if let Some(value) = value.as_ref() {
                            remill_semantics_parsed.template_params.push(value.to_string());
                        } else {
                            let type_ = type_.as_ref().unwrap();
                            let qual_type = &type_.qual_type;
                            remill_semantics_parsed.template_params.push(qual_type.to_string());
                        }
                    }
                    ASTNodeCleanedUp::ParmVarDecl { name, .. } => {
                        if let Some(name) = name {
                            remill_semantics_parsed.params.push(name.to_string());
                        } else {}
                    }
                    ASTNodeCleanedUp::CompoundStmt { inner, .. } => {
                        for inner in inner.as_ref().unwrap() {
                            remill_semantics_parsed.statements.extend(statement(inner));
                        }
                    }
                    ASTNodeCleanedUp::AlwaysInlineAttr { .. } |
                    ASTNodeCleanedUp::FlattenAttr { .. } => {}
                    _ => todo!("{inner:?}")
                }
            }
        }
        _ => todo!("{ast:?}")
    }
    Some(remill_semantics_parsed.to_init())
}

fn statement(inner: &ASTNodeCleanedUp) -> Vec<RemillSemanticsStatement> {
    match inner {
        ASTNodeCleanedUp::DeclStmt { inner, .. } => {
            let mut res = vec![];
            for inner in inner {
                match inner {
                    ASTNodeCleanedUp::VarDecl { inner, name, .. } => {
                        let inner = &inner.as_ref().unwrap()[0];
                        let name = name.as_ref().unwrap().to_string();
                        res.push(RemillSemanticsStatement::VarDecl { expression: expression(inner), name });
                    }
                    ASTNodeCleanedUp::StaticAssertDecl { .. } => {}
                    ASTNodeCleanedUp::TypedefDecl { .. } => {
                        return vec![];
                    }
                    other => todo!("{other:?}")
                }
            }
            res
        }
        ASTNodeCleanedUp::DoStmt { inner, .. } => {
            let should_be_false = &inner[1];
            let condition = match should_be_false {
                ASTNodeCleanedUp::CXXBoolLiteralExpr { value, .. } => {
                    assert_eq!(*value, false);
                    return statement(&inner[0]);
                }
                ASTNodeCleanedUp::CallExpr { .. } => {
                    expression(should_be_false)
                }
                other => todo!("{other:?}")
            };
            return vec![RemillSemanticsStatement::DoWhile {
                condition,
                statements: statement(&inner[0]),
            }];
        }
        ASTNodeCleanedUp::CallExpr { inner: _, .. } => {
            return vec![RemillSemanticsStatement::CallStatement { expression: expression(inner) }];
        }
        ASTNodeCleanedUp::ReturnStmt { inner, .. } => {
            let expression = inner.as_ref().map(|inner| {
                assert_eq!(inner.len(), 1);
                expression(&inner[0])
            });
            return vec![RemillSemanticsStatement::Return { expression }];
        }
        ASTNodeCleanedUp::CompoundStmt { inner, .. } => {
            let mut statements = vec![];
            for inner in inner.as_ref().unwrap() {
                statements.extend(statement(inner));
            }
            return vec![RemillSemanticsStatement::CompoundStatement { statements }];
        }
        ASTNodeCleanedUp::BinaryOperator { inner, opcode, .. } => {
            match opcode.as_str() {
                "=" => {
                    let variable = match &inner[0] {
                        ASTNodeCleanedUp::DeclRefExpr { referenced_decl, .. } => {
                            match referenced_decl.as_ref() {
                                ASTNodeCleanedUp::ParmVarDecl { name, .. } => {
                                    name.as_ref().unwrap().to_string()
                                }
                                ASTNodeCleanedUp::VarDecl { name, .. } => {
                                    name.as_ref().unwrap().to_string()
                                }
                                other => todo!("{other:?}")
                            }
                        }
                        ASTNodeCleanedUp::MemberExpr { .. } |
                        ASTNodeCleanedUp::ArraySubscriptExpr { .. } => {
                            return vec![RemillSemanticsStatement::LValueAssign { lvalue: expression(&inner[0]), rhs: expression(&inner[1]) }];
                        }
                        other => todo!("{other:?}")
                    };
                    return vec![RemillSemanticsStatement::VarAssign { expression: expression(&inner[1]), variable }];
                }
                "," => {
                    let mut res = vec![];
                    for inner in inner {
                        res.extend(statement(inner));
                    }
                    return res;
                }
                other => todo!("{other:?}")
            }
        }
        ASTNodeCleanedUp::ExprWithCleanups { inner, .. } => {
            let mut statements = vec![];
            for inner in inner {
                statements.extend(statement(inner));
            }
            return vec![RemillSemanticsStatement::CompoundStatement {
                statements,
            }];
        }
        ASTNodeCleanedUp::CXXOperatorCallExpr { inner, .. } => {
            assert_eq!(inner.len(), 3);
            let first = &inner[0];
            let operator_string = match first {
                ASTNodeCleanedUp::ImplicitCastExpr { inner, .. } => {
                    assert_eq!(inner.len(), 1);
                    match &inner[0] {
                        ASTNodeCleanedUp::DeclRefExpr { referenced_decl, .. } => {
                            match referenced_decl.as_ref() {
                                ASTNodeCleanedUp::CXXMethodDecl { name, .. } => {
                                    name.to_string()
                                }
                                other => todo!("{other:?}")
                            }
                        }
                        other => todo!("{other:?}")
                    }
                }
                other => todo!("{other:?}")
            };
            let third = &inner[2];
            let rhs_expression = expression(third);
            let second = &inner[1];
            match second {
                ASTNodeCleanedUp::DeclRefExpr { referenced_decl, .. } => {
                    match referenced_decl.as_ref() {
                        ASTNodeCleanedUp::VarDecl { name, .. } => {
                            let lhs_variable_name = name.as_ref().unwrap().to_string();
                            match operator_string.as_str() {
                                "operator=" => {
                                    return vec![RemillSemanticsStatement::VarAssign { expression: rhs_expression, variable: lhs_variable_name }];
                                }
                                other => todo!("{other:?}")
                            }
                        }
                        other => todo!("{other:?}")
                    }
                }
                ASTNodeCleanedUp::MemberExpr { .. } => {
                    match operator_string.as_str() {
                        "operator=" => {
                            return vec![RemillSemanticsStatement::LValueAssign { rhs: rhs_expression, lvalue: expression(second) }];
                        }
                        other => todo!("{other:?}")
                    }
                }
                other => todo!("{other:?}")
            }
        }
        ASTNodeCleanedUp::AttributedStmt { inner, .. } => {
            let mut statements = vec![];
            for inner in inner {
                statements.extend(statement(inner));
            }
            vec![RemillSemanticsStatement::CompoundStatement { statements }]
        }
        ASTNodeCleanedUp::LoopHintAttr { implicit, .. } => {
            assert!(*implicit);
            return vec![];
        }
        ASTNodeCleanedUp::ForStmt { inner, .. } => {
            let mut for_statements = vec![];
            let init_decl = statement(&inner[0]);
            let condition = expression(&inner[1]);
            let increment = statement(&inner[2]);
            for inner in &inner[3..] {
                for_statements.extend(statement(inner))
            }
            return vec![RemillSemanticsStatement::ForStatement { init_decl, condition, increment, for_statements }];
        }
        ASTNodeCleanedUp::UnaryOperator { inner, opcode, .. } => {
            assert_eq!(inner.len(), 1);
            let inner = expression(&inner[0]);
            match opcode.as_str() {
                "++" => {
                    return vec![RemillSemanticsStatement::Inc { inner }];
                }
                other => todo!("{other:?}")
            }
        }
        ASTNodeCleanedUp::IfStmt { inner, has_else, .. } => {
            vec![if has_else == &Some(true) {
                assert_eq!(inner.len(), 3);
                let condition = expression(&inner[0]);
                let if_body = statement(&inner[1]);
                let else_body = statement(&inner[1]);
                RemillSemanticsStatement::IfElse { condition, if_body, else_body }
            } else {
                assert_eq!(inner.len(), 2);
                let condition = expression(&inner[0]);
                let if_body = statement(&inner[1]);
                RemillSemanticsStatement::If { condition, if_body }
            }]
        }
        ASTNodeCleanedUp::CompoundAssignOperator { inner, opcode, .. } => {
            assert_eq!(inner.len(), 2);
            let inner_variable_name = &inner[0];
            let variable_lvalue = expression(inner_variable_name);
            let inner_expression = expression(&inner[1]);
            match opcode.as_str() {
                "-=" => {
                    return vec![RemillSemanticsStatement::VarMinusEqual { expression: inner_expression, variable_lvalue }];
                }
                "+=" => {
                    return vec![RemillSemanticsStatement::VarPlusEqual { expression: inner_expression, variable_lvalue }];
                }
                "*=" => {
                    return vec![RemillSemanticsStatement::VarMulEqual { expression: inner_expression, variable_lvalue }];
                }
                "|=" => {
                    return vec![RemillSemanticsStatement::VarOrEqual { expression: inner_expression, variable_lvalue }];
                }
                "/=" => {
                    return vec![RemillSemanticsStatement::VarDivEqual { expression: inner_expression, variable_lvalue }];
                }
                other => todo!("{other:?}")
            }
        }
        ASTNodeCleanedUp::SwitchStmt { inner, .. } => {
            let expression = expression(&inner[0]);
            let mut statements = vec![];
            for inner in &inner[1..] {
                statements.extend(statement(inner));
            }
            return vec![RemillSemanticsStatement::SwitchStatement { expression, statements }];
        }
        ASTNodeCleanedUp::CaseStmt { inner, .. } => {
            let mut res = vec![];
            let case = expression(&inner[0]);
            res.push(RemillSemanticsStatement::CaseStatement { case });
            for inner in &inner[1..] {
                res.extend(statement(inner))
            }
            return res;
        }
        ASTNodeCleanedUp::DefaultStmt { inner, .. } => {
            let mut res = vec![];
            res.push(RemillSemanticsStatement::DefaultStatement {});
            for inner in inner {
                res.extend(statement(inner))
            }
            return res;
        }
        ASTNodeCleanedUp::BreakStmt { .. } => {
            return vec![RemillSemanticsStatement::BreakStatement {}];
        }
        ASTNodeCleanedUp::WhileStmt { inner, .. } => {
            let mut statements = vec![];
            let condition = expression(&inner[0]);
            for inner in &inner[1..] {
                statements.extend(statement(inner))
            }
            return vec![RemillSemanticsStatement::WhileStatement {
                condition,
                statements,
            }];
        }
        ASTNodeCleanedUp::CStyleCastExpr { inner, type_, .. } => {
            if type_.qual_type.as_str() == "void" {
                assert_eq!(inner.len(), 1);
                match &inner[0] {
                    ASTNodeCleanedUp::StmtExpr { inner, .. } => {
                        return statement(&inner[0]);
                    }
                    ASTNodeCleanedUp::DeclRefExpr {..} => {
                        return vec![RemillSemanticsStatement::ExprStatement { expression: expression(&inner[0]) }];
                    }
                    other => todo!("{other:?}")
                }
            } else {
                todo!()
            }
        }
        ASTNodeCleanedUp::CXXConstructExpr { inner, .. } => {
            let inner = &inner.as_ref().unwrap()[0];
            return vec![RemillSemanticsStatement::ExprStatement { expression: expression(inner) }];
        }
        other => todo!("{other:?}")
    }
}
