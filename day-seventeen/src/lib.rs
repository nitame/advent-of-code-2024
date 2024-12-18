use std::fs::File;
use std::io::BufRead;
use std::path::{Path, PathBuf};
use std::{env, io};

fn get_file_path(filename: String) -> PathBuf {
    let current_dir = env::current_dir().expect("Attempt to get current dir");
    current_dir
        .join("assets")
        .join(Path::new(filename.as_str()))
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

struct Register {
    kind: RegisterKind,
    value: i32,
}

#[derive(Eq, PartialEq)]
enum RegisterKind {
    A,
    B,
    C,
}

type Registers = Vec<Register>;

type Program = Vec<i32>;

fn parse_input(filename: PathBuf) -> (Registers, Program) {
    let mut parse_program = false;
    let mut registers = Vec::new();
    let mut program = Vec::new();
    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(line) = line {
                if line.is_empty() {
                    parse_program = true
                }
                if parse_program {
                    let prog_seq = line.split(": ").last().unwrap();
                    program = prog_seq
                        .split(",")
                        .map(|s| {
                            s.to_string()
                                .parse::<i32>()
                                .expect("Could not parse program value")
                        })
                        .collect::<Vec<i32>>();
                } else {
                    let split = line
                        .split(": ")
                        .map(|s| s.to_string())
                        .collect::<Vec<String>>();
                    let reg_value = split[1]
                        .to_string()
                        .parse::<i32>()
                        .expect("Could not parse register value");
                    let reg = match split[0].chars().rev().take(1).collect::<String>().as_str() {
                        "A" => Register {
                            kind: RegisterKind::A,
                            value: reg_value,
                        },
                        "B" => Register {
                            kind: RegisterKind::B,
                            value: reg_value,
                        },
                        "c" => Register {
                            kind: RegisterKind::C,
                            value: reg_value,
                        },
                        _ => panic!("Unknown register name"),
                    };
                    registers.push(reg);
                }
            }
        }
    }
    (registers, program)
}

fn read_next_instruction(
    pointer: usize,
    program: Program,
    registers: &mut Registers,
    outputs: &mut Vec<i32>,
) {
    if pointer >= program.len() || pointer + 1 > program.len() {
        return;
    }
    let mut next_pointer = pointer + 2;
    let opcode = program[pointer];
    match opcode {
        0 => compute_adv(program[pointer + 1], registers),
        1 => compute_bxl(opcode, registers),
        2 => compute_bst(program[pointer + 1], registers),
        3 => compute_jnz(opcode, next_pointer, registers),
        4 => compute_bxc(registers),
        5 => compute_out(program[pointer + 1], outputs),
        6 => compute_bdv(program[pointer + 1], registers),
        7 => compute_cdv(program[pointer + 1], registers),
        _ => panic!("Unknown opcode: {}", opcode),
    }
    read_next_instruction(next_pointer, program, registers, outputs)
}

fn puzzle_1(mut data: (Registers, Program)) -> String {
    let (mut registers, program) = data;
    let mut outputs = Vec::new();
    read_next_instruction(0, program, &mut registers, &mut outputs);
    println!("{outputs:?}");
    "0".to_string()
}

fn get_combo_op_value(combo_op: i32, registers: &Registers) -> i32 {
    match combo_op {
        0..3 => combo_op,
        4 => {
            registers
                .iter()
                .find(|r| r.kind == RegisterKind::A)
                .unwrap()
                .value
        }
        5 => {
            registers
                .iter()
                .find(|r| r.kind == RegisterKind::B)
                .unwrap()
                .value
        }
        6 => {
            registers
                .iter()
                .find(|r| r.kind == RegisterKind::C)
                .unwrap()
                .value
        }
        _ => panic!("Unknown operation: {}", combo_op),
    }
}

fn compute_adv(combo_op: i32, registers: &mut Registers) {
    let n = registers
        .iter()
        .find(|r| r.kind == RegisterKind::A)
        .unwrap()
        .value;
    let power = get_combo_op_value(combo_op, &registers);
    let d = 2i32.pow(power as u32);
    let res = n as u32 / d as u32;
    match registers.iter_mut().find(|r| r.kind == RegisterKind::A) {
        Some(reg) => {
            reg.value = res as i32;
        }
        None => {}
    }
}
fn compute_bxl(literal_op: i32, registers: &mut Registers) {
    let b = registers
        .iter()
        .find(|r| r.kind == RegisterKind::B)
        .unwrap()
        .value;
    let res = b ^ literal_op;
    match registers.iter_mut().find(|r| r.kind == RegisterKind::B) {
        Some(reg) => {
            reg.value = res as i32;
        }
        None => {}
    }
}
fn compute_bst(combo_op: i32, registers: &mut Registers) {
    let rest = combo_op % 8;
    let bits = format!("{:b}", rest);
    let mut c_bits = bits.chars().collect::<Vec<char>>();
    c_bits.sort();
    let res = c_bits
        .iter()
        .take(3)
        .map(|c| c.to_digit(10).unwrap() as i32)
        .sum::<i32>();
    match registers.iter_mut().find(|r| r.kind == RegisterKind::B) {
        Some(reg) => {
            reg.value = res;
        }
        None => {}
    }
}
fn compute_jnz(literal_op: i32, mut next_pointer: usize, registers: &mut Registers) {
    let reg_a = registers
        .iter()
        .find(|r| r.kind == RegisterKind::A)
        .unwrap()
        .value;
    if reg_a != 0 {
        next_pointer = literal_op as usize;
    }
}
fn compute_bxc(registers: &mut Registers) {
    let b = registers
        .iter()
        .find(|r| r.kind == RegisterKind::B)
        .unwrap()
        .value;
    let c = registers
        .iter()
        .find(|r| r.kind == RegisterKind::C)
        .unwrap()
        .value;
    let res = b ^ c;
    match registers.iter_mut().find(|r| r.kind == RegisterKind::B) {
        Some(reg) => {
            reg.value = res as i32;
        }
        None => {}
    }
}
fn compute_out(combo_op: i32, outputs: &mut Vec<i32>) {
    outputs.push(combo_op % 8);
}
fn compute_bdv(combo_op: i32, registers: &mut Registers) {
    let n = registers
        .iter()
        .find(|r| r.kind == RegisterKind::A)
        .unwrap()
        .value;
    let power = get_combo_op_value(combo_op, &registers);
    let d = 2i32.pow(power as u32);
    let res = n as u32 / d as u32;
    match registers.iter_mut().find(|r| r.kind == RegisterKind::B) {
        Some(reg) => {
            reg.value = res as i32;
        }
        None => {}
    }
}
fn compute_cdv(combo_op: i32, registers: &mut Registers) {
    let n = registers
        .iter()
        .find(|r| r.kind == RegisterKind::A)
        .unwrap()
        .value;
    let power = get_combo_op_value(combo_op, &registers);
    let d = 2i32.pow(power as u32);
    let res = n as u32 / d as u32;
    match registers.iter_mut().find(|r| r.kind == RegisterKind::C) {
        Some(reg) => {
            reg.value = res as i32;
        }
        None => {}
    }
}

#[cfg(test)]
mod tests {
    use crate::{get_file_path, parse_input, puzzle_1};

    #[test]
    fn it_returns_4635635210() {
        let file_path = get_file_path("test-input.txt".to_string());
        let parsed = parse_input(file_path);
        let result = puzzle_1(parsed);
        assert_eq!(result, "4635635210".to_string());
    }
}
