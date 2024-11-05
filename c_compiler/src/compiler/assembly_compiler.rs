use std::collections::HashMap;

pub fn compile_assembly(raw_assembly: &str) -> Vec<i16> {
    let mut instructions: Vec<_> = raw_assembly.split("\n").enumerate().collect();
    let mut line = 0;
    let mut labels: HashMap<String, usize> = HashMap::new();

    instructions.retain(|(_, s)| {
        let trimmed = s.trim();

        if trimmed.len() == 0 || trimmed.starts_with("#") {
            return false;
        }

        if trimmed.starts_with(":") {
            labels.insert(s[1..].trim().to_string(), line);
            return false;
        }
        line += 1;
        true
    });

    let mut hex_code: Vec<i16> = Vec::new();

    for (index, instruction) in instructions {
        let line = index + 1;
        let words: Vec<&str> = instruction.split_ascii_whitespace().collect();

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
                hex = (3 << 12)
                    | (get_reg(&words, 1, line) << 9)
                    | (get_reg(&words, 2, line) << 6)
                    | (get_reg(&words, 3, line) << 3);
            }
            "ADDI" => {
                hex = (4 << 12)
                    | (get_reg(&words, 1, line) << 9)
                    | (get_immediate(&words, 2, line) << 0);
            }
            "SUB" => {
                hex = (5 << 12)
                    | (get_reg(&words, 1, line) << 9)
                    | (get_reg(&words, 2, line) << 6)
                    | (get_reg(&words, 3, line) << 3);
            }
            "AND" => {
                hex = (6 << 12)
                    | (get_reg(&words, 1, line) << 9)
                    | (get_reg(&words, 2, line) << 6)
                    | (get_reg(&words, 3, line) << 3);
            }
            "XOR" => {
                hex = (7 << 12)
                    | (get_reg(&words, 1, line) << 9)
                    | (get_reg(&words, 2, line) << 6)
                    | (get_reg(&words, 3, line) << 3);
            }
            "J" => {
                if words.len() == 2 {
                    hex = (8 << 12) | (get_reg(&words, 1, line) << 9);
                } else {
                    hex = (8 << 12)
                        | (get_reg(&words, 1, line) << 9)
                        | (get_reg(&words, 2, line) << 6)
                        | (get_reg(&words, 4, line) << 3)
                        | (get_flag(&words, 3, line) << 1);
                }
            }
            "JAL" => {
                if words.len() == 2 {
                    hex = (9 << 12) | (get_reg(&words, 1, line) << 9);
                } else {
                    hex = (9 << 12)
                        | (get_reg(&words, 1, line) << 9)
                        | (get_reg(&words, 2, line) << 6)
                        | (get_reg(&words, 4, line) << 3)
                        | (get_flag(&words, 3, line) << 1);
                }
            }
            "SSP" => {
                hex = (10 << 12) | (get_reg(&words, 1, line) << 6);
            }
            "SET" => {
                hex = (11 << 12)
                    | (get_reg(&words, 1, line) << 9)
                    | get_imm_or_label(&words, 2, line, &labels);
            }
            "RET" => hex = 12 << 12,
            "SFT" => {
                hex = (13 << 12)
                    | (get_reg(&words, 1, line) << 9)
                    | (get_reg(&words, 2, line) << 6)
                    | (get_reg(&words, 4, line) << 3)
                    | (get_sft_op(&words, 3, line) << 1);
            }
            "IN" => {
                hex = (14 << 12)
                    | (get_reg(&words, 1, line) << 9)
                    | (get_immediate(&words, 2, line) << 6);
            }
            "OUT" => {
                hex = (14 << 12)
                    | (get_reg(&words, 1, line) << 9)
                    | (get_immediate(&words, 2, line) << 6)
                    | 1 << 5;
            }
            "HALT" => {
                hex = 15 << 12;
            }
            operation => {
                panic!("{}", unknown_instruction_error(operation, line));
            }
        }
        hex_code.push(hex);
    }

    return hex_code;
}

fn get_imm_or_label(
    words: &Vec<&str>,
    argument: usize,
    line: usize,
    labels: &HashMap<String, usize>,
) -> i16 {
    return if get_arg(words, argument, line).parse::<i32>().is_ok() {
        get_immediate(words, argument, line)
    } else {
        get_label(words, argument, line, labels)
    };
}

fn get_label(
    words: &Vec<&str>,
    argument: usize,
    line: usize,
    labels: &HashMap<String, usize>,
) -> i16 {
    let arg = get_arg(words, argument, line);
    let address = *labels
        .get(&arg)
        .expect(format!("Undefined label: {} in line {}", arg, line).as_str());
    assert!(
        address <= 127,
        "{}",
        format!(
            "Label out of range: {} in line {}. Must be 0 <= imm <= 127",
            arg, line
        )
    );

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
            panic!(
                "{}",
                format!("Unknown shift operation: {} in line {}", flag, line)
            );
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
            panic!("{}", format!("Unknown flag: {} in line {}", flag, line));
        }
    }
}

fn get_immediate(words: &Vec<&str>, argument: usize, line: usize) -> i16 {
    let arg = get_arg(words, argument, line);
    let num = arg
        .parse::<i32>()
        .expect(unknown_argument_error(&arg, line).as_str());

    assert!(
        num >= -128 || num <= 127,
        "{}",
        format!(
            "Immediate out of range: {} in line {}. Must be -128 <= imm <= 127",
            arg, line
        )
    );

    return (num & 255) as i16;
}

fn get_reg(words: &Vec<&str>, argument: usize, line: usize) -> i16 {
    let arg = get_arg(words, argument, line);
    assert!(arg.len() == 2, "{}", unknown_argument_error(&arg, line));

    let num = String::from_utf8(vec![arg.as_bytes()[1]])
        .unwrap()
        .parse::<i16>()
        .expect(unknown_argument_error(&arg, line).as_str());
    assert!(
        num < 8,
        "{}",
        format!(
            "Register index too large: {} in line {}. A maximum of 7 is allowed",
            arg, line
        )
    );
    num
}

fn get_arg(words: &Vec<&str>, argument: usize, line: usize) -> String {
    let arg = words
        .get(argument)
        .expect(&missing_argument_error(argument, line));
    arg.trim().to_string()
}

fn unknown_argument_error(argument: &str, line: usize) -> String {
    format!("Unknown argument: {} in line {}", argument, line)
}

fn unknown_instruction_error(instruction: &str, line: usize) -> String {
    format!("Unknown instruction: {} in line {}", instruction, line)
}

fn missing_argument_error(argument_index: usize, line: usize) -> String {
    format!(
        "Missing argument: argument {} in line {}",
        argument_index, line
    )
}
