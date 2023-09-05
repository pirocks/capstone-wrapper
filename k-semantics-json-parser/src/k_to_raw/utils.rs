use crate::k_expressions::{KExpression, KSentence};

pub(crate) fn has_a_label_expr(expr: &KExpression, label: &str) -> bool {
    match expr {
        KExpression::KApply { label, args, .. } => {
            if label.as_str() == "execinstr" {
                return true;
            }
            for arg in args {
                if has_a_label_expr(arg, label) {
                    return true;
                }
            }
            false
        }
        KExpression::KVariable { .. } => {
            false
        }
        KExpression::KToken { .. } => {
            false
        }
        KExpression::KRewrite { lhs, rhs } => has_a_label_expr(lhs, label) || has_a_label_expr(rhs, label),
        KExpression::KSequence { items, .. } => {
            for x in items {
                if has_a_label_expr(x, label) {
                    return true;
                }
            }
            false
        }
    }
}

pub fn has_execinstr_label(sentence: &KSentence, label: &str) -> bool {
    match sentence {
        KSentence::KProduction { .. } => {
            false
        }
        KSentence::KModuleComment { .. } => false,
        KSentence::KSyntaxSort { .. } => false,
        KSentence::KRule { body, requires, ensures, .. } => {
            has_a_label_expr(body, label) || has_a_label_expr(requires, label) || has_a_label_expr(ensures, label)
        }
        KSentence::KSyntaxAssociativity { .. } => false,
        KSentence::KSyntaxPriority { .. } => false,
        KSentence::KContext { body, requires, .. } => {
            has_a_label_expr(body, label) || has_a_label_expr(requires, label)
        }
        KSentence::KBubble { .. } => false
    }
}


pub fn assert_token_is_true(expr: &KExpression) {
    match expr {
        KExpression::KToken { sort, token } => {
            assert_eq!(sort.as_str(), "Bool");
            assert_eq!(token.as_str(), "true");
        }
        KExpression::KApply { .. } |
        KExpression::KVariable { .. } |
        KExpression::KRewrite { .. } |
        KExpression::KSequence { .. } => panic!()
    }
}

pub fn extract_apply_args<'l>(expr: &'l KExpression, expected_label: &str) -> &'l [KExpression] {
    match expr {
        KExpression::KApply { label, variable: _, arity: _, args } => {
            assert_eq!(label.as_str(), expected_label);
            args
        }
        _ => panic!()
    }
}

pub fn extract_apply_label(expr: &KExpression) -> &str {
    match expr {
        KExpression::KApply { label, .. } => {
            label.as_str()
        }
        _ => panic!()
    }
}


pub fn single_extract<T>(arr: &[T]) -> &T{
    assert_eq!(arr.len(), 1);
    &arr[0]
}