#![allow(dead_code)]

pub mod clang_json_defs;
pub mod unneeded_data_stripped;
pub mod extract;
pub mod function_def_to_rust;


#[cfg(test)]
mod tests {
    use std::fs;
    use std::fs::File;
    use std::io::Cursor;

    use crate::clang_json_defs::ASTNode;
    use crate::extract::{extract_referenced_id_isel, functions_by_id, isels};
    use crate::function_def_to_rust::function_def_to_rust;
    use crate::unneeded_data_stripped::ASTNodeCleanedUp;

    #[ignore]
    #[test]
    fn compress_instructions_json() -> anyhow::Result<()>{
        let top_level: ASTNode = serde_json::from_slice(fs::read("/home/francis/CLionProjects/capstone-wrapper/remill-semantics-parser/data/Instructions.json")?.as_slice())?;
        let top_level: ASTNodeCleanedUp = ASTNodeCleanedUp::from_unclean(top_level);
        let compressed = zstd::encode_all(Cursor::new(serde_json::to_string(&top_level)?.as_bytes()),19)?;
        fs::write("/home/francis/CLionProjects/capstone-wrapper/remill-semantics-parser/data/Instructions.json.short.zstd",compressed)?;
        Ok(())
    }

    // #[ignore]
    #[test]
    fn it_works() -> anyhow::Result<()> {
        let compressed_file = File::open("/home/francis/CLionProjects/capstone-wrapper/remill-semantics-parser/data/Instructions.json.short.zstd")?;
        let decoder = zstd::Decoder::new(compressed_file)?;
        let top_level: ASTNodeCleanedUp = serde_json::from_reader(decoder)?;
        let indexed_functions = functions_by_id(&top_level);
        let isels = isels(&top_level);
        for (i,isel) in isels.iter().enumerate(){
            let id = extract_referenced_id_isel(isel);
            let extracted = indexed_functions.get(&id).unwrap();
            dbg!(i);
            dbg!(function_def_to_rust(extracted));
        }
        Ok(())
    }
}
