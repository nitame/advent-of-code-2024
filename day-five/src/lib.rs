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

fn parse_input_to_vectors(head: PathBuf, body: PathBuf) -> (Vec<(i32, i32)>, Vec<Vec<i32>>) {
    let mut rules: Vec<(i32, i32)> = Vec::new();
    let mut printing: Vec<Vec<i32>> = Vec::new();
    if let Ok(lines) = read_lines(head) {
        for line in lines {
            if let Ok(line) = line {
                let parsed = line
                    .split("|")
                    .map(|x| x.parse::<i32>().unwrap())
                    .collect::<Vec<i32>>();
                rules.push((parsed[0], parsed[1]));
            }
        }
    }
    if let Ok(lines) = read_lines(body) {
        for line in lines {
            if let Ok(line) = line {
                let parsed = line
                    .split(",")
                    .map(|x| x.parse::<i32>().unwrap())
                    .collect::<Vec<i32>>();
                printing.push(parsed);
            }
        }
    }
    (rules, printing)
}

fn puzzle_1(rules: Vec<(i32, i32)>, printing_sequences: Vec<Vec<i32>>) -> i32 {
    let mut data = Vec::new();
    for printing_sequence in printing_sequences {
        let mut breaking_rules = false;
        for (idx, page) in printing_sequence.iter().enumerate() {
            let rules_to_check = rules
                .iter()
                .filter(|x| x.0 == *page)
                .map(|x| x.1)
                .collect::<Vec<i32>>();
            let sub_seq = &printing_sequence[(idx + 1)..];
            for page in sub_seq {
                if !rules_to_check.contains(&page) {
                    breaking_rules = true;
                }
            }
        }
        if !breaking_rules {
            let middle_page = printing_sequence[printing_sequence.len() / 2];
            data.push(middle_page);
        }
    }
    data.iter().sum()
}

#[cfg(test)]
mod tests {
    use crate::{get_file_path, parse_input_to_vectors, puzzle_1};

    #[test]
    fn it_returns_143() {
        let head_file_path = get_file_path("test-input-head.txt".to_string());
        let body_file_path = get_file_path("test-input-body.txt".to_string());
        let (rules, printing_sequence) = parse_input_to_vectors(head_file_path, body_file_path);
        let result = puzzle_1(rules, printing_sequence);
        assert_eq!(result, 143);
    }

    #[test]
    fn it_returns_puzzle_1_score() {
        let head_file_path = get_file_path("input-head.txt".to_string());
        let body_file_path = get_file_path("input-body.txt".to_string());
        let (rules, printing_sequence) = parse_input_to_vectors(head_file_path, body_file_path);
        let result = puzzle_1(rules, printing_sequence);
        assert_eq!(result, 4924);
    }
}
