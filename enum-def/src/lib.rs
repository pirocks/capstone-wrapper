#![feature(core_panic)]

use enum_generator::{make_enums, make_from_detail};
use capstone::arch::x86::X86InsnDetail;
use capstone::arch::DetailsArchInsn;
use itertools::Itertools;
use wrapper_common::registers::*;
use wrapper_common::memory_operand::MemoryOperand;

make_enums!();
make_from_detail!();

#[cfg(test)]
pub mod test {
    use crate::*;

    #[test]
    #[allow(unused)]
    pub fn foo() {
        todo!()
    }

    pub mod another;
}

