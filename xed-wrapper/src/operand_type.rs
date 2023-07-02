use xed_sys::{xed_operand_type_enum_t, XED_OPERAND_TYPE_IMM, XED_OPERAND_TYPE_IMM_CONST, XED_OPERAND_TYPE_NT_LOOKUP_FN, XED_OPERAND_TYPE_NT_LOOKUP_FN2, XED_OPERAND_TYPE_NT_LOOKUP_FN4, XED_OPERAND_TYPE_REG};

#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum XedOperandType {
    IMM,
    IMM_CONST,
    NT_LOOKUP_FN,
    NT_LOOKUP_FN2,
    NT_LOOKUP_FN4,
    REG,
}

impl XedOperandType {
    pub fn new(xed: xed_operand_type_enum_t) -> Self {
        match xed {
            XED_OPERAND_TYPE_IMM => Self::IMM,
            XED_OPERAND_TYPE_IMM_CONST => Self::IMM_CONST,
            XED_OPERAND_TYPE_NT_LOOKUP_FN => Self::NT_LOOKUP_FN,
            XED_OPERAND_TYPE_NT_LOOKUP_FN2 => Self::NT_LOOKUP_FN2,
            XED_OPERAND_TYPE_NT_LOOKUP_FN4 => Self::NT_LOOKUP_FN4,
            XED_OPERAND_TYPE_REG => Self::REG,
            _ => panic!()
        }
    }
}