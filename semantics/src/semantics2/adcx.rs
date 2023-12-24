use xed_enum::ADCX;
use crate::semantics2::arena::Arena;
use crate::semantics2::builder::SemanticsBuilder;
use crate::semantics2::read_write::{Readable, Writeable};
use crate::semantics2::semantic_steps::InstructionSemanticsStep;

pub fn apply_iform_adcx(arena: Arena, adc: ADCX) -> Vec<InstructionSemanticsStep> {
    match adc {
        ADCX::ADCX_GPR32D_GPR32D { operand_0, operand_1 } => {
            adc_generic(arena, operand_0, operand_0,operand_1, 32)
        }
        ADCX::ADCX_GPR32D_MEMD { operand_0, operand_1 } => {
            adc_generic(arena, operand_0, operand_0,operand_1, 32)
        }
        ADCX::ADCX_GPR64Q_GPR64Q { operand_0, operand_1 } => {
            adc_generic(arena, operand_0, operand_0,operand_1, 64)
        }
        ADCX::ADCX_GPR64Q_MEMQ { operand_0, operand_1 } => {
            adc_generic(arena, operand_0, operand_0,operand_1, 64)
        }
    }
}

fn adc_generic<'arena, D1: Writeable<'arena>, S1: Readable<'arena>, S2: Readable<'arena>>(
    arena: Arena<'arena>,
    writeable1: D1,
    readable1: S1,
    readable2: S2,
    width: usize
) -> Vec<InstructionSemanticsStep<'arena>> {
    //IF OperandSize is 64-bit
    //     THEN CF:DEST[63:0] := DEST[63:0] + SRC[63:0] + CF;
    //     ELSE CF:DEST[31:0] := DEST[31:0] + SRC[31:0] + CF;
    // FI;
    let mut s = SemanticsBuilder::new(arena);
    let cf = s.cf();
    let dest = readable1.read(&s);
    let src = readable2.read(&s);
    let cf_dest = s.add(s.zext_to(s.add(dest, src),65), s.zext_to(cf,65));
    let dest = s.lower_bits(cf_dest, width);
    let cf = s.upper_bits(cf_dest, 1);
    writeable1.write(&mut s, dest);
    s.set_cf(cf);
    s.finalize()
}

