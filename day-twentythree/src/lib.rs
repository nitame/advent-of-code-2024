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

fn parse_input(filename: PathBuf) -> Vec<(String, String)> {
    let mut computer_conn = Vec::new();
    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(line) = line {
                let [l, r] = line.split("-").collect::<Vec<&str>>()[..] else {
                    panic!("Invalid line: {:?}", line)
                };
                computer_conn.push((l.to_string(), r.to_string()));
            }
        }
    }
    computer_conn
}

fn puzzle_1(data: Vec<(String, String)>) -> usize {
    let mut computer_names = HashSet::new();
    let mut computer_links = HashSet::new();
    let mut triconnect = HashSet::new();
    for connection in &data {
        computer_names.insert(connection.0.clone());
        computer_names.insert(connection.1.clone());
        computer_links.insert((connection.0.clone(), connection.1.clone()));
        computer_links.insert((connection.1.clone(), connection.0.clone()));
    }

    for connection in &data {
        for name in &computer_names {
            if computer_links.contains(&(name.clone(), connection.0.clone()))
                && computer_links.contains(&(connection.1.clone(), name.clone()))
            {
                let mut triple = vec![name.clone(), connection.0.clone(), connection.1.clone()];
                triple.sort();
                triconnect.insert(triple);
            }
        }
    }

    let t_filter = triconnect
        .iter()
        .filter(|p| {
            let [a, b, c] = &p[..] else {
                panic!("Failed to destructure vec: {:?}", p)
            };
            a.starts_with("t") || b.starts_with("t") || c.starts_with("t")
        })
        .collect::<Vec<_>>();

    t_filter.len()
}

#[cfg(test)]
mod tests {
    use crate::{get_file_path, parse_input, puzzle_1};

    #[test]
    fn it_returns_7() {
        let file_path = get_file_path("test-input.txt".to_string());
        let data = parse_input(file_path);
        let result = puzzle_1(data);
        assert_eq!(result, 7);
    }

    #[test]
    fn it_returns_puzzle_1_score() {
        let file_path = get_file_path("input.txt".to_string());
        let data = parse_input(file_path);
        let result = puzzle_1(data);
        assert_eq!(result, 1378);
    }
}
