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

const NUM_PAD: [[char; 3]; 4] = [
    ['7', '8', '9'],
    ['4', '5', '6'],
    ['1', '2', '3'],
    ['_', '0', 'A'],
];

const DIR_PAD: [[char; 3]; 2] = [['_', '^', 'A'], ['<', 'v', '>']];

fn parse_input(filename: PathBuf) -> Vec<String> {
    let mut codes = Vec::new();
    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(line) = line {
                codes.push(line);
            }
        }
    }
    codes
}

fn puzzle_1(data: Vec<String>) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use crate::{get_file_path, parse_input, puzzle_1};

    #[test]
    fn it_returns_126384() {
        let file_path = get_file_path("test-input.txt".to_string());
        let data = parse_input(file_path);
        let result = puzzle_1(data);
        assert_eq!(result, 126384);
    }
}
