use std::ffi::c_uint;

use xed_sys::{xed_decoded_inst_get_base_reg, xed_decoded_inst_get_branch_displacement, xed_decoded_inst_get_branch_displacement_width, xed_decoded_inst_get_immediate_is_signed, xed_decoded_inst_get_immediate_width, xed_decoded_inst_get_index_reg, xed_decoded_inst_get_memory_displacement, xed_decoded_inst_get_memory_displacement_width, xed_decoded_inst_get_scale, xed_decoded_inst_get_second_immediate, xed_decoded_inst_get_seg_reg, xed_decoded_inst_get_signed_immediate, xed_decoded_inst_get_unsigned_immediate, xed_decoded_inst_t, xed_encoder_operand_t, xed_imm0, xed_inst_t, XED_REG_INVALID, xed_uint_t};

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
                _ => todo!(""),
            }
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct Imm8(pub i8);

impl Imm8 {
    pub fn from_xed(xed: *const xed_decoded_inst_t, second_immediate: bool) -> Self {
        unsafe {
            if second_immediate {
                return Self(dbg!(xed_decoded_inst_get_second_immediate(xed) as i8));
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
                _ => todo!(),
            }
        }
    }

    pub fn to_xed(&self) -> xed_encoder_operand_t {
        unsafe { xed_imm0(self.0 as u64, 8) }
    }

    pub fn to_xed_byte_identifier(&self) -> u8 {
        self.0 as u8
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct Imm16(pub i16);

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
                _ => todo!(),
            }
        }
    }

    pub fn to_xed(&self) -> xed_encoder_operand_t {
        unsafe { xed_imm0(self.0 as u64, 16) }
    }

    pub fn to_xed_byte_identifier(&self) -> u8 {
        todo!()
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct Imm32(pub i32);

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
                _ => todo!(),
            }
        }
    }

    pub fn to_xed(&self) -> xed_encoder_operand_t {
        unsafe { xed_imm0(self.0 as u64, 32) }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct Imm64(u64);

impl Imm64 {
    pub fn from_xed(xed: *const xed_decoded_inst_t, second_immediate: bool) -> Self {
        unsafe {
            assert!(!second_immediate);
            let signed = xed_decoded_inst_get_immediate_is_signed(xed) != 0;
            match xed_decoded_inst_get_immediate_width(xed) {
                8 => {
                    if !signed {
                        Self(xed_decoded_inst_get_unsigned_immediate(xed))
                    } else {
                        todo!()
                    }
                }
                _ => todo!(),
            }
        }
    }

    pub fn to_xed(&self) -> xed_encoder_operand_t {
        unsafe { xed_imm0(self.0 as u64, 32) }
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
                _ => todo!(),
            }
        }
    }

    pub fn unwrap_i8(&self) -> i8 {
        todo!()
    }

    pub fn unwrap_i16(&self) -> i16 {
        todo!()
    }

    pub fn unwrap_i32(&self) -> i32 {
        todo!()
    }

    pub fn unwrap_u64(&self) -> u64 {
        todo!()
    }

    pub fn to_xed(&self) -> xed_encoder_operand_t {
        unsafe {
            match self {
                Immediate::I8(inner) => xed_imm0(*inner as u64, 8),
                Immediate::I16(inner) => xed_imm0(*inner as u64, 16),
                Immediate::I32(inner) => xed_imm0(*inner as u64, 32),
                Immediate::U64(inner) => xed_imm0(*inner, 64)
            }
        }
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
        disp_width: xed_uint_t
    },
}

impl MemoryOperands {
    pub fn from_xed(
        instr: *const xed_decoded_inst_t,
        _instr_template: *const xed_inst_t,
        mem_idx: c_uint,
    ) -> Self {
        unsafe {
            let base_reg_raw = xed_decoded_inst_get_base_reg(instr, mem_idx);
            let seg_reg_raw = xed_decoded_inst_get_seg_reg(instr, mem_idx);
            let index_raw = xed_decoded_inst_get_index_reg(instr, mem_idx);
            let memory_disp_raw = xed_decoded_inst_get_memory_displacement(instr, mem_idx);
            let disp_width = xed_decoded_inst_get_memory_displacement_width(instr, mem_idx);
            let scale_raw = xed_decoded_inst_get_scale(instr, mem_idx);
            let scale = if scale_raw != 0 { X86Scale::from_raw_scale(scale_raw as i32) } else { X86Scale::One };
            let index = if index_raw != 0 { Some(GeneralReg::try_new(index_raw, None).unwrap()) } else { None };
            let base = GeneralReg::try_new(base_reg_raw, None).unwrap();
            Self::SIBAddressing {
                segment: if seg_reg_raw != XED_REG_INVALID {
                    Some(RegSegment::try_new(seg_reg_raw).unwrap())
                } else {
                    None
                },
                scale,
                index,
                base,
                disp: memory_disp_raw,
                disp_width
            }
        }
    }
}
