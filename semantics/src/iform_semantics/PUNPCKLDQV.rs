use wrapper_common::registers::{RegMMX, RegXMM};
use xed_enum::PUNPCKLDQVisitor;
use xed_wrapper::operands::MemoryOperands;

pub struct PUNPCKLDQVSemanticsVisitor {}

impl PUNPCKLDQVisitor for PUNPCKLDQVSemanticsVisitor {
    fn visit_PUNPCKLDQ_MMXQ_MMXD(&self, operand_0: RegMMX, operand_1: RegMMX) {
        todo!()
    }

    fn visit_PUNPCKLDQ_XMMDQ_MEMDQ(&self, operand_0: RegXMM, operand_1: MemoryOperands) {
        todo!()
    }

    fn visit_PUNPCKLDQ_MMXQ_MEMD(&self, operand_0: RegMMX, operand_1: MemoryOperands) {
        todo!()
    }

    fn visit_PUNPCKLDQ_XMMDQ_XMMQ(&self, operand_0: RegXMM, operand_1: RegXMM) {
        todo!()
    }
}
