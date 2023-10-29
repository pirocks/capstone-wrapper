use std::fs::File;
use std::io::BufReader;

use wrapper_common::operand_type::{Imm, MemoryOperandType, MemoryOperandTypeKind, OperandType};
use wrapper_common::registers::RegisterType;

use crate::k_expressions::TopLevel;
use crate::{extract_rule_from_semantics, InstructionDescriptor};

#[test]
pub fn test_minimized() -> anyhow::Result<()> {
    let desc = InstructionDescriptor {
        operands: vec![
            OperandType::Reg(RegisterType::AllGP8),
            OperandType::Reg(RegisterType::AllGP8),
        ],
        name: "ADCB-R8-R8".to_string(),
    };
    let top_level: TopLevel =
        serde_json::from_reader(BufReader::new(File::open("data/formatted_parsed.json")?))?;
    let full_res = extract_rule_from_semantics(top_level, &desc);
    let top_level: TopLevel =
        serde_json::from_reader(BufReader::new(File::open("data/minimized.json")?))?;
    let minimized_res = extract_rule_from_semantics(top_level, &desc);
    assert_eq!(full_res, minimized_res);
    Ok(())
}

#[test]
pub fn test_extract_callq_m64() -> anyhow::Result<()> {
    let top_level: TopLevel =
        serde_json::from_reader(BufReader::new(File::open("data/minimized-CALLQ-M64.json")?))?;
    let _rule = extract_rule_from_semantics(
        top_level,
        &InstructionDescriptor {
            operands: vec![OperandType::Mem(MemoryOperandType {
                vsib: None,
                kind: MemoryOperandTypeKind::Mem64,
                load: true,
                store: false,
            })],
            name: "CALLQ-M64".to_string(),
        },
    );
    Ok(())
}

#[test]
pub fn test_extract_vfmadd213sd_xmm_xmm_xmm() -> anyhow::Result<()> {
    let top_level: TopLevel = serde_json::from_reader(BufReader::new(File::open(
        "data/minimized-VFMADD213SD-XMM-XMM-XMM.json",
    )?))?;
    let _rule = extract_rule_from_semantics(
        top_level,
        &InstructionDescriptor {
            operands: vec![
                OperandType::Reg(RegisterType::AllXmm32),
                OperandType::Reg(RegisterType::AllXmm32),
                OperandType::Reg(RegisterType::AllXmm32),
            ],
            name: "VFMADD213SD-XMM-XMM-XMM".to_string(),
        },
    );
    Ok(())
}

#[test]
pub fn test_extract_movq_r64_r64() -> anyhow::Result<()> {
    let top_level: TopLevel = serde_json::from_reader(BufReader::new(File::open(
        "data/minimized-MOVQ-R64-R64.json",
    )?))?;
    let _rule = extract_rule_from_semantics(
        top_level,
        &InstructionDescriptor {
            operands: vec![
                OperandType::Reg(RegisterType::AllGP64WithRIP),
                OperandType::Reg(RegisterType::AllGP64WithRIP),
            ],
            name: "MOVQ-R64-R64".to_string(),
        },
    );
    dbg!(_rule);
    Ok(())
}

#[test]
pub fn test_extract_andb_m8_rh() -> anyhow::Result<()> {
    let top_level: TopLevel = serde_json::from_reader(BufReader::new(File::open(
        "data/minimized-ANDB-M8-RH.json",
    )?))?;
    let _rule = extract_rule_from_semantics(
        top_level,
        &InstructionDescriptor {
            operands: vec![
                OperandType::Reg(RegisterType::AllGP8),
                OperandType::Mem(MemoryOperandType {
                    vsib: None,
                    kind: MemoryOperandTypeKind::Mem8,
                    load: true,
                    store: true,
                }),
            ],
            name: "ANDB-M8-RH".to_string(),
        },
    );
    Ok(())
}

#[test]
pub fn test_extract_andnps_xmm_m128() -> anyhow::Result<()> {
    let top_level: TopLevel = serde_json::from_reader(BufReader::new(File::open(
        "data/minimized-ANDNPS-XMM-M128.json",
    )?))?;
    let _rule = extract_rule_from_semantics(
        top_level,
        &InstructionDescriptor {
            operands: vec![
                OperandType::Mem(MemoryOperandType {
                    vsib: None,
                    kind: MemoryOperandTypeKind::Mem128,
                    load: true,
                    store: true,
                }),
                OperandType::Reg(RegisterType::AllXmm32),
            ],
            name: "ANDNPS-XMM-M128".to_string(),
        },
    );
    Ok(())
}

#[test]
pub fn test_extract_pblendvb_xmm_m128_xmm0() -> anyhow::Result<()> {
    let top_level: TopLevel = serde_json::from_reader(BufReader::new(File::open(
        "data/minimized-PBLENDVB-XMM-M128-XMM0.json",
    )?))?;
    let _rule = extract_rule_from_semantics(
        top_level,
        &InstructionDescriptor {
            operands: vec![
                OperandType::Reg(RegisterType::AllXmm32),
                OperandType::Mem(MemoryOperandType {
                    vsib: None,
                    kind: MemoryOperandTypeKind::Mem128,
                    load: true,
                    store: true,
                }),
                OperandType::Reg(RegisterType::AllXmm32),
            ],
            name: "PBLENDVB-XMM-M128-XMM0".to_string(),
        },
    );
    Ok(())
}

#[test]
pub fn test_extract_pextrw_mm16_xmm_imm8() -> anyhow::Result<()> {
    let top_level: TopLevel = serde_json::from_reader(BufReader::new(File::open(
        "data/minimized-PEXTRW-M16-XMM-IMM8.json",
    )?))?;
    let _rule = extract_rule_from_semantics(
        top_level,
        &InstructionDescriptor {
            operands: vec![
                OperandType::Imm(Imm::Imm8 {}),
                OperandType::Reg(RegisterType::AllXmm32),
                OperandType::Mem(MemoryOperandType {
                    vsib: None,
                    kind: MemoryOperandTypeKind::Mem16,
                    load: true,
                    store: true,
                }),
            ],
            name: "PEXTRW-M16-XMM-IMM8".to_string(),
        },
    );
    Ok(())
}

#[test]
pub fn test_extract_adcb_al_imm8() -> anyhow::Result<()> {
    let top_level: TopLevel = serde_json::from_reader(BufReader::new(File::open(
        "data/minimized-ADCB-AL-IMM8.json",
    )?))?;
    let _rule = extract_rule_from_semantics(
        top_level,
        &InstructionDescriptor {
            operands: vec![
                OperandType::Imm(Imm::Imm8 {}),
                OperandType::Reg(RegisterType::AllXmm32),
                OperandType::Mem(MemoryOperandType {
                    vsib: None,
                    kind: MemoryOperandTypeKind::Mem16,
                    load: true,
                    store: true,
                }),
            ],
            name: "ADCB-AL-IMM8".to_string(),
        },
    );
    Ok(())
}
