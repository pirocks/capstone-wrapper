// use std::ffi::c_uint;
// use std::mem::MaybeUninit;
//
// use xed_sys::{xed_decoded_inst_get_iform_enum, xed_decoded_inst_get_immediate_width_bits, xed_decoded_inst_get_index_reg, xed_decoded_inst_get_memory_displacement, xed_decoded_inst_get_reg, xed_decoded_inst_get_scale, xed_decoded_inst_get_second_immediate, xed_decoded_inst_get_seg_reg, xed_decoded_inst_get_signed_immediate, xed_decoded_inst_get_unsigned_immediate, xed_decoded_inst_inst, xed_decoded_inst_noperands, xed_decoded_inst_operands_const, xed_decoded_inst_t, XED_ERROR_NONE, xed_inst_operand, xed_inst_t, xed_operand_is_memory_addressing_register, xed_operand_is_register, xed_operand_name, xed_reg_enum_t, XED_REG_INVALID, xed_uint_t};
//
// use wrapper_common::memory_operand::{GeneralRegister, MemoryOperand, X86Scale};
//
// use crate::operand_name::OperandName;
//
// pub unsafe fn xed_decode(bytes: &[u8]) {
//     let mut instr = MaybeUninit::uninit();
//     let err = xed_sys::xed_decode(instr.as_mut_ptr(), bytes.as_ptr(), bytes.len() as c_uint);
//     if err != XED_ERROR_NONE {
//         panic!()
//     }
//     let iform = xed_decoded_inst_get_iform_enum(instr.as_ptr());
//     let instr_template = xed_decoded_inst_inst(instr.as_ptr());
//     let no_operands = xed_decoded_inst_noperands(instr.as_ptr());
//     let operands = xed_decoded_inst_operands_const(instr.as_ptr());
//     // for i in 0..no_operands {
//     //     let operand_template = xed_inst_operand(instr_template, i);
//     //     let operand_name_raw = xed_operand_name(operand_template);
//     //     let operand_name = OperandName::new(operand_name_raw);
//     //     let seg: xed_reg_enum_t;
//     //     let base: xed_reg_enum_t;
//     //     let indx: xed_reg_enum_t;
//     //     // MemoryOperand{
//     //     //     base: (),
//     //     //     scale: X86Scale::One,
//     //     //     index: None,
//     //     //     offset: (),
//     //     // };
//     //     // xed_decoded_inst_get_base_reg();
//     //     // xed_decoded_inst_get_seg_reg();
//     //     // xed_decoded_inst_get_index_reg();
//     //     // xed_decoded_inst_get_memory_displacement();
//     //     // xed_decoded_inst_get_scale()
//     //     if xed_operand_is_register(operand_name_raw) != 0 {
//     //         let decoded = xed_decoded_inst_get_reg(instr.as_ptr(), operand_name_raw);
//     //         todo!()
//     //     } else {
//     //         todo!()
//     //     }
//     // }
// }
