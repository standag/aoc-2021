use std::collections::HashMap;
use std::fs;

pub fn solve_part_1() {
    let codes = read_codes("day08/input.txt");
    println!("Result: {}", _solve_part_1(&codes))
}

pub fn solve_part_2() {
    let codes = read_codes("day08/input.txt");
    println!("Result: {}", _solve_part_2(&codes))
}

fn read_codes(filename: &str) -> Vec<Vec<String>> {
    fs::read_to_string(filename)
        .unwrap()
        .split_terminator("\n")
        .map(|x| x.split_whitespace().map(|y| y.to_string()).collect())
        .collect()
}

fn _solve_part_1(codes: &Vec<Vec<String>>) -> usize {
    let mut counter = 0;
    for code_line in codes {
        let delimiter_position = code_line.iter().position(|x| x == "|").unwrap();
        counter += code_line[delimiter_position + 1..]
            .iter()
            .filter(|x| x.len() == 2 || x.len() == 3 || x.len() == 4 || x.len() == 7)
            .count();
    }
    counter
}

fn consists(pattern: &String, from: &String) -> bool {
    for x in from.chars() {
        if !pattern.contains(x) {
            return false;
        }
    }
    true
}

fn sort_key(key: &String) -> String {
    let mut chars = key.chars().collect::<Vec<char>>();
    chars.sort();
    String::from_iter(chars)
}

fn detect_numbers(numbers: &Vec<String>) -> HashMap<String, String> {
    let mut decrypted = HashMap::new();
    let one = numbers.iter().find(|x| x.len() == 2).unwrap();
    let four = numbers.iter().find(|x| x.len() == 4).unwrap();
    let seven = numbers.iter().find(|x| x.len() == 3).unwrap();
    let eight = numbers.iter().find(|x| x.len() == 7).unwrap();
    let zero = numbers
        .iter()
        .find(|x| x.len() == 6 && consists(x, one) && !consists(x, four))
        .unwrap();
    let nine = numbers
        .iter()
        .find(|x| x.len() == 6 && consists(x, one) && consists(x, four))
        .unwrap();
    let six = numbers
        .iter()
        .find(|x| x.len() == 6 && !consists(x, one))
        .unwrap();
    let three = numbers
        .iter()
        .find(|x| x.len() == 5 && consists(x, one))
        .unwrap();
    let five = numbers
        .iter()
        .find(|x| x.len() == 5 && consists(six, x))
        .unwrap();
    let two = numbers
        .iter()
        .find(|x| x.len() == 5 && !consists(x, one) && !consists(six, x))
        .unwrap();

    decrypted.insert(sort_key(zero), "0".to_string());
    decrypted.insert(sort_key(one), "1".to_string());
    decrypted.insert(sort_key(two), "2".to_string());
    decrypted.insert(sort_key(three), "3".to_string());
    decrypted.insert(sort_key(four), "4".to_string());
    decrypted.insert(sort_key(five), "5".to_string());
    decrypted.insert(sort_key(six), "6".to_string());
    decrypted.insert(sort_key(seven), "7".to_string());
    decrypted.insert(sort_key(eight), "8".to_string());
    decrypted.insert(sort_key(nine), "9".to_string());
    println!("{:?}", decrypted);

    decrypted
}

fn _solve_part_2(codes: &Vec<Vec<String>>) -> usize {
    let mut sum = 0;
    for code_line in codes {
        println!("{:?}", code_line);
        let delimiter_position = code_line.iter().position(|x| x == "|").unwrap();
        let decrypted = detect_numbers(&code_line[..delimiter_position].to_vec());
        let number: Vec<String> = code_line[delimiter_position + 1..]
            .iter()
            .map(|x| decrypted.get(&sort_key(x)).unwrap().to_string())
            .collect();
        sum += String::from_iter(number).parse::<usize>().unwrap();
    }

    sum
}

#[cfg(test)]
mod tests {
    use crate::{_solve_part_1, _solve_part_2, read_codes};
    #[test]
    fn test_part_1() {
        let codes = read_codes("input_test.txt");
        assert_eq!(_solve_part_1(&codes), 26);
    }
    #[test]
    fn test_part_2() {
        let codes = read_codes("input_test.txt");
        assert_eq!(_solve_part_2(&codes), 61229);
    }
}
