use std::fs::File;
use std::io::BufRead;
use std::path::{Path, PathBuf};
use std::{env, io};

const MAX_LOOP_ITERATIONS: usize = 10000;

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

fn display_game_map(map: GameMap) {
    let mut print_map = String::new();
    for row in map {
        let mut line = row
            .iter()
            .fold(String::new(), |acc, x| acc + &x.to_string());
        line.push('\n');
        print_map += &line;
    }
    println!("{}", print_map);
}

enum Direction {
    Up,
    Right,
    Down,
    Left,
}

struct GameState {
    game_map: GameMap,
    guard_position: Position,
    direction: Direction,
    exited: bool,
}
type GameMap = Vec<Vec<char>>;

struct Position {
    x: i32,
    y: i32,
}

fn parse_input_to_game_map(input: PathBuf) -> GameState {
    let mut game_map: GameMap = Vec::new();
    let mut starting_pos: Position = Position { x: -1, y: -1 };
    if let Ok(lines) = read_lines(input) {
        for (y_idx, line) in lines.enumerate() {
            if let Ok(line) = line {
                let row = line
                    .chars()
                    .enumerate()
                    .map(|(x_idx, c)| {
                        if c == '^' {
                            starting_pos = Position {
                                x: x_idx as i32,
                                y: y_idx as i32,
                            };
                        }
                        c
                    })
                    .collect::<Vec<char>>();
                game_map.push(row);
            }
        }
    }
    if starting_pos.x < 0 || starting_pos.y < 0 {
        panic!("Starting position not found in map");
    }
    GameState {
        game_map,
        guard_position: starting_pos,
        direction: Direction::Up,
        exited: false,
    }
}

fn update_game_map(game_state: &mut GameState) {
    let can_move_up = game_state.guard_position.y - 1 >= 0
        && game_state.game_map[game_state.guard_position.y as usize - 1]
            [game_state.guard_position.x as usize]
            != '#';
    let can_move_right = game_state.guard_position.x + 1 < game_state.game_map[0].len() as i32
        && game_state.game_map[game_state.guard_position.y as usize]
            [game_state.guard_position.x as usize + 1]
            != '#';
    let can_move_down = game_state.guard_position.y + 1 < game_state.game_map.len() as i32
        && game_state.game_map[game_state.guard_position.y as usize + 1]
            [game_state.guard_position.x as usize]
            != '#';
    let can_move_left = game_state.guard_position.x - 1 >= 0
        && game_state.game_map[game_state.guard_position.y as usize]
            [game_state.guard_position.x as usize - 1]
            != '#';
    match game_state.direction {
        Direction::Up => {
            if game_state.guard_position.y - 1 < 0 {
                game_state.exited = true;
            } else if can_move_up {
                let next_pos = Position {
                    x: game_state.guard_position.x,
                    y: game_state.guard_position.y - 1,
                };
                game_state.game_map[game_state.guard_position.y as usize]
                    [game_state.guard_position.x as usize] = 'X';
                game_state.game_map[next_pos.y as usize][next_pos.x as usize] = '^';
                game_state.guard_position = next_pos;
            } else {
                game_state.game_map[game_state.guard_position.y as usize]
                    [game_state.guard_position.x as usize] = '>';
                game_state.direction = Direction::Right;
            }
        }
        Direction::Right => {
            if game_state.guard_position.x + 1 > game_state.game_map[0].len() as i32 - 1 {
                game_state.exited = true;
            } else if can_move_right {
                let next_pos = Position {
                    x: game_state.guard_position.x + 1,
                    y: game_state.guard_position.y,
                };
                game_state.game_map[game_state.guard_position.y as usize]
                    [game_state.guard_position.x as usize] = 'X';
                game_state.game_map[next_pos.y as usize][next_pos.x as usize] = '>';
                game_state.guard_position = next_pos;
            } else {
                game_state.game_map[game_state.guard_position.y as usize]
                    [game_state.guard_position.x as usize] = 'v';
                game_state.direction = Direction::Down;
            }
        }
        Direction::Down => {
            if game_state.guard_position.y + 1 > game_state.game_map.len() as i32 - 1 {
                game_state.exited = true;
            } else if can_move_down {
                let next_pos = Position {
                    x: game_state.guard_position.x,
                    y: game_state.guard_position.y + 1,
                };
                game_state.game_map[game_state.guard_position.y as usize]
                    [game_state.guard_position.x as usize] = 'X';
                game_state.game_map[next_pos.y as usize][next_pos.x as usize] = 'v';
                game_state.guard_position = next_pos;
            } else {
                game_state.game_map[game_state.guard_position.y as usize]
                    [game_state.guard_position.x as usize] = '<';
                game_state.direction = Direction::Left;
            }
        }
        Direction::Left => {
            if game_state.guard_position.x - 1 < 0 {
                game_state.exited = true;
            } else if can_move_left {
                let next_pos = Position {
                    x: game_state.guard_position.x - 1,
                    y: game_state.guard_position.y,
                };
                game_state.game_map[game_state.guard_position.y as usize]
                    [game_state.guard_position.x as usize] = 'X';
                game_state.game_map[next_pos.y as usize][next_pos.x as usize] = '<';
                game_state.guard_position = next_pos;
            } else {
                game_state.game_map[game_state.guard_position.y as usize]
                    [game_state.guard_position.x as usize] = '^';
                game_state.direction = Direction::Up;
            }
        }
    }
}

fn puzzle_1(game_state: &mut GameState) -> i32 {
    for i in 0..=MAX_LOOP_ITERATIONS {
        if game_state.exited {
            println!("Guard moved out of map");
            break;
        }
        if i == MAX_LOOP_ITERATIONS {
            println!("Maximum iterations reached");
        }
        update_game_map(game_state);
    }
    display_game_map(game_state.game_map.clone());
    let count = game_state
        .game_map
        .iter()
        .fold(String::new(), |acc, row| {
            acc + &row.iter().collect::<String>()
        })
        .chars()
        .filter(|c| *c == 'X')
        .count();
    count as i32 + 1
}

#[cfg(test)]
mod tests {
    use crate::{get_file_path, parse_input_to_game_map, puzzle_1};

    #[test]
    fn it_returns_41() {
        let input_path = get_file_path("test-input.txt".to_string());
        let mut game_state = parse_input_to_game_map(input_path);
        let result = puzzle_1(&mut game_state);
        assert_eq!(result, 41);
    }
    #[test]
    fn it_returns_max_loop_iterations() {
        let input_path = get_file_path("test-loop-input.txt".to_string());
        let mut game_state = parse_input_to_game_map(input_path);
        let result = puzzle_1(&mut game_state);
        assert_eq!(result, 20);
    }

    #[test]
    fn it_returns_puzzle_1_score() {
        let input_path = get_file_path("input.txt".to_string());
        let mut game_state = parse_input_to_game_map(input_path);
        let result = puzzle_1(&mut game_state);
        assert_eq!(result, 5129);
    }
}
