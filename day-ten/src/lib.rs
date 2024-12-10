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

type TopographicMap = Vec<Vec<u8>>;

fn parse_input(filename: PathBuf) -> TopographicMap {
    let mut data = Vec::new();
    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(line) = line {
                let row = line
                    .chars()
                    .map(|c| c.to_digit(10).unwrap() as u8)
                    .collect();
                data.push(row);
            }
        }
    }
    data
}

fn pathfinding(
    y: usize,
    x: usize,
    map: &TopographicMap,
    peaks: &mut HashSet<(usize, usize)>,
) -> u32 {
    let mut sum = 0;
    if map[y][x] == 9 {
        return if !peaks.contains(&(y, x)) {
            peaks.insert((y, x));
            1
        } else {
            0
        };
    }
    let elevation = map[y][x] + 1;
    let right = if x + 1 < map[0].len() {
        Some(map[y][x + 1])
    } else {
        None
    };
    let left = if x as i32 - 1 >= 0 {
        Some(map[y][x - 1])
    } else {
        None
    };
    let down = if y + 1 < map.len() {
        Some(map[y + 1][x])
    } else {
        None
    };
    let up = if y as i32 - 1 >= 0 {
        Some(map[y - 1][x])
    } else {
        None
    };

    if let Some(height) = right {
        if height == elevation {
            sum += pathfinding(y, x + 1, map, peaks)
        }
    }
    if let Some(height) = left {
        if height == elevation {
            sum += pathfinding(y, x - 1, map, peaks)
        }
    }
    if let Some(height) = down {
        if height == elevation {
            sum += pathfinding(y + 1, x, map, peaks)
        }
    }
    if let Some(height) = up {
        if height == elevation {
            sum += pathfinding(y - 1, x, map, peaks)
        }
    }
    sum
}

fn puzzle_1(data: TopographicMap) -> u32 {
    let mut result = 0;
    for (y, row) in data.iter().enumerate() {
        for (x, position) in row.iter().enumerate() {
            if *position == 0 {
                let mut peaks = HashSet::new();
                let path_count = pathfinding(y, x, &data, &mut peaks);
                result += path_count;
            }
        }
    }
    result
}

fn pathfinding_2(y: usize, x: usize, map: &TopographicMap) -> u32 {
    if map[y][x] == 9 {
        return 1;
    }
    let mut sum = 0;
    let elevation = map[y][x] + 1;
    let right = if x + 1 < map[0].len() {
        Some(map[y][x + 1])
    } else {
        None
    };
    let left = if x as i32 - 1 >= 0 {
        Some(map[y][x - 1])
    } else {
        None
    };
    let down = if y + 1 < map.len() {
        Some(map[y + 1][x])
    } else {
        None
    };
    let up = if y as i32 - 1 >= 0 {
        Some(map[y - 1][x])
    } else {
        None
    };

    if let Some(height) = right {
        if height == elevation {
            sum += crate::pathfinding_2(y, x + 1, map)
        }
    }
    if let Some(height) = left {
        if height == elevation {
            sum += crate::pathfinding_2(y, x - 1, map)
        }
    }
    if let Some(height) = down {
        if height == elevation {
            sum += crate::pathfinding_2(y + 1, x, map)
        }
    }
    if let Some(height) = up {
        if height == elevation {
            sum += crate::pathfinding_2(y - 1, x, map)
        }
    }
    sum
}

fn puzzle_2(data: TopographicMap) -> u32 {
    let mut result = 0;
    for (y, row) in data.iter().enumerate() {
        for (x, position) in row.iter().enumerate() {
            if *position == 0 {
                let path_count = pathfinding_2(y, x, &data);
                result += path_count;
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use crate::{get_file_path, parse_input, puzzle_1, puzzle_2};

    #[test]
    fn it_returns_1() {
        let file_path = get_file_path("small-test-input.txt".to_string());
        let data = parse_input(file_path);
        let result = puzzle_1(data);
        assert_eq!(result, 1);
    }

    #[test]
    fn it_returns_36() {
        let file_path = get_file_path("test-input.txt".to_string());
        let data = parse_input(file_path);
        let result = puzzle_1(data);
        assert_eq!(result, 36);
    }

    #[test]
    fn it_returns_puzzle_1_score() {
        let file_path = get_file_path("input.txt".to_string());
        let data = parse_input(file_path);
        let result = puzzle_1(data);
        assert_eq!(result, 694);
    }

    #[test]
    fn it_returns_81() {
        let file_path = get_file_path("test-input.txt".to_string());
        let data = parse_input(file_path);
        let result = puzzle_2(data);
        assert_eq!(result, 81);
    }

    #[test]
    fn it_returns_puzzle_2_score() {
        let file_path = get_file_path("input.txt".to_string());
        let data = parse_input(file_path);
        let result = puzzle_2(data);
        assert_eq!(result, 1497);
    }
}
