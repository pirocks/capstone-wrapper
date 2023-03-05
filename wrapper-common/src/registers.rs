use std::collections::HashSet;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum RegMMX {
    MM0,
    MM1,
    MM2,
    MM3,
    MM4,
    MM5,
    MM6,
    MM7,
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum RegXMM {
    XMM0,
    XMM1,
    XMM2,
    XMM3,
    XMM4,
    XMM5,
    XMM6,
    XMM7,
    XMM8,
    XMM9,
    XMM10,
    XMM11,
    XMM12,
    XMM13,
    XMM14,
    XMM15,
    XMM16,
    XMM17,
    XMM18,
    XMM19,
    XMM20,
    XMM21,
    XMM22,
    XMM23,
    XMM24,
    XMM25,
    XMM26,
    XMM27,
    XMM28,
    XMM29,
    XMM30,
    XMM31,
    XMM32,
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum RegYMM {
    YMM0,
    YMM1,
    YMM2,
    YMM3,
    YMM4,
    YMM5,
    YMM6,
    YMM7,
    YMM8,
    YMM9,
    YMM10,
    YMM11,
    YMM12,
    YMM13,
    YMM14,
    YMM15,
    YMM16,
    YMM17,
    YMM18,
    YMM19,
    YMM20,
    YMM21,
    YMM22,
    YMM23,
    YMM24,
    YMM25,
    YMM26,
    YMM27,
    YMM28,
    YMM29,
    YMM30,
    YMM31,
    YMM32,
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum RegZMM {
    ZMM0,
    ZMM1,
    ZMM2,
    ZMM3,
    ZMM4,
    ZMM5,
    ZMM6,
    ZMM7,
    ZMM8,
    ZMM9,
    ZMM10,
    ZMM11,
    ZMM12,
    ZMM13,
    ZMM14,
    ZMM15,
    ZMM16,
    ZMM17,
    ZMM18,
    ZMM19,
    ZMM20,
    ZMM21,
    ZMM22,
    ZMM23,
    ZMM24,
    ZMM25,
    ZMM26,
    ZMM27,
    ZMM28,
    ZMM29,
    ZMM30,
    ZMM31,
    ZMM32,
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum RegTMM {
    TMM0,
    TMM1,
    TMM2,
    TMM3,
    TMM4,
    TMM5,
    TMM6,
    TMM7,
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum RegMask {
    K0,
    K1,
    K2,
    K3,
    K4,
    K5,
    K6,
    K7,
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum Reg64WithRIP {
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
    RIP,
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum Reg64WithoutRIP {
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

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum Reg32WithRIP {
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
    EIP,
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum Reg32WithoutRIP {
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

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum Reg16WithRIP {
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
    IP,
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum Reg16WithoutRIP {
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


#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum Reg8 {
    AL,
    AH,
    BL,
    BH,
    CL,
    CH,
    DL,
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

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum RegFloat {
    ST0,
    ST1,
    ST2,
    ST3,
    ST4,
    ST5,
    ST6,
    ST7,
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum RegFloatControl {
    X87CONTROL,
    X87STATUS,
    X87TAG,
    X87POP,
    X87PUSH,
    X87POP2,
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum RegBnd {
    BND0,
    BND1,
    BND2,
    BND3,
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum RegBndConfig {
    BNDCFG,
    BNDCFU,
    BNDSTATUS,
}


#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum RegSpecial {
    GDTR,
    LDTR,
    IDTR,
    TR,
    TSC,
    TSCAUX,
    MSRS,
    UIF,
    SSP,
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum RegControl {
    //several of these exist in encoding but may not actually exist
    CR0,
    CR1,
    CR2,
    CR3,
    CR4,
    CR5,
    CR6,
    CR7,
    CR8,
    CR9,
    CR10,
    CR11,
    CR12,
    CR13,
    CR14,
    CR15,
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum RegControlExtra {
    EFER,
    XCR0,
    XSS,
    MXCSR,
}


#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum RegSegment {
    CS,
    DS,
    SS,
    ES,
    FS,
    GS,
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum RegSegmentBase {
    FSBase,
    GSBase,
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum RegDebug {
    DR0,
    DR1,
    DR2,
    DR3,
    // 4 and 5 exist in encoding but may not actually exist
    DR4,
    DR5,
    DR6,
    DR7,
}


#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum RegisterType {
    AllMmx,
    SomeMmx(HashSet<RegMMX>),
    AllXmm16,
    AllXmm32,
    SomeXmm(HashSet<RegXMM>),
    AllYmm16,
    AllYmm32,
    SomeYmm(HashSet<RegYMM>),
    AllZmm32,
    SomeZmm(HashSet<RegZMM>),
    AllTmm,
    SomeTmm(HashSet<RegTMM>),
    AllMask,
    SomeMask(HashSet<RegMask>),
    AllGP64WithoutRIP,
    AllGP64WithRIP,
    SomeGP64(HashSet<Reg64WithRIP>),
    SingleGP64(Reg64WithRIP),
    AllGP32WithoutRIP,
    AllGP32WithRIP,
    SomeGP32(HashSet<Reg32WithRIP>),
    SingleGP32(Reg32WithRIP),
    AllGP16WithoutRIP,
    AllGP16WithRIP,
    SomeGP16(HashSet<Reg16WithRIP>),
    SingleGP16(Reg16WithRIP),
    AllGP8,
    SomeGP8(HashSet<Reg8>),
    SingleGP8(Reg8),
    AllFloat,
    SomeFloat(HashSet<RegFloat>),
    AllBnd,
    SomeBnd(HashSet<RegBnd>),
    AllSegment,
    SingleSegment(RegSegment),
    SomeSegment(HashSet<RegSegment>),
    AllDebug,
    SomeDebug(HashSet<RegDebug>),
    AllControlRegisters,
    SomeControl(HashSet<RegControl>),
    SomeControlExtra(HashSet<RegControlExtra>),
    SingleSegmentBase(RegSegmentBase),
    SomeSpecial(HashSet<RegSpecial>),
    SomeFloatControl(HashSet<RegFloatControl>),
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum Register {
    Mmx(RegMMX),
    Xmm(RegXMM),
    Ymm(RegYMM),
    Zmm(RegZMM),
    Tmm(RegTMM),
    Mask(RegMask),
    GP64(Reg64WithRIP),
    GP32(Reg32WithRIP),
    GP16(Reg16WithRIP),
    GP8(Reg8),
    Float(RegFloat),
    Bnd(RegBnd),
    Special(RegSpecial),
}

