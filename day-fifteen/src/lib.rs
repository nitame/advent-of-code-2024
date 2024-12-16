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
enum Move {
    Up,
    Down,
    Left,
    Right,
}

struct Robot {
    x: usize,
    y: usize,
}

type WarehouseMap = Vec<Vec<char>>;

fn parse_input(filename: PathBuf) -> (WarehouseMap, Vec<Move>, Robot) {
    let mut parse_moves = false;
    let mut move_sequence = Vec::new();
    let mut map = Vec::new();
    let mut robot = Robot { x: 0, y: 0 };
    if let Ok(lines) = read_lines(filename) {
        for (y, line) in lines.enumerate() {
            if let Ok(line) = line {
                if line.is_empty() {
                    parse_moves = true;
                }
                if parse_moves {
                    line.chars().for_each(|c| match c {
                        '^' => move_sequence.push(Move::Up),
                        '>' => move_sequence.push(Move::Right),
                        'v' => move_sequence.push(Move::Down),
                        '<' => move_sequence.push(Move::Left),
                        _ => panic!("Invalid move character: {}", c),
                    });
                } else {
                    let row = line
                        .chars()
                        .enumerate()
                        .map(|(x, c)| {
                            if c == '@' {
                                robot = Robot { x, y }
                            }
                            c
                        })
                        .collect::<Vec<char>>();
                    map.push(row);
                }
            }
        }
    }
    (map, move_sequence, robot)
}

fn puzzle_1(mut map: WarehouseMap, moves: Vec<Move>, mut robot: Robot) -> i32 {
    for m in moves {
        match m {
            Move::Up => {
                let next = (robot.x, robot.y - 1);
                let item = map[next.1][next.0];
                match item {
                    '.' => {
                        map[next.1][next.0] = '@';
                        map[robot.y][robot.x] = '.';
                        robot.x = next.0;
                        robot.y = next.1;
                    }
                    'O' => {
                        let up = (0..=next.1).map(|n| map[n][next.0]).collect::<Vec<char>>();
                        let up_goods = up
                            .iter()
                            .rev()
                            .take_while(|&elem| *elem == 'O')
                            .collect::<Vec<_>>();
                        let further_up = map[next.1 - up_goods.len()][next.0];
                        if up_goods.len() > 0 && further_up == '.' {
                            // move
                            map[next.1][next.0] = '@';
                            map[robot.y][robot.x] = '.';
                            robot.x = next.0;
                            robot.y = next.1;
                            for i in 1..=up_goods.len() {
                                map[next.1 - i][next.0] = 'O';
                            }
                        }
                    }
                    _ => {}
                }
            }
            Move::Right => {
                let next = (robot.x + 1, robot.y);
                let item = map[next.1][next.0];
                match item {
                    '.' => {
                        map[next.1][next.0] = '@';
                        map[robot.y][robot.x] = '.';
                        robot.x = next.0;
                        robot.y = next.1;
                    }
                    'O' => {
                        let right = (next.0..map[next.1].len())
                            .map(|n| map[next.1][n])
                            .collect::<Vec<char>>();
                        let right_goods = right
                            .iter()
                            .take_while(|&elem| *elem == 'O')
                            .collect::<Vec<_>>();
                        let further_right = map[next.1][next.0 + right_goods.len()];
                        if right_goods.len() > 0 && further_right == '.' {
                            // move
                            map[next.1][next.0] = '@';
                            map[robot.y][robot.x] = '.';
                            robot.x = next.0;
                            robot.y = next.1;
                            for i in 1..=right_goods.len() {
                                map[next.1][next.0 + i] = 'O';
                            }
                        }
                    }
                    _ => {}
                }
            }
            Move::Down => {
                let next = (robot.x, robot.y + 1);
                let item = map[next.1][next.0];
                match item {
                    '.' => {
                        map[next.1][next.0] = '@';
                        map[robot.y][robot.x] = '.';
                        robot.x = next.0;
                        robot.y = next.1;
                    }
                    'O' => {
                        let down = (next.1..map.len())
                            .map(|n| map[n][next.0])
                            .collect::<Vec<char>>();
                        let down_goods = down
                            .iter()
                            .take_while(|&elem| *elem == 'O')
                            .collect::<Vec<_>>();
                        let further_down = map[next.1 + down_goods.len()][next.0];
                        if down_goods.len() > 0 && further_down == '.' {
                            // move
                            map[next.1][next.0] = '@';
                            map[robot.y][robot.x] = '.';
                            robot.x = next.0;
                            robot.y = next.1;
                            for i in 1..=down_goods.len() {
                                map[next.1 + i][next.0] = 'O';
                            }
                        }
                    }
                    _ => {}
                }
            }
            Move::Left => {
                let next = (robot.x - 1, robot.y);
                let item = map[next.1][next.0];
                match item {
                    '.' => {
                        map[next.1][next.0] = '@';
                        map[robot.y][robot.x] = '.';
                        robot.x = next.0;
                        robot.y = next.1;
                    }
                    'O' => {
                        let left = (0..=next.0).map(|n| map[next.1][n]).collect::<Vec<char>>();
                        let left_goods = left
                            .iter()
                            .rev()
                            .take_while(|&elem| *elem == 'O')
                            .collect::<Vec<_>>();
                        let further_left = map[next.1][next.0 - left_goods.len()];
                        if left_goods.len() > 0 && further_left == '.' {
                            // move
                            map[next.1][next.0] = '@';
                            map[robot.y][robot.x] = '.';
                            robot.x = next.0;
                            robot.y = next.1;
                            for i in 1..=left_goods.len() {
                                map[next.1][next.0 - i] = 'O';
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    let mut sum = 0;
    for (y, row) in map.iter().enumerate() {
        for (x, element) in row.iter().enumerate() {
            if *element == 'O' {
                sum += (100 * y + x) as i32;
            }
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use crate::{get_file_path, parse_input, puzzle_1};

    #[test]
    fn it_returns_2028() {
        let file_path = get_file_path("small-test-input.txt".to_string());
        let (map, moves, robot) = parse_input(file_path);
        let result = puzzle_1(map, moves, robot);
        assert_eq!(result, 2028);
    }

    #[test]
    fn it_returns_10092() {
        let file_path = get_file_path("test-input.txt".to_string());
        let (map, moves, robot) = parse_input(file_path);
        let result = puzzle_1(map, moves, robot);
        assert_eq!(result, 10092);
    }

    #[test]
    fn it_returns_puzzle_1_score() {
        let file_path = get_file_path("input.txt".to_string());
        let (map, moves, robot) = parse_input(file_path);
        let result = puzzle_1(map, moves, robot);
        assert_eq!(result, 1486930);
    }
}
