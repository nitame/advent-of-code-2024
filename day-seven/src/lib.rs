use radix_fmt::Radix;
use std::collections::HashSet;
use std::fmt::Write;
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

struct Assertion {
    result: u64,
    operands: Vec<u64>,
}

fn parse_input_to_assertions(input: PathBuf) -> Vec<Assertion> {
    let mut assertions = Vec::new();
    if let Ok(lines) = read_lines(input) {
        for line in lines {
            if let Ok(line) = line {
                let parsed = line.split(":").collect::<Vec<&str>>();
                let result = parsed[0].parse::<u64>().unwrap();
                let operands = parsed[1]
                    .trim()
                    .split(" ")
                    .map(|x| x.parse::<u64>().unwrap())
                    .collect::<Vec<u64>>();
                let assertion = Assertion { result, operands };
                assertions.push(assertion);
            }
        }
    }
    assertions
}

fn puzzle_1(assertions: Vec<Assertion>) -> u64 {
    let mut sum = 0;
    for assertion in assertions {
        let op_seq = generate_possible_operator_sequences(assertion.operands.len() - 1);
        for seq in op_seq {
            let computed = assertion.operands.iter().skip(1).enumerate().fold(
                assertion.operands[0],
                |acc, (idx, x)| {
                    if seq[idx] == '+' {
                        return acc + *x;
                    } else if seq[idx] == '*' {
                        return acc * *x;
                    } else {
                        panic!("Invalid operator: {}", seq[idx]);
                    }
                },
            );
            if computed == assertion.result {
                sum += assertion.result;
                break;
            }
        }
    }
    sum
}

fn generate_possible_operator_sequences(
    number_of_operators_in_equation: usize,
) -> HashSet<Vec<char>> {
    let mut result = HashSet::new();
    let signs = ['+', '*'];
    let op = 2_u32.pow(number_of_operators_in_equation as u32);

    for i in 0..op {
        let binary = format!("{i:012b}");
        let significant_binary = {
            let split_pos = binary
                .char_indices()
                .nth_back(number_of_operators_in_equation - 1)
                .unwrap()
                .0;
            &binary[split_pos..]
        };

        let binary_vec: Vec<usize> = significant_binary
            .chars()
            .map(|c| c.to_digit(10).unwrap() as usize)
            .collect();
        result.insert(binary_vec.iter().map(|&x| signs[x]).collect::<Vec<char>>());
    }
    result
}

fn puzzle_2(assertions: Vec<Assertion>) -> u64 {
    let mut sum = 0;
    for assertion in assertions {
        let op_seq = generate_possible_operator_sequences_improved(assertion.operands.len() - 1);
        for seq in op_seq {
            let computed = assertion.operands.iter().skip(1).enumerate().fold(
                assertion.operands[0],
                |acc, (idx, x)| {
                    if seq[idx] == '+' {
                        return acc + *x;
                    } else if seq[idx] == '*' {
                        return acc * *x;
                    } else if seq[idx] == '|' {
                        return (acc.to_string() + &x.to_string()).parse::<u64>().unwrap();
                    } else {
                        panic!("Invalid operator: {}", seq[idx]);
                    }
                },
            );
            if computed == assertion.result {
                sum += assertion.result;
                break;
            }
        }
    }
    sum
}

fn generate_possible_operator_sequences_improved(
    number_of_operators_in_equation: usize,
) -> HashSet<Vec<char>> {
    let mut result = HashSet::new();
    let signs = ['+', '*', '|'];
    let op = 3_u32.pow(number_of_operators_in_equation as u32);

    for i in 0..op {
        let mut s = String::new();
        write!(&mut s, "{}", Radix::new(i, 3)).expect("Error while writing base 3 to string");
        let base_3 = format!("{:0>12}", s);
        let significant_base_3 = {
            let split_pos = base_3
                .char_indices()
                .nth_back(number_of_operators_in_equation - 1)
                .unwrap()
                .0;
            &base_3[split_pos..]
        };
        let base_3_vec = significant_base_3
            .chars()
            .map(|c| c.to_digit(10).unwrap() as usize)
            .collect::<Vec<usize>>();
        result.insert(base_3_vec.iter().map(|&x| signs[x]).collect::<Vec<char>>());
    }
    result
}

#[cfg(test)]
mod tests {
    use crate::{
        generate_possible_operator_sequences, get_file_path, parse_input_to_assertions, puzzle_1,
        puzzle_2,
    };
    use std::collections::HashSet;

    #[test]
    fn it_returns_4_operation_sequences() {
        let result = generate_possible_operator_sequences(2);
        let mut expected = HashSet::new();
        expected.insert(vec!['+', '+']);
        expected.insert(vec!['*', '*']);
        expected.insert(vec!['+', '*']);
        expected.insert(vec!['*', '+']);
        assert_eq!(result, expected);
    }

    #[test]
    fn it_returns_8_operation_sequences() {
        let result = generate_possible_operator_sequences(3);

        let mut expected = HashSet::new();
        expected.insert(vec!['+', '+', '*']);
        expected.insert(vec!['+', '*', '*']);
        expected.insert(vec!['+', '*', '+']);
        expected.insert(vec!['*', '*', '*']);
        expected.insert(vec!['*', '+', '+']);
        expected.insert(vec!['*', '*', '+']);
        expected.insert(vec!['*', '+', '*']);
        expected.insert(vec!['+', '+', '+']);
        assert_eq!(result, expected);
    }

    #[test]
    fn it_returns_16_operation_sequences() {
        let result = generate_possible_operator_sequences(4);

        let mut expected = HashSet::new();
        expected.insert(vec!['+', '+', '+', '+']);
        expected.insert(vec!['*', '+', '+', '+']);
        expected.insert(vec!['+', '*', '+', '+']);
        expected.insert(vec!['+', '+', '*', '+']);
        expected.insert(vec!['+', '+', '+', '*']);
        expected.insert(vec!['*', '*', '+', '+']);
        expected.insert(vec!['+', '*', '*', '+']);
        expected.insert(vec!['+', '+', '*', '*']);
        expected.insert(vec!['*', '*', '*', '+']);
        expected.insert(vec!['+', '*', '*', '*']);
        expected.insert(vec!['*', '*', '*', '*']);
        expected.insert(vec!['*', '+', '*', '*']);
        expected.insert(vec!['*', '*', '+', '*']);
        expected.insert(vec!['*', '+', '+', '*']);
        expected.insert(vec!['+', '*', '+', '*']);
        expected.insert(vec!['*', '+', '*', '+']);
        assert_eq!(result, expected);
    }

    #[test]
    fn it_returns_3749() {
        let input_path = get_file_path("test-input.txt".to_string());
        let assertions = parse_input_to_assertions(input_path);
        let result = puzzle_1(assertions);
        assert_eq!(result, 3749);
    }

    #[test]
    fn it_returns_puzzle_1_score() {
        let input_path = get_file_path("input.txt".to_string());
        let assertions = parse_input_to_assertions(input_path);
        let result = puzzle_1(assertions);
        assert_eq!(result, 1611660863222);
    }

    #[test]
    fn it_returns_11387() {
        let input_path = get_file_path("test-input.txt".to_string());
        let assertions = parse_input_to_assertions(input_path);
        let result = puzzle_2(assertions);
        assert_eq!(result, 11387);
    }

    #[test]
    fn it_returns_puzzle_2_score() {
        let input_path = get_file_path("input.txt".to_string());
        let assertions = parse_input_to_assertions(input_path);
        let result = puzzle_2(assertions);
        assert_eq!(result, 945341732469724);
    }
}
