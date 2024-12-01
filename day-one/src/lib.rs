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

fn parse_input_to_vectors(filename: PathBuf) -> (Vec<i32>, Vec<i32>) {
    let mut left_vec: Vec<i32> = Vec::new();
    let mut right_vec: Vec<i32> = Vec::new();
    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(line) = line {
                let parsed = line.split("   ").collect::<Vec<&str>>();
                let num_1: i32 = parsed[0].parse().expect("Cannot parse input");
                let num_2: i32 = parsed[1].parse().expect("Cannot parse input");
                left_vec.push(num_1);
                right_vec.push(num_2);
            }
        }
    }
    (left_vec, right_vec)
}

fn puzzle_1(filename: PathBuf) -> i32 {
    let mut result = 0;
    let (mut left_vec, mut right_vec) = parse_input_to_vectors(filename);
    left_vec.sort();
    right_vec.sort();
    for i in 0..left_vec.len() {
        let delta = i32::abs(left_vec[i] - right_vec[i]);
        result += delta;
    }
    result
}

fn puzzle_2(filename: PathBuf) -> i32 {
    let mut result = 0;
    let (left_vec, right_vec) = parse_input_to_vectors(filename);
    let mut count_hashmap = HashMap::new();
    for value in right_vec {
        count_hashmap
            .entry(value)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }
    for l_value in left_vec {
        let right_value = count_hashmap.get(&l_value);
        if let Some(r_value) = right_value {
            let sim_score = r_value * l_value;
            result += sim_score;
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use crate::{get_file_path, puzzle_1, puzzle_2};

    #[test]
    fn it_returns_11() {
        let file_path = get_file_path("test-input.txt".to_string());
        let result = puzzle_1(file_path);
        assert_eq!(result, 11);
    }

    #[test]
    fn it_returns_puzzle_1_score() {
        let file_path = get_file_path("input.txt".to_string());
        let result = puzzle_1(file_path);
        assert_eq!(result, 2285373);
    }

    #[test]
    fn it_returns_31() {
        let file_path = get_file_path("test-input.txt".to_string());
        let result = puzzle_2(file_path);
        assert_eq!(result, 31);
    }

    #[test]
    fn it_returns_puzzle_2_score() {
        let file_path = get_file_path("input.txt".to_string());
        let result = puzzle_2(file_path);
        assert_eq!(result, 21142653);
    }
}
