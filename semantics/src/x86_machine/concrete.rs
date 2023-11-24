use std::collections::HashMap;
use std::ops::Range;
use crate::x86_machine::X86Mode;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ConcreteX86MachineState {
    pub(crate) mode: X86Mode,
    pub(crate) rax: u64,
    pub(crate) rbx: u64,
    pub(crate) rcx: u64,
    pub(crate) rdx: u64,
    pub(crate) rsi: u64,
    pub(crate) rdi: u64,
    pub(crate) rbp: u64,
    pub(crate) rsp: u64,
    pub(crate) r8: u64,
    pub(crate) r9: u64,
    pub(crate) r10: u64,
    pub(crate) r11: u64,
    pub(crate) r12: u64,
    pub(crate) r13: u64,
    pub(crate) r14: u64,
    pub(crate) r15: u64,
    pub(crate) rip: u64,
    pub(crate) flags: ConcreteFlags,
    pub(crate) memory: ConcreteMemorySpace,
    pub(crate) pending_exception: bool,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ConcreteFlags {
    pub(crate) pf: bool,
    pub(crate) of: bool,
    pub(crate) af: bool,
    pub(crate) zf: bool,
    pub(crate) sf: bool,
    pub(crate) cf: bool,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ConcreteMemorySpace {
    //todo use a proper sparse vec for whole memory space
    pub(crate) specified_bytes: HashMap<Range<usize>, u8>,
}
