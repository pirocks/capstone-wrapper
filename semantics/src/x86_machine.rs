use std::collections::HashMap;
use std::ops::{Add, BitAnd, BitOr, Range};

use bumpalo::Bump;

use wrapper_common::registers::Reg8;

use crate::x86_machine::concrete::{ConcreteFlags, ConcreteMemorySpace, ConcreteX86MachineState};
use crate::x86_machine::semantics_builder::ArenaRef;
use crate::x86_machine::values::{BoolValue, ByteValue, DWordValue, QWordValue, WordValue};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct QWordVariable(pub usize);

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct DWordVariable(pub usize);

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct WordVariable(pub usize);

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct ByteVariable(pub usize);


pub struct VariableMappings<'arena> {
    pub(crate) qword_variables: HashMap<QWordVariable, &'arena QWordValue<'arena>>,
    pub(crate) dword_variables: HashMap<DWordVariable, &'arena DWordValue<'arena>>,
    pub(crate) word_variables: HashMap<WordVariable, &'arena WordValue<'arena>>,
    pub(crate) byte_variables: HashMap<ByteVariable, &'arena ByteValue<'arena>>,
}

impl<'arena> VariableMappings<'arena> {
    pub fn new() -> Self {
        Self {
            qword_variables: Default::default(),
            dword_variables: Default::default(),
            word_variables: Default::default(),
            byte_variables: Default::default(),
        }
    }

    pub fn from_qword_variables(qword_variables: HashMap<QWordVariable, &'arena QWordValue<'arena>>) -> Self {
        Self {
            qword_variables,
            dword_variables: Default::default(),
            word_variables: Default::default(),
            byte_variables: Default::default(),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct MemorySpace<'arena> {
    //todo use a proper sparse vec for whole memory space
    specified_bytes: HashMap<Range<usize>, &'arena u8>,
}

impl<'arena> MemorySpace<'arena> {
    pub fn make_concrete(&self, _variable_mappings: &VariableMappings) -> ConcreteMemorySpace {
        ConcreteMemorySpace {
            specified_bytes: self.specified_bytes.iter().map(|(range, value)| {
                (range.clone(), **value)
            }).collect()
        }
    }
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


pub mod values;
pub mod accessors;
pub mod read_write;


#[derive(Debug, Clone, Eq, PartialEq)]
pub enum X86Mode {
    Real,
    Protected,
    _64Bit,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Flags<'arena> {
    pub(crate) pf: BoolValue<'arena>,
    pub(crate) of: BoolValue<'arena>,
    pub(crate) af: BoolValue<'arena>,
    pub(crate) zf: BoolValue<'arena>,
    pub(crate) sf: BoolValue<'arena>,
    pub(crate) cf: BoolValue<'arena>,
}

impl<'arena> Flags<'arena> {
    pub(crate) fn make_concrete(&self, variable_mappings: &VariableMappings<'arena>) -> ConcreteFlags {
        ConcreteFlags {
            pf: self.pf.make_concrete(variable_mappings),
            of: self.of.make_concrete(variable_mappings),
            af: self.af.make_concrete(variable_mappings),
            zf: self.zf.make_concrete(variable_mappings),
            sf: self.sf.make_concrete(variable_mappings),
            cf: self.cf.make_concrete(variable_mappings),
        }
    }
}

#[derive(Debug, Clone)]
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
    pub(crate) flags: Flags<'arena>,
    pub(crate) memory: MemorySpace<'arena>,
    pub(crate) pending_exception: bool,
    pub(crate) arena: &'arena Bump,
}

impl<'arena> X86MachineState<'arena> {
    pub fn new_all_zeroed(arena: &'arena Bump, mode: X86Mode) -> Self {
        Self {
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
            flags: Flags {
                pf: BoolValue::False,
                of: BoolValue::False,
                af: BoolValue::False,
                zf: BoolValue::False,
                sf: BoolValue::False,
                cf: BoolValue::False,

            },
            memory: MemorySpace::new_all_zeroed(),
            pending_exception: false,
            arena,
        }
    }

    pub fn a<T>(&self, to_alloc: T) -> &'arena T {
        self.arena.alloc(to_alloc)
    }

    pub fn undefined_instruction_exception(&mut self) {
        self.pending_exception = true;
    }

    pub fn make_concrete(&self, variable_mappings: &VariableMappings) -> ConcreteX86MachineState {
        ConcreteX86MachineState {
            mode: X86Mode::Real,
            rax: self.rax.make_concrete(variable_mappings),
            rbx: self.rbx.make_concrete(variable_mappings),
            rcx: self.rcx.make_concrete(variable_mappings),
            rdx: self.rdx.make_concrete(variable_mappings),
            rsi: self.rsi.make_concrete(variable_mappings),
            rdi: self.rdi.make_concrete(variable_mappings),
            rbp: self.rbp.make_concrete(variable_mappings),
            rsp: self.rsp.make_concrete(variable_mappings),
            r8: self.r8.make_concrete(variable_mappings),
            r9: self.r9.make_concrete(variable_mappings),
            r10: self.r10.make_concrete(variable_mappings),
            r11: self.r11.make_concrete(variable_mappings),
            r12: self.r12.make_concrete(variable_mappings),
            r13: self.r13.make_concrete(variable_mappings),
            r14: self.r14.make_concrete(variable_mappings),
            r15: self.r15.make_concrete(variable_mappings),
            rip: self.rip.make_concrete(variable_mappings),
            flags: self.flags.make_concrete(variable_mappings),
            memory: self.memory.make_concrete(variable_mappings),
            pending_exception: self.pending_exception,
        }
    }


    //flags:
    fn pf(&self) -> BoolValue<'arena> {
        todo!()
    }

    fn set_pf(&mut self, arena: &'arena Bump, flag: &'arena BoolValue<'arena>) {
        todo!()
    }

    fn of(&self) -> BoolValue<'arena> {
        todo!()
    }

    fn set_of(&mut self, arena: &'arena Bump, flag: &'arena BoolValue<'arena>) {
        todo!()
    }

    pub(crate) fn af(&self) -> &'arena BoolValue<'arena> {
        self.arena.alloc(self.flags.af)
    }

    pub(crate) fn set_af(&mut self, arena: &'arena Bump, flag: impl ArenaRef<'arena, BoolValue<'arena>>) {
        self.flags.af = *flag.arena_ref(arena);
    }

    fn zf(&self) -> BoolValue<'arena> {
        todo!()
    }

    fn set_zf(&mut self, arena: &'arena Bump, flag: &'arena BoolValue<'arena>) {
        todo!()
    }

    fn sf(&self) -> BoolValue<'arena> {
        todo!()
    }

    fn set_sf(&mut self, arena: &'arena Bump, flag: impl ArenaRef<'arena, BoolValue<'arena>>) {
        todo!()
    }

    pub(crate) fn cf(&self) -> BoolValue<'arena> {
        todo!()
    }

    pub(crate) fn set_cf(&mut self, arena: &'arena Bump, flag: impl ArenaRef<'arena, BoolValue<'arena>>) {
        self.flags.cf = *flag.arena_ref(arena);
    }
    // 8 bit regs:
    fn ah(&self) -> &'arena ByteValue<'arena> {
        todo!()
    }

    pub(crate) fn al(&self) -> &'arena ByteValue<'arena> {
        self.get_reg_8(&Reg8::AL)
    }

    pub(crate) fn set_al(&mut self, arena: &'arena Bump, val: &'arena ByteValue<'arena>) {
        self.rax = QWordValue::ZeroExtendByte(val);
    }

    pub fn get_reg_8(&self, reg: &Reg8) -> &'arena ByteValue<'arena> {
        self.a(match reg {
            Reg8::AL => ByteValue::QExtractByte {
                to_extract: self.a(self.rax),
                byte_from_lower: 0,
            },
            Reg8::AH => ByteValue::QExtractByte {
                to_extract: self.a(self.rax),
                byte_from_lower: 1,
            },
            Reg8::BL => ByteValue::QExtractByte {
                to_extract: self.a(self.rbx),
                byte_from_lower: 0,
            },
            Reg8::BH => ByteValue::QExtractByte {
                to_extract: self.a(self.rbx),
                byte_from_lower: 1,
            },
            Reg8::CL => ByteValue::QExtractByte {
                to_extract: self.a(self.rcx),
                byte_from_lower: 0,
            },
            Reg8::CH => ByteValue::QExtractByte {
                to_extract: self.a(self.rcx),
                byte_from_lower: 1,
            },
            Reg8::DL => ByteValue::QExtractByte {
                to_extract: self.a(self.rdx),
                byte_from_lower: 0,
            },
            Reg8::DH => ByteValue::QExtractByte {
                to_extract: self.a(self.rdx),
                byte_from_lower: 1,
            },
            Reg8::SPL => ByteValue::QExtractByte {
                to_extract: self.a(self.rsp),
                byte_from_lower: 0,
            },
            Reg8::BPL => ByteValue::QExtractByte {
                to_extract: self.a(self.rbp),
                byte_from_lower: 0,
            },
            Reg8::SIL => ByteValue::QExtractByte {
                to_extract: self.a(self.rsi),
                byte_from_lower: 0,
            },
            Reg8::DIL => ByteValue::QExtractByte {
                to_extract: self.a(self.rdi),
                byte_from_lower: 0,
            },
            Reg8::R8B => ByteValue::QExtractByte {
                to_extract: self.a(self.r8),
                byte_from_lower: 0,
            },
            Reg8::R9B => ByteValue::QExtractByte {
                to_extract: self.a(self.r9),
                byte_from_lower: 0,
            },
            Reg8::R10B => ByteValue::QExtractByte {
                to_extract: self.a(self.r10),
                byte_from_lower: 0,
            },
            Reg8::R11B => ByteValue::QExtractByte {
                to_extract: self.a(self.r11),
                byte_from_lower: 0,
            },
            Reg8::R12B => ByteValue::QExtractByte {
                to_extract: self.a(self.r12),
                byte_from_lower: 0,
            },
            Reg8::R13B => ByteValue::QExtractByte {
                to_extract: self.a(self.r13),
                byte_from_lower: 0,
            },
            Reg8::R14B => ByteValue::QExtractByte {
                to_extract: self.a(self.r14),
                byte_from_lower: 0,
            },
            Reg8::R15B => ByteValue::QExtractByte {
                to_extract: self.a(self.r15),
                byte_from_lower: 0,
            },
        })
    }


    // 16 bit regs:
    pub(crate) fn ax(&self) -> &'arena WordValue<'arena> {
        self.arena.alloc(WordValue::LowerBits(self.arena.alloc(self.rax)))
    }

    pub(crate) fn set_ax(&mut self, arena: &'arena Bump, val: impl ArenaRef<'arena, WordValue<'arena>>) {
        self.rax = QWordValue::ZeroExtendWord(val.arena_ref(arena));
    }
}

pub mod concrete;
pub mod semantics_builder;