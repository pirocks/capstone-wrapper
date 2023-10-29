use std::collections::HashMap;

pub struct RawXedIndex(u32);

pub struct XedInstructionName(String);

pub struct XedInstructionVariantName(String); //aka iform

pub enum XedAccessType {
    Read,
    Write,
    ReadWrite,
}

pub enum XedVisibility {
    Supressed,
    Explicit,
    Implicit,
}

pub enum X86Extension {
    Base,
}

pub enum XedOperandKind {
    REG,
    ImmConst,
    NtLookupFn,
}

pub struct XedOperand {
    visibility: XedVisibility,
    access_type: XedAccessType,
    extension: X86Extension,
}

pub struct XedInstructionVariant {
    index: RawXedIndex,
    operands: Vec<XedOperand>,
}

pub struct XedInstructionData {
    instruction_variants: HashMap<XedInstructionVariantName, XedInstructionVariant>,
}

pub struct XedTableData {
    instructions: HashMap<XedInstructionName, XedInstructionData>,
}

fn test() {}

#[cfg(test)]
mod tests {
    use std::ffi::c_uint;
    use std::mem::MaybeUninit;

    use xed_sys::{
        xed_encode, xed_encoder_request_set_base0, xed_encoder_request_set_effective_address_size,
        xed_encoder_request_set_iclass, xed_encoder_request_set_mem0,
        xed_encoder_request_set_operand_order, xed_encoder_request_set_reg, xed_encoder_request_t,
        xed_iclass_enum_t, XED_ICLASS_ADC, XED_OPERAND_MEM0, XED_OPERAND_MEM1, XED_OPERAND_REG0,
        XED_OPERAND_REG1, XED_REG_RAX, XED_REG_RBX,
    };

    #[test]
    fn parse_builtin_table() {
        let table_str = include_str!("../data/xed-table");
    }
}
