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

type Locks = Vec<Vec<usize>>;

type Keys = Vec<Vec<usize>>;

type Schemes = Vec<Vec<Vec<char>>>;

fn parse_input(filename: PathBuf) -> Schemes {
    let mut schemes = Vec::new();
    let mut scheme = Vec::new();
    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(line) = line {
                if line.is_empty() {
                    schemes.push(scheme);
                    scheme = Vec::new();
                    continue;
                }
                let inscription = line.chars().collect::<Vec<char>>();
                scheme.push(inscription);
            }
        }
    }
    schemes
}

fn parse_schemes_to_locks_and_keys(schemes: Vec<Vec<Vec<char>>>) -> (Locks, Keys) {
    let mut locks_scheme = Vec::new();
    let mut keys_scheme = Vec::new();
    for scheme in schemes {
        if scheme[0].iter().all(|c| *c == '#') {
            let lock = scheme[1..].to_vec();
            locks_scheme.push(lock);
        }
        if scheme[0].iter().all(|c| *c == '.') {
            let key = scheme[..scheme.len() - 1].to_vec();
            keys_scheme.push(key);
        }
    }

    (
        build_locks_and_keys(locks_scheme),
        build_locks_and_keys(keys_scheme),
    )
}

fn build_locks_and_keys(schemes: Vec<Vec<Vec<char>>>) -> Vec<Vec<usize>> {
    let mut kl = Vec::new();
    for scheme in schemes {
        let mut sch = Vec::new();
        for x in 0..scheme[0].len() {
            let mut col = Vec::new();
            for y in 0..scheme.len() {
                col.push(scheme[y][x]);
            }
            let count = col.iter().filter(|c| **c == '#').count();
            sch.push(count);
        }
        kl.push(sch);
    }
    kl
}

fn puzzle_1(data: Schemes) -> u64 {
    let (locks, keys) = parse_schemes_to_locks_and_keys(data.clone());
    let mut count: u64 = 0;
    for lock in locks {
        for key in &keys {
            if lock.iter().zip(key).all(|(a, b)| a + b <= 5) {
                count += 1;
            }
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use crate::{get_file_path, parse_input, puzzle_1};

    #[test]
    fn it_returns_3() {
        let file_path = get_file_path("test-input.txt".to_string());
        let data = parse_input(file_path);
        let result = puzzle_1(data);
        assert_eq!(result, 3);
    }

    #[test]
    fn it_returns_puzzle1_score() {
        let file_path = get_file_path("input.txt".to_string());
        let data = parse_input(file_path);
        let result = puzzle_1(data);
        assert_eq!(result, 3255);
    }
}
