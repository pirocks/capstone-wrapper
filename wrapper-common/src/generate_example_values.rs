use std::iter;
use enum_iterator::all;
use crate::memory_operand::{GeneralReg, X86Scale};
use crate::registers::{Reg16WithRIP, Reg32WithRIP, Reg64WithRIP, Reg8, RegSegment};

pub trait GenerateExampleValues {
    fn generate() -> impl Iterator<Item=Self>;
}

impl<T: GenerateExampleValues> GenerateExampleValues for Option<T> {
    fn generate() -> impl Iterator<Item=Self> {
        iter::once(None).chain(T::generate().map(|it| Some(it)))
    }
}

impl GenerateExampleValues for RegSegment {
    fn generate() -> impl Iterator<Item=Self> {
        all::<Self>()
    }
}

impl GenerateExampleValues for Reg64WithRIP {
    fn generate() -> impl Iterator<Item=Self> {
        all::<Self>()
    }
}


impl GenerateExampleValues for Reg32WithRIP {
    fn generate() -> impl Iterator<Item=Self> {
        all::<Self>()
    }
}

impl GenerateExampleValues for Reg16WithRIP {
    fn generate() -> impl Iterator<Item=Self> {
        all::<Self>()
    }
}

impl GenerateExampleValues for Reg8 {
    fn generate() -> impl Iterator<Item=Self> {
        all::<Self>()
    }
}

impl GenerateExampleValues for X86Scale {
    fn generate() -> impl Iterator<Item=Self> {
        all::<Self>()
    }
}



impl GenerateExampleValues for GeneralReg {
    fn generate() -> impl Iterator<Item=Self> {
        let reg64 = Reg64WithRIP::generate().map(GeneralReg::Reg64);
        let reg32 = Reg32WithRIP::generate().map(GeneralReg::Reg32);
        let reg16 = Reg16WithRIP::generate().map(GeneralReg::Reg16);
        let reg8 = Reg8::generate().map(GeneralReg::Reg8);
        reg64.chain(reg32).chain(reg16).chain(reg8)
    }
}

impl GenerateExampleValues for i64 {
    fn generate() -> impl Iterator<Item=Self> {
        vec![i64::MIN, 0, 1, 2, i64::MAX].into_iter()
    }
}

impl GenerateExampleValues for u64 {
    fn generate() -> impl Iterator<Item=Self> {
        vec![0, 1, 2, u64::MAX].into_iter()
    }
}


