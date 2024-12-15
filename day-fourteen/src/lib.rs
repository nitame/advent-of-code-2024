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
struct Robot {
    position: (i32, i32),
    speed: (i32, i32),
}

fn parse_input(filename: PathBuf) -> Vec<Robot> {
    let mut robots = Vec::new();
    if let Ok(lines) = read_lines(filename) {
        for (y, line) in lines.enumerate() {
            if let Ok(line) = line {
                let [ref position, ref speed] = line
                    .split_whitespace()
                    .map(|pattern| {
                        if pattern.starts_with("p") {
                            pattern
                                .strip_prefix("p=")
                                .unwrap()
                                .split(",")
                                .map(|s| s.parse::<i32>().unwrap())
                                .collect::<Vec<i32>>()
                        } else if pattern.starts_with("v") {
                            pattern
                                .strip_prefix("v=")
                                .unwrap()
                                .split(",")
                                .map(|s| s.parse::<i32>().unwrap())
                                .collect::<Vec<i32>>()
                        } else {
                            panic!("Invalid line: {}", line);
                        }
                    })
                    .collect::<Vec<Vec<i32>>>()[..]
                else {
                    panic!("Error while parsing line to position and speed")
                };
                robots.push(Robot {
                    position: (position[0], position[1]),
                    speed: (speed[0], speed[1]),
                });
            }
        }
    }
    robots
}

fn puzzle_1(robots: &mut Vec<Robot>, width: i32, height: i32, iteration: i32) -> i32 {
    for i in 0..iteration {
        for robot in &mut *robots {
            let mut next_x = robot.position.0 + robot.speed.0;
            let mut next_y = robot.position.1 + robot.speed.1;
            if next_x > width - 1 {
                next_x = next_x - width;
            }
            if next_x < 0 {
                next_x = width + next_x;
            }
            if next_y > height - 1 {
                next_y = next_y - height;
            }
            if next_y < 0 {
                next_y = height + next_y;
            }
            robot.position = (next_x, next_y);
        }
    }
    let mid_width_line = width / 2;
    let mid_height_line = height / 2;

    let counters = robots.iter().fold((0, 0, 0, 0), |mut acc, curr| {
        if curr.position.0 < mid_width_line && curr.position.1 < mid_height_line {
            acc.0 += 1;
        }
        if curr.position.0 > mid_width_line && curr.position.1 < mid_height_line {
            acc.1 += 1;
        }
        if curr.position.0 < mid_width_line && curr.position.1 > mid_height_line {
            acc.2 += 1;
        }
        if curr.position.0 > mid_width_line && curr.position.1 > mid_height_line {
            acc.3 += 1;
        }
        return acc;
    });

    counters.0 * counters.1 * counters.2 * counters.3
}

#[cfg(test)]
mod tests {
    use crate::{get_file_path, parse_input, puzzle_1};

    #[test]
    fn it_returns_12() {
        let file_path = get_file_path("test-input.txt".to_string());
        let mut data = parse_input(file_path);
        let result = puzzle_1(&mut data, 11, 7, 100);
        assert_eq!(result, 12);
    }

    #[test]
    fn it_returns_puzzle_1_score() {
        let file_path = get_file_path("input.txt".to_string());
        let mut data = parse_input(file_path);
        let result = puzzle_1(&mut data, 101, 103, 100);
        assert_eq!(result, 218433348);
    }
}
