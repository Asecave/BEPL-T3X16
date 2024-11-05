
mod assembly_compiler;
mod c_compiler;

pub trait Compiler {
    fn compile(&self, raw_code: &String) -> Vec<i16>;
}

pub struct AssemblyCompiler;
impl Compiler for AssemblyCompiler {
    fn compile(&self, raw_code: &String) -> Vec<i16> {
        assembly_compiler::compile_assembly(raw_code)
    }
}

pub struct CCompiler;
impl Compiler for CCompiler {
    fn compile(&self, raw_code: &String) -> Vec<i16> {
        c_compiler::compiel(raw_code)
    }
}
