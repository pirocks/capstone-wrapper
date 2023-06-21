#![feature(exit_status_error)]
#![feature(pattern)]

use std::collections::HashMap;
use std::fs;
use std::hash::Hash;

use itertools::Itertools;
use regex::Regex;

fn split_regex<'l>(s: &'l str, until: &Regex) -> (&'l str, &'l str) {
    let index = until.find(s).unwrap().start();
    (&s[..index], &s[index..])
}

fn try_split<'l>(s: &'l str, until: &str) -> Option<(&'l str, &'l str)> {
    let index = s.find(until)?;
    Some((&s[..index], &s[index..]))
}

fn split<'l>(s: &'l str, until: &str) -> (&'l str, &'l str) {
    let index = s.find(until).unwrap();
    (&s[..index], &s[index..])
}

fn slice_after<'l>(s: &'l str, until: &str) -> &'l str {
    let index = s.find(until).unwrap();
    &s[index..]
}

fn slice_before<'l>(s: &'l str, before: &str) -> &'l str {
    let index = s.find(before).unwrap();
    &s[..index]
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct UnExpandedInstruction(String);

fn main() -> anyhow::Result<()> {
    //gs -dNOPAUSE -dBATCH -sDEVICE=txtwrite -sOutputFile=output.xml Downloads/325462-sdm-vol-1-2abcd-3abcd.pdf
    // std::process::Command::new("gs")
    //     .arg("-dNOPAUSE")
    //     .arg("-dBATCH")
    //     .arg("-sDEVICE=txtwrite")
    //     .arg("-sOutputFile=output.txt")
    //     .arg("/home/francis/Downloads/325462-sdm-vol-1-2abcd-3abcd.pdf").spawn()?.wait()?.exit_ok()?;

    let top_level = fs::read_to_string("output.txt")?;
    // let top_level = slice_until(top_level.as_str(), "Vol. 1 E-11");
    let (contents, after_contents) = get_unexpanded(top_level.as_str());
    let mut instruction_contents = extract_instruction_contents(contents, after_contents);
    remove_headers_and_footers(&mut instruction_contents);
    let mut instruction_operations = HashMap::new();
    let regex = Regex::new("(Intel C/C\\+\\+ Compiler Intrinsic Equivalent)|(Flags Affected)|(Protected Mode Exceptions)|(SIMD Floating-Point Exceptions)|(C/C\\+\\+ Compiler Intrinsic Equivalent)|(FPU Flags Affected)|(Numeric Exceptions)").unwrap();
    for (instr, instr_content) in instruction_contents {
        if let Some((_, after_operation)) = try_split(instr_content.as_str(),"Operation"){
            let operation = split_regex(after_operation, &regex).0;
            instruction_operations.insert(instr, Some(operation.to_string()));
        }else {
            instruction_operations.insert(instr, None);
        }
    }

    for (instruction, instruction_content) in instruction_operations {
        fs::write(format!("instruction_{}.txt",instruction.0.replace("/", "_")), instruction_content.unwrap_or("".to_string()))?;
    }

    fs::write("debug.txt", after_contents)?;
    // // let top_level = slice_until(top_level, "INSTRUCTIONS");
    // dbg!(top_level);
    // let lines = top_level.split("\n");
    // let tables = top_level.split("Opcode");
    // for (i,table) in tables.filter(|table|table.contains("INSTRUCTION SET REFERENCE")).enumerate(){
    //     fs::write(format!("table_{i}.txt"),table)?;
    // }
    Ok(())
}

fn remove_headers_and_footers(instruction_contents: &mut HashMap<UnExpandedInstruction, String>) {
    for (instr, instructions) in instruction_contents.iter_mut() {
        let mut new_lines = vec![];
        for line in instructions.lines() {
            if !line.contains(instr.0.as_str()) &&
                !line.contains("INSTRUCTION SET REFERENCE, A-L") &&
                !line.contains("INSTRUCTION SET REFERENCE, M-U") &&
                !line.contains("INSTRUCTION SET REFERENCE, V") &&
                !line.contains("INSTRUCTION SET REFERENCE, W-Z") {
                new_lines.push(line);
            }
        }
        *instructions = new_lines.join("\n");
    }
}

fn extract_instruction_contents(contents: Vec<UnExpandedInstruction>, after_contents: &str) -> HashMap<UnExpandedInstruction, String> {
    let mut remaining_instructions = after_contents;
    let mut delimiters_and_instruction: Vec<(String, String, UnExpandedInstruction)> = vec![];
    let mut prev: Option<UnExpandedInstruction> = None;
    for unexpanded_instruction in contents.iter() {
        if let Some(prev) = prev {
            delimiters_and_instruction.push((prev.0.to_string(), unexpanded_instruction.0.to_string(), prev));
        }
        prev = Some(unexpanded_instruction.clone());
    }
    let mut instruction_contents = HashMap::new();
    for (start, end, instr) in delimiters_and_instruction {
        let (_before_start, after_start) = split(remaining_instructions, start.as_str());
        let (before_end, after_end) = split(after_start, end.as_str());
        remaining_instructions = after_end;
        instruction_contents.insert(instr, before_end.to_string());
    }
    instruction_contents
}

fn get_unexpanded(top_level: &str) -> (Vec<UnExpandedInstruction>, &str) {
    let top_level = slice_after(slice_after(top_level, "Volume 2 (2A, 2B, 2C, & 2D):"), "Instruction Set Reference, A-Z");
    let top_level = slice_after(top_level, "3.3       INSTRUCTIONS (A-L)");
    let contents = slice_before(top_level, "APPENDIX A");
    let regex = Regex::new("\\s+(?P<instruction_name>[A-Z][0-9A-Za-z/\\[\\],]*—)").unwrap();
    (regex.find_iter(contents).map(|match_| {
        UnExpandedInstruction(regex.captures(match_.as_str()).unwrap().name("instruction_name").unwrap().as_str().to_string())
    }).collect_vec(), slice_after(top_level, "APPENDIX A"))
}

