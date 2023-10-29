use std::arch::asm;
use std::ffi::c_void;

use crate::utils::get_function_bytes;
use crate::{disassemble, function_end_guard};

#[no_mangle]
fn sample_assembly() {
    unsafe {
        asm!("mov rax, rcx", "shl r8w, 1", "add rax, rbx",);
        function_end_guard!();
    }
}

#[test]
pub fn disassemble_sample_assembly() {
    let raw_function_ptr = sample_assembly as *const c_void;
    let function_bytes = get_function_bytes(raw_function_ptr);
    let res = disassemble(function_bytes, raw_function_ptr as u64).unwrap();
}
