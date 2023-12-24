use bumpalo::Bump;
use crate::semantics2::expression::Expression;

#[derive(Copy, Clone)]
pub struct Arena<'arena> {
    bump: &'arena Bump,
}

impl<'arena> Arena<'arena> {
    pub fn new(bump: &'arena Bump) -> Self {
        Self {
            bump,
        }
    }

    pub fn a<T>(&self, expr: T) -> &'arena T {
        self.bump.alloc(expr)
    }
}
