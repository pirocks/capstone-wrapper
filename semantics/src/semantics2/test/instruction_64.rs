use std::arch::asm;
use std::arch::x86_64::__cpuid;
use std::ffi::{c_uint, CStr};
use std::ptr::null_mut;
use xed_sys::{xed_encode, xed_error_enum_t2str};
use wrapper_common::memory_operand::{GeneralReg, X86Scale};
use wrapper_common::registers::Reg64WithRIP;
use xed_enum::{EncodeDecodeContext, JMP, X86Instruction};
use xed_wrapper::operands::MemoryOperands;
use crate::semantics2::state::{ConcreteFlags, ConcreteX86MachineState64};

#[repr(C, align(64))]
pub struct Registers {
    rsp: u64,
    rax: u64,
    rbx: u64,
    rcx: u64,
    rdx: u64,
    rsi: u64,
    rdi: u64,
    rbp_unused: u64,
    r8: u64,
    r9: u64,
    r10: u64,
    r11: u64,
    r12: u64,
    r13: u64,
    r14: u64,
    flags: u64,
    rip: u64,
    return_address: u64,
    zmms: [[u64;8];32],
}

pub(crate) fn run_instruction_64(instr: X86Instruction, start: ConcreteX86MachineState64) -> ConcreteX86MachineState64 {
    let mut encode = EncodeDecodeContext::new();
    let mut array = [0u8; 32];
    let mut encoder_request = instr.to_xed(&mut encode);
    let mut olen = 0;
    let err = unsafe { xed_encode(&mut encoder_request, array.as_mut_ptr(), array.len() as c_uint, &mut olen) };
    if err != 0 {
        panic!()
    }
    let mut encoder_request = X86Instruction::JMP(JMP::JMP_MEMV_64 {
        operand_0: MemoryOperands::SIBAddressing {
            segment: None,
            scale: X86Scale::One,
            index: None,
            base: GeneralReg::Reg64(Reg64WithRIP::R15),
            disp: 136,
            disp_width: 32,
        }
    }).to_xed(&mut encode);
    let err = unsafe { xed_encode(&mut encoder_request, array[(olen as usize)..].as_mut_ptr(), array.len() as c_uint, &mut olen) };
    if err != 0 {
        unsafe { dbg!(CStr::from_ptr(unsafe { xed_error_enum_t2str(err) }).to_str().unwrap()); }
        panic!()
    }
    let instructions = unsafe {
        libc::mmap(null_mut(), 4096, libc::PROT_READ | libc::PROT_WRITE | libc::PROT_EXEC, libc::MAP_ANON | libc::MAP_PRIVATE, -1, 0)
    };
    if instructions.is_null() {
        panic!()
    }
    unsafe { libc::memmove(instructions, array.as_ptr() as *const libc::c_void, array.len()); }

    unsafe { std::hint::black_box(__cpuid(0)); }

    let mut stack = unsafe {
        libc::mmap(null_mut(), 4096, libc::PROT_READ | libc::PROT_WRITE, libc::MAP_ANON | libc::MAP_PRIVATE, -1, 0)
    };
    if stack.is_null() {
        panic!()
    }
    unsafe {
        libc::memset(stack, 0, 4096);
        stack = stack.add(2048);
        let align = stack.align_offset(64);
        stack = stack.add(align);
    }
    let mut zmms = start.zmms;
    let mut registers = Registers {
        rsp: stack as u64,
        rax: start.rax,
        rbx: start.rbx,
        rcx: start.rcx,
        rdx: start.rdx,
        rsi: start.rsi,
        rdi: start.rdi,
        rbp_unused: 0,
        r8: start.r8,
        r9: start.r9,
        r10: start.r10,
        r11: start.r11,
        r12: start.r12,
        r13: start.r13,
        r14: start.r14,
        flags: start.flags.to_u64(),
        rip: instructions as u64,
        return_address: 0,
        zmms,
    };
    run_instruction_64_impl(&mut registers);
    ConcreteX86MachineState64 {
        rax: registers.rax,
        rbx: registers.rbx,
        rcx: registers.rcx,
        rdx: registers.rdx,
        rsi: registers.rsi,
        rdi: registers.rdi,
        rsp: registers.rsp,
        rbp: registers.rbp_unused,
        r8: registers.r8,
        r9: registers.r9,
        r10: registers.r10,
        r11: registers.r11,
        r12: registers.r12,
        r13: registers.r13,
        r14: registers.r14,
        r15: 0,
        rip: registers.rip,
        flags: ConcreteFlags {
            cf: registers.flags & 1 != 0,
            pf: registers.flags & 4 != 0,
            af: registers.flags & 16 != 0,
            zf: registers.flags & 64 != 0,
            sf: registers.flags & 128 != 0,
            of: registers.flags & 2048 != 0,
        },
        zmms: registers.zmms,
    }
}

#[inline(never)]
#[allow(named_asm_labels)]
fn run_instruction_64_impl(registers: &mut Registers) {
    unsafe {
        let registers_ptr = ((registers) as *mut Registers);
        asm!(
        "push r15",
        "push rbx",
        "mov r15, {0}",
        "xchg rsp, [r15]",
        "mov rax, [r15 + 120]",
        "push rax",
        "popfq",
        "lea rax, [rip + ___capstone_wrapper_semantics_test_internal]",
        "mov [r15 + 136], rax",
        "mov rax, [r15 + 8]",
        "mov rbx, [r15 + 16]",
        "mov rcx, [r15 + 24]",
        "mov rdx, [r15 + 32]",
        "mov rsi, [r15 + 40]",
        "mov rdi, [r15 + 48]",
        // "mov rbp, [r15 + 56]",
        "mov r8, [r15 + 64]",
        "mov r9, [r15 + 72]",
        "mov r10, [r15 + 80]",
        "mov r11, [r15 + 88]",
        "mov r12, [r15 + 96]",
        "mov r13, [r15 + 104]",
        "mov r14, [r15 + 112]",
        "vmovdqu64 zmm0, [r15 + 144]",
        "vmovdqu64 zmm1, [r15 + 144 + 1*64]",
        "vmovdqu64 zmm2, [r15 + 144 + 2*64]",
        "vmovdqu64 zmm3, [r15 + 144 + 3*64]",
        "vmovdqu64 zmm4, [r15 + 144 + 4*64]",
        "vmovdqu64 zmm5, [r15 + 144 + 5*64]",
        "vmovdqu64 zmm6, [r15 + 144 + 6*64]",
        "vmovdqu64 zmm7, [r15 + 144 + 7*64]",
        "vmovdqu64 zmm8, [r15 + 144 + 8*64]",
        "vmovdqu64 zmm9, [r15 + 144 + 9*64]",
        "vmovdqu64 zmm10, [r15 + 144 + 10*64]",
        "vmovdqu64 zmm11, [r15 + 144 + 11*64]",
        "vmovdqu64 zmm12, [r15 + 144 + 12*64]",
        "vmovdqu64 zmm13, [r15 + 144 + 13*64]",
        "vmovdqu64 zmm14, [r15 + 144 + 14*64]",
        "vmovdqu64 zmm15, [r15 + 144 + 15*64]",
        "vmovdqu64 zmm16, [r15 + 144 + 16*64]",
        "vmovdqu64 zmm17, [r15 + 144 + 17*64]",
        "vmovdqu64 zmm18, [r15 + 144 + 18*64]",
        "vmovdqu64 zmm19, [r15 + 144 + 19*64]",
        "vmovdqu64 zmm20, [r15 + 144 + 20*64]",
        "vmovdqu64 zmm21, [r15 + 144 + 21*64]",
        "vmovdqu64 zmm22, [r15 + 144 + 22*64]",
        "vmovdqu64 zmm23, [r15 + 144 + 23*64]",
        "vmovdqu64 zmm24, [r15 + 144 + 24*64]",
        "vmovdqu64 zmm25, [r15 + 144 + 25*64]",
        "vmovdqu64 zmm26, [r15 + 144 + 26*64]",
        "vmovdqu64 zmm27, [r15 + 144 + 27*64]",
        "vmovdqu64 zmm28, [r15 + 144 + 28*64]",
        "vmovdqu64 zmm29, [r15 + 144 + 29*64]",
        "vmovdqu64 zmm30, [r15 + 144 + 30*64]",
        "vmovdqu64 zmm31, [r15 + 144 + 31*64]",
        "jmp [r15 + 128]",
        "___capstone_wrapper_semantics_test_internal:",
        "xchg rsp, [r15]",
        "pushfq",
        "pop qword ptr [r15 + 120]",
        "mov [r15 + 8], rax",
        "mov [r15 + 16], rbx",
        "mov [r15 + 24], rcx",
        "mov [r15 + 32], rdx",
        "mov [r15 + 40], rsi",
        "mov [r15 + 48], rdi",
        // "mov [r15 + 56], rbp",
        "mov [r15 + 64], r8",
        "mov [r15 + 72], r9",
        "mov [r15 + 80], r10",
        "mov [r15 + 88], r11",
        "mov [r15 + 96], r12",
        "mov [r15 + 104], r13",
        "mov [r15 + 112], r14",
        "vmovdqu64 [r15 + 144], zmm0",
        "vmovdqu64 [r15 + 144 + 1*64], zmm1",
        "vmovdqu64 [r15 + 144 + 2*64], zmm2",
        "vmovdqu64 [r15 + 144 + 3*64], zmm3",
        "vmovdqu64 [r15 + 144 + 4*64], zmm4",
        "vmovdqu64 [r15 + 144 + 5*64], zmm5",
        "vmovdqu64 [r15 + 144 + 6*64], zmm6",
        "vmovdqu64 [r15 + 144 + 7*64], zmm7",
        "vmovdqu64 [r15 + 144 + 8*64], zmm8",
        "vmovdqu64 [r15 + 144 + 9*64], zmm9",
        "vmovdqu64 [r15 + 144 + 10*64], zmm10",
        "vmovdqu64 [r15 + 144 + 11*64], zmm11",
        "vmovdqu64 [r15 + 144 + 12*64], zmm12",
        "vmovdqu64 [r15 + 144 + 13*64], zmm13",
        "vmovdqu64 [r15 + 144 + 14*64], zmm14",
        "vmovdqu64 [r15 + 144 + 15*64], zmm15",
        "vmovdqu64 [r15 + 144 + 16*64], zmm16",
        "vmovdqu64 [r15 + 144 + 17*64], zmm17",
        "vmovdqu64 [r15 + 144 + 18*64], zmm18",
        "vmovdqu64 [r15 + 144 + 19*64], zmm19",
        "vmovdqu64 [r15 + 144 + 20*64], zmm20",
        "vmovdqu64 [r15 + 144 + 21*64], zmm21",
        "vmovdqu64 [r15 + 144 + 22*64], zmm22",
        "vmovdqu64 [r15 + 144 + 23*64], zmm23",
        "vmovdqu64 [r15 + 144 + 24*64], zmm24",
        "vmovdqu64 [r15 + 144 + 25*64], zmm25",
        "vmovdqu64 [r15 + 144 + 26*64], zmm26",
        "vmovdqu64 [r15 + 144 + 27*64], zmm27",
        "vmovdqu64 [r15 + 144 + 28*64], zmm28",
        "vmovdqu64 [r15 + 144 + 29*64], zmm29",
        "vmovdqu64 [r15 + 144 + 30*64], zmm30",
        "vmovdqu64 [r15 + 144 + 31*64], zmm31",
        "pop rbx",
        "pop r15",
        in(reg) registers_ptr,
        out("zmm0") _,
        out("zmm1") _,
        out("zmm2") _,
        out("zmm3") _,
        out("zmm4") _,
        out("zmm5") _,
        out("zmm6") _,
        out("zmm8") _,
        out("zmm9") _,
        out("zmm10") _,
        out("zmm11") _,
        out("zmm12") _,
        out("zmm13") _,
        out("zmm14") _,
        out("zmm15") _,
        out("zmm16") _,
        out("zmm17") _,
        out("zmm18") _,
        out("zmm19") _,
        out("zmm20") _,
        out("zmm21") _,
        out("zmm22") _,
        out("zmm23") _,
        out("zmm24") _,
        out("zmm25") _,
        out("zmm26") _,
        out("zmm27") _,
        out("zmm28") _,
        out("zmm29") _,
        out("zmm30") _,
        out("zmm31") _,
        out("rax") _,
        // out("rbx") _,
        out("rcx") _,
        out("rdx") _,
        out("rsi") _,
        out("rdi") _,
        out("r8") _,
        out("r9") _,
        out("r10") _,
        out("r11") _,
        out("r12") _,
        out("r13") _,
        out("r14") _,
        out("r15") _,
        )
    }
}
