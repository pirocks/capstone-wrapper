use wrapper_common::registers::Reg8;
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

impl<'arena> X86MachineState64<'arena> {

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
