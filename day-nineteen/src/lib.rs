use std::collections::HashMap;
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

type Towels = Vec<String>;
type Patterns = Vec<String>;

fn parse_input(filename: PathBuf) -> (Towels, Patterns) {
    let mut towels = Vec::new();
    let mut patterns = Vec::new();
    let mut parse_pattern = false;
    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(line) = line {
                if line.is_empty() {
                    parse_pattern = true;
                    continue;
                }
                if parse_pattern {
                    patterns.push(line);
                } else {
                    towels = line.split(", ").map(|s| s.to_string()).collect::<Vec<_>>();
                }
            }
        }
    }
    (towels, patterns)
}

fn count(pattern: &str, towels: &Towels, memory: &mut HashMap<String, usize>) -> usize {
    let mut sum = 0;
    if !memory.contains_key(pattern) {
        if !pattern.is_empty() {
            for towel in towels {
                if pattern.starts_with(towel) {
                    sum += count(pattern.strip_prefix(towel).unwrap(), towels, memory);
                }
            }
            memory.insert(pattern.to_string(), sum);
        } else {
            memory.insert(pattern.to_string(), 1);
        }
    }
    *memory.get(pattern).unwrap()
}

fn puzzle_1(data: (Towels, Patterns)) -> usize {
    let mut mem: HashMap<String, usize> = HashMap::new();
    let (towels, patterns) = data;
    patterns
        .iter()
        .filter(|p| count(p, &towels, &mut mem) > 0)
        .count()
}

fn puzzle_2(data: (Towels, Patterns)) -> usize {
    let mut mem: HashMap<String, usize> = HashMap::new();
    let (towels, patterns) = data;
    patterns.iter().map(|p| count(p, &towels, &mut mem)).sum()
}

#[cfg(test)]
mod tests {
    use crate::{get_file_path, parse_input, puzzle_1, puzzle_2};

    #[test]
    fn it_returns_6() {
        let file_path = get_file_path("test-input.txt".to_string());
        let data = parse_input(file_path);
        let result = puzzle_1(data);
        assert_eq!(result, 6);
    }

    #[test]
    fn it_returns_puzzle_1_score() {
        let file_path = get_file_path("input.txt".to_string());
        let data = parse_input(file_path);
        let result = puzzle_1(data);
        assert_eq!(result, 258);
    }

    #[test]
    fn it_returns_puzzle_2_score() {
        let file_path = get_file_path("input.txt".to_string());
        let data = parse_input(file_path);
        let result = puzzle_2(data);
        assert_eq!(result, 632423618484345);
    }
}
