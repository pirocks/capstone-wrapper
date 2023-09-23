pub mod clang_json_defs;

#[cfg(test)]
mod tests {
    use std::fs;
    use crate::clang_json_defs::ASTNode;


    #[test]
    fn it_works() -> anyhow::Result<()> {
        let _top_level: ASTNode = serde_json::from_slice(fs::read("/home/francis/CLionProjects/capstone-wrapper/remill-semantics-parser/data/Instructions.json")?.as_slice())?;
        Ok(())
    }
}
