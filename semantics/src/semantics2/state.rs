use wrapper_common::memory_operand::GeneralReg;
use crate::semantics2::arena::Arena;
use crate::semantics2::expression::Expression;
use crate::x86_machine::X86Mode;

pub struct Flags<'arena> {
    pub(crate) cf: Expression<'arena>,
    pub(crate) pf: Expression<'arena>,
    pub(crate) af: Expression<'arena>,
    pub(crate) zf: Expression<'arena>,
    pub(crate) sf: Expression<'arena>,
    pub(crate) of: Expression<'arena>,
}

pub struct X86MachineState64<'arena> {
    pub(crate) arena: Arena<'arena>,
    pub(crate) mode: X86Mode,
    pub(crate) rax: Expression<'arena>,
    pub(crate) rbx: Expression<'arena>,
    pub(crate) rcx: Expression<'arena>,
    pub(crate) rdx: Expression<'arena>,
    pub(crate) rsi: Expression<'arena>,
    pub(crate) rdi: Expression<'arena>,
    pub(crate) rsp: Expression<'arena>,
    pub(crate) rbp: Expression<'arena>,
    pub(crate) r8: Expression<'arena>,
    pub(crate) r9: Expression<'arena>,
    pub(crate) r10: Expression<'arena>,
    pub(crate) r11: Expression<'arena>,
    pub(crate) r12: Expression<'arena>,
    pub(crate) r13: Expression<'arena>,
    pub(crate) r14: Expression<'arena>,
    pub(crate) r15: Expression<'arena>,
    pub(crate) rip: Expression<'arena>,
    pub(crate) flags: Flags<'arena>,
}

impl<'arena> X86MachineState64<'arena> {}

#[derive(Debug, Clone, Copy)]
pub struct ConcreteX86MachineState64 {
    pub(crate) rax: u64,
    pub(crate) rbx: u64,
    pub(crate) rcx: u64,
    pub(crate) rdx: u64,
    pub(crate) rsi: u64,
    pub(crate) rdi: u64,
    pub(crate) rsp: u64,
    pub(crate) rbp: u64,
    pub(crate) r8: u64,
    pub(crate) r9: u64,
    pub(crate) r10: u64,
    pub(crate) r11: u64,
    pub(crate) r12: u64,
    pub(crate) r13: u64,
    pub(crate) r14: u64,
    pub(crate) r15: u64,
    pub(crate) rip: u64,
    pub(crate) flags: ConcreteFlags,
    pub(crate) zmms: [[u64;8];32]

}

pub trait XMMValue {
    fn to_u64(&self) -> [u64;2];
}

impl XMMValue for [u64;2] {
    fn to_u64(&self) -> [u64;2] {
        *self
    }
}

impl XMMValue for [u32;4] {
    fn to_u64(&self) -> [u64;2] {
        todo!()
    }
}

impl XMMValue for [f64;2] {
    fn to_u64(&self) -> [u64; 2] {
        unsafe { std::mem::transmute(*self) }
    }
}

impl ConcreteX86MachineState64 {
    pub fn zeroed() -> Self {
        Self{
            rax: 0,
            rbx: 0,
            rcx: 0,
            rdx: 0,
            rsi: 0,
            rdi: 0,
            rsp: 0,
            rbp: 0,
            r8: 0,
            r9: 0,
            r10: 0,
            r11: 0,
            r12: 0,
            r13: 0,
            r14: 0,
            r15: 0,
            rip: 0,
            flags: ConcreteFlags {
                cf: false,
                pf: false,
                af: false,
                zf: false,
                sf: false,
                of: false,
            },
            zmms: [[0;8]; 32],
        }
    }

    pub fn rax(mut self, value: u64) -> Self {
        self.rax = value;
        self
    }

    pub fn rbx(mut self, value: u64) -> Self {
        self.rbx = value;
        self
    }

    pub fn rcx(mut self, value: u64) -> Self {
        self.rcx = value;
        self
    }

    pub fn rdx(mut self, value: u64) -> Self {
        self.rdx = value;
        self
    }

    pub fn rsi(mut self, value: u64) -> Self {
        self.rsi = value;
        self
    }

    pub fn rdi(mut self, value: u64) -> Self {
        self.rdi = value;
        self
    }

    pub fn r8(mut self, value: u64) -> Self {
        self.r8 = value;
        self
    }

    pub fn r9(mut self, value: u64) -> Self {
        self.r9 = value;
        self
    }

    pub fn r10(mut self, value: u64) -> Self {
        self.r10 = value;
        self
    }

    pub fn r11(mut self, value: u64) -> Self {
        self.r11 = value;
        self
    }

    pub fn r12(mut self, value: u64) -> Self {
        self.r12 = value;
        self
    }

    pub fn r13(mut self, value: u64) -> Self {
        self.r13 = value;
        self
    }

    pub fn r14(mut self, value: u64) -> Self {
        self.r14 = value;
        self
    }


    pub fn r15(mut self, value: u64) -> Self {
        self.r15 = value;
        self
    }

    pub fn flags(mut self, value: ConcreteFlags) -> Self {
        self.flags = value;
        self
    }

    pub fn xmm0(mut self, value: impl XMMValue) -> Self {
        self.zmms[0][0] = value.to_u64()[0];
        self.zmms[0][1] = value.to_u64()[1];
        self
    }

    pub fn xmm1(mut self, value: impl XMMValue) -> Self {
        self.zmms[1][0] = value.to_u64()[0];
        self.zmms[1][1] = value.to_u64()[1];
        self
    }

    pub fn xmm2(mut self, value: impl XMMValue) -> Self {
        self.zmms[2][0] = value.to_u64()[0];
        self.zmms[2][1] = value.to_u64()[1];
        self
    }

    pub fn xmm3(mut self, value: impl XMMValue) -> Self {
        self.zmms[3][0] = value.to_u64()[0];
        self.zmms[3][1] = value.to_u64()[1];
        self
    }

    pub fn xmm4(mut self, value: impl XMMValue) -> Self {
        self.zmms[4][0] = value.to_u64()[0];
        self.zmms[4][1] = value.to_u64()[1];
        self
    }

    pub fn xmm5(mut self, value: impl XMMValue) -> Self {
        self.zmms[5][0] = value.to_u64()[0];
        self.zmms[5][1] = value.to_u64()[1];
        self
    }

    pub fn xmm6(mut self, value: impl XMMValue) -> Self {
        self.zmms[6][0] = value.to_u64()[0];
        self.zmms[6][1] = value.to_u64()[1];
        self
    }

    pub fn xmm7(mut self, value: impl XMMValue) -> Self {
        self.zmms[7][0] = value.to_u64()[0];
        self.zmms[7][1] = value.to_u64()[1];
        self
    }

    pub fn xmm8(mut self, value: impl XMMValue) -> Self {
        self.zmms[8][0] = value.to_u64()[0];
        self.zmms[8][1] = value.to_u64()[1];
        self
    }

    pub fn xmm9(mut self, value: impl XMMValue) -> Self {
        self.zmms[9][0] = value.to_u64()[0];
        self.zmms[9][1] = value.to_u64()[1];
        self
    }

    pub fn xmm10(mut self, value: impl XMMValue) -> Self {
        self.zmms[10][0] = value.to_u64()[0];
        self.zmms[10][1] = value.to_u64()[1];
        self
    }

    pub fn xmm11(mut self, value: impl XMMValue) -> Self {
        self.zmms[11][0] = value.to_u64()[0];
        self.zmms[11][1] = value.to_u64()[1];
        self
    }

    pub fn xmm12(mut self, value: impl XMMValue) -> Self {
        self.zmms[12][0] = value.to_u64()[0];
        self.zmms[12][1] = value.to_u64()[1];
        self
    }

    pub fn xmm13(mut self, value: impl XMMValue) -> Self {
        self.zmms[13][0] = value.to_u64()[0];
        self.zmms[13][1] = value.to_u64()[1];
        self
    }

    pub fn xmm14(mut self, value: impl XMMValue) -> Self {
        self.zmms[14][0] = value.to_u64()[0];
        self.zmms[14][1] = value.to_u64()[1];
        self
    }

    pub fn xmm15(mut self, value: impl XMMValue) -> Self {
        self.zmms[15][0] = value.to_u64()[0];
        self.zmms[15][1] = value.to_u64()[1];
        self
    }

    pub fn xmm16(mut self, value: impl XMMValue) -> Self {
        self.zmms[16][0] = value.to_u64()[0];
        self.zmms[16][1] = value.to_u64()[1];
        self
    }

    pub fn xmm17(mut self, value: impl XMMValue) -> Self {
        self.zmms[17][0] = value.to_u64()[0];
        self.zmms[17][1] = value.to_u64()[1];
        self
    }

    pub fn xmm18(mut self, value: impl XMMValue) -> Self {
        self.zmms[18][0] = value.to_u64()[0];
        self.zmms[18][1] = value.to_u64()[1];
        self
    }

    pub fn xmm19(mut self, value: impl XMMValue) -> Self {
        self.zmms[19][0] = value.to_u64()[0];
        self.zmms[19][1] = value.to_u64()[1];
        self
    }

    pub fn xmm20(mut self, value: impl XMMValue) -> Self {
        self.zmms[20][0] = value.to_u64()[0];
        self.zmms[20][1] = value.to_u64()[1];
        self
    }

    pub fn xmm21(mut self, value: impl XMMValue) -> Self {
        self.zmms[21][0] = value.to_u64()[0];
        self.zmms[21][1] = value.to_u64()[1];
        self
    }

    pub fn xmm22(mut self, value: impl XMMValue) -> Self {
        self.zmms[22][0] = value.to_u64()[0];
        self.zmms[22][1] = value.to_u64()[1];
        self
    }


    pub fn xmm23(mut self, value: impl XMMValue) -> Self {
        self.zmms[23][0] = value.to_u64()[0];
        self.zmms[23][1] = value.to_u64()[1];
        self
    }


    pub fn get_reg(reg: GeneralReg)
}

#[derive(Debug, Clone, Copy)]
pub struct ConcreteFlags {
    pub(crate) cf: bool,
    pub(crate) pf: bool,
    pub(crate) af: bool,
    pub(crate) zf: bool,
    pub(crate) sf: bool,
    pub(crate) of: bool,
}

impl ConcreteFlags {
    pub fn zeroed() -> Self {
        Self {
            cf: false,
            pf: false,
            af: false,
            zf: false,
            sf: false,
            of: false,
        }
    }

    pub fn cf(mut self, value: bool) -> Self {
        self.cf = value;
        self
    }

    pub fn pf(mut self, value: bool) -> Self {
        self.pf = value;
        self
    }

    pub fn af(mut self, value: bool) -> Self {
        self.af = value;
        self
    }

    pub fn zf(mut self, value: bool) -> Self {
        self.zf = value;
        self
    }

    pub fn sf(mut self, value: bool) -> Self {
        self.sf = value;
        self
    }

    pub fn of(mut self, value: bool) -> Self {
        self.of = value;
        self
    }



    pub fn to_u64(&self) -> u64 {
        let mut res = 0;
        if self.cf {
            res |= 1;
        }
        if self.pf {
            res |= 4;
        }
        if self.af {
            res |= 16;
        }
        if self.zf {
            res |= 64;
        }
        if self.sf {
            res |= 128;
        }
        if self.of {
            res |= 2048;
        }
        res
    }
}


pub struct ConcreteX86MachineState32 {
    pub(crate) eax: u32,
    pub(crate) ebx: u32,
    pub(crate) ecx: u32,
    pub(crate) edx: u32,
    pub(crate) esi: u32,
    pub(crate) edi: u32,
    pub(crate) esp: u32,
    pub(crate) ebp: u32,
    pub(crate) eip: u32,
    pub(crate) flags: ConcreteFlags,
}

// pub struct X86MachineState32<'arena> {
//     pub(crate) arena: Arena<'arena>,
//     pub(crate) eax: Expression<'arena>,
//     pub(crate) ebx: Expression<'arena>,
//     pub(crate) ecx: Expression<'arena>,
//     pub(crate) edx: Expression<'arena>,
//     pub(crate) esi: Expression<'arena>,
//     pub(crate) edi: Expression<'arena>,
//     pub(crate) esp: Expression<'arena>,
//     pub(crate) ebp: Expression<'arena>,
//     pub(crate) eip: Expression<'arena>,
//     pub(crate) flags: Flags<'arena>,
// }
//
// pub struct X86MachineState16<'arena> {
//     pub(crate) arena: Arena<'arena>,
//     pub(crate) ax: Expression<'arena>,
//     pub(crate) bx: Expression<'arena>,
//     pub(crate) cx: Expression<'arena>,
//     pub(crate) dx: Expression<'arena>,
//     pub(crate) si: Expression<'arena>,
//     pub(crate) di: Expression<'arena>,
//     pub(crate) sp: Expression<'arena>,
//     pub(crate) bp: Expression<'arena>,
//     pub(crate) ip: Expression<'arena>,
//     pub(crate) flags: Flags<'arena>,
// }
