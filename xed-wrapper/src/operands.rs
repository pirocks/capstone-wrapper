use std::ffi::c_uint;

use xed_sys::{xed_decoded_inst_get_base_reg, xed_decoded_inst_get_branch_displacement, xed_decoded_inst_get_branch_displacement_width, xed_decoded_inst_get_immediate_is_signed, xed_decoded_inst_get_immediate_width, xed_decoded_inst_get_index_reg, xed_decoded_inst_get_memory_displacement, xed_decoded_inst_get_scale, xed_decoded_inst_get_second_immediate, xed_decoded_inst_get_seg_reg, xed_decoded_inst_get_signed_immediate, xed_decoded_inst_get_unsigned_immediate, xed_decoded_inst_t, xed_inst_t, XED_REG_INVALID};

use wrapper_common::memory_operand::{GeneralReg, X86Scale};
use wrapper_common::registers::RegSegment;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum RelativeBr {
    Disp8(i8),
    Disp16(i16),
    Disp32(i32),
}

impl RelativeBr {
    pub fn from_xed(xed: *const xed_decoded_inst_t) -> Self {
        unsafe {
            match xed_decoded_inst_get_branch_displacement_width(xed) {
                1 => RelativeBr::Disp8(xed_decoded_inst_get_branch_displacement(xed) as i8),
                2 => RelativeBr::Disp16(xed_decoded_inst_get_branch_displacement(xed) as i16),
                4 => RelativeBr::Disp32(xed_decoded_inst_get_branch_displacement(xed)),
                _ => todo!("")
            }
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct Imm8(i8);

impl Imm8 {
    pub fn from_xed(xed: *const xed_decoded_inst_t, second_immediate: bool) -> Self {
        unsafe {
            if second_immediate {
                return Self(xed_decoded_inst_get_second_immediate(xed) as i8);
            }
            let signed = xed_decoded_inst_get_immediate_is_signed(xed) != 0;
            match xed_decoded_inst_get_immediate_width(xed) {
                1 => {
                    if signed {
                        Self(xed_decoded_inst_get_signed_immediate(xed) as i8)
                    } else {
                        todo!()
                    }
                }
                _ => todo!()
            }
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct Imm16(i16);

impl Imm16 {
    pub fn from_xed(xed: *const xed_decoded_inst_t, second_immediate: bool) -> Self {
        unsafe {
            assert!(!second_immediate);
            let signed = xed_decoded_inst_get_immediate_is_signed(xed) != 0;
            match xed_decoded_inst_get_immediate_width(xed) {
                2 => {
                    if signed {
                        Self(xed_decoded_inst_get_signed_immediate(xed) as i16)
                    } else {
                        todo!()
                    }
                }
                _ => todo!()
            }
        }
    }
}


#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct Imm32(i32);

impl Imm32 {
    pub fn from_xed(xed: *const xed_decoded_inst_t, second_immediate: bool) -> Self {
        unsafe {
            assert!(!second_immediate);
            let signed = xed_decoded_inst_get_immediate_is_signed(xed) != 0;
            match xed_decoded_inst_get_immediate_width(xed) {
                4 => {
                    if signed {
                        Self(xed_decoded_inst_get_signed_immediate(xed) as i32)
                    } else {
                        todo!()
                    }
                }
                _ => todo!()
            }
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum Immediate {
    I8(i8),
    I16(i16),
    I32(i32),
    U64(u64),
}

impl Immediate {
    pub fn from_xed(xed: *const xed_decoded_inst_t, second_immediate: bool) -> Self {
        unsafe {
            if second_immediate {
                return Self::I8(xed_decoded_inst_get_second_immediate(xed) as i8);
            }
            let signed = xed_decoded_inst_get_immediate_is_signed(xed) != 0;
            match xed_decoded_inst_get_immediate_width(xed) {
                1 => {
                    if signed {
                        Self::I8(xed_decoded_inst_get_signed_immediate(xed) as i8)
                    } else {
                        todo!()
                    }
                }
                2 => {
                    if !signed {
                        todo!()
                    }
                    Self::I16(xed_decoded_inst_get_signed_immediate(xed) as i16)
                }
                4 => {
                    if !signed {
                        todo!()
                    }
                    Self::I32(xed_decoded_inst_get_signed_immediate(xed) as i32)
                }
                8 => {
                    if signed {
                        todo!()
                    }
                    Self::U64(xed_decoded_inst_get_unsigned_immediate(xed))
                }
                _ => todo!()
            }
        }
    }

    pub fn unwrap_i8(&self) -> i8{
        todo!()
    }

    pub fn unwrap_i16(&self) -> i16{
        todo!()
    }

    pub fn unwrap_i32(&self) -> i32{
        todo!()
    }

    pub fn unwrap_u64(&self) -> u64{
        todo!()
    }
}


#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum MemoryOperands {
    SIBAddressing {
        segment: Option<RegSegment>,
        scale: X86Scale,
        index: Option<GeneralReg>,
        base: GeneralReg,
        disp: i64,
    }
}

impl MemoryOperands {
    pub fn from_xed(instr: *const xed_decoded_inst_t, _instr_template: *const xed_inst_t, mem_idx: c_uint) -> Self {
        unsafe {
            let base_reg_raw = xed_decoded_inst_get_base_reg(instr, mem_idx);
            let seg_reg_raw = xed_decoded_inst_get_seg_reg(instr, mem_idx);
            let index_raw = xed_decoded_inst_get_index_reg(instr, mem_idx);
            let memory_disp_raw = xed_decoded_inst_get_memory_displacement(instr, mem_idx);
            let scale_raw = xed_decoded_inst_get_scale(instr, mem_idx);
            let scale = X86Scale::from_raw_scale(scale_raw as i32);
            let index = GeneralReg::try_new(index_raw).unwrap();
            let base = GeneralReg::try_new(base_reg_raw).unwrap();
            Self::SIBAddressing {
                segment: if seg_reg_raw == XED_REG_INVALID {
                    Some(RegSegment::try_new(seg_reg_raw).unwrap())
                } else {
                    None
                },
                scale,
                index: Some(index),
                base,
                disp: memory_disp_raw,
            }
        }
    }
}

