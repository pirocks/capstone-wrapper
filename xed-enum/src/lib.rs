use std::ffi::c_uint;
use std::mem::MaybeUninit;
use std::sync::Once;

use xed_sys::{XED_ADDRESS_WIDTH_64b, xed_decode, xed_decoded_inst_get_length, xed_decoded_inst_zero_set_mode, xed_error_enum_t2str, XED_MACHINE_MODE_LONG_64, xed_state_init, xed_state_t, xed_state_zero};

use xed_enum_generator::{enum_from_xed, enum_to_xed, instruction_enums, top_level_instruction_enum};

static START: Once = Once::new();

pub struct EncodeDecodeContext {
    xed_state: MaybeUninit<xed_state_t>,
}

impl EncodeDecodeContext {
    pub fn new() -> Self {
        let mut xed_state: MaybeUninit<xed_state_t> = MaybeUninit::zeroed();
        unsafe {
            xed_state_zero(xed_state.as_mut_ptr());
            xed_state_init(xed_state.as_mut_ptr(), XED_MACHINE_MODE_LONG_64, XED_ADDRESS_WIDTH_64b, XED_ADDRESS_WIDTH_64b);
        }
        Self {
            xed_state,
        }
    }
}

top_level_instruction_enum!();
instruction_enums!();
enum_from_xed!();
enum_to_xed!();


pub enum DecodeError{
    XedError(String)
}

impl X86Instruction{
    pub fn encode(&self, encode_context: &mut EncodeDecodeContext) -> ([u8;16], usize){
        let xed_request = self.to_xed(encode_context);
        todo!();
    }

    pub fn decode_one<'a,'b>(&self, bytes: &'a [u8], context: &'b mut EncodeDecodeContext) -> Result<(X86Instruction, &'a [u8]),DecodeError> {
        let mut decoded = MaybeUninit::zeroed();
        unsafe { xed_decoded_inst_zero_set_mode(decoded.as_mut_ptr(), context.xed_state.as_ptr()); }
        let error = unsafe { xed_decode(decoded.as_mut_ptr(), bytes.as_ptr(), bytes.len() as c_uint) };
        if error != 0 {
            return Err(DecodeError::XedError(unsafe { CStr::from_ptr(xed_error_enum_t2str(error)) }.to_str().unwrap().to_string()))
        }
        let decoded_length = unsafe { xed_decoded_inst_get_length(decoded.as_ptr()) } as usize;
        Ok((Self::from_xed(decoded.as_ptr()), &bytes[decoded_length..]))
    }
}

#[cfg(test)]
pub mod test;
