use std::fs;

pub fn solve_part_1() {
    let input = read_submarine_instructions("day02/input.txt");
    println!("Result: {}", _solve_part_1(&input))
}

pub fn solve_part_2() {
    let input = read_submarine_instructions("day02/input.txt");
    println!("Result: {}", _solve_part_2(&input))
}

enum Direction {
    Forward,
    Down,
    Up,
}

struct Instruction {
    direction: Direction,
    value: u8,
}

impl Instruction {
    fn new(line: &str) -> Self {
        let _line = line
            .split_whitespace()
            .map(|x| x.to_string())
            .collect::<Vec<String>>();
        let direction = match _line[0].as_str() {
            "forward" => Direction::Forward,
            "down" => Direction::Down,
            "up" => Direction::Up,
            _ => panic!("{:?}", _line),
        };
        Instruction {
            direction,
            value: _line[1].parse::<u8>().unwrap(),
        }
    }
}
fn read_submarine_instructions(filename: &str) -> Vec<Instruction> {
    fs::read_to_string(filename)
        .unwrap()
        .split_terminator("\n")
        .map(|x| Instruction::new(x))
        .collect()
}

fn _solve_part_1(instructions: &Vec<Instruction>) -> usize {
    let mut x: usize = 0;
    let mut y: usize = 0;
    for instruction in instructions {
        match instruction.direction {
            Direction::Forward => x += instruction.value as usize,
            Direction::Up => y -= instruction.value as usize,
            Direction::Down => y += instruction.value as usize,
        }
    }
    (x * y).into()
}

fn _solve_part_2(instructions: &Vec<Instruction>) -> usize {
    let mut x: usize = 0;
    let mut y: usize = 0;
    let mut aim: usize = 0;
    for instruction in instructions {
        match instruction.direction {
            Direction::Forward => {
                x += instruction.value as usize;
                y += instruction.value as usize * aim
            }
            Direction::Up => aim -= instruction.value as usize,
            Direction::Down => aim += instruction.value as usize,
        }
    }
    (x * y).into()
}

#[cfg(test)]
mod tests {
    use crate::{_solve_part_1, _solve_part_2, read_submarine_instructions};

    #[test]
    fn test_part_1() {
        let input = read_submarine_instructions("input_test.txt");
        assert_eq!(_solve_part_1(&input), 150);
    }
    #[test]
    fn test_part_2() {
        let input = read_submarine_instructions("input_test.txt");
        assert_eq!(_solve_part_2(&input), 900);
    }
}
