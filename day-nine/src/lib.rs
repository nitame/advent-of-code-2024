use crate::BlockKind::{FileCell, FreeCell};
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

#[derive(Clone, Debug, PartialEq)]
enum BlockKind {
    FreeCell,
    FileCell,
}

#[derive(Debug, PartialEq)]
struct Block {
    kind: BlockKind,
    index: usize,
}

fn parse_input_to_blocks(filename: PathBuf) -> Vec<Block> {
    let mut data = Vec::new();
    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(line) = line {
                let mut idx_file_block = 0;
                for (i, char) in line.chars().enumerate() {
                    if i % 2 == 0 {
                        let block_size = char.to_digit(10).unwrap();
                        for _ in 0..block_size {
                            let block = Block {
                                kind: FileCell,
                                index: idx_file_block,
                            };
                            data.push(block);
                        }
                        idx_file_block += 1;
                    } else {
                        let block_size = char.to_digit(10).unwrap();
                        for _ in 0..block_size {
                            data.push(Block {
                                kind: FreeCell,
                                index: i,
                            })
                        }
                    }
                }
            }
        }
    }
    data
}

fn shrink_cells(mut data: Vec<Block>) -> Vec<Block> {
    let mut idx_right_to_left = data.len() - 1;
    let breakpoint = data.len() - data.iter().filter(|b| b.kind == FreeCell).count();
    for i in 0..data.len() {
        if i >= breakpoint {
            break;
        }
        let block = data.get(i).unwrap();
        if block.kind == FreeCell {
            let mut most_right_cell = data.get(idx_right_to_left).unwrap();
            while most_right_cell.kind == FreeCell {
                idx_right_to_left -= 1;
                most_right_cell = data.get(idx_right_to_left).unwrap();
            }
            data.swap(i, idx_right_to_left);
        }
    }
    data
}

fn puzzle_1(data: Vec<Block>) -> u64 {
    let mut result = 0;
    for (position, block) in data.iter().enumerate() {
        if block.kind == FileCell {
            let idx = block.index as u64;
            result += idx * position as u64;
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use crate::{get_file_path, parse_input_to_blocks, puzzle_1, shrink_cells};

    #[test]
    fn it_returns_1928() {
        let file_path = get_file_path("test-input.txt".to_string());
        let data = parse_input_to_blocks(file_path);
        let cells = shrink_cells(data);
        let result = puzzle_1(cells);
        assert_eq!(result, 1928);
    }

    #[test]
    fn it_returns_puzzle_1_score() {
        let file_path = get_file_path("input.txt".to_string());
        let data = parse_input_to_blocks(file_path);
        let cells = shrink_cells(data);
        let result = puzzle_1(cells);
        assert_eq!(result, 6301895872542);
    }
}
