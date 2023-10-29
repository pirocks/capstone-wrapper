use bumpalo::Bump;
use std::collections::HashMap;
use std::ops::{Add, BitAnd, BitOr, Range};
use wrapper_common::registers::{Reg16WithRIP, Reg32WithRIP, Reg64WithRIP, Reg8};

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
        Self { specified_bytes }
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
        right: &'arena WordValue<'arena>,
    },
}

impl BitOr<BoolValue<'static>> for BoolValue<'static> {
    type Output = BoolValue<'static>;

    fn bitor(self, rhs: BoolValue<'_>) -> Self::Output {
        todo!()
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ByteValue<'arena> {
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
    IfElse {
        condition: &'arena BoolValue<'arena>,
        true_case: &'arena ByteValue<'arena>,
        false_case: &'arena ByteValue<'arena>,
    },
}

impl BitAnd for ByteValue<'static> {
    type Output = ByteValue<'static>;

    fn bitand(self, rhs: Self) -> Self::Output {
        todo!()
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
    IfElse {
        condition: &'arena BoolValue<'arena>,
        true_case: &'arena WordValue<'arena>,
        false_case: &'arena WordValue<'arena>,
    },
}

impl Add<WordValue<'static>> for WordValue<'static> {
    type Output = WordValue<'static>;

    fn add(self, rhs: WordValue<'static>) -> Self::Output {
        todo!()
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum DWordValue<'arena> {
    Constant(u32),
    And {
        left: &'arena DWordValue<'arena>,
        right: &'arena DWordValue<'arena>,
    },
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
    WriteLowerBits {
        prev: &'arena QWordValue<'arena>,
        lower: &'arena WordValue<'arena>,
    },
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

    pub fn a<T>(&self, to_alloc: T) -> &'arena T {
        todo!() /*self.bumpalo.alloc(to_alloc)*/
    }

    pub fn undefined_instruction_exception(&mut self) {
        self.pending_exception = true;
    }
}

pub struct SemanticsBuilder {}

impl SemanticsBuilder {
    pub fn new() -> Self {
        todo!()
    }

    pub fn undefined_exception_if_64_bit(&self) -> SemanticsBuilder32 {
        todo!()
    }

    //64 bit registers:

    pub(crate) fn rax(&self) -> QWordValue<'static> {
        todo!()
    }

    pub(crate) fn get_reg_64(&self, reg: Reg64WithRIP) -> QWordValue<'static> {
        todo!()
    }
}

impl SemanticBuilderCommon for SemanticsBuilder {}

pub struct SemanticsBuilder32 {}

impl SemanticsBuilder32 {
    pub fn emit_conditional(
        &self,
        condition: BoolValue<'static>,
        true_case: impl FnOnce(SemanticsBuilder32),
        false_case: impl FnOnce(SemanticsBuilder32),
    ) {
        todo!()
    }
}

impl SemanticBuilderCommon for SemanticsBuilder32 {}

pub trait SemanticBuilderCommon {
    fn constant<NumIn, Out>(&self, num_in: NumIn) -> Out {
        todo!()
    }

    //flags:
    fn af(&self) -> BoolValue<'static> {
        todo!()
    }

    fn set_af(&self, flag: BoolValue<'static>) {
        todo!()
    }

    fn cf(&self) -> BoolValue<'static> {
        todo!()
    }

    fn set_cf(&self, flag: BoolValue<'static>) {
        todo!()
    }

    // 8 bit regs:
    fn ah(&self) -> ByteValue<'static> {
        todo!()
    }

    fn al(&self) -> ByteValue<'static> {
        todo!()
    }

    fn set_al(&self, val: ByteValue<'static>) {
        todo!()
    }

    // 16 bit regs:
    fn ax(&self) -> WordValue<'static> {
        todo!()
    }

    fn set_ax(&self, val: WordValue<'static>) {
        todo!()
    }

    // generic regs:

    fn get_reg_8(&self, reg: Reg8) -> ByteValue<'static> {
        todo!()
    }

    fn get_reg_16(&self, reg: Reg16WithRIP) -> WordValue<'static> {
        todo!()
    }

    fn get_reg_32(&self, reg: Reg32WithRIP) -> DWordValue<'static> {
        todo!()
    }

    //comparison

    fn less<T>(&self, left: T, right: T) -> BoolValue<'static> {
        todo!()
    }

    fn equal<T>(&self, left: T, right: T) -> BoolValue<'static> {
        todo!()
    }
}
