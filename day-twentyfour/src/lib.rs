use std::collections::{HashMap, HashSet, VecDeque};
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

type Instructions = VecDeque<Vec<String>>;
type Wires = HashMap<String, String>;

fn parse_input(filename: PathBuf) -> (Instructions, Wires) {
    let mut instructions = VecDeque::new();
    let mut wires = HashMap::new();
    let mut parse_wires = false;
    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(line) = line {
                if line.is_empty() {
                    parse_wires = true;
                    continue;
                }
                if parse_wires {
                    let instruction = line
                        .split_whitespace()
                        .filter(|s| s != &"->")
                        .map(|s| s.to_string())
                        .collect::<Vec<_>>();
                    instructions.push_back(instruction);
                } else {
                    let [input, value] = line.split(": ").collect::<Vec<_>>()[..] else {
                        panic!("Failed to parse line");
                    };
                    wires.insert(input.to_string(), value.to_string());
                }
            }
        }
    }
    (instructions, wires)
}

fn puzzle_1(data: (Instructions, Wires)) -> u64 {
    let (mut instructions, mut wires) = data;

    while let Some(ins) = instructions.pop_front() {
        let [left, op, right, out] = &ins[..] else {
            panic!("Failed to parse instruction")
        };
        if wires.get(left).is_none() || wires.get(right).is_none() {
            instructions.push_back(ins);
            continue;
        }
        if let Some(left_value) = wires.get(left) {
            if let Some(right_value) = wires.get(right) {
                let out_value = match op.as_str() {
                    "AND" => {
                        left_value.parse::<i32>().unwrap() & right_value.parse::<i32>().unwrap()
                    }
                    "OR" => {
                        left_value.parse::<i32>().unwrap() | right_value.parse::<i32>().unwrap()
                    }
                    "XOR" => {
                        left_value.parse::<i32>().unwrap() ^ right_value.parse::<i32>().unwrap()
                    }
                    _ => panic!("Unknown operator"),
                };
                wires.insert(out.to_string(), out_value.to_string());
            }
        }
    }
    let mut filtered_wires = wires
        .iter()
        .filter(|p| p.0.starts_with("z"))
        .map(|x| (x.0, x.1))
        .collect::<Vec<_>>();
    filtered_wires.sort();
    let bin = filtered_wires
        .iter()
        .rev()
        .map(|x| x.1.to_string())
        .collect::<Vec<String>>()
        .join("");
    u64::from_str_radix(&bin, 2).unwrap()
}

#[cfg(test)]
mod tests {
    use crate::{get_file_path, parse_input, puzzle_1};

    #[test]
    fn it_returns_2024() {
        let file_path = get_file_path("test-input.txt".to_string());
        let data = parse_input(file_path);
        let result = puzzle_1(data);
        assert_eq!(result, 2024);
    }

    #[test]
    fn it_returns_puzzle_1_score() {
        let file_path = get_file_path("input.txt".to_string());
        let data = parse_input(file_path);
        let result = puzzle_1(data);
        assert_eq!(result, 42049478636360);
    }
}
