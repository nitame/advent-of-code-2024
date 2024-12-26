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

fn is_safe(report: &Vec<i32>) -> bool {
    let order = report.first().cmp(&report.last());
    let is_ordered = report.windows(2).all(|w| w[0].cmp(&w[1]) == order);
    let is_under_threshold = report
        .windows(2)
        .all(|w| i32::abs(w[1] - w[0]) <= 3 && i32::abs(w[1] - w[0]) > 0);
    is_ordered && is_under_threshold
}

fn safety_counter(reports: Vec<Vec<i32>>) -> i32 {
    reports.into_iter().filter(|report| is_safe(report)).count() as i32
}

fn is_safe_with_tolerance(report: &Vec<i32>) -> bool {
    let mut unsafe_count = 0;
    if is_safe(report) {
        return true;
    }
    for idx in 0..report.len() {
        let rest = &report[idx + 1..];
        if !is_safe(&rest.to_vec()) {
            unsafe_count += 1;
        }
    }
    unsafe_count < 2
}

fn safety_counter_with_tolerance(reports: Vec<Vec<i32>>) -> i32 {
    reports
        .into_iter()
        .filter(|report| is_safe_with_tolerance(report))
        .count() as i32
}

fn puzzle_1(filename: PathBuf) -> i32 {
    let data = parse_input_to_vectors(filename);
    safety_counter(data)
}

fn puzzle_2(filename: PathBuf) -> i32 {
    let data = parse_input_to_vectors(filename);
    safety_counter_with_tolerance(data)
}

#[cfg(test)]
mod tests {
    use crate::{get_file_path, puzzle_1, puzzle_2};

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

    #[test]
    fn it_returns_4() {
        let file_path = get_file_path("test-input.txt".to_string());
        let result = puzzle_2(file_path);
        assert_eq!(result, 4);
    }

    #[test]
    fn it_returns_puzzle_2_score() {
        let file_path = get_file_path("input.txt".to_string());
        let result = puzzle_2(file_path);
        assert_eq!(result, 439);
    }
}
