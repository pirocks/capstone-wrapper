use capstone::arch::x86::X86Reg::*;
use capstone::RegId;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum SegmentRegister {
    CS,
    SS,
    DS,
    ES,
    FS,
    GS,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Register8 {
    AL,
    BL,
    CL,
    DL,
    AH,
    BH,
    CH,
    DH,
    SIL,
    DIL,
    BPL,
    SPL,
    R8B,
    R9B,
    R10B,
    R11B,
    R12B,
    R13B,
    R14B,
    R15B,
}

impl Register8 {
    pub fn new(register_id: RegId) -> Self {
        match register_id.0 as u32 {
            X86_REG_AL => Register8::AL,
            X86_REG_BL => Register8::BL,
            X86_REG_CL => Register8::CL,
            X86_REG_DL => Register8::DL,
            X86_REG_SIL => Register8::SIL,
            X86_REG_DIL => Register8::DIL,
            X86_REG_BPL => Register8::BPL,
            X86_REG_SPL => Register8::SPL,
            X86_REG_R8B => Register8::R8B,
            X86_REG_R9B => Register8::R9B,
            X86_REG_R10B => Register8::R10B,
            X86_REG_R11B => Register8::R11B,
            X86_REG_R12B => Register8::R12B,
            X86_REG_R13B => Register8::R13B,
            X86_REG_R14B => Register8::R14B,
            X86_REG_R15B => Register8::R15B,
            _ => {
                panic!(
                    "Capstone should never give us an unknown register id: {}",
                    register_id.0
                )
            }
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Register16 {
    AX,
    BX,
    CX,
    DX,
    SI,
    DI,
    BP,
    SP,
    R8W,
    R9W,
    R10W,
    R11W,
    R12W,
    R13W,
    R14W,
    R15W,
}

impl Register16 {
    pub fn new(register_id: RegId) -> Self {
        match register_id.0 as u32 {
            X86_REG_AX => Register16::AX,
            X86_REG_BX => Register16::BX,
            X86_REG_CX => Register16::CX,
            X86_REG_DX => Register16::DX,
            X86_REG_SI => Register16::SI,
            X86_REG_DI => Register16::DI,
            X86_REG_BP => Register16::BP,
            X86_REG_SP => Register16::SP,
            X86_REG_R8W => Register16::R8W,
            X86_REG_R9W => Register16::R9W,
            X86_REG_R10W => Register16::R10W,
            X86_REG_R11W => Register16::R11W,
            X86_REG_R12W => Register16::R12W,
            X86_REG_R13W => Register16::R13W,
            X86_REG_R14W => Register16::R14W,
            X86_REG_R15W => Register16::R15W,
            _ => {
                panic!(
                    "Capstone should never give us an unknown register id: {}",
                    register_id.0
                )
            }
        }
    }

    pub fn widen_register(&self) -> Register64 {
        match self {
            Register16::AX => Register64::RAX,
            Register16::BX => Register64::RBX,
            Register16::CX => Register64::RCX,
            Register16::DX => Register64::RDX,
            Register16::SI => Register64::RSI,
            Register16::DI => Register64::RDI,
            Register16::BP => Register64::RBP,
            Register16::SP => Register64::RSP,
            Register16::R8W => Register64::R8,
            Register16::R9W => Register64::R9,
            Register16::R10W => Register64::R10,
            Register16::R11W => Register64::R11,
            Register16::R12W => Register64::R12,
            Register16::R13W => Register64::R13,
            Register16::R14W => Register64::R14,
            Register16::R15W => Register64::R15,
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Register32 {
    EAX,
    EBX,
    ECX,
    EDX,
    ESI,
    EDI,
    EBP,
    ESP,
    R8D,
    R9D,
    R10D,
    R11D,
    R12D,
    R13D,
    R14D,
    R15D,
}

impl Register32 {
    pub fn new(register_id: RegId) -> Self {
        match register_id.0 as u32 {
            X86_REG_EAX => Register32::EAX,
            X86_REG_EBX => Register32::EBX,
            X86_REG_ECX => Register32::ECX,
            X86_REG_EDX => Register32::EDX,
            X86_REG_ESI => Register32::ESI,
            X86_REG_EDI => Register32::EDI,
            X86_REG_EBP => Register32::EBP,
            X86_REG_ESP => Register32::ESP,
            X86_REG_R8D => Register32::R8D,
            X86_REG_R9D => Register32::R9D,
            X86_REG_R10D => Register32::R10D,
            X86_REG_R11D => Register32::R11D,
            X86_REG_R12D => Register32::R12D,
            X86_REG_R13D => Register32::R13D,
            X86_REG_R14D => Register32::R14D,
            X86_REG_R15D => Register32::R15D,
            _ => {
                panic!(
                    "Capstone should never give us an unknown register id: {}",
                    register_id.0
                )
            }
        }
    }

    pub fn widen_register(&self) -> Register64 {
        match self {
            Register32::EAX => Register64::RAX,
            Register32::EBX => Register64::RBX,
            Register32::ECX => Register64::RCX,
            Register32::EDX => Register64::RDX,
            Register32::ESI => Register64::RSI,
            Register32::EDI => Register64::RDI,
            Register32::EBP => Register64::RBP,
            Register32::ESP => Register64::RSP,
            Register32::R8D => Register64::R8,
            Register32::R9D => Register64::R9,
            Register32::R10D => Register64::R10,
            Register32::R11D => Register64::R11,
            Register32::R12D => Register64::R12,
            Register32::R13D => Register64::R13,
            Register32::R14D => Register64::R14,
            Register32::R15D => Register64::R15,
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Register64 {
    RAX,
    RBX,
    RCX,
    RDX,
    RSI,
    RDI,
    RBP,
    RSP,
    R8,
    R9,
    R10,
    R11,
    R12,
    R13,
    R14,
    R15,
}

impl Register64 {
    pub fn new(register_id: RegId) -> Self {
        match register_id.0 as u32 {
            X86_REG_RAX => Register64::RAX,
            X86_REG_RBX => Register64::RBX,
            X86_REG_RCX => Register64::RCX,
            X86_REG_RDX => Register64::RDX,
            X86_REG_RSI => Register64::RSI,
            X86_REG_RDI => Register64::RDI,
            X86_REG_RBP => Register64::RBP,
            X86_REG_RSP => Register64::RSP,
            X86_REG_R8 => Register64::R8,
            X86_REG_R9 => Register64::R9,
            X86_REG_R10 => Register64::R10,
            X86_REG_R11 => Register64::R11,
            X86_REG_R12 => Register64::R12,
            X86_REG_R13 => Register64::R13,
            X86_REG_R14 => Register64::R14,
            X86_REG_R15 => Register64::R15,
            _ => {
                panic!(
                    "Capstone should never give us an unknown register id: {}",
                    register_id.0
                )
            }
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum OperandSize {
    QuadWord,
    DoubleWord,
    Word,
    HalfWord,
}

impl OperandSize {
    pub fn from_capstone_size(size: u8) -> OperandSize {
        match size {
            8 => Self::QuadWord,
            4 => Self::DoubleWord,
            2 => Self::Word,
            1 => Self::HalfWord,
            _ => {
                panic!("Unexpected operand size from capstone")
            }
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum GeneralRegister {
    Register64(Register64),
    Register32(Register32),
    Register16(Register16),
    Register8(Register8),
}

impl GeneralRegister {
    pub fn new(register_id: RegId, operand_size: OperandSize) -> Self {
        match operand_size {
            OperandSize::QuadWord => GeneralRegister::Register64(Register64::new(register_id)),
            OperandSize::DoubleWord => GeneralRegister::Register32(Register32::new(register_id)),
            OperandSize::Word => GeneralRegister::Register16(Register16::new(register_id)),
            OperandSize::HalfWord => GeneralRegister::Register8(Register8::new(register_id)),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum GeneralRegisterWordAndBigger {
    Register64(Register64),
    Register32(Register32),
    Register16(Register16),
}

impl GeneralRegisterWordAndBigger {
    pub fn new(register_id: RegId, operand_size: OperandSize) -> Self {
        match operand_size {
            OperandSize::QuadWord => {
                GeneralRegisterWordAndBigger::Register64(Register64::new(register_id))
            }
            OperandSize::DoubleWord => {
                GeneralRegisterWordAndBigger::Register32(Register32::new(register_id))
            }
            OperandSize::Word => {
                GeneralRegisterWordAndBigger::Register16(Register16::new(register_id))
            }
            OperandSize::HalfWord => {
                panic!("Shouldn't use half words in this function")
            }
        }
    }

    pub fn widen_register(&self) -> Register64 {
        match self {
            GeneralRegisterWordAndBigger::Register64(register64) => *register64,
            GeneralRegisterWordAndBigger::Register32(register32) => register32.widen_register(),
            GeneralRegisterWordAndBigger::Register16(register16) => register16.widen_register(),
        }
    }
}
