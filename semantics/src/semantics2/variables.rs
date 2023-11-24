use std::collections::HashMap;
use crate::semantics2::value::Value;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct VariableName(u64);


pub struct VariableMappings<'arena> {
    pub(crate) inner: HashMap<VariableName, Value<'arena>>
}
