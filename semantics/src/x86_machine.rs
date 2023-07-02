use std::collections::HashMap;
use std::ops::Range;

pub struct MemorySpace<'arena> {
    //todo use a proper sparse vec for whole memory space
    specified_bytes: HashMap<Range<usize>, &'arena u8>
}

pub const ZERO_U8: u8  = 0;
pub const ZERO_U8_REF: &'static u8 = &ZERO_U8;

impl <'arena> MemorySpace<'arena> {
    pub fn new_all_zeroed() -> Self{
        let mut specified_bytes = HashMap::new();
        specified_bytes.insert(0..usize::MAX, ZERO_U8_REF);
        Self {
            specified_bytes,
        }
    }
}

pub const ZERO_U64: u64 = 0;
pub const ZERO_U64_REF: &'static u64 = &ZERO_U64;

pub enum QWordValue<'arena> {
    Constant(&'arena u64)
}


pub enum X86Mode{
    Real,
    _64Bit
}

pub struct X86MachineState<'arena> {
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
    pub fn new_all_zeroed(mode: X86Mode) -> Self{
        Self{
            mode,
            rax: QWordValue::Constant(ZERO_U64_REF),
            rbx: QWordValue::Constant(ZERO_U64_REF),
            rcx: QWordValue::Constant(ZERO_U64_REF),
            rdx: QWordValue::Constant(ZERO_U64_REF),
            rsi: QWordValue::Constant(ZERO_U64_REF),
            rdi: QWordValue::Constant(ZERO_U64_REF),
            rbp: QWordValue::Constant(ZERO_U64_REF),
            rsp: QWordValue::Constant(ZERO_U64_REF),
            r8: QWordValue::Constant(ZERO_U64_REF),
            r9: QWordValue::Constant(ZERO_U64_REF),
            r10: QWordValue::Constant(ZERO_U64_REF),
            r11: QWordValue::Constant(ZERO_U64_REF),
            r12: QWordValue::Constant(ZERO_U64_REF),
            r13: QWordValue::Constant(ZERO_U64_REF),
            r14: QWordValue::Constant(ZERO_U64_REF),
            r15: QWordValue::Constant(ZERO_U64_REF),
            rip: QWordValue::Constant(ZERO_U64_REF),
            memory: MemorySpace::new_all_zeroed(),
            pending_exception: false,
        }
    }

    pub fn undefined_instruction_exception(&mut self) {
        self.pending_exception = true;
    }
}
