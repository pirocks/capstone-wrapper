use std::collections::HashMap;
use bumpalo::Bump;
use wrapper_common::registers::Reg8;
use xed_enum::{AAA, ADC, X86Instruction};
use xed_wrapper::operands::MemoryOperands;
use crate::x86_machine::{QWordVariable, VariableMappings, X86MachineState, X86Mode};
use crate::x86_machine::values::QWordValue;

#[test]
pub fn test_aaa_does_something() {
    let mut bump = Bump::new();
    let mut state = X86MachineState::new_all_zeroed(&bump, X86Mode::Real);
    state.rax = QWordValue::Var(QWordVariable(0));
    state.apply_instruction(&bump, X86Instruction::AAA(AAA::AAA {}));
    let mut qword_mappings = HashMap::new();
    qword_mappings.insert(QWordVariable(0), &*bump.alloc(QWordValue::Constant('Z' as u64 + 1)));
    let variable_mappings = VariableMappings::from_qword_variables(qword_mappings);
    let concrete = state.make_concrete(&variable_mappings);
    assert_eq!(concrete.rax, 1);
}


#[test]
pub fn test_adc() {
    let mut bump = Bump::new();
    let mut state = X86MachineState::new_all_zeroed(&bump, X86Mode::_64Bit);
    state.rax = QWordValue::Var(QWordVariable(0));
    state.apply_instruction(&bump, X86Instruction::ADC(ADC::ADC_GPR8_GPR8_12 { operand_0: Reg8::BL, operand_1: Reg8::CH }));
    let mut qword_mappings = HashMap::new();
    qword_mappings.insert(QWordVariable(0), &*bump.alloc(QWordValue::Constant(1)));
    let variable_mappings = VariableMappings::from_qword_variables(qword_mappings);
    let concrete = state.make_concrete(&variable_mappings);
    dbg!(concrete);
    panic!()
}
