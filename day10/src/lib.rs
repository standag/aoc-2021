extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::error::InputLocation;
use pest::Parser;
use std::fs;

pub fn solve_part_1() {
    let input = read_input("day10/input.txt");
    println!("Result: {}", _solve_part_1(&input))
}
pub fn solve_part_2() {
    let input = read_input("day10/input.txt");
    println!("Result: {}", _solve_part_2(&input))
}

fn read_input(filename: &str) -> Vec<String> {
    fs::read_to_string(filename)
        .unwrap()
        .split_whitespace()
        .map(|line| line.to_string())
        .collect()
}

fn _solve_part_1(input: &Vec<String>) -> usize {
    let mut sum = 0;
    for line in input {
        let parsed = MyParser::parse(Rule::x, line);
        if let Err(_) = parsed {
            let location = parsed.err().unwrap().location;
            if let InputLocation::Pos(position) = location {
                let illegal = line.chars().nth(position);
                sum += match illegal {
                    Some(')') => 3,
                    Some(']') => 57,
                    Some('}') => 1197,
                    Some('>') => 25137,
                    None => 0,
                    _ => panic!(),
                };
                println!("{} {:?}", line, illegal)
            }
        }
    }
    sum
}

#[derive(Debug, PartialEq)]
enum ValidationResult {
    Incomplete,
    Corrupted,
    Fine,
}

fn validate(s: &String) -> ValidationResult {
    let parsed = MyParser::parse(Rule::y, s);
    return match parsed {
        Ok(_) => ValidationResult::Fine,
        Err(_) => match parsed.err().unwrap().location {
            InputLocation::Pos(position) => match position {
                _ if position == s.len() => ValidationResult::Incomplete,
                _ => ValidationResult::Corrupted,
            },
            _ => panic!(),
        },
    };
}

fn _solve_part_2(input: &Vec<String>) -> usize {
    let mut score = vec![];
    for line in input {
        let first_validation = validate(line);
        if first_validation != ValidationResult::Incomplete {
            continue;
        }
        println!("{} {:?}", line, validate(line));
        let mut append = String::new();
        loop {
            for c in ")}]>".chars() {
                if validate(&(line.to_string() + &append + &c.to_string()))
                    != ValidationResult::Corrupted
                {
                    append.push(c);
                }
            }
            if validate(&(line.to_string() + &append)) == ValidationResult::Fine {
                break;
            }
        }
        println!("{}", append);
        score.push(
            append
                .chars()
                .map(|c| match c {
                    ')' => 1,
                    ']' => 2,
                    '}' => 3,
                    '>' => 4,
                    _ => panic!(),
                })
                .fold(0, |acc, x| acc * 5 + x),
        )
    }
    score.sort();
    println!("{:?}", score);
    score[score.len() / 2]
}

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct MyParser;

#[cfg(test)]
mod tests {
    use crate::pest::Parser;
    use crate::{MyParser, Rule, _solve_part_1, _solve_part_2, read_input};
    #[test]
    fn it_works() {
        let result = MyParser::parse(Rule::x, "[({(<(())[]>[[{[]{<()<>>");
    }
    #[test]
    fn test_part_1() {
        assert_eq!(_solve_part_1(&read_input("input_test.txt")), 26397)
    }

    #[test]
    fn test_part_2() {
        assert_eq!(_solve_part_2(&read_input("input_test.txt")), 288957)
    }
}
