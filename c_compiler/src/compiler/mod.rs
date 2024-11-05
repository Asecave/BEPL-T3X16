pub mod assembly_compiler;
pub use assembly_compiler::AssemblyCompiler;
pub mod c_compiler;

pub trait Compiler {
    fn compile(&self, raw_code: &str) -> Vec<i16>;
}
