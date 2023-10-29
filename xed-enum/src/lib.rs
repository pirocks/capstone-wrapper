use std::mem::MaybeUninit;
use std::sync::Once;

use xed_sys::{XED_ADDRESS_WIDTH_64b, XED_MACHINE_MODE_LONG_64, xed_state_init, xed_state_t, xed_state_zero};

use xed_enum_generator::{enum_from_xed, enum_to_xed, instruction_enums, top_level_instruction_enum};

static START: Once = Once::new();

pub struct EncodeContext {
    xed_state: MaybeUninit<xed_state_t>,
}

impl EncodeContext {
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

#[cfg(test)]
pub mod test;
