use regex::Regex;
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

#[derive(Clone, Copy, Debug)]
struct Button {
    right_move: i32,
    forward_move: i32,
}

#[derive(Clone, Copy, Debug)]
struct Prize {
    x: i32,
    y: i32,
}

#[derive(Clone, Copy, Debug)]
struct MachineClaw {
    button_a: Button,
    button_b: Button,
    prize: Prize,
}

fn parse_input_to_machines(filename: PathBuf) -> Vec<MachineClaw> {
    let mut machines = Vec::new();
    let button_regex = Regex::new(r" X\+(?<x>[0-9]{1,2}), Y\+(?<y>[0-9]{1,2})").unwrap();
    let prize_regex = Regex::new(r"X=(?<x>[0-9]{1,5}), Y=(?<y>[0-9]{1,5})").unwrap();
    if let Ok(lines) = read_lines(filename) {
        let mut button_a = Button {
            right_move: 0,
            forward_move: 0,
        };
        let mut button_b = Button {
            right_move: 0,
            forward_move: 0,
        };
        let mut prize = Prize { x: 0, y: 0 };
        for line in lines {
            if let Ok(line) = line {
                if line.is_empty() {
                    machines.push(MachineClaw {
                        button_a,
                        button_b,
                        prize,
                    });
                } else {
                    let instruction = line.split(":").collect::<Vec<_>>();
                    match instruction[0] {
                        "Button A" => {
                            let caps = button_regex.captures(&instruction[1]).unwrap();
                            button_a.right_move =
                                caps.name("x").unwrap().as_str().parse::<i32>().unwrap();
                            button_a.forward_move =
                                caps.name("y").unwrap().as_str().parse::<i32>().unwrap();
                        }
                        "Button B" => {
                            let caps = button_regex.captures(&instruction[1]).unwrap();
                            button_b.right_move =
                                caps.name("x").unwrap().as_str().parse::<i32>().unwrap();
                            button_b.forward_move =
                                caps.name("y").unwrap().as_str().parse::<i32>().unwrap();
                        }
                        "Prize" => {
                            let caps = prize_regex.captures(&instruction[1]).unwrap();
                            prize.x = caps.name("x").unwrap().as_str().parse::<i32>().unwrap();
                            prize.y = caps.name("y").unwrap().as_str().parse::<i32>().unwrap();
                        }
                        _ => {
                            panic!("Unknown instruction {}", instruction[0])
                        }
                    }
                }
            }
        }
    }
    machines
}

fn puzzle_1(machines: Vec<MachineClaw>) -> i32 {
    let mut result = 0;
    println!("machines: {:?}", machines.len());
    for machine in machines {
        let x1 = machine.button_a.right_move;
        let x2 = machine.button_a.forward_move;
        let y1 = machine.button_b.right_move;
        let y2 = machine.button_b.forward_move;
        let z1 = machine.prize.x;
        let z2 = machine.prize.y;

        // x1 * a + y1 * b = z1
        // x2 * a + y2 * b = z2

        let b = (z2 * x1 - z1 * x2) / (y2 * x1 - y1 * x2);
        let a = (z1 - b * y1) / x1;
        if (x1 * a + y1 * b, x2 * a + y2 * b) == (z1, z2) {
            result += 3 * a + b;
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use crate::{get_file_path, parse_input_to_machines, puzzle_1};

    #[test]
    fn it_returns_480() {
        let file_path = get_file_path("test-input.txt".to_string());
        let data = parse_input_to_machines(file_path);
        let result = puzzle_1(data);
        assert_eq!(result, 480);
    }

    #[test]
    fn it_returns_puzzle_1_score() {
        let file_path = get_file_path("input.txt".to_string());
        let data = parse_input_to_machines(file_path);
        let result = puzzle_1(data);
        assert_eq!(result, 31065);
    }
}
