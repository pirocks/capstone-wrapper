use std::collections::HashMap;

pub struct RawXedIndex(u32);

pub struct XedInstructionName(String);
pub struct XedInstructionVariantName(String);//aka iform

pub enum XedAccessType{
    Read,
    Write,
    ReadWrite
}

pub enum XedVisibility {
    Supressed,
    Explicit,
    Implicit
}

pub enum X86Extension{
    Base
}

pub enum XedOperandKind{
    REG,
    ImmConst,
    NtLookupFn,

}

pub struct XedOperand{
    visibility: XedVisibility,
    access_type: XedAccessType,
    extension: X86Extension,

}

pub struct XedInstructionVariant{
    index: RawXedIndex,
    operands: Vec<XedOperand>
}

pub struct XedInstructionData {
    instruction_variants: HashMap<XedInstructionVariantName, XedInstructionVariant>
}

pub struct XedTableData{
    instructions: HashMap<XedInstructionName, XedInstructionData>
}

fn test() {

}


#[cfg(test)]
mod tests {

    #[test]
    fn parse_builtin_table() {
        let table_str = include_str!("../data/xed-table");
    }
}
