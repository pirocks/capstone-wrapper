use std::{io, vec};
use std::collections::HashMap;
use std::fs::File;
use std::io::{Cursor, Read, Write};
use std::path::PathBuf;

use anyhow::{anyhow, Context};
use clap::Parser;

#[derive(Parser)]
pub struct Opts {
    generate_to: Option<PathBuf>,
    csv: Option<PathBuf>,
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct OpCode(String);

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct CSVOperand(String);

#[derive(Clone, Debug)]
pub struct CSVEncoding {
    csv_operands: Vec<Vec<ParsedOperandSimple>>,
}

#[derive(Clone, Debug)]
pub struct ParsedCSV {
    instructions: HashMap<OpCode, Vec<CSVEncoding>>,
}

#[derive(Clone, Debug)]
pub enum ParsedOperandSimple {
    IMM8,
    IMM16,
    IMM32,
    IMM64,
    MOFFS8,
    MOFFS16,
    MOFFS32,
    MOFFS64,
    REL8,
    REL16,
    REL32,
    DS,
    ES,
    SS,
    FS,
    GS,
    CS,
    AL,
    CL,
    AX,
    DX,
    EAX,
    RAX,
    R8,
    R16,
    R32,
    R64,
    M8,
    M16,
    M32,
    M32BCST,
    M64BCST,
    M64,
    M128,
    M256,
    M512,
    M512BYTE,
    M1428BYTE,
    M94108BYTE,
    M16INT,
    M32INT,
    M64INT,
    M1616,
    M1632,
    M1664,
    M3232,
    MEM,
    MM,
    XMM,
    YMM,
    ZMM,
    SegmentRegister,
    K,
    KMask,
    BND,
    MIB,
    PTR1616,
    PTR1632,
    M32FP,
    M64FP,
    VM32X,
    VM32Y,
    VM32Z,
    VM64X,
    VM64V,
    VM64Y,
    VM64Z,
    ST0,
    STI,
    ST,
    M80DEC,
    M80BCD,
    M80FP,
    M2BYTE,
    CR0CR7,
    CR8,
    DR0DR7,
    DR8,
    _0,
    _1,
    _3,
    M
}

#[derive(Clone, Debug)]
pub enum ParsedOperand {
    AL,
    AX,
    DX,
    EAX,
    RAX,
    R8,
    R16,
    R32,
    R64,
    RM8,
    RM16,
    RM32,
    RM32er,
    RM64,
    RM64er,
    IMM8,
    IMM8R,
    IMM16,
    IMM32,
    IMM64,
    REL8,
    REL16,
    REL32,
    PTR88,
    PTR1616,
    PTR1632,
    PTR3232,
    M1616,
    M1632,
    M1664,
    M3232,
    M8,
    M16,
    M32,
    M64,
    M128,
    XMM,
    XMMM8,
    XMMM16,
    XMMM32,
    XMMM32er,
    XMMM32sae,
    XMMM32M64M32BCST,
    XMMM64,
    XMMM64er,
    XMMM64sae,
    XMMM128,
    XMMM128M32BCST,
    XMMM128M64BCST,
    YMM,
    YMMM256,
    YMMM256sae,
    YMMM256M32BCST,
    YMMM256M32BCSTer,
    YMMM256M32BCSTsae,
    YMMM256M64BCST,
    ZMM,
    ZMMsae,
    ZMMM512,
    ZMMM512M32BCST,
    ZMMM512M32BCSTer,
    ZMMM512M64BCST,
    ZMMM512M64BCSTer,
    ZMMM512M32BCSTsae,
    ZMMM512M64BCSTsae,
    KZ,
    K,
    BND,
    BNDM64,
    BNDM128,
    MIB,
    MM,
    MMM32,
    MMM64,
    _0,
    _1,
    _3,
    M32FP,
    M64FP,
    ST0,
    STI,
    ST,
    M16INT,
    M32INT,
    M64INT,
    M80DEC,
    M80BCD,
    M80FP,
    M2BYTE,
    M1428BYTE,
    M94108BYTE,
    M512BYTE,
    M,
    KM8,
    KM16,
    KM32,
    KM64,
    REG,
    R32M16,
    R64M16,
    MEM,
    M256,
    SREG,
    MOFFS8,
    MOFFS16,
    MOFFS32,
    MOFFS64,
    CR0CR7,
    CR8,
    DR0DR7,
    DR8,
    M512,
    REGM8,
    REGM16,
    R32M8,
    DS,
    ES,
    SS,
    FS,
    GS,
    CS,
    CL,
    VM32X,
    VM32Y,
    VM32Z,
    VM64X,
    VM64V,
    VM64Y,
    VM64Z,
}


fn main() -> anyhow::Result<()> {
    let Opts { generate_to, csv } = Opts::parse();
    let write_to = get_write_to(generate_to)?;
    let csv_string = get_csv(csv)?;
    let parsed = parse_csv(csv_string)?;
    dbg!(parsed);
    Ok(())
}

fn parse_csv_operands(operand: &str) -> anyhow::Result<Vec<ParsedOperandSimple>> {
    Ok(match operand {
        "imm8" => vec![ParsedOperandSimple::IMM8],
        "imm16" => vec![ParsedOperandSimple::IMM16],
        "imm32" => vec![ParsedOperandSimple::IMM32],
        "imm64" => vec![ParsedOperandSimple::IMM64],
        "DS" => vec![ParsedOperandSimple::DS],
        "ES" => vec![ParsedOperandSimple::ES],
        "SS" => vec![ParsedOperandSimple::SS],
        "FS" => vec![ParsedOperandSimple::FS],
        "GS" => vec![ParsedOperandSimple::GS],
        "AL" => vec![ParsedOperandSimple::AL],
        "AX" => vec![ParsedOperandSimple::AX],
        "DX" => vec![ParsedOperandSimple::DX],
        "EAX" => vec![ParsedOperandSimple::EAX],
        "RAX" => vec![ParsedOperandSimple::RAX],
        "r8" => vec![ParsedOperandSimple::R8],
        "r16" => vec![ParsedOperandSimple::R16],
        "r32" => vec![ParsedOperandSimple::R32],
        "r64" => vec![ParsedOperandSimple::R64],
        "m16&16" => vec![ParsedOperandSimple::M1616],
        "m16&32" => vec![ParsedOperandSimple::M1632],
        "m16&64" => vec![ParsedOperandSimple::M1664],
        "m32&32" => vec![ParsedOperandSimple::M3232],
        "m32" => vec![ParsedOperandSimple::M32],
        "m64" => vec![ParsedOperandSimple::M64],
        "r/m8" => vec![ParsedOperandSimple::R8, ParsedOperandSimple::M8],
        "r/m16" => vec![ParsedOperandSimple::R16, ParsedOperandSimple::M16],
        "r/m32" => vec![ParsedOperandSimple::R32, ParsedOperandSimple::M32],
        "r32/m32" => vec![ParsedOperandSimple::R32, ParsedOperandSimple::M32],
        "r/m64" => vec![ParsedOperandSimple::R64, ParsedOperandSimple::M64],
        "xmm" => vec![ParsedOperandSimple::XMM],
        "xmm0" => vec![ParsedOperandSimple::XMM],
        "xmm1" => vec![ParsedOperandSimple::XMM],
        "xmm2" => vec![ParsedOperandSimple::XMM],
        "xmm3" => vec![ParsedOperandSimple::XMM],
        "xmm4" => vec![ParsedOperandSimple::XMM],
        "<XMM0>" => vec![ParsedOperandSimple::XMM],
        "xmm2/m8" => vec![ParsedOperandSimple::XMM, ParsedOperandSimple::M8],
        "xmm1/m16" => vec![ParsedOperandSimple::XMM, ParsedOperandSimple::M16],
        "xmm2/m16" => vec![ParsedOperandSimple::XMM, ParsedOperandSimple::M16],
        "xmm1/m32" => vec![ParsedOperandSimple::XMM, ParsedOperandSimple::M32],
        "xmm2/m32" => vec![ParsedOperandSimple::XMM, ParsedOperandSimple::M32],
        "xmm3/m32" => vec![ParsedOperandSimple::XMM, ParsedOperandSimple::M32],
        "xmm2/m64/m32bcst" => vec![ParsedOperandSimple::XMM, ParsedOperandSimple::M64, ParsedOperandSimple::M32BCST],
        "xmm/m64" => vec![ParsedOperandSimple::XMM, ParsedOperandSimple::M64],
        "xmm1/m64" => vec![ParsedOperandSimple::XMM, ParsedOperandSimple::M64],
        "xmm2/m64" => vec![ParsedOperandSimple::XMM, ParsedOperandSimple::M64],
        "xmm3/m64" => vec![ParsedOperandSimple::XMM, ParsedOperandSimple::M64],
        "xmm/m128" => vec![ParsedOperandSimple::XMM, ParsedOperandSimple::M128],
        "xmm1/m128" => vec![ParsedOperandSimple::XMM, ParsedOperandSimple::M128],
        "xmm2/m128" => vec![ParsedOperandSimple::XMM, ParsedOperandSimple::M128],
        "xmm3/m128" => vec![ParsedOperandSimple::XMM, ParsedOperandSimple::M128],
        "xmm2/m128/m32bcst" => vec![ParsedOperandSimple::XMM, ParsedOperandSimple::M128, ParsedOperandSimple::M32BCST],
        "xmm3/m128/m32bcst" => vec![ParsedOperandSimple::XMM, ParsedOperandSimple::M128, ParsedOperandSimple::M32BCST],
        "xmm2/m128/m64bcst" => vec![ParsedOperandSimple::XMM, ParsedOperandSimple::M128, ParsedOperandSimple::M64BCST],
        "xmm3/m128/m64bcst" => vec![ParsedOperandSimple::XMM, ParsedOperandSimple::M128, ParsedOperandSimple::M64BCST],
        "ymm1" => vec![ParsedOperandSimple::YMM],
        "ymm2" => vec![ParsedOperandSimple::YMM],
        "ymm4" => vec![ParsedOperandSimple::YMM],
        "ymm1/m256" => vec![ParsedOperandSimple::YMM, ParsedOperandSimple::M256],
        "ymm2/m256" => vec![ParsedOperandSimple::YMM, ParsedOperandSimple::M256],
        "ymm3/m256" => vec![ParsedOperandSimple::YMM, ParsedOperandSimple::M256],
        "ymm2/m256/m32bcst" => vec![ParsedOperandSimple::YMM, ParsedOperandSimple::M256, ParsedOperandSimple::M32BCST],
        "ymm3/m256/m32bcst" => vec![ParsedOperandSimple::YMM, ParsedOperandSimple::M256, ParsedOperandSimple::M32BCST],
        "ymm2/m256/m64bcst" => vec![ParsedOperandSimple::YMM, ParsedOperandSimple::M256, ParsedOperandSimple::M64BCST],
        "ymm3/m256/m64bcst" => vec![ParsedOperandSimple::YMM, ParsedOperandSimple::M256, ParsedOperandSimple::M64BCST],
        "zmm1" => vec![ParsedOperandSimple::ZMM],
        "zmm2" => vec![ParsedOperandSimple::ZMM],
        "zmm1/m512" => vec![ParsedOperandSimple::ZMM, ParsedOperandSimple::M512],
        "zmm2/m512" => vec![ParsedOperandSimple::ZMM, ParsedOperandSimple::M512],
        "zmm3/m512" => vec![ParsedOperandSimple::ZMM, ParsedOperandSimple::M512],
        "zmm2/m512/m32bcst" => vec![ParsedOperandSimple::ZMM, ParsedOperandSimple::M512, ParsedOperandSimple::M32BCST],
        "zmm3/m512/m32bcst" => vec![ParsedOperandSimple::ZMM, ParsedOperandSimple::M512, ParsedOperandSimple::M32BCST],
        "zmm2/m512/m64bcst" => vec![ParsedOperandSimple::ZMM, ParsedOperandSimple::M512, ParsedOperandSimple::M64BCST],
        "zmm3/m512/m64bcst" => vec![ParsedOperandSimple::ZMM, ParsedOperandSimple::M512, ParsedOperandSimple::M64BCST],
        "k1" => vec![ParsedOperandSimple::K],
        "{k1}" => vec![ParsedOperandSimple::KMask],
        "k2" => vec![ParsedOperandSimple::K],
        "k3" => vec![ParsedOperandSimple::K],
        "{k2}" => vec![ParsedOperandSimple::KMask],
        "bnd" => vec![ParsedOperandSimple::BND],
        "bnd1" => vec![ParsedOperandSimple::BND],
        "bnd2" => vec![ParsedOperandSimple::BND],
        "bnd1/m64" => vec![ParsedOperandSimple::BND, ParsedOperandSimple::M64],
        "bnd2/m64" => vec![ParsedOperandSimple::BND, ParsedOperandSimple::M64],
        "bnd1/m128" => vec![ParsedOperandSimple::BND, ParsedOperandSimple::M128],
        "bnd2/m128" => vec![ParsedOperandSimple::BND, ParsedOperandSimple::M128],
        "mib" => vec![ParsedOperandSimple::MIB],
        "rel8" => vec![ParsedOperandSimple::REL8],
        "rel16" => vec![ParsedOperandSimple::REL16],
        "rel32" => vec![ParsedOperandSimple::REL32],
        "ptr16:16" => vec![ParsedOperandSimple::PTR1616],
        "ptr16:32" => vec![ParsedOperandSimple::PTR1632],
        "m16:16" => vec![ParsedOperandSimple::M1616],
        "m16:32" => vec![ParsedOperandSimple::M1632],
        "m16:64" => vec![ParsedOperandSimple::M1664],
        "m8" => vec![ParsedOperandSimple::M8],
        "m16" => vec![ParsedOperandSimple::M16],
        "m128" => vec![ParsedOperandSimple::M128],
        "m512" => vec![ParsedOperandSimple::M512],
        "mm" => vec![ParsedOperandSimple::MM],
        "mm1" => vec![ParsedOperandSimple::MM],
        "mm2" => vec![ParsedOperandSimple::MM],
        "mm/m32" => vec![ParsedOperandSimple::MM, ParsedOperandSimple::M32],
        "mm/m64" => vec![ParsedOperandSimple::MM, ParsedOperandSimple::M64],
        "mm2/m64" => vec![ParsedOperandSimple::MM, ParsedOperandSimple::M64],
        "reg/m32" => vec![ParsedOperandSimple::R32, ParsedOperandSimple::R64, ParsedOperandSimple::M32],
        "m32fp" => vec![ParsedOperandSimple::M32FP],
        "m64fp" => vec![ParsedOperandSimple::M64FP],
        "ST(0)" => vec![ParsedOperandSimple::ST0],
        "ST(i)" => vec![ParsedOperandSimple::STI],
        "ST" => vec![ParsedOperandSimple::ST],
        "m16int" => vec![ParsedOperandSimple::M16INT],
        "m32int" => vec![ParsedOperandSimple::M32INT],
        "m64int" => vec![ParsedOperandSimple::M64INT],
        "m80dec" => vec![ParsedOperandSimple::M80DEC],
        "m80bcd" => vec![ParsedOperandSimple::M80BCD],
        "m80fp" => vec![ParsedOperandSimple::M80FP],
        "m2byte" => vec![ParsedOperandSimple::M2BYTE],
        "m14/28byte" => vec![ParsedOperandSimple::M1428BYTE],
        "m94/108byte" => vec![ParsedOperandSimple::M94108BYTE],
        "m512byte" => vec![ParsedOperandSimple::M512BYTE],
        "k2/m8" => vec![ParsedOperandSimple::K, ParsedOperandSimple::M8],
        "k2/m16" => vec![ParsedOperandSimple::K, ParsedOperandSimple::M16],
        "k2/m32" => vec![ParsedOperandSimple::K, ParsedOperandSimple::M32],
        "k2/m64" => vec![ParsedOperandSimple::K, ParsedOperandSimple::M64],
        "r16/m16" => vec![ParsedOperandSimple::R16, ParsedOperandSimple::M16],
        "r32/m16" => vec![ParsedOperandSimple::R32, ParsedOperandSimple::M16],
        "r64/m16" => vec![ParsedOperandSimple::R64, ParsedOperandSimple::M16],
        "r32/m8" => vec![ParsedOperandSimple::R32, ParsedOperandSimple::M8],
        "reg" => vec![ParsedOperandSimple::R32, ParsedOperandSimple::R64],
        "mem" => vec![ParsedOperandSimple::MEM],
        "m256" => vec![ParsedOperandSimple::M256],
        "Sreg" => vec![ParsedOperandSimple::SegmentRegister],
        "moffs8" => vec![ParsedOperandSimple::MOFFS8],
        "moffs16" => vec![ParsedOperandSimple::MOFFS16],
        "moffs32" => vec![ParsedOperandSimple::MOFFS32],
        "moffs64" => vec![ParsedOperandSimple::MOFFS64],
        "CR0-CR7" => vec![ParsedOperandSimple::CR0CR7],
        "CR8" => vec![ParsedOperandSimple::CR8],
        "DR0-DR7" => vec![ParsedOperandSimple::DR0DR7],
        "DR8" => vec![ParsedOperandSimple::DR8],
        "r64/m64" => vec![ParsedOperandSimple::R64, ParsedOperandSimple::M64],
        "reg/m8" => vec![ParsedOperandSimple::R32, ParsedOperandSimple::R64, ParsedOperandSimple::M8],
        "reg/m16" => vec![ParsedOperandSimple::R32, ParsedOperandSimple::R64, ParsedOperandSimple::M16],
        "CS" => vec![ParsedOperandSimple::CS],
        "CL" => vec![ParsedOperandSimple::CL],
        "vm32x" => vec![ParsedOperandSimple::VM32X],
        "vm32y" => vec![ParsedOperandSimple::VM32Y],
        "vm32z" => vec![ParsedOperandSimple::VM32Z],
        "vm64x" => vec![ParsedOperandSimple::VM64X],
        "vm64v" => vec![ParsedOperandSimple::VM64V],
        "vm64y" => vec![ParsedOperandSimple::VM64Y],
        "vm64z" => vec![ParsedOperandSimple::VM64Z],
        "0" => vec![ParsedOperandSimple::_0],
        "1" => vec![ParsedOperandSimple::_1],
        "3" => vec![ParsedOperandSimple::_3],
        "m" => vec![ParsedOperandSimple::M],
        _ => {
            dbg!(operand);
            todo!()
        }
    })
}

fn parse_csv(csv_string: Box<dyn Read>) -> anyhow::Result<ParsedCSV> {
    let mut reader = csv::Reader::from_reader(csv_string);
    let mut instructions: HashMap<OpCode, Vec<CSVEncoding>> = HashMap::new();
    for record in reader.records() {
        let record = record.with_context(|| "parse error")?;
        let mut record_iter = record.iter();
        let instruction_string = record_iter.next().ok_or(anyhow!("Unexpected end of csv record"))?;
        let arg_split_regex = regex::Regex::new(" |,|(, )").expect("regex compile failure");
        let mut instruction_split = arg_split_regex.split(instruction_string);
        let opcode = OpCode(instruction_split.next().ok_or(anyhow!("Unexpected instruction name"))?.to_string());
        let mut csv_operands = vec![];
        dbg!(&opcode);
        for operand in instruction_split {
            if operand.is_empty() {
                continue;
            }
            let mut has_rounding_mode = false;
            if operand.contains("{er}") {
                has_rounding_mode = true;
            }
            let without_rounding_mode = operand.replace("{er}", "");
            let mut has_suppress_all_exceptions = false;
            if operand.contains("{sae}") {
                has_suppress_all_exceptions = true;
            }
            let without_sae_er = without_rounding_mode.replace("{sae}", "");
            let mut avx_zero_op_mask = false;
            if operand.contains("{sae}") {
                avx_zero_op_mask = true;
            }
            let without_sae_er_z = without_sae_er.replace("{z}", "");
            csv_operands.push(parse_csv_operands(without_sae_er_z.as_str())?);
        }

        instructions.entry(opcode).or_default().push(CSVEncoding { csv_operands: csv_operands });

        let _valid_64_bit = record_iter.next().ok_or(anyhow!("Unexpected end of csv record"))?;
        let _valid_32_bit = record_iter.next().ok_or(anyhow!("Unexpected end of csv record"))?;
        let _valid_16_bit = record_iter.next().ok_or(anyhow!("Unexpected end of csv record"))?;
        let _feature_flags = record_iter.next().ok_or(anyhow!("Unexpected end of csv record"))?;
    }
    Ok(ParsedCSV { instructions })
}

fn get_csv(csv: Option<PathBuf>) -> anyhow::Result<Box<dyn Read>> {
    Ok(match csv {
        None => {
            Box::new(Cursor::new(include_bytes!("../resources/x86.csv"))) as Box<dyn Read>
        }
        Some(csv) => {
            Box::new(File::open(csv).with_context(|| "Cannot open csv file")?) as Box<dyn Read>
        }
    })
}

fn get_write_to(generate_to: Option<PathBuf>) -> anyhow::Result<Box<dyn Write>> {
    Ok(match generate_to {
        None => {
            Box::new(io::stdout().lock()) as Box<dyn Write>
        }
        Some(generate_to) => {
            Box::new(File::create(generate_to).with_context(|| "Cannot open output file")?) as Box<dyn Write>
        }
    })
}
