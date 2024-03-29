use xed_sys::xed_operand_enum_t;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
#[allow(non_camel_case_types)]
pub enum OperandName {
    AGEN,
    AMD3DNOW,
    ASZ,
    BASE0,
    BASE1,
    BCAST,
    BCRC,
    BRDISP_WIDTH,
    CET,
    CHIP,
    CLDEMOTE,
    DEFAULT_SEG,
    DF32,
    DF64,
    DISP,
    DISP_WIDTH,
    DUMMY,
    EASZ,
    ELEMENT_SIZE,
    ENCODER_PREFERRED,
    ENCODE_FORCE,
    EOSZ,
    ERROR,
    ESRC,
    FIRST_F2F3,
    HAS_MODRM,
    HAS_SIB,
    HINT,
    ICLASS,
    ILD_F2,
    ILD_F3,
    ILD_SEG,
    IMM0,
    IMM0SIGNED,
    IMM1,
    IMM1_BYTES,
    IMM_WIDTH,
    INDEX,
    LAST_F2F3,
    LLRC,
    LOCK,
    LZCNT,
    MAP,
    MASK,
    MAX_BYTES,
    MEM0,
    MEM1,
    MEM_WIDTH,
    MOD,
    MODE,
    MODEP5,
    MODEP55C,
    MODE_FIRST_PREFIX,
    MODE_SHORT_UD0,
    MODRM_BYTE,
    MPXMODE,
    MUST_USE_EVEX,
    NEEDREX,
    NEED_MEMDISP,
    NEED_SIB,
    NELEM,
    NOMINAL_OPCODE,
    NOREX,
    NO_SCALE_DISP8,
    NPREFIXES,
    NREXES,
    NSEG_PREFIXES,
    OSZ,
    OUTREG,
    OUT_OF_BYTES,
    P4,
    POS_DISP,
    POS_IMM,
    POS_IMM1,
    POS_MODRM,
    POS_NOMINAL_OPCODE,
    POS_SIB,
    PREFIX66,
    PTR,
    REALMODE,
    REG,
    REG0,
    REG1,
    REG2,
    REG3,
    REG4,
    REG5,
    REG6,
    REG7,
    REG8,
    REG9,
    RELBR,
    REP,
    REX,
    REXB,
    REXR,
    REXRR,
    REXW,
    REXX,
    RM,
    ROUNDC,
    SAE,
    SCALE,
    SEG0,
    SEG1,
    SEG_OVD,
    SIBBASE,
    SIBINDEX,
    SIBSCALE,
    SMODE,
    SRM,
    TZCNT,
    UBIT,
    UIMM0,
    UIMM1,
    USING_DEFAULT_SEGMENT0,
    USING_DEFAULT_SEGMENT1,
    VEXDEST210,
    VEXDEST3,
    VEXDEST4,
    VEXVALID,
    VEX_C4,
    VEX_PREFIX,
    VL,
    WBNOINVD,
    ZEROING,
}

impl OperandName {
    pub fn new(xed: xed_operand_enum_t) -> Self {
        use xed_sys::{
            XED_OPERAND_AGEN, XED_OPERAND_AMD3DNOW, XED_OPERAND_ASZ, XED_OPERAND_BASE0,
            XED_OPERAND_BASE1, XED_OPERAND_BCAST, XED_OPERAND_BCRC, XED_OPERAND_BRDISP_WIDTH,
            XED_OPERAND_CET, XED_OPERAND_CHIP, XED_OPERAND_CLDEMOTE, XED_OPERAND_DEFAULT_SEG,
            XED_OPERAND_DF32, XED_OPERAND_DF64, XED_OPERAND_DISP, XED_OPERAND_DISP_WIDTH,
            XED_OPERAND_DUMMY, XED_OPERAND_EASZ, XED_OPERAND_ELEMENT_SIZE,
            XED_OPERAND_ENCODER_PREFERRED, XED_OPERAND_ENCODE_FORCE, XED_OPERAND_EOSZ,
            XED_OPERAND_ERROR, XED_OPERAND_ESRC, XED_OPERAND_FIRST_F2F3, XED_OPERAND_HAS_MODRM,
            XED_OPERAND_HAS_SIB, XED_OPERAND_HINT, XED_OPERAND_ICLASS, XED_OPERAND_ILD_F2,
            XED_OPERAND_ILD_F3, XED_OPERAND_ILD_SEG, XED_OPERAND_IMM0, XED_OPERAND_IMM0SIGNED,
            XED_OPERAND_IMM1, XED_OPERAND_IMM1_BYTES, XED_OPERAND_IMM_WIDTH, XED_OPERAND_INDEX,
            XED_OPERAND_LAST_F2F3, XED_OPERAND_LLRC, XED_OPERAND_LOCK, XED_OPERAND_LZCNT,
            XED_OPERAND_MAP, XED_OPERAND_MASK, XED_OPERAND_MAX_BYTES, XED_OPERAND_MEM0,
            XED_OPERAND_MEM1, XED_OPERAND_MEM_WIDTH, XED_OPERAND_MOD, XED_OPERAND_MODE,
            XED_OPERAND_MODEP5, XED_OPERAND_MODEP55C, XED_OPERAND_MODE_FIRST_PREFIX,
            XED_OPERAND_MODE_SHORT_UD0, XED_OPERAND_MODRM_BYTE, XED_OPERAND_MPXMODE,
            XED_OPERAND_MUST_USE_EVEX, XED_OPERAND_NEEDREX, XED_OPERAND_NEED_MEMDISP,
            XED_OPERAND_NEED_SIB, XED_OPERAND_NELEM, XED_OPERAND_NOMINAL_OPCODE, XED_OPERAND_NOREX,
            XED_OPERAND_NO_SCALE_DISP8, XED_OPERAND_NPREFIXES, XED_OPERAND_NREXES,
            XED_OPERAND_NSEG_PREFIXES, XED_OPERAND_OSZ, XED_OPERAND_OUTREG,
            XED_OPERAND_OUT_OF_BYTES, XED_OPERAND_P4, XED_OPERAND_POS_DISP, XED_OPERAND_POS_IMM,
            XED_OPERAND_POS_IMM1, XED_OPERAND_POS_MODRM, XED_OPERAND_POS_NOMINAL_OPCODE,
            XED_OPERAND_POS_SIB, XED_OPERAND_PREFIX66, XED_OPERAND_PTR, XED_OPERAND_REALMODE,
            XED_OPERAND_REG, XED_OPERAND_REG0, XED_OPERAND_REG1, XED_OPERAND_REG2,
            XED_OPERAND_REG3, XED_OPERAND_REG4, XED_OPERAND_REG5, XED_OPERAND_REG6,
            XED_OPERAND_REG7, XED_OPERAND_REG8, XED_OPERAND_REG9, XED_OPERAND_RELBR,
            XED_OPERAND_REP, XED_OPERAND_REX, XED_OPERAND_REXB, XED_OPERAND_REXR,
            XED_OPERAND_REXRR, XED_OPERAND_REXW, XED_OPERAND_REXX, XED_OPERAND_RM,
            XED_OPERAND_ROUNDC, XED_OPERAND_SAE, XED_OPERAND_SCALE, XED_OPERAND_SEG0,
            XED_OPERAND_SEG1, XED_OPERAND_SEG_OVD, XED_OPERAND_SIBBASE, XED_OPERAND_SIBINDEX,
            XED_OPERAND_SIBSCALE, XED_OPERAND_SMODE, XED_OPERAND_SRM, XED_OPERAND_TZCNT,
            XED_OPERAND_UBIT, XED_OPERAND_UIMM0, XED_OPERAND_UIMM1,
            XED_OPERAND_USING_DEFAULT_SEGMENT0, XED_OPERAND_USING_DEFAULT_SEGMENT1,
            XED_OPERAND_VEXDEST210, XED_OPERAND_VEXDEST3, XED_OPERAND_VEXDEST4,
            XED_OPERAND_VEXVALID, XED_OPERAND_VEX_C4, XED_OPERAND_VEX_PREFIX, XED_OPERAND_VL,
            XED_OPERAND_WBNOINVD, XED_OPERAND_ZEROING,
        };
        match xed {
            XED_OPERAND_AGEN => Self::AGEN,
            XED_OPERAND_AMD3DNOW => Self::AMD3DNOW,
            XED_OPERAND_ASZ => Self::ASZ,
            XED_OPERAND_BASE0 => Self::BASE0,
            XED_OPERAND_BASE1 => Self::BASE1,
            XED_OPERAND_BCAST => Self::BCAST,
            XED_OPERAND_BCRC => Self::BCRC,
            XED_OPERAND_BRDISP_WIDTH => Self::BRDISP_WIDTH,
            XED_OPERAND_CET => Self::CET,
            XED_OPERAND_CHIP => Self::CHIP,
            XED_OPERAND_CLDEMOTE => Self::CLDEMOTE,
            XED_OPERAND_DEFAULT_SEG => Self::DEFAULT_SEG,
            XED_OPERAND_DF32 => Self::DF32,
            XED_OPERAND_DF64 => Self::DF64,
            XED_OPERAND_DISP => Self::DISP,
            XED_OPERAND_DISP_WIDTH => Self::DISP_WIDTH,
            XED_OPERAND_DUMMY => Self::DUMMY,
            XED_OPERAND_EASZ => Self::EASZ,
            XED_OPERAND_ELEMENT_SIZE => Self::ELEMENT_SIZE,
            XED_OPERAND_ENCODER_PREFERRED => Self::ENCODER_PREFERRED,
            XED_OPERAND_ENCODE_FORCE => Self::ENCODE_FORCE,
            XED_OPERAND_EOSZ => Self::EOSZ,
            XED_OPERAND_ERROR => Self::ERROR,
            XED_OPERAND_ESRC => Self::ESRC,
            XED_OPERAND_FIRST_F2F3 => Self::FIRST_F2F3,
            XED_OPERAND_HAS_MODRM => Self::HAS_MODRM,
            XED_OPERAND_HAS_SIB => Self::HAS_SIB,
            XED_OPERAND_HINT => Self::HINT,
            XED_OPERAND_ICLASS => Self::ICLASS,
            XED_OPERAND_ILD_F2 => Self::ILD_F2,
            XED_OPERAND_ILD_F3 => Self::ILD_F3,
            XED_OPERAND_ILD_SEG => Self::ILD_SEG,
            XED_OPERAND_IMM0 => Self::IMM0,
            XED_OPERAND_IMM0SIGNED => Self::IMM0SIGNED,
            XED_OPERAND_IMM1 => Self::IMM1,
            XED_OPERAND_IMM1_BYTES => Self::IMM1_BYTES,
            XED_OPERAND_IMM_WIDTH => Self::IMM_WIDTH,
            XED_OPERAND_INDEX => Self::INDEX,
            XED_OPERAND_LAST_F2F3 => Self::LAST_F2F3,
            XED_OPERAND_LLRC => Self::LLRC,
            XED_OPERAND_LOCK => Self::LOCK,
            XED_OPERAND_LZCNT => Self::LZCNT,
            XED_OPERAND_MAP => Self::MAP,
            XED_OPERAND_MASK => Self::MASK,
            XED_OPERAND_MAX_BYTES => Self::MAX_BYTES,
            XED_OPERAND_MEM0 => Self::MEM0,
            XED_OPERAND_MEM1 => Self::MEM1,
            XED_OPERAND_MEM_WIDTH => Self::MEM_WIDTH,
            XED_OPERAND_MOD => Self::MOD,
            XED_OPERAND_MODE => Self::MODE,
            XED_OPERAND_MODEP5 => Self::MODEP5,
            XED_OPERAND_MODEP55C => Self::MODEP55C,
            XED_OPERAND_MODE_FIRST_PREFIX => Self::MODE_FIRST_PREFIX,
            XED_OPERAND_MODE_SHORT_UD0 => Self::MODE_SHORT_UD0,
            XED_OPERAND_MODRM_BYTE => Self::MODRM_BYTE,
            XED_OPERAND_MPXMODE => Self::MPXMODE,
            XED_OPERAND_MUST_USE_EVEX => Self::MUST_USE_EVEX,
            XED_OPERAND_NEEDREX => Self::NEEDREX,
            XED_OPERAND_NEED_MEMDISP => Self::NEED_MEMDISP,
            XED_OPERAND_NEED_SIB => Self::NEED_SIB,
            XED_OPERAND_NELEM => Self::NELEM,
            XED_OPERAND_NOMINAL_OPCODE => Self::NOMINAL_OPCODE,
            XED_OPERAND_NOREX => Self::NOREX,
            XED_OPERAND_NO_SCALE_DISP8 => Self::NO_SCALE_DISP8,
            XED_OPERAND_NPREFIXES => Self::NPREFIXES,
            XED_OPERAND_NREXES => Self::NREXES,
            XED_OPERAND_NSEG_PREFIXES => Self::NSEG_PREFIXES,
            XED_OPERAND_OSZ => Self::OSZ,
            XED_OPERAND_OUTREG => Self::OUTREG,
            XED_OPERAND_OUT_OF_BYTES => Self::OUT_OF_BYTES,
            XED_OPERAND_P4 => Self::P4,
            XED_OPERAND_POS_DISP => Self::POS_DISP,
            XED_OPERAND_POS_IMM => Self::POS_IMM,
            XED_OPERAND_POS_IMM1 => Self::POS_IMM1,
            XED_OPERAND_POS_MODRM => Self::POS_MODRM,
            XED_OPERAND_POS_NOMINAL_OPCODE => Self::POS_NOMINAL_OPCODE,
            XED_OPERAND_POS_SIB => Self::POS_SIB,
            XED_OPERAND_PREFIX66 => Self::PREFIX66,
            XED_OPERAND_PTR => Self::PTR,
            XED_OPERAND_REALMODE => Self::REALMODE,
            XED_OPERAND_REG => Self::REG,
            XED_OPERAND_REG0 => Self::REG0,
            XED_OPERAND_REG1 => Self::REG1,
            XED_OPERAND_REG2 => Self::REG2,
            XED_OPERAND_REG3 => Self::REG3,
            XED_OPERAND_REG4 => Self::REG4,
            XED_OPERAND_REG5 => Self::REG5,
            XED_OPERAND_REG6 => Self::REG6,
            XED_OPERAND_REG7 => Self::REG7,
            XED_OPERAND_REG8 => Self::REG8,
            XED_OPERAND_REG9 => Self::REG9,
            XED_OPERAND_RELBR => Self::RELBR,
            XED_OPERAND_REP => Self::REP,
            XED_OPERAND_REX => Self::REX,
            XED_OPERAND_REXB => Self::REXB,
            XED_OPERAND_REXR => Self::REXR,
            XED_OPERAND_REXRR => Self::REXRR,
            XED_OPERAND_REXW => Self::REXW,
            XED_OPERAND_REXX => Self::REXX,
            XED_OPERAND_RM => Self::RM,
            XED_OPERAND_ROUNDC => Self::ROUNDC,
            XED_OPERAND_SAE => Self::SAE,
            XED_OPERAND_SCALE => Self::SCALE,
            XED_OPERAND_SEG0 => Self::SEG0,
            XED_OPERAND_SEG1 => Self::SEG1,
            XED_OPERAND_SEG_OVD => Self::SEG_OVD,
            XED_OPERAND_SIBBASE => Self::SIBBASE,
            XED_OPERAND_SIBINDEX => Self::SIBINDEX,
            XED_OPERAND_SIBSCALE => Self::SIBSCALE,
            XED_OPERAND_SMODE => Self::SMODE,
            XED_OPERAND_SRM => Self::SRM,
            XED_OPERAND_TZCNT => Self::TZCNT,
            XED_OPERAND_UBIT => Self::UBIT,
            XED_OPERAND_UIMM0 => Self::UIMM0,
            XED_OPERAND_UIMM1 => Self::UIMM1,
            XED_OPERAND_USING_DEFAULT_SEGMENT0 => Self::USING_DEFAULT_SEGMENT0,
            XED_OPERAND_USING_DEFAULT_SEGMENT1 => Self::USING_DEFAULT_SEGMENT1,
            XED_OPERAND_VEXDEST210 => Self::VEXDEST210,
            XED_OPERAND_VEXDEST3 => Self::VEXDEST3,
            XED_OPERAND_VEXDEST4 => Self::VEXDEST4,
            XED_OPERAND_VEXVALID => Self::VEXVALID,
            XED_OPERAND_VEX_C4 => Self::VEX_C4,
            XED_OPERAND_VEX_PREFIX => Self::VEX_PREFIX,
            XED_OPERAND_VL => Self::VL,
            XED_OPERAND_WBNOINVD => Self::WBNOINVD,
            XED_OPERAND_ZEROING => Self::ZEROING,
            _ => panic!(),
        }
    }
}
