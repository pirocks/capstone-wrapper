use crate::k_expressions::{has_execinstr_label, TopLevel};
use std::fs::File;
use std::io::BufReader;

#[test]
pub fn parse_output() -> anyhow::Result<()> {
    let _top_level: TopLevel = serde_json::from_reader(BufReader::new(File::open(
        "/home/francis/x86-semantics-workspace/X86-64-semantics/formatted.json",
    )?))?;
    Ok(())
}

#[test]
pub fn find_adcb() -> anyhow::Result<()> {
    let top_level: TopLevel = serde_json::from_reader(BufReader::new(File::open(
        "/home/francis/x86-semantics-workspace/X86-64-semantics/formatted_parsed.json",
    )?))?;
    for m in top_level.term.modules {
        if m.name == "ADCB-R8-R8".to_string() {
            for sentence in &m.localSentences {
                if has_execinstr_label(sentence) {
                    println!("{}", serde_json::to_string_pretty(&sentence)?);
                }
            }
        }
    }
    Ok(())
}
