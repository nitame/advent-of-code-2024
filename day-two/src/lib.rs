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

fn parse_input_to_vectors(filename: PathBuf) -> Vec<Vec<i32>> {
    let mut data = Vec::new();
    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(line) = line {
                let parsed = line
                    .split_whitespace()
                    .map(|x| x.parse::<i32>().unwrap())
                    .collect::<Vec<i32>>();
                data.push(parsed);
            }
        }
    }
    data
}

fn reactor_safety_rule_compute(data: &Vec<Vec<i32>>) -> i32 {
    let mut safe_count = 0;
    for row in data {
        let mut safe = true;
        let croissante = if row.first() < row.last() {
            true
        } else {
            false
        };
        for (index, datum) in row.clone().into_iter().enumerate() {
            if index < row.len() - 1 {
                let next = row[index + 1];
                let delta = next - datum;
                if i32::abs(delta) == 0 || i32::abs(delta) > 3 {
                    safe = false;
                    break;
                }
                if delta > 0 && !croissante || delta < 0 && croissante {
                    safe = false;
                    break;
                }
            }
        }
        if safe {
            safe_count += 1;
        }
    }
    safe_count
}

fn puzzle_1(filename: PathBuf) -> i32 {
    let data = parse_input_to_vectors(filename);
    reactor_safety_rule_compute(&data)
}

#[cfg(test)]
mod tests {
    use crate::{get_file_path, puzzle_1};

    #[test]
    fn it_returns_2_safe_report() {
        let file_path = get_file_path("test-input.txt".to_string());
        let result = puzzle_1(file_path);
        assert_eq!(result, 2);
    }

    #[test]
    fn it_returns_puzzle_1_score() {
        let file_path = get_file_path("input.txt".to_string());
        let result = puzzle_1(file_path);
        assert_eq!(result, 390);
    }
}
