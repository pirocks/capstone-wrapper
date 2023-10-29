use std::ffi::c_void;
use std::ptr::slice_from_raw_parts;

fn find_subsequence(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    haystack
        .windows(needle.len())
        .position(|window| window == needle)
}

#[macro_export]
macro_rules! function_end_guard {
($var: expr) => {
    use std::arch::asm;
    unsafe {
        asm!(
            "mov rax, {0}",
            "ret",
            ".byte 'T', 'H', 'I', 'S', ' ', 'G', 'U', 'A', 'R', 'D', 'S', ' ', 'E', 'N', 'D'",
            in(reg) $var
        )
    };
};
() => {
    use std::arch::asm;
    unsafe {asm!(".byte 'T', 'H', 'I', 'S', ' ', 'G', 'U', 'A', 'R', 'D', 'S', ' ', 'E', 'N', 'D'")};
};
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

pub fn imm_i8(imm: i64) -> i8 {
    if imm < 0 {
        imm.try_into()
            .expect("Unexpectedly large constant from capstone")
    } else {
        let unsigned: u8 = imm
            .try_into()
            .expect("Unexpectedly large constant from capstone");
        unsigned as i8
    }
}

pub fn imm_i16(imm: i64) -> i16 {
    if imm < 0 {
        imm.try_into()
            .expect("Unexpectedly large constant from capstone")
    } else {
        let unsigned: u16 = imm
            .try_into()
            .expect("Unexpectedly large constant from capstone");
        unsigned as i16
    }
}

pub fn imm_i32(imm: i64) -> i32 {
    if imm < 0 {
        imm.try_into()
            .expect("Unexpectedly large constant from capstone")
    } else {
        let unsigned: u32 = imm
            .try_into()
            .expect("Unexpectedly large constant from capstone");
        unsigned as i32
    }
}
