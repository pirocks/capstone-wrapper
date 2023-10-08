use anyhow::anyhow;
use cpp_demangle::ast::MangledName;
use cpp_demangle::DemangleOptions;
use llvm_ir::Module;

fn main() -> anyhow::Result<()> {
    let module = Module::from_bc_path("/home/francis/CLionProjects/remill/lib/Arch/X86/Runtime/Instructions.bc")
        .map_err(|s|anyhow!("{s}"))?;
    for f in module.functions.iter() {
        dbg!(&f.name);
        if !f.name.starts_with("__"){
            for x in f.parameters.iter() {
                dbg!(x);
            }
            for bb in f.basic_blocks.iter() {
                dbg!(bb.instrs.as_slice());
            }
        }
    }
    Ok(())
}
