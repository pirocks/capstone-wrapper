use xed_sys::{XED_OPERAND_WIDTH_A16, XED_OPERAND_WIDTH_A32, XED_OPERAND_WIDTH_ASZ, XED_OPERAND_WIDTH_B, XED_OPERAND_WIDTH_BND32, XED_OPERAND_WIDTH_BND64, XED_OPERAND_WIDTH_D, XED_OPERAND_WIDTH_DQ, xed_operand_width_enum_t, XED_OPERAND_WIDTH_F16, XED_OPERAND_WIDTH_F32, XED_OPERAND_WIDTH_F64, XED_OPERAND_WIDTH_F80, XED_OPERAND_WIDTH_I1, XED_OPERAND_WIDTH_I16, XED_OPERAND_WIDTH_I2, XED_OPERAND_WIDTH_I3, XED_OPERAND_WIDTH_I32, XED_OPERAND_WIDTH_I4, XED_OPERAND_WIDTH_I5, XED_OPERAND_WIDTH_I6, XED_OPERAND_WIDTH_I64, XED_OPERAND_WIDTH_I7, XED_OPERAND_WIDTH_I8, XED_OPERAND_WIDTH_M384, XED_OPERAND_WIDTH_M512, XED_OPERAND_WIDTH_M64INT, XED_OPERAND_WIDTH_M64REAL, XED_OPERAND_WIDTH_MB, XED_OPERAND_WIDTH_MD, XED_OPERAND_WIDTH_MEM108, XED_OPERAND_WIDTH_MEM14, XED_OPERAND_WIDTH_MEM16, XED_OPERAND_WIDTH_MEM16INT, XED_OPERAND_WIDTH_MEM28, XED_OPERAND_WIDTH_MEM32INT, XED_OPERAND_WIDTH_MEM32REAL, XED_OPERAND_WIDTH_MEM80DEC, XED_OPERAND_WIDTH_MEM80REAL, XED_OPERAND_WIDTH_MEM94, XED_OPERAND_WIDTH_MFPXENV, XED_OPERAND_WIDTH_MPREFETCH, XED_OPERAND_WIDTH_MQ, XED_OPERAND_WIDTH_MSKW, XED_OPERAND_WIDTH_MW, XED_OPERAND_WIDTH_MXSAVE, XED_OPERAND_WIDTH_P, XED_OPERAND_WIDTH_P2, XED_OPERAND_WIDTH_PD, XED_OPERAND_WIDTH_PI, XED_OPERAND_WIDTH_PMMSZ16, XED_OPERAND_WIDTH_PMMSZ32, XED_OPERAND_WIDTH_PS, XED_OPERAND_WIDTH_PSEUDO, XED_OPERAND_WIDTH_PSEUDOX87, XED_OPERAND_WIDTH_PTR, XED_OPERAND_WIDTH_Q, XED_OPERAND_WIDTH_QQ, XED_OPERAND_WIDTH_S, XED_OPERAND_WIDTH_S64, XED_OPERAND_WIDTH_SD, XED_OPERAND_WIDTH_SI, XED_OPERAND_WIDTH_SPW, XED_OPERAND_WIDTH_SPW2, XED_OPERAND_WIDTH_SPW3, XED_OPERAND_WIDTH_SPW5, XED_OPERAND_WIDTH_SPW8, XED_OPERAND_WIDTH_SS, XED_OPERAND_WIDTH_SSZ, XED_OPERAND_WIDTH_TMEMCOL, XED_OPERAND_WIDTH_TMEMROW, XED_OPERAND_WIDTH_TV, XED_OPERAND_WIDTH_U16, XED_OPERAND_WIDTH_U32, XED_OPERAND_WIDTH_U64, XED_OPERAND_WIDTH_U8, XED_OPERAND_WIDTH_V, XED_OPERAND_WIDTH_VAR, XED_OPERAND_WIDTH_VV, XED_OPERAND_WIDTH_W, XED_OPERAND_WIDTH_WRD, XED_OPERAND_WIDTH_X128, XED_OPERAND_WIDTH_XB, XED_OPERAND_WIDTH_XD, XED_OPERAND_WIDTH_XQ, XED_OPERAND_WIDTH_XUB, XED_OPERAND_WIDTH_XUD, XED_OPERAND_WIDTH_XUQ, XED_OPERAND_WIDTH_XUW, XED_OPERAND_WIDTH_XW, XED_OPERAND_WIDTH_Y, XED_OPERAND_WIDTH_Y128, XED_OPERAND_WIDTH_YB, XED_OPERAND_WIDTH_YD, XED_OPERAND_WIDTH_YPD, XED_OPERAND_WIDTH_YPS, XED_OPERAND_WIDTH_YQ, XED_OPERAND_WIDTH_YUB, XED_OPERAND_WIDTH_YUD, XED_OPERAND_WIDTH_YUQ, XED_OPERAND_WIDTH_YUW, XED_OPERAND_WIDTH_YW, XED_OPERAND_WIDTH_Z, XED_OPERAND_WIDTH_ZB, XED_OPERAND_WIDTH_ZBF16, XED_OPERAND_WIDTH_ZD, XED_OPERAND_WIDTH_ZF32, XED_OPERAND_WIDTH_ZF64, XED_OPERAND_WIDTH_ZI16, XED_OPERAND_WIDTH_ZI32, XED_OPERAND_WIDTH_ZI64, XED_OPERAND_WIDTH_ZI8, XED_OPERAND_WIDTH_ZMSKW, XED_OPERAND_WIDTH_ZQ, XED_OPERAND_WIDTH_ZU128, XED_OPERAND_WIDTH_ZU16, XED_OPERAND_WIDTH_ZU32, XED_OPERAND_WIDTH_ZU64, XED_OPERAND_WIDTH_ZU8, XED_OPERAND_WIDTH_ZUB, XED_OPERAND_WIDTH_ZUD, XED_OPERAND_WIDTH_ZUQ, XED_OPERAND_WIDTH_ZUW, XED_OPERAND_WIDTH_ZV, XED_OPERAND_WIDTH_ZW};

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum OperandWidth {
    ASZ,
    SSZ,
    PSEUDO,
    PSEUDOX87,
    A16,
    A32,
    B,
    D,
    I8,
    U8,
    I16,
    U16,
    I32,
    U32,
    I64,
    U64,
    F16,
    F32,
    F64,
    DQ,
    XUB,
    XUW,
    XUD,
    XUQ,
    X128,
    XB,
    XW,
    XD,
    XQ,
    ZB,
    ZW,
    ZD,
    ZQ,
    MB,
    MW,
    MD,
    MQ,
    M64INT,
    M64REAL,
    MEM108,
    MEM14,
    MEM16,
    MEM16INT,
    MEM28,
    MEM32INT,
    MEM32REAL,
    MEM80DEC,
    MEM80REAL,
    F80,
    MEM94,
    MFPXENV,
    MXSAVE,
    MPREFETCH,
    P,
    P2,
    PD,
    PS,
    PI,
    Q,
    S,
    S64,
    SD,
    SI,
    SS,
    V,
    Y,
    W,
    Z,
    SPW8,
    SPW,
    SPW5,
    SPW3,
    SPW2,
    I1,
    I2,
    I3,
    I4,
    I5,
    I6,
    I7,
    VAR,
    BND32,
    BND64,
    PMMSZ16,
    PMMSZ32,
    QQ,
    YUB,
    YUW,
    YUD,
    YUQ,
    Y128,
    YB,
    YW,
    YD,
    YQ,
    YPS,
    YPD,
    ZBF16,
    VV,
    ZV,
    WRD,
    MSKW,
    ZMSKW,
    ZF32,
    ZF64,
    ZUB,
    ZUW,
    ZUD,
    ZUQ,
    ZI8,
    ZI16,
    ZI32,
    ZI64,
    ZU8,
    ZU16,
    ZU32,
    ZU64,
    ZU128,
    M384,
    M512,
    PTR,
    TMEMROW,
    TMEMCOL,
    TV,
}


impl OperandWidth {
    pub fn new(xed: xed_operand_width_enum_t) -> Option<Self> {
        Some(match xed {
            XED_OPERAND_WIDTH_ASZ => Self::ASZ,
            XED_OPERAND_WIDTH_SSZ => Self::SSZ,
            XED_OPERAND_WIDTH_PSEUDO => Self::PSEUDO,
            XED_OPERAND_WIDTH_PSEUDOX87 => Self::PSEUDOX87,
            XED_OPERAND_WIDTH_A16 => Self::A16,
            XED_OPERAND_WIDTH_A32 => Self::A32,
            XED_OPERAND_WIDTH_B => Self::B,
            XED_OPERAND_WIDTH_D => Self::D,
            XED_OPERAND_WIDTH_I8 => Self::I8,
            XED_OPERAND_WIDTH_U8 => Self::U8,
            XED_OPERAND_WIDTH_I16 => Self::I16,
            XED_OPERAND_WIDTH_U16 => Self::U16,
            XED_OPERAND_WIDTH_I32 => Self::I32,
            XED_OPERAND_WIDTH_U32 => Self::U32,
            XED_OPERAND_WIDTH_I64 => Self::I64,
            XED_OPERAND_WIDTH_U64 => Self::U64,
            XED_OPERAND_WIDTH_F16 => Self::F16,
            XED_OPERAND_WIDTH_F32 => Self::F32,
            XED_OPERAND_WIDTH_F64 => Self::F64,
            XED_OPERAND_WIDTH_DQ => Self::DQ,
            XED_OPERAND_WIDTH_XUB => Self::XUB,
            XED_OPERAND_WIDTH_XUW => Self::XUW,
            XED_OPERAND_WIDTH_XUD => Self::XUD,
            XED_OPERAND_WIDTH_XUQ => Self::XUQ,
            XED_OPERAND_WIDTH_X128 => Self::X128,
            XED_OPERAND_WIDTH_XB => Self::XB,
            XED_OPERAND_WIDTH_XW => Self::XW,
            XED_OPERAND_WIDTH_XD => Self::XD,
            XED_OPERAND_WIDTH_XQ => Self::XQ,
            XED_OPERAND_WIDTH_ZB => Self::ZB,
            XED_OPERAND_WIDTH_ZW => Self::ZW,
            XED_OPERAND_WIDTH_ZD => Self::ZD,
            XED_OPERAND_WIDTH_ZQ => Self::ZQ,
            XED_OPERAND_WIDTH_MB => Self::MB,
            XED_OPERAND_WIDTH_MW => Self::MW,
            XED_OPERAND_WIDTH_MD => Self::MD,
            XED_OPERAND_WIDTH_MQ => Self::MQ,
            XED_OPERAND_WIDTH_M64INT => Self::M64INT,
            XED_OPERAND_WIDTH_M64REAL => Self::M64REAL,
            XED_OPERAND_WIDTH_MEM108 => Self::MEM108,
            XED_OPERAND_WIDTH_MEM14 => Self::MEM14,
            XED_OPERAND_WIDTH_MEM16 => Self::MEM16,
            XED_OPERAND_WIDTH_MEM16INT => Self::MEM16INT,
            XED_OPERAND_WIDTH_MEM28 => Self::MEM28,
            XED_OPERAND_WIDTH_MEM32INT => Self::MEM32INT,
            XED_OPERAND_WIDTH_MEM32REAL => Self::MEM32REAL,
            XED_OPERAND_WIDTH_MEM80DEC => Self::MEM80DEC,
            XED_OPERAND_WIDTH_MEM80REAL => Self::MEM80REAL,
            XED_OPERAND_WIDTH_F80 => Self::F80,
            XED_OPERAND_WIDTH_MEM94 => Self::MEM94,
            XED_OPERAND_WIDTH_MFPXENV => Self::MFPXENV,
            XED_OPERAND_WIDTH_MXSAVE => Self::MXSAVE,
            XED_OPERAND_WIDTH_MPREFETCH => Self::MPREFETCH,
            XED_OPERAND_WIDTH_P => Self::P,
            XED_OPERAND_WIDTH_P2 => Self::P2,
            XED_OPERAND_WIDTH_PD => Self::PD,
            XED_OPERAND_WIDTH_PS => Self::PS,
            XED_OPERAND_WIDTH_PI => Self::PI,
            XED_OPERAND_WIDTH_Q => Self::Q,
            XED_OPERAND_WIDTH_S => Self::S,
            XED_OPERAND_WIDTH_S64 => Self::S64,
            XED_OPERAND_WIDTH_SD => Self::SD,
            XED_OPERAND_WIDTH_SI => Self::SI,
            XED_OPERAND_WIDTH_SS => Self::SS,
            XED_OPERAND_WIDTH_V => Self::V,
            XED_OPERAND_WIDTH_Y => Self::Y,
            XED_OPERAND_WIDTH_W => Self::W,
            XED_OPERAND_WIDTH_Z => Self::Z,
            XED_OPERAND_WIDTH_SPW8 => Self::SPW8,
            XED_OPERAND_WIDTH_SPW => Self::SPW,
            XED_OPERAND_WIDTH_SPW5 => Self::SPW5,
            XED_OPERAND_WIDTH_SPW3 => Self::SPW3,
            XED_OPERAND_WIDTH_SPW2 => Self::SPW2,
            XED_OPERAND_WIDTH_I1 => Self::I1,
            XED_OPERAND_WIDTH_I2 => Self::I2,
            XED_OPERAND_WIDTH_I3 => Self::I3,
            XED_OPERAND_WIDTH_I4 => Self::I4,
            XED_OPERAND_WIDTH_I5 => Self::I5,
            XED_OPERAND_WIDTH_I6 => Self::I6,
            XED_OPERAND_WIDTH_I7 => Self::I7,
            XED_OPERAND_WIDTH_VAR => Self::VAR,
            XED_OPERAND_WIDTH_BND32 => Self::BND32,
            XED_OPERAND_WIDTH_BND64 => Self::BND64,
            XED_OPERAND_WIDTH_PMMSZ16 => Self::PMMSZ16,
            XED_OPERAND_WIDTH_PMMSZ32 => Self::PMMSZ32,
            XED_OPERAND_WIDTH_QQ => Self::QQ,
            XED_OPERAND_WIDTH_YUB => Self::YUB,
            XED_OPERAND_WIDTH_YUW => Self::YUW,
            XED_OPERAND_WIDTH_YUD => Self::YUD,
            XED_OPERAND_WIDTH_YUQ => Self::YUQ,
            XED_OPERAND_WIDTH_Y128 => Self::Y128,
            XED_OPERAND_WIDTH_YB => Self::YB,
            XED_OPERAND_WIDTH_YW => Self::YW,
            XED_OPERAND_WIDTH_YD => Self::YD,
            XED_OPERAND_WIDTH_YQ => Self::YQ,
            XED_OPERAND_WIDTH_YPS => Self::YPS,
            XED_OPERAND_WIDTH_YPD => Self::YPD,
            XED_OPERAND_WIDTH_ZBF16 => Self::ZBF16,
            XED_OPERAND_WIDTH_VV => Self::VV,
            XED_OPERAND_WIDTH_ZV => Self::ZV,
            XED_OPERAND_WIDTH_WRD => Self::WRD,
            XED_OPERAND_WIDTH_MSKW => Self::MSKW,
            XED_OPERAND_WIDTH_ZMSKW => Self::ZMSKW,
            XED_OPERAND_WIDTH_ZF32 => Self::ZF32,
            XED_OPERAND_WIDTH_ZF64 => Self::ZF64,
            XED_OPERAND_WIDTH_ZUB => Self::ZUB,
            XED_OPERAND_WIDTH_ZUW => Self::ZUW,
            XED_OPERAND_WIDTH_ZUD => Self::ZUD,
            XED_OPERAND_WIDTH_ZUQ => Self::ZUQ,
            XED_OPERAND_WIDTH_ZI8 => Self::ZI8,
            XED_OPERAND_WIDTH_ZI16 => Self::ZI16,
            XED_OPERAND_WIDTH_ZI32 => Self::ZI32,
            XED_OPERAND_WIDTH_ZI64 => Self::ZI64,
            XED_OPERAND_WIDTH_ZU8 => Self::ZU8,
            XED_OPERAND_WIDTH_ZU16 => Self::ZU16,
            XED_OPERAND_WIDTH_ZU32 => Self::ZU32,
            XED_OPERAND_WIDTH_ZU64 => Self::ZU64,
            XED_OPERAND_WIDTH_ZU128 => Self::ZU128,
            XED_OPERAND_WIDTH_M384 => Self::M384,
            XED_OPERAND_WIDTH_M512 => Self::M512,
            XED_OPERAND_WIDTH_PTR => Self::PTR,
            XED_OPERAND_WIDTH_TMEMROW => Self::TMEMROW,
            XED_OPERAND_WIDTH_TMEMCOL => Self::TMEMCOL,
            XED_OPERAND_WIDTH_TV => Self::TV,
            _ => return None
        })
    }
}
