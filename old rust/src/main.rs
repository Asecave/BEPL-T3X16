use std::{env, fs};
use std::collections::{BTreeMap, HashMap};
use std::process::exit;
use cli_clipboard::{ClipboardContext, ClipboardProvider};
use log::error;
use mc_schem::schem::{Schematic, WorldEdit13SaveOption};
use mc_schem::{error, Block, Region};

fn main() {

    env_logger::init();

    let file_location = match env::var("CODE") {
        Ok(file) => file,
        Err(_) => {
            error!("No code file specified. Please add an environment variable with name CODE.");
            exit(1);
        }
    };

    let file = match fs::read_to_string(file_location) {
        Ok(file) => file,
        Err(_) => {
            error!("Code file not found.");
            exit(1);
        }
    };

    let instructions: Vec<String> = file.split("\n").map(|s| s.to_string()).collect();
    let mut instructions: Vec<(usize, &String)> = instructions.iter().enumerate().collect();
    let mut line = 0;
    let mut labels: HashMap<String, usize> = HashMap::new();
    instructions.retain(|(_, s)| {
        let mut blank = s.trim().len() == 0;
        if s.trim().starts_with("#") {
            blank = true;
        }
        let is_label = s.trim().starts_with(":");
        if !blank {
            if is_label {
                labels.insert(s.to_string()[1..].trim().to_string(), line);
            } else {
                line += 1;
            }
        }
        !blank && !is_label
    });

    let mut hex_code: Vec<i16> = Vec::new();

    for (index, instruction) in instructions {
        let line = index + 1;
        let words: Vec<&str> = instruction.split(" ").map(|s| s).collect();
        if words.first().is_none() {
            continue;
        }
        let hex: i16;
        match words.first().unwrap().to_uppercase().trim() {
            "NOP" => {
                hex = 0;
            }
            "LOAD" => {
                hex = (1 << 12) | (get_reg(&words, 1, line) << 9) | (get_reg(&words, 2, line) << 3);
            }
            "STORE" => {
                hex = (2 << 12) | (get_reg(&words, 1, line) << 6) | (get_reg(&words, 2, line) << 3);
            }
            "ADD" => {
                hex = (3 << 12) | (get_reg(&words, 1, line) << 9) | (get_reg(&words, 2, line) << 6) | (get_reg(&words, 3, line) << 3);
            }
            "ADDI" => {
                hex = (4 << 12) | (get_reg(&words, 1, line) << 9) | (get_immediate(&words, 2, line) << 0);
            }
            "SUB" => {
                hex = (5 << 12) | (get_reg(&words, 1, line) << 9) | (get_reg(&words, 2, line) << 6) | (get_reg(&words, 3, line) << 3);
            }
            "AND" => {
                hex = (6 << 12) | (get_reg(&words, 1, line) << 9) | (get_reg(&words, 2, line) << 6) | (get_reg(&words, 3, line) << 3);
            }
            "XOR" => {
                hex = (7 << 12) | (get_reg(&words, 1, line) << 9) | (get_reg(&words, 2, line) << 6) | (get_reg(&words, 3, line) << 3);
            }
            "J" => {
                if words.len() == 2 {
                    hex = (8 << 12) | (get_reg(&words, 1, line) << 9);
                } else {
                    hex = (8 << 12) | (get_reg(&words, 1, line) << 9) | (get_reg(&words, 2, line) << 6) | (get_reg(&words, 4, line) << 3) | (get_flag(&words, 3, line) << 1);
                }
            }
            "JAL" => {
                if words.len() == 2 {
                    hex = (9 << 12) | (get_reg(&words, 1, line) << 9);
                } else {
                    hex = (9 << 12) | (get_reg(&words, 1, line) << 9) | (get_reg(&words, 2, line) << 6) | (get_reg(&words, 4, line) << 3) | (get_flag(&words, 3, line) << 1);
                }
            }
            "SSP" => {
                hex = (10 << 12) | (get_reg(&words, 1, line) << 6);
            }
            "SET" => {
                hex = (11 << 12) | (get_reg(&words, 1, line) << 9) | get_imm_or_label(&words, 2, line, &labels);
            }
            "RET" => {
                hex = 12 << 12
            }
            "SFT" => {
                hex = (13 << 12) | (get_reg(&words, 1, line) << 9) | (get_reg(&words, 2, line) << 6) | (get_reg(&words, 4, line) << 3) | (get_sft_op(&words, 3, line) << 1);
            }
            "IN" => {
                hex = (14 << 12) | (get_reg(&words, 1, line) << 9) | (get_immediate(&words, 2, line) << 6);
            }
            "OUT" => {
                hex = (14 << 12) | (get_reg(&words, 1, line) << 9) | (get_immediate(&words, 2, line) << 6) | 1 << 5;
            }
            "HALT" => {
                hex = 15 << 12;
            }
            operation => {
                unknown_instruction_error(operation, line);
            }
        }
        hex_code.push(hex);
    }

    let mut clipboard = String::new();

    for h in hex_code {
        print!("0b");
        clipboard.push_str(&*"0b".to_string());
        for bit in (0..16).rev().map(|n| (h >> n) & 1) {
            print!("{}", bit);
            clipboard.push_str(&*format!("{}", bit).to_string());
        };
        println!();
        clipboard.push_str("\n");
    }
    if ClipboardContext::new().unwrap().set_contents(clipboard).is_err() {
        error!("Unable to save to clipboard.")
    }

    match env::var("SCHEM") {
        Ok(file) => {

            let offset_x = match env::var("OFFSET_X") {
                Ok(x) => x,
                Err(_) => {
                    error!("No x offset (OFFSET_X) for schematic specified.");
                    exit(1);
                }
            };
            let offset_y = match env::var("OFFSET_Y") {
                Ok(y) => y,
                Err(_) => {
                    error!("No y offset (OFFSET_Y) for schematic specified.");
                    exit(1);
                }
            };

            let mut schem = Schematic::new();

            let block = Block {
                id: "minecraft:barrel".to_string(),
                namespace: "".to_string(),
                attributes: BTreeMap::new()
            };

            let mut region = Region::with_shape([5,5,5]);
            let _ = region.set_block([0, 1, 0], &block);

            schem.regions.push(region);

            let _ = schem.save_world_edit_13_file(&file, &WorldEdit13SaveOption::default());
            

        },
        Err(_) => ()
    }
    
}

fn get_imm_or_label(words: &Vec<&str>, argument: usize, line: usize, labels: &HashMap<String, usize>) -> i16 {
    let arg = get_arg(words, argument, line);
    let is_number = match arg.parse::<i32>() {
        Ok(_) => true,
        Err(_) => false
    };
    return if is_number {
        get_immediate(words, argument, line)
    } else {
        get_label(words, argument, line, &labels)
    }
}

fn get_label(words: &Vec<&str>, argument: usize, line: usize, labels: &HashMap<String, usize>) -> i16 {
    let arg = get_arg(words, argument, line);
    let address = match labels.get(&arg) {
        Some(address) => *address,
        None => {
            error!("Undefined label: {} in line {}", arg, line);
            exit(1);
        }
    };
    if address > 127 {
        error!("Label out of range: {} in line {}. Must be 0 <= imm <= 127", arg, line);
        exit(1);
    }
    return address as i16;
}

fn get_sft_op(words: &Vec<&str>, argument: usize, line: usize) -> i16 {
    let arg = get_arg(words, argument, line);
    match arg.as_str() {
        "<<" => return 0,
        "<<<" => return 0,
        ">>" => return 2,
        ">>>" => return 1,
        flag => {
            unknown_shift_operation_error(&flag.to_string(), line);
        }
    }
}

fn get_flag(words: &Vec<&str>, argument: usize, line: usize) -> i16 {
    let arg = get_arg(words, argument, line);
    match arg.as_str() {
        "<" => return 1,
        "=" => return 2,
        ">" => return 3,
        flag => {
            unknown_flag_error(&flag.to_string(), line);
        }
    }
}

fn get_immediate(words: &Vec<&str>, argument: usize, line: usize) -> i16 {
    let arg = get_arg(words, argument, line);
    let num = match arg.to_string().parse::<i32>() {
        Ok(n) => n,
        Err(_) => unknown_argument_error(&arg, line)
    };
    if num > 127 || num < -128 {
        error!("Immediate out of range: {} in line {}. Must be -128 <= imm <= 127", arg, line);
        exit(1);
    }
    let num = num & 255;
    return num as i16;
}

fn get_reg(words: &Vec<&str>, argument: usize, line: usize) -> i16 {
    let arg = get_arg(words, argument, line);
    if arg.len() != 2 {
        unknown_argument_error(&arg, line);
    }
    let num = match String::from_utf8(vec![arg.as_bytes()[1]]).unwrap().parse::<i16>() {
        Ok(n) => n,
        Err(_) => unknown_argument_error(&arg, line)
    };
    if num > 7 {
        error!("Register index too large: {} in line {}. A maximum of 7 is allowed.", arg, line);
        exit(1);
    }
    return num;
}

fn get_arg(words: &Vec<&str>, argument: usize, line: usize) -> String {
    let arg = match words.get(argument) {
        Some(arg) => arg.trim(),
        None => {
            missing_argument_error(argument, line);
        }
    };
    return arg.to_string();
}

fn unknown_shift_operation_error(flag: &String, line: usize) -> ! {
    error!("Unknown shift operation: {} in line {}", flag, line);
    exit(1);
}

fn unknown_flag_error(flag: &String, line: usize) -> ! {
    error!("Unknown flag: {} in line {}", flag, line);
    exit(1);
}

fn unknown_argument_error(argument: &String, line: usize) -> ! {
    error!("Unknown argument: {} in line {}", argument, line);
    exit(1);
}

fn unknown_instruction_error(instruction: &str, line: usize) -> ! {
    error!("Unknown instruction: {} in line {}", instruction, line);
    exit(1);
}

fn missing_argument_error(argument_index: usize, line: usize) -> ! {
    error!("Missing argument: argument {} in line {}", argument_index, line);
    exit(1);
}