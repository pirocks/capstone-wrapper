use std::collections::HashMap;
use std::ops::Range;
use bumpalo::Bump;

pub struct MemorySpace<'arena> {
    //todo use a proper sparse vec for whole memory space
    specified_bytes: HashMap<Range<usize>, &'arena u8>,
}

pub const ZERO_U8: u8 = 0;
pub const ZERO_U8_REF: &'static u8 = &ZERO_U8;

impl<'arena> MemorySpace<'arena> {
    pub fn new_all_zeroed() -> Self {
        let mut specified_bytes = HashMap::new();
        specified_bytes.insert(0..usize::MAX, ZERO_U8_REF);
        Self {
            specified_bytes,
        }
    }
}

pub const ZERO_U64: u64 = 0;
pub const ZERO_U64_REF: &'static u64 = &ZERO_U64;

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum BoolValue<'arena> {
    True,
    False,
    EqWord {
        left: &'arena WordValue<'arena>,
        right: &'arena WordValue<'arena>
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ByteValue<'arena>{
    Constant(u8),
    LowerBits(&'arena QWordValue<'arena>),
    And {
        left: &'arena ByteValue<'arena>,
        right: &'arena ByteValue<'arena>,
    },
    Or {
        left: &'arena ByteValue<'arena>,
        right: &'arena ByteValue<'arena>,
    },
    IfElse{
        condition: &'arena BoolValue<'arena>,
        true_case: &'arena ByteValue<'arena>,
        false_case: &'arena ByteValue<'arena>
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum WordValue<'arena> {
    Constant(u16),
    LowerBits(&'arena QWordValue<'arena>),
    And {
        left: &'arena WordValue<'arena>,
        right: &'arena WordValue<'arena>,
    },
    Or {
        left: &'arena WordValue<'arena>,
        right: &'arena WordValue<'arena>,
    },
    IfElse{
        condition: &'arena BoolValue<'arena>,
        true_case: &'arena WordValue<'arena>,
        false_case: &'arena WordValue<'arena>
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum QWordValue<'arena> {
    Constant(u64),
    And {
        left: &'arena QWordValue<'arena>,
        right: &'arena QWordValue<'arena>,
    },
    Or {
        left: &'arena QWordValue<'arena>,
        right: &'arena QWordValue<'arena>,
    },
    WriteLowerBits{
        prev: &'arena QWordValue<'arena>,
        lower: &'arena WordValue<'arena>
    }
}


pub enum X86Mode {
    Real,
    Protected,
    _64Bit,
}

pub struct X86MachineState<'arena> {
    bumpalo: &'arena mut Bump,
    pub(crate) mode: X86Mode,
    pub(crate) rax: QWordValue<'arena>,
    pub(crate) rbx: QWordValue<'arena>,
    pub(crate) rcx: QWordValue<'arena>,
    pub(crate) rdx: QWordValue<'arena>,
    pub(crate) rsi: QWordValue<'arena>,
    pub(crate) rdi: QWordValue<'arena>,
    pub(crate) rbp: QWordValue<'arena>,
    pub(crate) rsp: QWordValue<'arena>,
    pub(crate) r8: QWordValue<'arena>,
    pub(crate) r9: QWordValue<'arena>,
    pub(crate) r10: QWordValue<'arena>,
    pub(crate) r11: QWordValue<'arena>,
    pub(crate) r12: QWordValue<'arena>,
    pub(crate) r13: QWordValue<'arena>,
    pub(crate) r14: QWordValue<'arena>,
    pub(crate) r15: QWordValue<'arena>,
    pub(crate) rip: QWordValue<'arena>,
    pub(crate) memory: MemorySpace<'arena>,
    pub(crate) pending_exception: bool,
}

impl<'arena> X86MachineState<'arena> {
    pub fn new_all_zeroed(bump: &'arena mut Bump, mode: X86Mode) -> Self {
        Self {
            bumpalo: bump,
            mode,
            rax: QWordValue::Constant(0),
            rbx: QWordValue::Constant(0),
            rcx: QWordValue::Constant(0),
            rdx: QWordValue::Constant(0),
            rsi: QWordValue::Constant(0),
            rdi: QWordValue::Constant(0),
            rbp: QWordValue::Constant(0),
            rsp: QWordValue::Constant(0),
            r8: QWordValue::Constant(0),
            r9: QWordValue::Constant(0),
            r10: QWordValue::Constant(0),
            r11: QWordValue::Constant(0),
            r12: QWordValue::Constant(0),
            r13: QWordValue::Constant(0),
            r14: QWordValue::Constant(0),
            r15: QWordValue::Constant(0),
            rip: QWordValue::Constant(0),
            memory: MemorySpace::new_all_zeroed(),
            pending_exception: false,
        }
    }

    pub fn a<T>(&self, to_alloc: T) -> &'arena T{
        self.bumpalo.alloc(to_alloc)
    }

    pub fn undefined_instruction_exception(&mut self) {
        self.pending_exception = true;
    }
}
