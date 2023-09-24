use std::collections::HashMap;
use derive_visitor::{Drive, Visitor};

use crate::clang_json_defs::ASTNode;

#[derive(Visitor)]
#[visitor(ASTNode(enter))]
struct ASTNodeFunctionsExtractorVisitor {
    functions_by_id: HashMap<u64, ASTNode>,
}

impl ASTNodeFunctionsExtractorVisitor {
    fn enter_ast_node(&mut self, ast_node: &ASTNode) {
        match ast_node {
            ASTNode::FunctionDecl { id,inner,  .. } => {
                if inner.is_some(){
                    self.functions_by_id.insert(*id, ast_node.clone());
                }
            }
            _ => {}
        }
    }
}

pub fn functions_by_id(top_level: &ASTNode) -> HashMap<u64, ASTNode> {
    let mut visitor = ASTNodeFunctionsExtractorVisitor { functions_by_id: HashMap::new() };
    top_level.drive(&mut visitor);
    visitor.functions_by_id
}


#[derive(Visitor)]
#[visitor(ASTNode(enter))]
struct ASTNodeISELExtractorVisitor {
    isel_names: Vec<ASTNode>,
}

impl ASTNodeISELExtractorVisitor {
    fn enter_ast_node(&mut self, ast_node: &ASTNode) {
        match ast_node {
            ASTNode::VarDecl { name, .. } => {
                if let Some(name) = name.as_ref(){
                    if name.starts_with("ISEL"){
                        self.isel_names.push(ast_node.clone())
                    }
                }
            }
            _ => {}
        }
    }
}



pub fn isels(top_level: &ASTNode) -> Vec<ASTNode> {
    let mut visitor = ASTNodeISELExtractorVisitor { isel_names: vec![] };
    top_level.drive(&mut visitor);
    visitor.isel_names
}

pub fn extract_referenced_id_isel(isel: &ASTNode) -> u64 {
    if let ASTNode::VarDecl {inner,.. } = isel {
        let implicit_cast = &inner.as_ref().unwrap()[0];
        if let ASTNode::ImplicitCastExpr {inner, ..} = implicit_cast{
            let decl_ref_expr = &inner[0];
            if let ASTNode::DeclRefExpr {referenced_decl, ..} = decl_ref_expr{
                if let ASTNode::FunctionDecl {id, .. } = referenced_decl.as_ref(){
                    return *id;
                }
            }
        }
    }
    panic!()
}