#![feature(core_panic)]

use enum_generator::{make_enums, make_from_detail, top_level_make_from_detail};
use capstone::arch::x86::X86InsnDetail;
use capstone::arch::DetailsArchInsn;
use itertools::Itertools;
use wrapper_common::registers::*;
use wrapper_common::memory_operand::MemoryOperand;
use wrapper_common::operand_type::{MemoryOperandType, MemoryOperandTypeKind, OperandType};
use wrapper_common::operands::Operand;
use capstone::arch::x86::X86Operand;
use wrapper_common::operand_type::Agen;
use wrapper_common::operand_type::Imm;
use wrapper_common::operand_type::VectorRegisterKind;

make_enums!();
make_from_detail!();
top_level_make_from_detail!();


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
    asm!(".byte 'T', 'H', 'I', 'S', ' ', 'G', 'U', 'A', 'R', 'D', 'S', ' ', 'E', 'N', 'D'");
};
}

#[cfg(test)]
pub mod test;

