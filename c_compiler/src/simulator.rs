// OP Codes
const NOP  : u16 = 0x0 << 12;
const LOAD : u16 = 0x1 << 12;
const STORE: u16 = 0x2 << 12;
const ADD  : u16 = 0x3 << 12;
const ADDI : u16 = 0x4 << 12;
const SUB  : u16 = 0x5 << 12;
const AND  : u16 = 0x6 << 12;
const XOR  : u16 = 0x7 << 12;
const J    : u16 = 0x8 << 12;
const JAL  : u16 = 0x9 << 12;
const SSP  : u16 = 0xA << 12;
const SET  : u16 = 0xB << 12;
const RET  : u16 = 0xC << 12;
const SFT  : u16 = 0xD << 12;
const IO   : u16 = 0xE << 12;
const HALT : u16 = 0xF << 12;

// Memory sizes
const ROM_SIZE: usize = 1 << 16;
const MEMORY_SIZE: usize = 1 << 16;
const REG_COUNT: usize = 8;
const IO_DEVICES: usize = 8;

// Instruction parts
const TARGET_MSK: u16 = 0b1110_0000_0000;
const ADDR_MSK  : u16 = 0b0000_0011_1000;
const REG_A_MSK : u16 = 0b0001_1100_0000;
const REG_B_MSK : u16 = 0b0000_0011_1000;
const FLAG_MSK  : u16 = 0b0000_0000_0110;
const IMM_MSK   : u16 = 0b0000_1111_1111;
const STEPS_MSK : u16 = 0b0000_0011_1000;
const SFT_OP_MSK: u16 = 0b0000_0000_0110;
const DATA_MSK  : u16 = 0b1110_0000_0000;
const DEV_MSK   : u16 = 0b0001_1100_0000;
const R_W_MSK   : u16 = 0b0000_0010_0000;

const TARGET_POS: u8 = 9;
const ADDR_POS  : u8 = 3;
const REG_A_POS : u8 = 6;
const REG_B_POS : u8 = 3;
const FLAG_POS  : u8 = 1;
const IMM_POS   : u8 = 0;
const STEPS_POS : u8 = 3;
const SFT_OP_POS: u8 = 1;
const DATA_POS  : u8 = 9;
const DEV_POS   : u8 = 6;
const R_W_POS   : u8 = 5;

pub fn simulate(hex_code: &Vec<u16>) {

    let mut rom: [u16; ROM_SIZE] = [0; ROM_SIZE];
    let mut memory: [i16; MEMORY_SIZE] = [0; MEMORY_SIZE];
    let mut reg: [i16; REG_COUNT] = [0; REG_COUNT];
    let mut io: [i16; IO_DEVICES] = [0; IO_DEVICES];

    let mut pc: usize = 0;
    let mut sp: usize = 0;

    for (i, line) in hex_code.iter().enumerate() {
        rom[i] = *line;
    }
    let rom = rom;

    loop {
        let instr = rom[pc];
        match instr & 0xF000 {
            NOP => (),
            LOAD => {
                reg[((instr & TARGET_MSK) >> TARGET_POS) as usize] = memory[((instr & ADDR_MSK) >> ADDR_POS) as usize];
            },
            STORE => {
                memory[((instr & ADDR_MSK) >> ADDR_POS) as usize] = reg[((instr & REG_A_MSK) >> REG_A_POS) as usize];
            },
            ADD => {
                let sum = reg[((instr & REG_A_MSK) >> REG_A_POS) as usize] + reg[((instr & REG_B_MSK) >> REG_B_POS) as usize];
                reg[((instr & TARGET_MSK) >> TARGET_POS) as usize] = sum;
            },
            ADDI => {
                let imm = ((instr & IMM_MSK) >> IMM_POS) as i8;
                reg[((instr & TARGET_MSK) >> TARGET_POS) as usize] += imm as i16;
            },
            SUB => {
                let dif = reg[((instr & REG_A_MSK) >> REG_A_POS) as usize] - reg[((instr & REG_B_MSK) >> REG_B_POS) as usize];
                reg[((instr & TARGET_MSK) >> TARGET_POS) as usize] = dif;
            },
            AND => {
                let and = reg[((instr & REG_A_MSK) >> REG_A_POS) as usize] & reg[((instr & REG_B_MSK) >> REG_B_POS) as usize];
                reg[((instr & TARGET_MSK) >> TARGET_POS) as usize] = and;
            },
            XOR => {
                let xor = reg[((instr & REG_A_MSK) >> REG_A_POS) as usize] ^ reg[((instr & REG_B_MSK) >> REG_B_POS) as usize];
                reg[((instr & TARGET_MSK) >> TARGET_POS) as usize] = xor;
            },
            J => {
                let flag = (instr & FLAG_MSK) >> FLAG_POS;
                if flag == 0 {
                    pc = (reg[((instr & TARGET_MSK) >> TARGET_POS) as usize] - 1) as usize;
                } else {
                    let a = reg[((instr & REG_A_MSK) >> REG_A_POS) as usize];
                    let b = reg[((instr & REG_B_MSK) >> REG_B_POS) as usize];
                    if flag == 2 && a == b {
                        pc = (reg[((instr & TARGET_MSK) >> TARGET_POS) as usize] - 1) as usize;
                    } else if flag == 1 && a < b {
                        pc = (reg[((instr & TARGET_MSK) >> TARGET_POS) as usize] - 1) as usize;
                    } else if flag == 3 && a > b {
                        pc = (reg[((instr & TARGET_MSK) >> TARGET_POS) as usize] - 1) as usize;
                    }
                }
            },
            JAL => {
                let flag = (instr & FLAG_MSK) >> FLAG_POS;
                if flag == 0 {
                    memory[sp] = pc as i16;
                    sp += 1;
                    pc = (reg[((instr & TARGET_MSK) >> TARGET_POS) as usize] - 1) as usize;
                } else {
                    let a = reg[((instr & REG_A_MSK) >> REG_A_POS) as usize];
                    let b = reg[((instr & REG_B_MSK) >> REG_B_POS) as usize];
                    if flag == 2 && a == b {
                        memory[sp] = pc as i16;
                        sp += 1;
                        pc = (reg[((instr & TARGET_MSK) >> TARGET_POS) as usize] - 1) as usize;
                    } else if flag == 1 && a < b {
                        memory[sp] = pc as i16;
                        sp += 1;
                        pc = (reg[((instr & TARGET_MSK) >> TARGET_POS) as usize] - 1) as usize;
                    } else if flag == 3 && a > b {
                        memory[sp] = pc as i16;
                        sp += 1;
                        pc = (reg[((instr & TARGET_MSK) >> TARGET_POS) as usize] - 1) as usize;
                    }
                }
            },
            SSP => {
                sp = reg[((instr & REG_A_MSK) >> REG_A_POS) as usize] as usize;
            },
            SET => {
                let imm = ((instr & IMM_MSK) >> IMM_POS) as i8;
                reg[((instr & TARGET_MSK) >> TARGET_POS) as usize] = imm as i16;
            },
            RET => {
                sp -= 1;
                pc = memory[sp] as usize;
            },
            SFT => {
                let a = reg[((instr & REG_A_MSK) >> REG_A_POS) as usize];
                let steps = reg[((instr & STEPS_MSK) >> STEPS_POS) as usize];
                match (instr & SFT_OP_MSK) >> SFT_OP_POS {
                    0 => {
                        reg[((instr & TARGET_MSK) >> TARGET_POS) as usize] = a << steps;
                    },
                    1 => {
                        reg[((instr & TARGET_MSK) >> TARGET_POS) as usize] = ((a as u16) >> steps) as i16;
                    },
                    2 => {
                        reg[((instr & TARGET_MSK) >> TARGET_POS) as usize] = a >> steps;
                    },
                    _ => panic!("Invalid Shift Op")
                }
            },
            IO => {
                if (instr & R_W_MSK) >> R_W_POS == 0 {
                    reg[((instr & DATA_MSK) >> DATA_POS) as usize] = io[((instr & DEV_MSK) >> DEV_POS) as usize];
                } else {
                    io[((instr & DEV_MSK) >> DEV_POS) as usize] = reg[((instr & DATA_MSK) >> DATA_POS) as usize];
                    println!("{}", io[((instr & DEV_MSK) >> DEV_POS) as usize]);
                }
            },
            HALT => {
                break;
            },
            _ => panic!("Invalid Op Code")
        }

        pc += 1;
    }
}
