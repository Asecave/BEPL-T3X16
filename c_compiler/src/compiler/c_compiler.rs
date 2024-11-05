use crate::Compiler;

pub struct CCompiler;

impl Compiler for CCompiler {
    fn compile(&self, raw_code: &str) -> Vec<i16> {
        vec![0i16, 1i16, 2i16]
    }
}

impl CCompiler {}
