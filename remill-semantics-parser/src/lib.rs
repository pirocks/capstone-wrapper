pub mod clang_json_defs;
pub mod unneeded_data_stripped{

}
pub mod extract;

#[cfg(test)]
mod tests {
    use std::fs;
    use crate::clang_json_defs::ASTNode;
    use crate::extract::{extract_referenced_id_isel, functions_by_id, isels};

    #[test]
    fn it_works() -> anyhow::Result<()> {
        let top_level: ASTNode = serde_json::from_slice(fs::read("/home/francis/CLionProjects/capstone-wrapper/remill-semantics-parser/data/Instructions.json")?.as_slice())?;
        let indexed_functions = functions_by_id(&top_level);
        let isels = isels(&top_level);
        let isel = &isels[100];
        let id = extract_referenced_id_isel(isel);
        let extracted = indexed_functions.get(&id).unwrap();
        dbg!(extracted);
        panic!();
        Ok(())
    }
}
