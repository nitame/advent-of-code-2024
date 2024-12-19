use std::collections::VecDeque;
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
struct Coord {
    x: usize,
    y: usize,
}

fn parse_input(filename: PathBuf) -> Vec<Coord> {
    let mut coords: Vec<Coord> = Vec::new();
    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(line) = line {
                let coord = line
                    .split(",")
                    .map(|coord| coord.parse::<usize>().unwrap())
                    .collect::<Vec<usize>>();
                coords.push(Coord {
                    x: coord[0],
                    y: coord[1],
                });
            }
        }
    }
    coords
}

type MemoryMap = Vec<Vec<char>>;
fn init_map(mat: &MemoryMap, bytes: &[Coord]) -> MemoryMap {
    let mut map = mat.clone();
    for byte in bytes {
        map[byte.y][byte.x] = '#';
    }
    map[0][0] = 'S';
    map[mat[0].len() - 1][mat.len() - 1] = 'E';
    map
}

type MemoryGraph = Vec<(usize, u32, (usize, usize))>;

fn coord_i32(v: (usize, usize), dir: (i32, i32)) -> (i32, i32) {
    ((v.0 as i32 + dir.0), (v.1 as i32 + dir.1))
}

fn coord_u32(c: (i32, i32)) -> (usize, usize) {
    (c.0 as usize, c.1 as usize)
}

fn get_adj_edg(v: (usize, usize), map: &MemoryMap) -> Vec<(usize, usize)> {
    let mut adj_edg = Vec::new();
    let directions = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];
    for direction in directions {
        let coord = coord_i32(v, direction);
        let value = if coord.0 >= 0
            && coord.1 >= 0
            && coord.0 < (map[0].len()) as i32
            && coord.1 < (map.len()) as i32
        {
            Some(map[coord.1 as usize][coord.0 as usize])
        } else {
            None
        };
        if let Some(value) = value {
            if value == '.' || value == 'E' {
                adj_edg.push(coord_u32(coord));
            }
        }
    }
    adj_edg
}

fn explore(
    start: (usize, usize),
    end: (usize, usize),
    map: &MemoryMap,
) -> Option<Vec<(usize, usize)>> {
    let mut queue = VecDeque::new();
    let mut path = Vec::new();
    let mut visited = vec![vec![None; map[0].len()]; map.len()];
    queue.push_front(start);
    visited[0][0] = Some(start);
    while !queue.is_empty() {
        let vertex = queue.pop_front();
        if let Some(value) = vertex {
            if value == end {
                path.push(value);
                let mut prev = value;
                while prev != start {
                    let coord = visited[prev.1][prev.0].unwrap();
                    path.push(coord);
                    prev = coord;
                }
                return Some(path.into_iter().rev().collect::<Vec<_>>());
            }
            let adjacent_edges = get_adj_edg(value, &map);
            for adjacent in adjacent_edges {
                if visited[adjacent.1][adjacent.0].is_none() {
                    visited[adjacent.1][adjacent.0] = Some(value);
                    queue.push_back(adjacent);
                }
            }
        }
    }
    None
}

fn puzzle_1(data: Vec<Coord>, matrix_size: (usize, usize), fall_bytes: usize) -> usize {
    let mut matrix = vec![vec!['.'; matrix_size.1]; matrix_size.0];
    let bytes = &data[0..fall_bytes];
    let map = init_map(&mut matrix, bytes);
    let path = explore((0, 0), (matrix_size.0 - 1, matrix_size.1 - 1), &map);
    //display_map(&map);
    if let Some(path) = path {
        //trace_map(&map, &path);
        return path.iter().count() - 1;
    }
    0
}

#[cfg(test)]
mod tests {
    use crate::{get_file_path, parse_input, puzzle_1};

    #[test]
    fn it_returns_22() {
        let file_path = get_file_path("test-input.txt".to_string());
        let data = parse_input(file_path);
        let matrix_size = (7, 7);
        let result = puzzle_1(data, matrix_size, 12);
        assert_eq!(result, 22);
    }

    #[test]
    fn it_returns_puzzle_1_score() {
        let file_path = get_file_path("input.txt".to_string());
        let data = parse_input(file_path);
        let matrix_size = (71, 71);
        let result = puzzle_1(data, matrix_size, 1024);
        assert_eq!(result, 338);
    }
}
