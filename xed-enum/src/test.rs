use std::ffi::{c_uint, CStr};
use std::mem::MaybeUninit;

use capstone::arch::x86::ArchMode;
use capstone::prelude::BuildsCapstone;
use xed_sys::{XED_ADDRESS_WIDTH_64b, xed_convert_to_encoder_request, xed_decode, xed_decoded_inst_dump, xed_decoded_inst_t, xed_decoded_inst_zero_set_mode, xed_encode, xed_encoder_instruction_t, xed_encoder_request_t, xed_encoder_request_zero_set_mode, xed_error_enum_t2str, XED_ERROR_NONE, XED_ICLASS_ADD, xed_inst2, XED_MACHINE_MODE_LONG_64, XED_MAX_INSTRUCTION_BYTES, xed_reg, XED_REG_RBX, XED_REG_RCX, xed_state_init, xed_state_t, xed_state_zero, xed_tables_init};

use wrapper_common::registers::Reg32WithRIP;

use crate::{CMP, EncodeContext, X86Instruction};

pub struct EncodedInstr {
    bytes: [u8; 15],
    actual_instruction_len: usize,
}

fn encode_xed() -> EncodedInstr {
    crate::START.call_once(|| {
        unsafe { xed_tables_init(); }
    });

    let mut instr_bytes = [0u8; 15];
    let mut encoder_request: MaybeUninit<xed_encoder_request_t> = MaybeUninit::zeroed();
    let mut encoder_inst: MaybeUninit<xed_encoder_instruction_t> = MaybeUninit::zeroed();
    let mut xed_state: MaybeUninit<xed_state_t> = MaybeUninit::zeroed();
    let mut actual_len = 0;
    unsafe {
        xed_state_zero(xed_state.as_mut_ptr());
        xed_state_init(xed_state.as_mut_ptr(), XED_MACHINE_MODE_LONG_64, XED_ADDRESS_WIDTH_64b, XED_ADDRESS_WIDTH_64b);
        // xed_encoder_request_zero_set_mode(encoder_request.as_mut_ptr(), xed_state.as_ptr());
        // xed_encoder_request_set_iclass(encoder_request.as_mut_ptr(), XED_ICLASS_POP);
        // xed_encoder_request_set_operand_order(encoder_request.as_mut_ptr(), 0, XED_OPERAND_REG0);
        // xed_encoder_request_set_reg(encoder_request.as_mut_ptr(), XED_OPERAND_REG0, XED_REG_RAX);
        // xed_encoder_request_set_effective_operand_width(encoder_request.as_mut_ptr(), 64);
        /*xed_inst2(encoder_inst.as_mut_ptr(), xed_state.assume_init_ref().clone(), XED_ICLASS_MOV, 64,xed_reg(XED_REG_RAX),
                  xed_mem_bisd( XED_REG_RDX, XED_REG_RAX, 4, xed_disp(0x11223344, 32), 64));*/
        xed_inst2(encoder_inst.as_mut_ptr(), xed_state.assume_init_ref().clone(), XED_ICLASS_ADD, 64,
                  xed_reg(XED_REG_RBX), xed_reg(XED_REG_RCX));

        xed_encoder_request_zero_set_mode(encoder_request.as_mut_ptr(), xed_state.as_ptr());
        let convert_ok = xed_convert_to_encoder_request(encoder_request.as_mut_ptr(), encoder_inst.as_mut_ptr()) != 0;
        if !convert_ok {
            panic!()
        }


        // xed_encoder_request_set_operand_order(encoder_request.as_mut_ptr(), 1,XED_OPERAND_MEM0);
        // xed_encoder_request_set_mem0(encoder_request.as_mut_ptr());
        // xed_encoder_request_set_effective_address_size(encoder_request.as_mut_ptr(), 64);
        // xed_encoder_request_set_base0(encoder_request.as_mut_ptr(),XED_REG_RBX);
        let error = xed_encode(encoder_request.as_mut_ptr(), instr_bytes.as_mut_ptr(), XED_MAX_INSTRUCTION_BYTES, (&mut actual_len) as *mut usize as *mut c_uint);
        dbg!(CStr::from_ptr(xed_error_enum_t2str(error)).to_str().unwrap());
        dbg!(instr_bytes);
    }
    EncodedInstr {
        bytes: instr_bytes,
        actual_instruction_len: actual_len,
    }
}

#[test]
pub fn encode() {
    let EncodedInstr { bytes, actual_instruction_len } = encode_xed();
    let mut xed_decoded = MaybeUninit::uninit();
    let mut xed_state: MaybeUninit<xed_state_t> = MaybeUninit::zeroed();
    unsafe {
        xed_state_zero(xed_state.as_mut_ptr());
        xed_state_init(xed_state.as_mut_ptr(), XED_MACHINE_MODE_LONG_64, XED_ADDRESS_WIDTH_64b, XED_ADDRESS_WIDTH_64b);
        xed_decoded_inst_zero_set_mode(xed_decoded.as_mut_ptr(), xed_state.as_ptr());
    }
    let error = unsafe { xed_decode(xed_decoded.as_mut_ptr(), bytes.as_ptr(), actual_instruction_len as c_uint) };
    if error == XED_ERROR_NONE {
        decoded_dump(xed_decoded.as_ptr());
        let decoded = X86Instruction::from_xed(xed_decoded.as_ptr());
        dbg!(decoded);
        todo!()
    } else {
        panic!()
    }
}


#[test]
pub fn round_trip() {
    let mut encode_context = EncodeContext::new();
    let mut xed: xed_encoder_request_t = CMP::CMP_GPRV_GPRV_3B_32 {
        operand_0: Reg32WithRIP::EDX,
        operand_1: Reg32WithRIP::ESI,
    }.to_xed(&mut encode_context);
    let mut array = [0u8; 16];
    let mut len = 0;

    let error = unsafe { xed_encode((&mut xed) as *mut xed_encoder_request_t, array.as_mut_ptr(), 15, (&mut len) as *mut c_uint) };
    decoded_dump((&xed) as *const xed_encoder_request_t);
    dbg!(xed);
    if error != 0 {
        unsafe { dbg!(CStr::from_ptr(xed_error_enum_t2str(error)).to_str().unwrap()); }
        panic!();
    }
    let capstone = capstone::Capstone::new().x86().mode(ArchMode::Mode64).build().unwrap();
    let instrs = capstone.disasm_all(array.as_slice(), 0).unwrap();
    dbg!(instrs.iter().next().unwrap().to_string());
    let mut decoded = MaybeUninit::zeroed();
    unsafe { xed_decoded_inst_zero_set_mode(decoded.as_mut_ptr(), encode_context.xed_state.as_ptr()); }
    dbg!(&array);
    let error = unsafe { xed_decode(decoded.as_mut_ptr(), array.as_ptr(), len) };
    if error != 0 {
        unsafe { dbg!(CStr::from_ptr(xed_error_enum_t2str(error)).to_str().unwrap()); }
        panic!();
    }
    unsafe { dbg!(decoded.assume_init_ref()); }
    decoded_dump(decoded.as_ptr());
    let res = X86Instruction::from_xed(decoded.as_ptr());
    dbg!(res);
    todo!()
}

fn decoded_dump(decoded: *const xed_decoded_inst_t) {
    let mut chars = vec![0i8; 10000];
    unsafe { xed_decoded_inst_dump(decoded, chars.as_mut_ptr(), 10000); }
     unsafe { dbg!(CStr::from_ptr(chars.as_ptr()).to_str().unwrap()); }
}