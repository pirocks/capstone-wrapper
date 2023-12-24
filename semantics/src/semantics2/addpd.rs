use xed_enum::{ADDPD, VADDPD};
use crate::semantics2::arena::Arena;
use crate::semantics2::builder::SemanticsBuilder;
use crate::semantics2::read_write::{Readable, Writeable};
use crate::semantics2::semantic_steps::InstructionSemanticsStep;

pub fn addpd_generic<'arena, D1: Writeable<'arena>, S1: Readable<'arena>, S2: Readable<'arena>>(
    arena: Arena<'arena>,
    writeable1: D1,
    readable1: S1,
    readable2: S2,
) -> Vec<InstructionSemanticsStep<'arena>> {
    // DEST[63:0] := DEST[63:0] + SRC[63:0]
    // DEST[127:64] := DEST[127:64] + SRC[127:64]
    // DEST[MAXVL-1:128] (Unmodified)

    let mut s = SemanticsBuilder::new(arena);
    let dest = readable1.read(&s);
    let src = readable2.read(&s);
    let after_0_64 = s.change(dest, 0, 64, s.fadd(s.extract(dest, 0, 64), s.extract(src, 0, 64)));
    let after_64_128 = s.change(after_0_64, 64, 128, s.fadd(s.extract(dest, 64, 128), s.extract(src, 64, 128)));
    writeable1.write(&mut s, after_64_128);
    s.finalize()
}

pub fn apply_iform_addpd(arena: Arena, instr: ADDPD) -> Vec<InstructionSemanticsStep> {
    match instr {
        ADDPD::ADDPD_XMMPD_MEMPD_128 { operand_0, operand_1 } => {
            addpd_generic(arena, operand_0, operand_0, operand_1)
        }
        ADDPD::ADDPD_XMMPD_XMMPD { operand_0, operand_1 } => {
            addpd_generic(arena, operand_0, operand_0, operand_1)
        }
    }
}
