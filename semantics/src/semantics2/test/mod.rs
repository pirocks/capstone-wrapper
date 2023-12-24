use std::arch::asm;
use std::arch::x86_64::{__cpuid, __get_cpuid_max};
use std::ffi::{c_uint, CStr};
use std::mem::transmute;
use std::ops::Add;
use std::ptr::null_mut;
use bumpalo::Bump;
use rand::{Rng, SeedableRng};

use xed_sys::{xed_encode, xed_error_enum_t2str};

use wrapper_common::memory_operand::{GeneralReg, X86Scale};
use wrapper_common::registers::{Reg32WithRIP, Reg64WithRIP, Reg8, RegXMM};
use xed_enum::{ADC, ADCX, ADD, EncodeDecodeContext, JMP, X86Instruction};
use xed_wrapper::operands::MemoryOperands;
use crate::semantics2::apply_instruction;
use crate::semantics2::arena::Arena;

use crate::semantics2::state::{ConcreteFlags, ConcreteX86MachineState64};
use crate::semantics2::test::instruction_64::run_instruction_64;

//todo actually execute the semantics to compare.

#[test]
pub fn test_adc() {
    let bump = Bump::new();
    let instr = X86Instruction::ADC(ADC::ADC_GPR8_GPR8_12 { operand_0: Reg8::BL, operand_1: Reg8::CL });
    let b = 0x12u8;
    let c = 0x34u8;
    let state = run_instruction_64(instr, ConcreteX86MachineState64::zeroed().rbx(b as u64).rcx(c as u64));
    let res = apply_instruction(Arena::new(&bump), instr);

    assert_eq!(state.rbx, (b + c) as u64);
    //generate a random number from a seeded entropy source
    let mut rng = rand::rngs::StdRng::seed_from_u64(0);
    for _ in 0..100 {
        let r8 = rng.gen::<u64>();
        let r9 = rng.gen::<u64>();
        let carry_in = rng.gen::<bool>();

        let instr = X86Instruction::ADC(ADC::ADC_GPRV_GPRV_13_64 { operand_0: Reg64WithRIP::R8, operand_1: Reg64WithRIP::R9 });
        let state = run_instruction_64(instr, ConcreteX86MachineState64::zeroed().r8(r8).r9(r9).flags(ConcreteFlags::zeroed().cf(carry_in)));
        let carry_in = if carry_in { 1 } else { 0 };
        let mut out = 0;
        let carry_out = unsafe { core::arch::x86_64::_addcarry_u64(carry_in, r8, r9, &mut out) };
        assert_eq!(state.r8, out);
        assert_eq!(state.flags.cf, carry_out != 0);
    }
}

#[test]
pub fn test_adcx() {
    let mut rng = rand::rngs::StdRng::seed_from_u64(0);
    for _ in 0..100 {
        let r8 = rng.gen::<u64>();
        let r9 = rng.gen::<u64>();
        let carry_in = rng.gen::<bool>();

        let instr = X86Instruction::ADCX(ADCX::ADCX_GPR64Q_GPR64Q { operand_0: Reg64WithRIP::R8, operand_1: Reg64WithRIP::R9 });
        let state = run_instruction_64(instr, ConcreteX86MachineState64::zeroed().r8(r8).r9(r9).flags(ConcreteFlags::zeroed().cf(carry_in)));
        let carry_in = if carry_in { 1 } else { 0 };
        let mut out = 0;
        let carry_out = unsafe { core::arch::x86_64::_addcarryx_u64(carry_in, r8, r9, &mut out) };
        assert_eq!(state.r8, out);
        assert_eq!(state.flags.cf, carry_out != 0);

        let instr = X86Instruction::ADCX(ADCX::ADCX_GPR32D_GPR32D { operand_0: Reg32WithRIP::EDX, operand_1: Reg32WithRIP::EBX });
        let edx = rng.gen::<u32>();
        let ebx = rng.gen::<u32>();
        let carry_in = rng.gen::<bool>();
        let state = run_instruction_64(instr, ConcreteX86MachineState64::zeroed().rdx(edx as u64).rbx(ebx as u64).flags(ConcreteFlags::zeroed().cf(carry_in)));
        let carry_in = if carry_in { 1 } else { 0 };
        let mut out = 0;
        let carry_out = unsafe { core::arch::x86_64::_addcarryx_u32(carry_in, edx, ebx, &mut out) };
        assert_eq!(state.rdx, out as u64);
        assert_eq!(state.flags.cf, carry_out != 0);
    }
}

#[test]
pub fn test_add() {
    let mut rng = rand::rngs::StdRng::seed_from_u64(0);
    for _ in 0..100 {
        let r8 = rng.gen::<u64>();
        let r9 = rng.gen::<u64>();
        let instr = X86Instruction::ADD(ADD::ADD_GPRV_GPRV_01_64 { operand_0: Reg64WithRIP::R8, operand_1: Reg64WithRIP::R9 });
        let state = run_instruction_64(instr, ConcreteX86MachineState64::zeroed().r8(r8).r9(r9));
        assert_eq!(state.r8, r8.wrapping_add(r9));
    }
}

#[test]
pub fn test_addpd() {
    let mut rng = rand::rngs::StdRng::seed_from_u64(0);
    for _ in 0..100 {
        let xmm0 = rng.gen::<[f64;2]>();
        let xmm10 = rng.gen::<[f64;2]>();
        let instr = X86Instruction::ADDPD(xed_enum::ADDPD::ADDPD_XMMPD_XMMPD { operand_0: RegXMM::XMM0, operand_1: RegXMM::XMM10 });
        let state = run_instruction_64(instr, ConcreteX86MachineState64::zeroed().xmm0(xmm0).xmm10(xmm10));
        unsafe {
            assert_eq!(transmute::<u64,f64>(state.zmms[0][0]), xmm0[0].add(xmm10[0]));
            assert_eq!(transmute::<u64,f64>(state.zmms[0][1]), xmm0[1].add(xmm10[1]));
        }
    }
}

pub mod instruction_64;