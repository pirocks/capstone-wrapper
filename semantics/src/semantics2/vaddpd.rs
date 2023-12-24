use wrapper_common::registers::{RegMask, RegXMM, RegYMM};
use xed_enum::VADDPD;

use crate::semantics2::arena::Arena;
use crate::semantics2::builder::SemanticsBuilder;
use crate::semantics2::read_write::{Readable, Writeable};
use crate::semantics2::semantic_steps::InstructionSemanticsStep;

fn vaddpd_generic<'arena, D1: Writeable<'arena>, S1: Readable<'arena>, S2: Readable<'arena>>(
    arena: Arena<'arena>,
    writeable1: D1,
    readable1: S1,
    readable2: S2,
    mask: Option<RegMask>,
    width: usize,
) -> Vec<InstructionSemanticsStep<'arena>> {
    match width {
        128 => {
            todo!("no avx512 for now")
        }
        256 => {
            match mask {
                None => {
                    //DEST[63:0] := SRC1[63:0] + SRC2[63:0]
                    // DEST[127:64] := SRC1[127:64] + SRC2[127:64]
                    // DEST[191:128] := SRC1[191:128] + SRC2[191:128]
                    // DEST[255:192] := SRC1[255:192] + SRC2[255:192]
                    // DEST[MAXVL-1:256] := 0
                    let mut s = SemanticsBuilder::new(arena);
                    let dest = readable1.read(&s);
                    let src = readable2.read(&s);
                    let after_0_64 = s.change(dest, 0, 64, s.fadd(s.extract(dest, 0, 64), s.extract(src, 0, 64)));
                    let after_64_128 = s.change(after_0_64, 64, 128, s.fadd(s.extract(dest, 64, 128), s.extract(src, 64, 128)));
                    let after_128_192 = s.change(after_64_128, 128, 192, s.fadd(s.extract(dest, 128, 192), s.extract(src, 128, 192)));
                    let after_192_256 = s.change(after_128_192, 192, 256, s.fadd(s.extract(dest, 192, 256), s.extract(src, 192, 256)));
                    writeable1.write(&mut s, after_192_256);
                    s.finalize()
                }
                Some(mask) => {
                    todo!()
                }
            }
        }
        _ => {
            todo!()
        }
    }
}

pub fn apply_iform_vaddpd(arena: Arena, instr: VADDPD) -> Vec<InstructionSemanticsStep> {
    match instr {
        VADDPD::VADDPD_XMMDQ_XMMDQ_MEMDQ { operand_0, operand_1, operand_2 } => {
            vex_128_generic(arena, operand_0, operand_1, operand_2)
        }
        VADDPD::VADDPD_XMMDQ_XMMDQ_XMMDQ { operand_0, operand_1, operand_2 } => {
            vex_128_generic(arena, operand_0, operand_1, operand_2)
        }
        VADDPD::VADDPD_YMMQQ_YMMQQ_MEMQQ { operand_0, operand_1, operand_2 } => {
            vec_256_generic(arena, operand_0, operand_1, operand_2)
        }
        VADDPD::VADDPD_YMMQQ_YMMQQ_YMMQQ { operand_0, operand_1, operand_2 } => {
            vec_256_generic(arena, operand_0, operand_1, operand_2)
        }
        VADDPD::VADDPD_XMMF64_MASKMSKW_XMMF64_MEMF64_AVX512 { operand_0, operand_1, operand_2, operand_3 } => {
            todo!()
        }
        VADDPD::VADDPD_XMMF64_MASKMSKW_XMMF64_XMMF64_AVX512 { operand_0, operand_1, operand_2, operand_3 } => {
            todo!()
        }

        VADDPD::VADDPD_YMMF64_MASKMSKW_YMMF64_MEMF64_AVX512 { operand_0, operand_1, operand_2, operand_3 } => {
            todo!()
        }
        VADDPD::VADDPD_YMMF64_MASKMSKW_YMMF64_YMMF64_AVX512 { operand_0, operand_1, operand_2, operand_3 } => {
            todo!()
        }

        VADDPD::VADDPD_ZMMF64_MASKMSKW_ZMMF64_MEMF64_AVX512 { operand_0, operand_1, operand_2, operand_3 } => {
            todo!()
        }
        VADDPD::VADDPD_ZMMF64_MASKMSKW_ZMMF64_ZMMF64_AVX512 { operand_0, operand_1, operand_2, operand_3 } => {
            todo!()
        }
    }
}

fn evex_generic_memory<'arena, D1: Writeable<'arena>, S1: Readable<'arena>, S2: Readable<'arena>>(
    arena: Arena<'arena>,
    writeable1: D1,
    readable1: S1,
    readable2: S2,
    mask: Option<RegMask>,
    width: usize,
) -> Vec<InstructionSemanticsStep<'arena>> {
    //(KL, VL) = (2, 128), (4, 256), (8, 512)
    // FOR j := 0 TO KL-1
    //     i := j * 64
    //     IF k1[j] OR *no writemask*
    //         THEN
    //             IF (EVEX.b = 1)
    //                 THEN
    //                     DEST[i+63:i] := SRC1[i+63:i] + SRC2[63:0]
    //                 ELSE
    //                     DEST[i+63:i] := SRC1[i+63:i] + SRC2[i+63:i]
    //             FI;
    //         ELSE
    //             IF *merging-masking* ; merging-masking
    //                 THEN *DEST[i+63:i] remains unchanged*
    //                 ELSE ; zeroing-masking
    //                     DEST[i+63:i] := 0
    //             FI
    //     FI;
    // ENDFOR
    // DEST[MAXVL-1:VL] := 0
    let kl = width / 64;
    let mut s = SemanticsBuilder::new(arena);
    let src1 = readable1.read(&s);
    let src2 = readable2.read(&s);
    // for j in 0..kl {
    //     let i = j * 64;
    //
    // }
    todo!("no avx512 for now")
}

fn vex_128_generic<'arena, S1: Readable<'arena>, S2: Readable<'arena>>(
    arena: Arena<'arena>,
    writeable1: RegXMM,
    readable1: S1,
    readable2: S2) -> Vec<InstructionSemanticsStep> {
    // DEST[63:0] := SRC1[63:0] + SRC2[63:0]
    // DEST[127:64] := SRC1[127:64] + SRC2[127:64]
    // DEST[MAXVL-1:128] := 0
    let mut s = SemanticsBuilder::new(arena);
    let dest = readable1.read(&s);
    let src = readable2.read(&s);
    let after_0_64 = s.change(dest, 0, 64, s.fadd(s.extract(dest, 0, 64), s.extract(src, 0, 64)));
    let after_64_128 = s.change(after_0_64, 64, 128, s.fadd(s.extract(dest, 64, 128), s.extract(src, 64, 128)));
    let widened = s.zext_to(after_64_128, 512);
    writeable1.widen_to_zmm().write(&mut s, widened);
    s.finalize()
}

fn vec_256_generic<'arena, S1: Readable<'arena>, S2: Readable<'arena>>(
    arena: Arena<'arena>,
    writeable1: RegYMM,
    readable1: S1,
    readable2: S2) -> Vec<InstructionSemanticsStep> {
    // DEST[63:0] := SRC1[63:0] + SRC2[63:0]
    // DEST[127:64] := SRC1[127:64] + SRC2[127:64]
    // DEST[191:128] := SRC1[191:128] + SRC2[191:128]
    // DEST[255:192] := SRC1[255:192] + SRC2[255:192]
    // DEST[MAXVL-1:256] := 0
    let mut s = SemanticsBuilder::new(arena);
    let dest = readable1.read(&s);
    let src = readable2.read(&s);
    let after_0_64 = s.change(dest, 0, 64, s.fadd(s.extract(dest, 0, 64), s.extract(src, 0, 64)));
    let after_64_128 = s.change(after_0_64, 64, 128, s.fadd(s.extract(dest, 64, 128), s.extract(src, 64, 128)));
    let after_128_192 = s.change(after_64_128, 128, 192, s.fadd(s.extract(dest, 128, 192), s.extract(src, 128, 192)));
    let after_192_256 = s.change(after_128_192, 192, 256, s.fadd(s.extract(dest, 192, 256), s.extract(src, 192, 256)));
    let widened = s.zext_to(after_192_256, 512);
    writeable1.widen_to_zmm().write(&mut s, widened);
    s.finalize()
}