use xed_enum::AAD;
use crate::semantics2::arena::Arena;
use crate::semantics2::builder::SemanticsBuilder;
use crate::semantics2::read_write::Readable;
use crate::semantics2::semantic_steps::InstructionSemanticsStep;

pub fn apply_iform_aad<'arena>(arena: Arena<'arena>, aad: AAD) -> Vec<InstructionSemanticsStep<'arena>> {
    match aad {
        AAD::AAD_IMMB { operand_0 } => {
            ///IF 64-Bit Mode
            //     THEN
            //         #UD;
            //     ELSE
            //         tempAL := AL;
            //         tempAH := AH;
            //         AL := (tempAL + (tempAH ∗ imm8)) AND FFH;
            //         (* imm8 is set to 0AH for the AAD mnemonic.*)
            //         AH := 0;
            // FI;
            let mut s = SemanticsBuilder::new(arena);
            s.undefined_exception_if_64_bit();
            //todo how to deal with saving expression values when regs get changed.
            s.set_al(s.bitand(
                s.add(s.al(), s.umul(s.ah(), operand_0.read(&s))),
                s.constant(0xFFu8),
            ));
            s.set_ah(s.constant(0u8));
            s.finalize()
        }
    }
}

