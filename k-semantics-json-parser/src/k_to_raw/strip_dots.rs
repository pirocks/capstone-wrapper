use crate::k_expressions::KExpression;
use crate::k_to_raw::extract_register_expression::empty_kapply;
use itertools::Itertools;

pub(crate) fn remove_dots_and_nodots(exprs: &[KExpression]) -> Vec<KExpression> {
    exprs.iter()
        .filter(|expr|expr != &&empty_kapply("#noDots"))
        .filter(|expr|expr != &&empty_kapply("#dots"))
        .cloned()
        .collect_vec()
}