use crate::instructions::Instruction;
use std::collections::HashMap;

fn tokenize(line: &str) -> Vec<String> {
    line.split([' ', ',', '\t'])
        .filter(|s| !s.is_empty())
        .map(|s| s.to_lowercase())
        .collect()
}

// Returns corrensponding u8 value of the register
fn parse_reg(s: &str) -> u8 {
    match s {
        "a" => 0,
        "b" => 1,
        "c" => 2,
        "d" => 3,
        _ => panic!("Unknown register {}", s),
    }
}

fn is_reg(s: &str) -> bool {
    matches!(s, "a" | "b" | "c" | "d")
}

fn instr_size(tokens: &[String]) -> u16 {
    match tokens[0].as_str() {
        "mov" | "add" | "sub" | "jmp" | "jz" | "jnz" | "cmp" | "mul" | "div" | "call" => 3,
        "hlt" | "ret" => 1,
        _ => panic!("Unknown instruction {}", tokens[0]),
    }
}

fn first_pass(source: &str) -> HashMap<String, u16> {
    let mut symbols = HashMap::new();
    let mut addr: u16 = 0;

    for line in source.lines() {
        let line = line.trim();

        // Ignoring comments and empty lines
        if line.is_empty() || line.starts_with(";") {
            continue;
        }

        // Labels (ends with ":")
        if line.ends_with(":") {
            let label = line.trim_end_matches(":").to_string();
            symbols.insert(label, addr);
            continue;
        }

        let tokens = tokenize(line);
        addr += instr_size(&tokens);
    }

    symbols
}

pub fn assembler(source: &str) -> Vec<u8> {
    let symbols = first_pass(source);
    let mut bytes = Vec::new();

    for (line_no, line) in source.lines().enumerate() {
        let line = line.trim();

        // Comments in assembly start with ";"
        if line.is_empty() || line.starts_with(';') || line.ends_with(":") {
            continue;
        }

        let tokens = tokenize(line);

        match tokens[0].as_str() {
            "mov" => {
                // mov reg, imm
                let r1= parse_reg(&tokens[1]);
                if is_reg(&tokens[2]) {
                    let r2 = parse_reg(&tokens[2]);
                    bytes.push(Instruction::MOV_RR as u8);
                    bytes.push(r1);
                    bytes.push(r2);
                } else {
                    let imm: u8 = tokens[2].parse().unwrap();
                    bytes.push(Instruction::MOV_RI as u8);
                    bytes.push(r1);
                    bytes.push(imm);
                }
            }

            "add" => {
                // add a, b
                let r1 = parse_reg(&tokens[1]);

                if is_reg(&tokens[2]) {
                    let r2 = parse_reg(&tokens[2]);
                    bytes.push(Instruction::ADD_RR as u8);
                    bytes.push(r1);
                    bytes.push(r2);
                } else {
                    let imm: u8 = tokens[2].parse().unwrap();
                    bytes.push(Instruction::ADD_RI as u8);
                    bytes.push(r1);
                    bytes.push(imm);
                }

            }

            "sub" => {
                // sub a, b
                let r1 = parse_reg(&tokens[1]);
                if is_reg(&tokens[2]) {
                    let r2 = parse_reg(&tokens[2]);
                    bytes.push(Instruction::SUB_RR as u8);
                    bytes.push(r1);
                    bytes.push(r2);
                } else {
                    let imm: u8 = tokens[2].parse().unwrap();
                    bytes.push(Instruction::SUB_RI as u8);
                    bytes.push(r1);
                    bytes.push(imm);
                }
            }

            "jmp" | "jz" | "jnz" => {
                let opcode = match tokens[0].as_str() {
                    "jmp" => Instruction::JMP,
                    "jz" => Instruction::JZ,
                    "jnz" => Instruction::JNZ,
                    _ => unreachable!(),
                };

                let label = &tokens[1];
                let addr = *symbols.get(label).expect("Uknown label");

                bytes.push(opcode as u8);
                bytes.push((addr & 0xFF) as u8); // low
                bytes.push((addr >> 8) as u8); // high
            }

            "cmp" => {
                let r1 = parse_reg(&tokens[1]);
                if tokens[2].chars().all(|c| c.is_ascii_digit()) {
                    let imm: u8 = tokens[2].parse().unwrap();
                    bytes.push(Instruction::CMP_RI as u8);
                    bytes.push(r1);
                    bytes.push(imm);
                } else {
                    let r2 = parse_reg(&tokens[2]);
                    bytes.push(Instruction::CMP_RR as u8);
                    bytes.push(r1);
                    bytes.push(r2);
                }
            }

            "mul" => {
                let r1 = parse_reg(&tokens[1]);
                let r2 = parse_reg(&tokens[2]);

                bytes.push(Instruction::MUL as u8);
                bytes.push(r1);
                bytes.push(r2);

            }

            "div" => {
                let r1 = parse_reg(&tokens[1]);
                let r2 = parse_reg(&tokens[2]);

                bytes.push(Instruction::DIV as u8);
                bytes.push(r1);
                bytes.push(r2);
            }

            "call" => {
                let addr = *symbols
                    .get(&tokens[1])
                    .expect("Unknown label");

                bytes.push(Instruction::CALL as u8);
                bytes.push((addr & 0xFF) as u8);   // low
                bytes.push((addr >> 8) as u8);     // high
            }

            "ret" => {
                bytes.push(Instruction::RET as u8);
            }

            "hlt" => {
                bytes.push(Instruction::HLT as u8);
            }

            _ => panic!("Line {}: unknown instruction", line_no + 1),
        }
    }

    bytes
}
