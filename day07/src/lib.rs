use std::fs;

pub fn solve_part_1() {
    let positions = read_positions("day07/input.txt");
    println!("Result: {}", _solve_part_1(&positions));
}
pub fn solve_part_2() {
    let positions = read_positions("day07/input.txt");
    println!("Result: {}", _solve_part_2(&positions));
}

fn median(numbers: &Vec<usize>) -> usize {
    let mut _numbers: Vec<usize> = numbers.to_vec();
    _numbers.sort();
    if _numbers.len() % 2 == 0 {
        (_numbers[_numbers.len() / 2] + _numbers[_numbers.len() / 2 - 1]) / 2
    } else {
        _numbers[_numbers.len() / 2]
    }
}

fn average(numbers: &Vec<usize>) -> usize {
    numbers.iter().sum::<usize>() / numbers.len()
}

fn csum(number: usize) -> usize {
    (1..=number).sum()
}

fn calc_fuel(numbers: &Vec<usize>, shift: usize) -> usize {
    numbers
        .iter()
        .map(|x| (*x as isize - shift as isize).abs())
        .sum::<isize>() as usize
}

fn calc_fuel_v2(numbers: &Vec<usize>, shift: usize) -> usize {
    numbers
        .iter()
        .map(|x| csum((*x as isize - shift as isize).abs() as usize))
        .sum::<usize>()
}

fn _solve_part_1(positions: &Vec<usize>) -> usize {
    let _median = median(positions);
    calc_fuel(positions, _median)
}

fn _solve_part_2(positions: &Vec<usize>) -> usize {
    let _average = average(positions);
    let fuels = vec![
        calc_fuel_v2(positions, _average - 1),
        calc_fuel_v2(positions, _average),
        calc_fuel_v2(positions, _average + 1),
    ];
    *fuels.iter().min().unwrap()
}

fn read_positions(filename: &str) -> Vec<usize> {
    fs::read_to_string(filename)
        .unwrap()
        .split_terminator(",")
        .map(|x| x.trim().parse::<usize>().unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::{_solve_part_1, _solve_part_2, csum, median, read_positions};
    #[test]
    fn test_median() {
        assert_eq!(median(&vec![2, 1, 3]), 2);
        assert_eq!(median(&vec![3, 1, 1, 3]), 2);
    }
    #[test]
    fn test_part_1() {
        assert_eq!(_solve_part_1(&read_positions("input_test.txt")), 37);
    }
    #[test]
    fn test_csum() {
        assert_eq!(csum(3), 6);
        assert_eq!(csum(11), 66);
    }
    #[test]
    fn test_part_2() {
        assert_eq!(_solve_part_2(&read_positions("input_test.txt")), 168);
    }
}
