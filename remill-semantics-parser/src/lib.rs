#![feature(extend_one)]
#![allow(dead_code)]

use std::collections::HashMap;
use std::fs::File;

use crate::extract::{extract_referenced_id_isel, functions_by_id, isels};
use crate::function_def_to_intermediate::{function_def_to_rust, RemillSemanticsParsed};
use crate::unneeded_data_stripped::ASTNodeCleanedUp;

pub(crate) mod clang_json_defs;
pub(crate) mod unneeded_data_stripped;
pub(crate) mod extract;
pub(crate) mod function_def_to_intermediate;
pub(crate) mod intermediate_to_rust;

pub(crate) fn load_simplified_semantics() -> anyhow::Result<HashMap<String, RemillSemanticsParsed>> {
    let compressed_file = File::open("/home/francis/CLionProjects/capstone-wrapper/remill-semantics-parser/data/Instructions.json.short.zstd")?;
    let decoder = zstd::Decoder::new(compressed_file)?;
    let top_level: ASTNodeCleanedUp = serde_json::from_reader(decoder)?;
    let indexed_functions = functions_by_id(&top_level);
    let isels = isels(&top_level);

    let mut simplified_semantics_by_name = HashMap::new();
    for (isel_name, isel) in isels.into_iter() {
        let id = extract_referenced_id_isel(&isel);
        let extracted = indexed_functions.get(&id).unwrap();
        if let Some(semantics) = function_def_to_rust(extracted) {
            simplified_semantics_by_name.insert(isel_name, semantics);
        }
    }
    Ok(simplified_semantics_by_name)
}


#[cfg(test)]
mod tests {
    use std::fs;
    use std::io::Cursor;
    use itertools::Itertools;

    use crate::clang_json_defs::ASTNode;
    use crate::intermediate_to_rust::to_rust;
    use crate::load_simplified_semantics;
    use crate::unneeded_data_stripped::ASTNodeCleanedUp;

    #[ignore]
    #[test]
    fn compress_instructions_json() -> anyhow::Result<()> {
        let top_level: ASTNode = serde_json::from_slice(fs::read("/home/francis/CLionProjects/capstone-wrapper/remill-semantics-parser/data/Instructions.json")?.as_slice())?;
        let top_level: ASTNodeCleanedUp = ASTNodeCleanedUp::from_unclean(top_level);
        let compressed = zstd::encode_all(Cursor::new(serde_json::to_string(&top_level)?.as_bytes()), 19)?;
        fs::write("/home/francis/CLionProjects/capstone-wrapper/remill-semantics-parser/data/Instructions.json.short.zstd", compressed)?;
        Ok(())
    }

    // #[ignore]
    #[test]
    fn it_works() -> anyhow::Result<()> {
        for (name, semantics) in load_simplified_semantics()?.iter().sorted_by_key(|(name,_)|name.as_str()) {
            dbg!(name);
            if let Some(option) = to_rust(semantics){
                eprintln!("{}", option.to_string());
            }
        }
        Ok(())
    }
}
