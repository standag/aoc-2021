use std::fs;

pub fn solve_part_1() {
    let report = read_report("day03/input.txt");
    println!("Result: {}", _solve_part_1(&report))
}
pub fn solve_part_2() {
    let report = read_report("day03/input.txt");
    println!("Result: {}", _solve_part_2(&report))
}

fn bits_to_number(bits: &Vec<Bit>) -> usize {
    let mut number = 0;
    let mut _bits = bits.to_vec();
    _bits.reverse();
    for i in 0..bits.len() {
        if _bits[i] == Bit::One {
            number += 2_usize.pow(i as u32);
        }
    }
    number
}

fn most(vbits: &Vec<Vec<Bit>>, position: usize) -> Bit {
    let size = vbits.len() as f32;
    let x = vbits
        .into_iter()
        .filter(|x| x[position] == Bit::One)
        .count() as f32;
    if x >= size / 2.0 {
        Bit::One
    } else {
        Bit::Zero
    }
}

fn revert_bit(bit: &Bit) -> Bit {
    match bit {
        Bit::Zero => Bit::One,
        Bit::One => Bit::Zero,
    }
}

fn _solve_part_1(report: &Vec<Vec<Bit>>) -> usize {
    let mut bits = Vec::new();
    for position in 0..report[0].len() {
        bits.push(most(report, position));
    }
    println!("{:?}", bits);
    bits_to_number(&bits) * bits_to_number(&bits.iter().map(|x| revert_bit(x)).collect())
}

fn _solve_part_2(report: &Vec<Vec<Bit>>) -> usize {
    let mut vbits_o2 = report.to_vec();
    let mut vbits_co2 = report.to_vec();

    for position in 0..report[0].len() {
        let _most = most(&vbits_o2, position);
        let _less = revert_bit(&most(&vbits_co2, position));
        if vbits_o2.len() > 1 {
            vbits_o2 = vbits_o2
                .iter()
                .filter(|x| x[position] == _most)
                .map(|x| x.to_vec())
                .collect();
        }
        if vbits_co2.len() > 1 {
            vbits_co2 = vbits_co2
                .iter()
                .filter(|x| x[position] == _less)
                .map(|x| x.to_vec())
                .collect();
        }
    }
    bits_to_number(&vbits_o2[0]) * bits_to_number(&vbits_co2[0])
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Bit {
    One,
    Zero,
}

fn read_report(filename: &str) -> Vec<Vec<Bit>> {
    fs::read_to_string(filename)
        .unwrap()
        .split_whitespace()
        .map(|x| {
            x.chars()
                .map(|y| match y {
                    '0' => Bit::Zero,
                    '1' => Bit::One,
                    _ => panic!(),
                })
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::{_solve_part_1, _solve_part_2, read_report};
    #[test]
    fn test_part_one() {
        assert_eq!(_solve_part_1(&read_report(&"input_test.txt")), 198);
    }
    #[test]
    fn test_part_two() {
        assert_eq!(_solve_part_2(&read_report(&"input_test.txt")), 230);
    }
}
