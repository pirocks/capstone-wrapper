use wrapper_common::registers::{Reg64WithRIP, Reg8};
use xed_enum::ADD;

use crate::semantics2::arena::Arena;
use crate::semantics2::builder::SemanticsBuilder;
use crate::semantics2::read_write::{Readable, Writeable};
use crate::semantics2::semantic_steps::InstructionSemanticsStep;

pub fn add_generic<'arena, D1: Writeable<'arena>, S1: Readable<'arena>, S2: Readable<'arena>>(
    arena: Arena<'arena>,
    writeable1: D1,
    readable1: S1,
    readable2: S2,
    width: usize,
) -> Vec<InstructionSemanticsStep<'arena>> {
    //DEST := DEST + SRC;
    let mut s = SemanticsBuilder::new(arena);
    let dest = readable1.read(&s);
    let src = readable2.read(&s);
    let dest = s.add(dest, src);
    writeable1.write(&mut s, dest);
    s.finalize()
}

pub fn apply_iform_add(arena: Arena, instr: ADD) -> Vec<InstructionSemanticsStep> {
    match instr {
        ADD::ADD_AL_IMMB { operand_0 } => {
            add_generic(arena, Reg8::AL, Reg8::AL, operand_0,  8)
        }
        ADD::ADD_GPR8_GPR8_00 { operand_0, operand_1 } => {
            add_generic(arena, operand_0, operand_0, operand_1, 8)
        }
        ADD::ADD_GPR8_GPR8_02 { operand_0, operand_1 } => {
            add_generic(arena, operand_0, operand_0, operand_1, 8)
        }
        ADD::ADD_GPR8_IMMB_80R0 { operand_0, operand_1 } => {
            add_generic(arena, operand_0, operand_0, operand_1, 8)
        }
        ADD::ADD_GPR8_IMMB_82R0 { operand_0, operand_1 } => {
            add_generic(arena, operand_0, operand_0, operand_1, 8)
        }
        ADD::ADD_GPR8_MEMB { operand_0, operand_1 } => {
            add_generic(arena, operand_0, operand_0, operand_1, 8)
        }
        ADD::ADD_GPRV_GPRV_01_16 { operand_0, operand_1 } => {
            add_generic(arena, operand_0, operand_0, operand_1, 16)
        }
        ADD::ADD_GPRV_GPRV_01_32 { operand_0, operand_1 } => {
            add_generic(arena, operand_0, operand_0, operand_1, 32)
        }
        ADD::ADD_GPRV_GPRV_01_64 { operand_0, operand_1 } => {
            add_generic(arena, operand_0, operand_0, operand_1, 64)
        }
        ADD::ADD_GPRV_GPRV_03_16 { operand_0, operand_1 } => {
            add_generic(arena, operand_0, operand_0, operand_1, 16)
        }
        ADD::ADD_GPRV_GPRV_03_32 { operand_0, operand_1 } => {
            add_generic(arena, operand_0, operand_0, operand_1, 32)
        }
        ADD::ADD_GPRV_GPRV_03_64 { operand_0, operand_1 } => {
            add_generic(arena, operand_0, operand_0, operand_1, 64)
        }
        ADD::ADD_GPRV_IMMB_16 { operand_0, operand_1 } => {
            add_generic(arena, operand_0, operand_0, operand_1, 16)
        }
        ADD::ADD_GPRV_IMMB_32 { operand_0, operand_1 } => {
            add_generic(arena, operand_0, operand_0, operand_1, 32)
        }
        ADD::ADD_GPRV_IMMB_64 { operand_0, operand_1 } => {
            add_generic(arena, operand_0, operand_0, operand_1, 64)
        }
        ADD::ADD_GPRV_IMMZ_16 { operand_0, operand_1 } => {
            add_generic(arena, operand_0, operand_0, operand_1, 16)
        }
        ADD::ADD_GPRV_IMMZ_32 { operand_0, operand_1 } => {
            add_generic(arena, operand_0, operand_0, operand_1, 32)
        }
        ADD::ADD_GPRV_IMMZ_64 { operand_0, operand_1 } => {
            add_generic(arena, operand_0, operand_0, operand_1, 64)
        }
        ADD::ADD_GPRV_MEMV_16 { operand_0, operand_1 } => {
            add_generic(arena, operand_0, operand_0, operand_1, 16)
        }
        ADD::ADD_GPRV_MEMV_32 { operand_0, operand_1 } => {
            add_generic(arena, operand_0, operand_0, operand_1, 32)
        }
        ADD::ADD_GPRV_MEMV_64 { operand_0, operand_1 } => {
            add_generic(arena, operand_0, operand_0, operand_1, 64)
        }
        ADD::ADD_MEMB_GPR8 { operand_0, operand_1 } => {
            add_generic(arena, operand_0, operand_0, operand_1, 8)
        }
        ADD::ADD_MEMB_IMMB_80R0 { operand_0, operand_1 } => {
            add_generic(arena, operand_0, operand_0, operand_1, 8)
        }
        ADD::ADD_MEMB_IMMB_82R0 { operand_0, operand_1 } => {
            add_generic(arena, operand_0, operand_0, operand_1, 8)
        }
        ADD::ADD_MEMV_GPRV_16 { operand_0, operand_1 } => {
            add_generic(arena, operand_0, operand_0, operand_1, 16)
        }
        ADD::ADD_MEMV_GPRV_32 { operand_0, operand_1 } => {
            add_generic(arena, operand_0, operand_0, operand_1, 32)
        }
        ADD::ADD_MEMV_GPRV_64 { operand_0, operand_1 } => {
            add_generic(arena, operand_0, operand_0, operand_1, 64)
        }
        ADD::ADD_MEMV_IMMB_16 { operand_0, operand_1 } => {
            add_generic(arena, operand_0, operand_0, operand_1, 16)
        }
        ADD::ADD_MEMV_IMMB_32 { operand_0, operand_1 } => {
            add_generic(arena, operand_0, operand_0, operand_1, 32)
        }
        ADD::ADD_MEMV_IMMB_64 { operand_0, operand_1 } => {
            add_generic(arena, operand_0, operand_0, operand_1, 64)
        }
        ADD::ADD_MEMV_IMMZ_16 { operand_0, operand_1 } => {
            add_generic(arena, operand_0, operand_0, operand_1, 16)
        }
        ADD::ADD_MEMV_IMMZ_32 { operand_0, operand_1 } => {
            add_generic(arena, operand_0, operand_0, operand_1, 32)
        }
        ADD::ADD_MEMV_IMMZ_64 { operand_0, operand_1 } => {
            add_generic(arena, operand_0, operand_0, operand_1, 64)
        }
        ADD::ADD_ORAX_IMMZ_16 { operand_0 } => {
            add_generic(arena, Reg64WithRIP::RAX,Reg64WithRIP::RAX, operand_0, 16)
        }
        ADD::ADD_ORAX_IMMZ_32 { operand_0  } => {
            add_generic(arena, Reg64WithRIP::RAX,Reg64WithRIP::RAX, operand_0, 32)
        }
        ADD::ADD_ORAX_IMMZ_64 { operand_0 } => {
            add_generic(arena, Reg64WithRIP::RAX,Reg64WithRIP::RAX, operand_0, 64)
        }
    }
}
