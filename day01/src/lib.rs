use std::fs;

pub fn solve_part_1() {
    println!("Hello from lib day01");
    let measurements = read_sonar_measurements("./day01/input.txt");
    println!("Result: {}", _solve_part_1(&measurements));
}

pub fn solve_part_2() {
    println!("Hello from lib day01");
    let measurements = read_sonar_measurements("./day01/input.txt");
    println!("Result: {}", _solve_part_2(&measurements));
}

fn _solve_part_1(input: &Vec<u16>) -> u16 {
    let mut counter = 0;
    for i in 1..input.len() {
        if input[i - 1] < input[i] {
            println!("{} {} increased", i, input[i]);
            counter += 1;
        }
    }
    counter
}

fn _solve_part_2(input: &Vec<u16>) -> u16 {
    let mut windowed_measurements = Vec::new();
    for i in 2..input.len() {
        windowed_measurements.push(input[i - 2..=i].into_iter().sum());
    }
    _solve_part_1(&windowed_measurements)
}

fn read_sonar_measurements(filename: &str) -> Vec<u16> {
    fs::read_to_string(filename)
        .unwrap()
        .split_terminator("\n")
        .map(|x| x.parse::<u16>().unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::{_solve_part_1, _solve_part_2, read_sonar_measurements};

    #[test]
    fn part_one() {
        let test_input = read_sonar_measurements("input_test.txt");
        assert_eq!(_solve_part_1(&test_input), 7);
    }

    #[test]
    fn part_two() {
        let test_input = read_sonar_measurements("input_test.txt");
        assert_eq!(_solve_part_2(&test_input), 5);
    }
}
