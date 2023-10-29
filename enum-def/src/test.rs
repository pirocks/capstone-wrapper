use capstone::arch;
use capstone::arch::x86::X86Insn;
use capstone::prelude::{BuildsCapstone, BuildsCapstoneSyntax};
use std::arch::asm;
use std::ffi::{c_uint, c_void, CStr};
use std::mem::MaybeUninit;
use std::ptr::slice_from_raw_parts;
use wrapper_common::instructions::Instructions;
use xed_sys::{
    xed_decode, xed_decoded_inst_get_iclass, xed_decoded_inst_get_length, xed_decoded_inst_inst,
    xed_decoded_inst_operand_element_type, xed_decoded_inst_zero, xed_error_enum_t2str,
    xed_iclass_enum_t2str, xed_inst_operand, xed_operand_element_type_enum_t2str, xed_operand_type,
    xed_operand_type_enum_t2str, XED_ERROR_NONE,
};

#[no_mangle]
fn sample_assembly() {
    unsafe {
        asm!(
            "pop rax",
            "mov rax, rcx",
            "shl r8w, 1",
            "add rax, rbx",
            "xchg eax, ecx",
            "imul ecx, [eax + 7], 1",
        );
        function_end_guard!();
    }
}

#[test]
#[ignore]
pub fn disassemble_sample_assembly() {
    let raw_function_ptr = sample_assembly as *const c_void;
    let function_bytes = get_function_bytes(raw_function_ptr);
    let res = disassemble(function_bytes, raw_function_ptr as u64).unwrap();
}

#[test]
pub fn disassemble_asm_xed() {
    let mut decoded = MaybeUninit::uninit();
    let raw_function_ptr = sample_assembly as *const c_void;
    let mut function_bytes = get_function_bytes(raw_function_ptr);
    dbg!(function_bytes);
    unsafe {
        xed_decoded_inst_zero(decoded.as_mut_ptr());
    }
    let _err = unsafe { xed_decode(decoded.as_mut_ptr(), function_bytes.as_ptr(), 1) };
    unsafe {
        function_bytes =
            &function_bytes[dbg!(xed_decoded_inst_get_length(decoded.as_mut_ptr()) as usize)..];
    }
    unsafe {
        xed_decoded_inst_zero(decoded.as_mut_ptr());
    }
    let err = unsafe { xed_decode(decoded.as_mut_ptr(), function_bytes.as_ptr(), 1) };
    if err != XED_ERROR_NONE {
        unsafe {
            dbg!(CStr::from_ptr(xed_error_enum_t2str(err)).to_str().unwrap());
        }
        panic!()
    }
    let inst = unsafe { xed_decoded_inst_inst(decoded.as_ptr()) };
    unsafe {
        dbg!(
            CStr::from_ptr(xed_iclass_enum_t2str(xed_decoded_inst_get_iclass(
                decoded.as_ptr()
            )))
            .to_str()
            .unwrap()
        );

        let operand_0 = xed_inst_operand(inst, 0);
        let operand_1 = xed_inst_operand(inst, 1);
        dbg!(
            CStr::from_ptr(xed_operand_type_enum_t2str(xed_operand_type(operand_0)))
                .to_str()
                .unwrap()
        );
        dbg!(
            CStr::from_ptr(xed_operand_type_enum_t2str(xed_operand_type(operand_1)))
                .to_str()
                .unwrap()
        );
    }
}

pub fn disassemble(bytes: &[u8], address: u64) -> anyhow::Result<Vec<Instructions>> {
    let capstone = capstone::Capstone::new()
        .x86()
        .mode(arch::x86::ArchMode::Mode64)
        .syntax(arch::x86::ArchSyntax::Intel)
        .detail(true)
        .build()
        .expect("Shouldn't fail to instantiate capstone.");
    let instructions = capstone.disasm_all(bytes, address).unwrap();
    instructions
        .iter()
        .map(|instruction| {
            let instruction_type = X86Insn::from(instruction.id().0);
            let details = capstone.insn_detail(instruction).unwrap();
            let arch_detail = details.arch_detail();
            let x86_detail = arch_detail.x86().unwrap();
            Ok(
                todo!(), /*Instructions::from_detail(instruction_type, &x86_detail)*/
            )
        })
        .collect::<Result<Vec<_>, _>>()
}

fn find_subsequence(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    haystack
        .windows(needle.len())
        .position(|window| window == needle)
}

pub fn get_function_bytes(raw_function_ptr: *const c_void) -> &'static [u8] {
    const LEN_MAX: usize = 100_000;
    let raw_function_ptr = raw_function_ptr as *const u8;
    let function_bytes =
        unsafe { slice_from_raw_parts(raw_function_ptr, LEN_MAX).as_ref() }.unwrap();
    // Safety: the search for function end exits immediately when the end is found, so we won't access
    // bytes outside the function.
    let function_end = find_subsequence(function_bytes, b"THIS GUARDS END").unwrap();
    let function_bytes = &function_bytes[..function_end];
    function_bytes
}
