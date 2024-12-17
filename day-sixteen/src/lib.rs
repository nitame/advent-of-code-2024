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

struct ReindeerGame {
    map: Vec<Vec<char>>,
    reindeer_pos: (usize, usize),
    reindeer_orientation: char,
    end_pos: (usize, usize),
}

fn parse_input(filename: PathBuf) -> ReindeerGame {
    let mut map = Vec::new();
    let mut reindeer_pos = (0, 0);
    let mut end_pos = (0, 0);
    if let Ok(lines) = read_lines(filename) {
        for (y, line) in lines.enumerate() {
            if let Ok(line) = line {
                let l = line
                    .chars()
                    .enumerate()
                    .map(|(x, c)| {
                        if c == 'S' {
                            reindeer_pos = (x, y);
                        }
                        if c == 'E' {
                            end_pos = (x, y);
                        }
                        c
                    })
                    .collect::<Vec<char>>();
                map.push(l);
            }
        }
    }
    ReindeerGame {
        map,
        reindeer_pos,
        reindeer_orientation: '<',
        end_pos,
    }
}

fn puzzle_1(mut data: ReindeerGame) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use crate::{get_file_path, parse_input, puzzle_1};

    #[test]
    fn it_returns_7036() {
        let file_path = get_file_path("test-input-one.txt".to_string());
        let game = parse_input(file_path);
        let result = puzzle_1(game);
        assert_eq!(result, 7036);
    }

    #[test]
    fn it_returns_11048() {
        let file_path = get_file_path("test-input-two.txt".to_string());
        let game = parse_input(file_path);
        let result = puzzle_1(game);
        assert_eq!(result, 11048);
    }
}
