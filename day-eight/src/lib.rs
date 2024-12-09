use std::collections::HashSet;
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

#[derive(Debug)]
struct AntennaMap {
    map: Vec<Vec<char>>,
    antenna_frequencies: HashSet<char>,
}

fn parse_input_to_vectors(filename: PathBuf) -> AntennaMap {
    let mut map = Vec::new();
    let mut antenna_frequencies = HashSet::new();
    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(line) = line {
                let c = line
                    .chars()
                    .map(|x| {
                        if x != '.' {
                            antenna_frequencies.insert(x);
                        }
                        x
                    })
                    .collect::<Vec<char>>();
                map.push(c);
            }
        }
    }
    AntennaMap {
        map,
        antenna_frequencies,
    }
}

fn puzzle_1(filename: PathBuf) -> i32 {
    let mut result = 0;
    let mut antenna_map = parse_input_to_vectors(filename);
    println!("{:?}", antenna_map);
    result
}

#[cfg(test)]
mod tests {
    use crate::{get_file_path, puzzle_1};

    #[test]
    fn it_returns_14() {
        let file_path = get_file_path("test-input.txt".to_string());
        let result = puzzle_1(file_path);
        assert_eq!(result, 14);
    }

    #[test]
    fn it_returns_puzzle_1_score() {
        let file_path = get_file_path("input.txt".to_string());
        let result = puzzle_1(file_path);
        assert_eq!(result, 0);
    }
}
