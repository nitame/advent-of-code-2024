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

fn puzzle_1(filename: PathBuf) -> i32 {
    let mut data = Vec::new();
    let re = Regex::new(r"mul\((?<left>\d+),(?<right>\d+)\)").unwrap();
    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(line) = line {
                let mul_op: i32 = re
                    .captures_iter(line.as_str())
                    .map(|x| {
                        let left = x.name("left").unwrap().as_str().parse::<i32>().unwrap();
                        let right = x.name("right").unwrap().as_str().parse::<i32>().unwrap();
                        left * right
                    })
                    .sum();
                data.push(mul_op);
            }
        }
    }
    data.iter().sum()
}

fn puzzle_2(filename: PathBuf) -> i32 {
    let mut sum_matches = Vec::new();
    let mut sum = 0;
    let mut process = true;
    let re = Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)").unwrap();
    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(line) = line {
                let mul_sum: i32 = re
                    .captures_iter(line.as_str())
                    .map(|caps| {
                        match &caps[0] {
                            "do()" => process = true,
                            "don't()" => process = false,
                            _ => {
                                if process {
                                    let a = caps[1].parse::<i32>().unwrap();
                                    let b = caps[2].parse::<i32>().unwrap();
                                    return a * b;
                                }
                            }
                        }
                        0
                    })
                    .sum();
                sum_matches.push(mul_sum);
            }
        }
    }
    sum_matches.iter().sum()
}

#[cfg(test)]
mod tests {
    use crate::{get_file_path, puzzle_1, puzzle_2};

    #[test]
    fn it_returns_161() {
        let file_path = get_file_path("test-input.txt".to_string());
        let result = puzzle_1(file_path);
        assert_eq!(result, 161);
    }

    #[test]
    fn it_returns_puzzle_1() {
        let file_path = get_file_path("input.txt".to_string());
        let result = puzzle_1(file_path);
        assert_eq!(result, 174336360);
    }

    #[test]
    fn it_returns_48() {
        let file_path = get_file_path("two-test-input.txt".to_string());
        let result = puzzle_2(file_path);
        assert_eq!(result, 48);
    }

    #[test]
    fn it_returns_puzzle_2_score() {
        let file_path = get_file_path("input.txt".to_string());
        let result = puzzle_2(file_path);
        assert_eq!(result, 88802350);
    }
}
