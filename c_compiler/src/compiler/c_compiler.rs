use crate::Compiler;

pub struct CCompiler;

impl Compiler for CCompiler {
    fn compile(&self, raw_code: &str) -> Vec<u16> {
        vec![0u16, 1u16, 2u16]
    }
}

impl CCompiler {}
