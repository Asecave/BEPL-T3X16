use std::{env, fs::read_to_string};

use compiler::Compiler;

mod compiler;
mod schematic;
mod simulator;

fn main() {
    let compiler: Box<dyn Compiler> = match env::args().any(|arg| arg == "--asm") {
        true => Box::new(compiler::AssemblyCompiler),
        false => Box::new(compiler::CCompiler),
    };

    let source_file = env::args().last().expect("No source file specified");
    let raw_assembly = read_to_string(source_file).expect("Could not read file");

    let hex_code = compiler.compile(&raw_assembly);

    // let binary = hex_code_to_binary(&hex_code);
    // println!("{}", binary);

    // schematic::create_rom_schematic(&hex_code);

    simulator::simulate(&hex_code);
}

fn hex_code_to_binary(hex_code: &Vec<u16>) -> String {
    hex_code
        .iter()
        .map(|code| format!("{:#018b}", code))
        .fold(String::new(), |acc, s| {
            if acc.is_empty() {
                s
            } else {
                format!("{}\n{}", acc, s)
            }
        })
}
