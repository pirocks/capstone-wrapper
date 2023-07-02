use capstone::arch::x86::{X86Operand, X86OperandType};
use crate::registers::{Reg16WithoutRIP, Reg16WithRIP, Reg32WithoutRIP, Reg32WithRIP, Reg64WithoutRIP, Reg64WithRIP, Reg8, RegBnd, RegControl, RegControlExtra, RegDebug, RegFloat, Register, RegMask, RegMMX, RegSegment, RegTMM, RegXMM, RegYMM, RegZMM};
use serde::{Deserialize, Serialize};
use crate::memory_operand::MemoryOperand;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum OperandImm{
    Imm64(i64),
    Imm32(i32),
    Imm16(i16),
    Imm8(i8),
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum Operand {
    Reg(Register),
    Imm(OperandImm),
}

impl Operand{
    pub fn from_capstone(operand: &X86Operand) -> Self{
        match operand.op_type {
            X86OperandType::Reg(reg) => {
                Self::Reg(Register::from_capstone(reg, operand))
            }
            X86OperandType::Imm(imm) => {
                if operand.size == 8{
                    Self::Imm(OperandImm::Imm64(imm))
                } else if operand.size == 4{
                    Self::Imm(OperandImm::Imm32(imm as i32))
                } else if operand.size == 2{
                    Self::Imm(OperandImm::Imm16(imm as i16))
                }else if operand.size == 1{
                    Self::Imm(OperandImm::Imm8(imm as i8))
                }
                else {
                    dbg!(operand.size);
                    todo!()
                }
            }
            X86OperandType::Mem(_mem) => {
                todo!()
            }
            X86OperandType::Invalid => {
                todo!()
            }
        }
    }

    pub fn unwrap_memoryoperand(&self) -> MemoryOperand {
        todo!()
    }

    pub fn unwrap_reg(&self) -> &Register{
        match self {
            Operand::Reg(reg) => reg,
            _ => panic!()
        }
    }

    pub fn unwrap_regmmx(&self) -> RegMMX {
        self.unwrap_reg().unwrap_regmmx()
    }

    pub fn unwrap_regxmm(&self) -> RegXMM {
        self.unwrap_reg().unwrap_regxmm()
    }

    pub fn unwrap_regymm(&self) -> RegYMM {
        self.unwrap_reg().unwrap_regymm()
    }

    pub fn unwrap_regzmm(&self) -> RegZMM {
        self.unwrap_reg().unwrap_regzmm()
    }

    pub fn unwrap_regtmm(&self) -> RegTMM {
        self.unwrap_reg().unwrap_regtmm()
    }

    pub fn unwrap_regmask(&self) -> RegMask {
        self.unwrap_reg().unwrap_regmask()
    }

    pub fn unwrap_regfloat(&self) -> RegFloat {
        self.unwrap_reg().unwrap_regfloat()
    }

    pub fn unwrap_regsegment(&self) -> RegSegment {
        self.unwrap_reg().unwrap_regsegment()
    }

    pub fn unwrap_reg64withrip(&self) -> Reg64WithRIP {
        self.unwrap_reg().unwrap_reg64withrip()
    }

    pub fn unwrap_reg32withrip(&self) -> Reg32WithRIP {
        self.unwrap_reg().unwrap_reg32withrip()
    }

    pub fn unwrap_reg16withrip(&self) -> Reg16WithRIP {
        self.unwrap_reg().unwrap_reg16withrip()
    }

    pub fn unwrap_reg8(&self) -> Reg8 {
        self.unwrap_reg().unwrap_reg8()
    }

    pub fn unwrap_reg64withoutrip(&self) -> Reg64WithoutRIP {
        self.unwrap_reg().unwrap_reg64withoutrip()
    }

    pub fn unwrap_reg32withoutrip(&self) -> Reg32WithoutRIP {
        self.unwrap_reg().unwrap_reg32withoutrip()
    }

    pub fn unwrap_reg16withoutrip(&self) -> Reg16WithoutRIP {
        self.unwrap_reg().unwrap_reg16withoutrip()
    }

    pub fn unwrap_regbnd(&self) -> RegBnd {
        self.unwrap_reg().unwrap_regbnd()
    }

    pub fn unwrap_regdebug(&self) -> RegDebug {
        self.unwrap_reg().unwrap_regdebug()
    }

    pub fn unwrap_regcontrol(&self) -> RegControl {
        self.unwrap_reg().unwrap_regcontrol()
    }

    pub fn unwrap_regcontrolextra(&self) -> RegControlExtra {
        self.unwrap_reg().unwrap_regcontrolextra()
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

    pub fn unwrap_i64(&self) -> i64 {
        todo!()
    }
}