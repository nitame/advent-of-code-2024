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

fn parse_input_to_vectors(filename: PathBuf) -> (i32, i32, Vec<Vec<char>>) {
    let mut x = 0;
    let mut data = Vec::new();
    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(line) = line {
                x = line.len() as i32;
                let letters = line.chars().collect::<Vec<char>>();
                data.push(letters);
            }
        }
    }
    (x, data.len() as i32, data)
}

fn xmas_counter(
    matrix: &Vec<Vec<char>>,
    matrix_width: i32,
    matrix_height: i32,
    pos_x: i32,
    pos_y: i32,
) -> i32 {
    let sequence = vec!['M', 'A', 'S'];
    let mut counter = 0;
    // search West to East
    if pos_x + 3 < matrix_width {
        let seq = vec![
            matrix[pos_y as usize][(pos_x + 1) as usize],
            matrix[pos_y as usize][(pos_x + 2) as usize],
            matrix[pos_y as usize][(pos_x + 3) as usize],
        ];
        if seq == sequence {
            counter += 1;
        }
    }
    // search NorthWest to SouthEast
    if pos_x + 3 < matrix_width && pos_y + 3 < matrix_height {
        let seq = vec![
            matrix[(pos_y + 1) as usize][(pos_x + 1) as usize],
            matrix[(pos_y + 2) as usize][(pos_x + 2) as usize],
            matrix[(pos_y + 3) as usize][(pos_x + 3) as usize],
        ];
        if seq == sequence {
            counter += 1;
        }
    }
    // search North to South
    if pos_y + 3 < matrix_height {
        let seq = vec![
            matrix[(pos_y + 1) as usize][pos_x as usize],
            matrix[(pos_y + 2) as usize][pos_x as usize],
            matrix[(pos_y + 3) as usize][pos_x as usize],
        ];
        if seq == sequence {
            counter += 1;
        }
    }
    // search NorthEast to SouthWest
    if pos_x - 3 >= 0 && pos_y + 3 < matrix_height {
        let seq = vec![
            matrix[(pos_y + 1) as usize][(pos_x - 1) as usize],
            matrix[(pos_y + 2) as usize][(pos_x - 2) as usize],
            matrix[(pos_y + 3) as usize][(pos_x - 3) as usize],
        ];
        if seq == sequence {
            counter += 1;
        }
    }
    // search East to West
    if pos_x - 3 >= 0 {
        let seq = vec![
            matrix[pos_y as usize][(pos_x - 1) as usize],
            matrix[pos_y as usize][(pos_x - 2) as usize],
            matrix[pos_y as usize][(pos_x - 3) as usize],
        ];
        if seq == sequence {
            counter += 1;
        }
    }
    // search SouthEast to NorthWest
    if pos_x - 3 >= 0 && pos_y - 3 >= 0 {
        let seq = vec![
            matrix[(pos_y - 1) as usize][(pos_x - 1) as usize],
            matrix[(pos_y - 2) as usize][(pos_x - 2) as usize],
            matrix[(pos_y - 3) as usize][(pos_x - 3) as usize],
        ];
        if seq == sequence {
            counter += 1;
        }
    }
    // search South to North
    if pos_y - 3 >= 0 {
        let seq = vec![
            matrix[(pos_y - 1) as usize][pos_x as usize],
            matrix[(pos_y - 2) as usize][pos_x as usize],
            matrix[(pos_y - 3) as usize][pos_x as usize],
        ];
        if seq == sequence {
            counter += 1;
        }
    }
    // search SouthWest to NorthEast
    if pos_x + 3 < matrix_width && pos_y - 3 >= 0 {
        let seq = vec![
            matrix[(pos_y - 1) as usize][(pos_x + 1) as usize],
            matrix[(pos_y - 2) as usize][(pos_x + 2) as usize],
            matrix[(pos_y - 3) as usize][(pos_x + 3) as usize],
        ];
        if seq == sequence {
            counter += 1;
        }
    }

    counter
}

fn xmas_counter_on_steroide(
    matrix: &Vec<Vec<char>>,
    matrix_width: i32,
    matrix_height: i32,
    pos_x: i32,
    pos_y: i32,
) -> i32 {
    let pattern_1 = vec!['M', 'S', 'A', 'M', 'S'];
    let pattern_2 = vec!['M', 'M', 'A', 'S', 'S'];
    let pattern_3 = vec!['S', 'M', 'A', 'S', 'M'];
    let pattern_4 = vec!['S', 'S', 'A', 'M', 'M'];
    let mut counter = 0;

    if pos_x + 2 < matrix_width && pos_y + 2 < matrix_height {
        let seq = vec![
            matrix[(pos_y) as usize][(pos_x) as usize],
            matrix[(pos_y) as usize][(pos_x + 2) as usize],
            matrix[(pos_y + 1) as usize][(pos_x + 1) as usize],
            matrix[(pos_y + 2) as usize][(pos_x) as usize],
            matrix[(pos_y + 2) as usize][(pos_x + 2) as usize],
        ];
        if seq == pattern_1 || seq == pattern_2 || seq == pattern_3 || seq == pattern_4 {
            counter += 1;
        }
    }
    counter
}

fn puzzle_1(filename: PathBuf) -> i32 {
    let mut count = 0;
    let (width, height, data) = parse_input_to_vectors(filename);
    for (y, row) in data.iter().enumerate() {
        for (x, letter) in row.iter().enumerate() {
            match letter {
                'X' => {
                    count += xmas_counter(&data, width, height, x as i32, y as i32);
                }
                _ => {}
            }
        }
    }
    count
}

fn puzzle_2(filename: PathBuf) -> i32 {
    let mut count = 0;
    let (width, height, data) = parse_input_to_vectors(filename);
    for (y, row) in data.iter().enumerate() {
        for (x, letter) in row.iter().enumerate() {
            match letter {
                'M' | 'S' => {
                    count += xmas_counter_on_steroide(&data, width, height, x as i32, y as i32);
                }
                _ => {}
            }
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use crate::{get_file_path, puzzle_1, puzzle_2};

    #[test]
    fn it_returns_4_xmas() {
        let file_path = get_file_path("small-test-input.txt".to_string());
        let result = puzzle_1(file_path);
        assert_eq!(result, 4);
    }

    #[test]
    fn it_returns_18_xmas() {
        let file_path = get_file_path("test-input.txt".to_string());
        let result = puzzle_1(file_path);
        assert_eq!(result, 18);
    }

    #[test]
    fn it_returns_puzzle_1_score() {
        let file_path = get_file_path("input.txt".to_string());
        let result = puzzle_1(file_path);
        assert_eq!(result, 2521);
    }

    #[test]
    fn it_returns_9_x_mas() {
        let file_path = get_file_path("part-two-test-input.txt".to_string());
        let result = puzzle_2(file_path);
        assert_eq!(result, 9);
    }

    #[test]
    fn it_returns_puzzle_2_score() {
        let file_path = get_file_path("input.txt".to_string());
        let result = puzzle_2(file_path);
        assert_eq!(result, 1912);
    }
}
