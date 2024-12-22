use std::collections::{HashMap, HashSet};
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

fn parse_input(filename: PathBuf) -> Vec<u64> {
    let mut secret_numbers = Vec::new();
    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(line) = line {
                secret_numbers.push(line.parse::<u64>().unwrap());
            }
        }
    }
    secret_numbers
}

fn first_step(secret_number: u64) -> u64 {
    let mul = secret_number * 64;
    let mixed = mix(mul, secret_number);
    prune(mixed)
}

fn second_step(secret_number: u64) -> u64 {
    let div = secret_number / 32;
    let mixed = mix(div, secret_number);
    prune(mixed)
}

fn third_step(secret_number: u64) -> u64 {
    let mul = secret_number * 2048;
    let mixed = mix(mul, secret_number);
    prune(mixed)
}

fn mix(left_op: u64, right_op: u64) -> u64 {
    left_op ^ right_op
}

fn prune(value: u64) -> u64 {
    value % 16777216
}

fn puzzle_1(data: Vec<u64>) -> u64 {
    let mut secrets_nth = Vec::new();
    for secret_number in data {
        let nth_number = (1..=2000).fold(secret_number, |mut acc, _| {
            acc = first_step(acc);
            acc = second_step(acc);
            acc = third_step(acc);
            acc
        });
        secrets_nth.push(nth_number);
    }
    secrets_nth.iter().sum()
}

#[cfg(test)]
mod tests {
    use crate::{get_file_path, parse_input, puzzle_1};

    #[test]
    fn it_returns_37327623() {
        let file_path = get_file_path("test-input.txt".to_string());
        let data = parse_input(file_path);
        let result = puzzle_1(data);
        assert_eq!(result, 37327623);
    }

    #[test]
    fn it_returns_puzzle_1_score() {
        let file_path = get_file_path("input.txt".to_string());
        let data = parse_input(file_path);
        let result = puzzle_1(data);
        assert_eq!(result, 17724064040);
    }
}
