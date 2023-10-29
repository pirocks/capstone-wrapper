use itertools::{iproduct, Itertools};

use wrapper_common::generate_example_values::GenerateExampleValues;
use wrapper_common::memory_operand::{GeneralReg, X86Scale};
use wrapper_common::registers::RegSegment;

use crate::operands::MemoryOperands;

impl GenerateExampleValues for MemoryOperands {
    fn generate() -> impl Iterator<Item=Self> {
        let segments = Option::<RegSegment>::generate().collect_vec();
        let scale = X86Scale::generate().collect_vec();
        let index = Option::<GeneralReg>::generate().collect_vec();
        let base = GeneralReg::generate().collect_vec();
        let disp = i64::generate().collect_vec();
        let disp_width = [8, 16, 32, 64].into_iter().collect_vec();
        iproduct!(segments,scale, index,base,disp,disp_width).map(|(segment, scale, index, base, disp, disp_width)| {
            let segment = segment.clone();
            let scale = scale.clone();
            let index = index.clone();
            let base = base.clone();
            let disp = disp.clone();
            let disp_width = disp_width.clone();
            MemoryOperands::SIBAddressing {
                segment,
                scale,
                index,
                base,
                disp,
                disp_width,
            }
        })
    }
}

#[test]
pub fn check_len() {
    let len = MemoryOperands::generate().count();
    dbg!(len);
    panic!();
}

#[test]
pub fn check_iter_time() {
    use rayon::iter::ParallelIterator;
    let len = MemoryOperands::generate().count();
    dbg!(len);
    MemoryOperands::generate().step_by(10000).collect_vec().into_par_iter().for_each(|operand| {
        for _ in MemoryOperands::generate().step_by(10000) {
            for _ in MemoryOperands::generate().step_by(10000) {
                let operand: MemoryOperands = operand;
                let MemoryOperands::SIBAddressing { segment, base, scale, index, disp, disp_width } = operand;
                std::hint::black_box(unsafe { xed_mem_gbisd(segment.as_ref().map(|seg| seg.to_xed()).unwrap_or(0), base.to_xed(), index.as_ref().map(|index| index.to_xed()).unwrap_or(0), scale.to_xed(), xed_disp(disp, disp_width), 64) });
            }
        }
    })
}