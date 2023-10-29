use derive_visitor::{Drive, Visitor};
use std::collections::HashMap;

use crate::unneeded_data_stripped::ASTNodeCleanedUp;

#[derive(Visitor)]
#[visitor(ASTNodeCleanedUp(enter))]
struct ASTNodeCleanedUpFunctionsExtractorVisitor {
    functions_by_id: HashMap<u64, ASTNodeCleanedUp>,
}

impl ASTNodeCleanedUpFunctionsExtractorVisitor {
    fn enter_ast_node_cleaned_up(&mut self, ast_node: &ASTNodeCleanedUp) {
        match ast_node {
            ASTNodeCleanedUp::FunctionDecl { id, inner, .. } => {
                if inner.is_some() {
                    self.functions_by_id.insert(*id, ast_node.clone());
                }
            }
            _ => {}
        }
    }
}

pub fn functions_by_id(top_level: &ASTNodeCleanedUp) -> HashMap<u64, ASTNodeCleanedUp> {
    let mut visitor = ASTNodeCleanedUpFunctionsExtractorVisitor {
        functions_by_id: HashMap::new(),
    };
    top_level.drive(&mut visitor);
    visitor.functions_by_id
}

#[derive(Visitor)]
#[visitor(ASTNodeCleanedUp(enter))]
struct ASTNodeCleanedUpISELExtractorVisitor {
    isel_by_name: HashMap<String, ASTNodeCleanedUp>,
}

impl ASTNodeCleanedUpISELExtractorVisitor {
    fn enter_ast_node_cleaned_up(&mut self, ast_node: &ASTNodeCleanedUp) {
        match ast_node {
            ASTNodeCleanedUp::VarDecl { name, .. } => {
                if let Some(name) = name.as_ref() {
                    if name.starts_with("ISEL") {
                        self.isel_by_name.insert(name.to_string(), ast_node.clone());
                    }
                }
            }
            _ => {}
        }
    }
}

pub fn isels(top_level: &ASTNodeCleanedUp) -> HashMap<String, ASTNodeCleanedUp> {
    let mut visitor = ASTNodeCleanedUpISELExtractorVisitor {
        isel_by_name: HashMap::new(),
    };
    top_level.drive(&mut visitor);
    visitor.isel_by_name
}

pub fn extract_referenced_id_isel(isel: &ASTNodeCleanedUp) -> u64 {
    if let ASTNodeCleanedUp::VarDecl { inner, .. } = isel {
        let implicit_cast = &inner.as_ref().unwrap()[0];
        if let ASTNodeCleanedUp::ImplicitCastExpr { inner, .. } = implicit_cast {
            let decl_ref_expr = &inner[0];
            if let ASTNodeCleanedUp::DeclRefExpr {
                referenced_decl, ..
            } = decl_ref_expr
            {
                if let ASTNodeCleanedUp::FunctionDecl { id, .. } = referenced_decl.as_ref() {
                    return *id;
                }
            }
        }
    }
    panic!()
}
