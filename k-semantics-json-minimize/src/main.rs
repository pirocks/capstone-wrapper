use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use clap::Parser;
use k_semantics_json_parser::k_expressions::{KSentence, TopLevel};
use k_semantics_json_parser::k_to_raw;

#[derive(Parser)]
pub struct Opts{
    path: PathBuf,
    module_name: String
}


fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
    let json_file_path = opts.path;
    let module_name = opts.module_name;
    let mut top_level: TopLevel = serde_json::from_reader(BufReader::new(File::open(json_file_path)?))?;
    top_level.term.modules.retain(|module|module.name == module_name);
    for module in top_level.term.modules.iter_mut() {
        module.localSentences.retain(|sentence|{
            if let KSentence::KRule {.. } = sentence{
                k_to_raw::utils::has_execinstr_label(&sentence, "execinstr")
            }else {
                false
            }
        })
    }
    println!("{}", serde_json::to_string_pretty(&top_level)?);
    Ok(())
}
