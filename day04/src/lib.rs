use std::fs;

pub fn solve_part_1() {
    let (numbers, cards) = read_input("day04/input.txt");
    println!("Result: {}", _solve_part_1(&numbers, &cards))
}
pub fn solve_part_2() {
    let (numbers, cards) = read_input("day04/input.txt");
    println!("Result: {}", _solve_part_2(&numbers, &cards))
}

#[derive(Debug, Clone)]
struct BingoCard {
    rows: Vec<Vec<u8>>,
    columns: Vec<Vec<u8>>,
    all_numbers: Vec<u8>,
}

impl BingoCard {
    fn new(numbers: &Vec<Vec<u8>>) -> Self {
        let mut _columns = vec![];
        for position in 0..5 {
            _columns.push(
                numbers
                    .iter()
                    .filter_map(|x| {
                        x.iter()
                            .enumerate()
                            .filter(|(i, _y)| *i == position)
                            .map(|(_i, y)| Some(*y))
                            .collect::<Option<Vec<u8>>>()
                    })
                    .flatten()
                    .collect(),
            );
        }
        BingoCard {
            rows: numbers.to_vec(),
            columns: _columns,
            all_numbers: numbers.iter().flatten().map(|x| *x).collect(),
        }
    }
    fn check_numbers(&self, numbers: &Vec<u8>) -> bool {
        self.rows
            .iter()
            .chain(self.columns.iter())
            .any(|x| x.iter().all(|y| numbers.contains(y)))
    }

    fn missed_numbers(&self, numbers: &Vec<u8>) -> Vec<u8> {
        self.all_numbers
            .iter()
            .filter(|x| !numbers.contains(x))
            .map(|x| *x)
            .collect()
    }
}

fn _solve_part_1(numbers: &Vec<u8>, cards: &Vec<BingoCard>) -> usize {
    let winner: &BingoCard;
    let mut number: u8 = 0;
    let mut missed_numbers: Vec<u8> = vec![];
    for i in 5..numbers.len() {
        let _numbers = &numbers[..i];
        let winners: Vec<&BingoCard> = cards
            .iter()
            .filter(|x| x.check_numbers(&_numbers.to_vec()))
            .collect();
        if !winners.is_empty() {
            winner = winners[0];
            number = *_numbers.last().unwrap();
            missed_numbers = winner.missed_numbers(&_numbers.to_vec());
            break;
        }
    }
    missed_numbers.iter().map(|x| *x as usize).sum::<usize>() * number as usize
}
fn _solve_part_2(numbers: &Vec<u8>, cards: &Vec<BingoCard>) -> usize {
    let mut number: u8 = 0;

    let mut last_card: Option<BingoCard> = None;
    let mut missed_numbers: Vec<u8> = vec![];
    let mut _cards = cards.to_vec();
    let mut i = 5;
    loop {
        let _numbers = &numbers[..i];
        _cards.retain(|x| !x.check_numbers(&_numbers.to_vec()));
        if _cards.len() == 1 {
            last_card = Some(_cards[0].clone());
        }
        if _cards.len() == 0 {
            if let Some(_) = last_card {
                missed_numbers = last_card.unwrap().missed_numbers(&_numbers.to_vec());
            }
            number = *_numbers.last().unwrap();
            break;
        }
        i += 1;
    }

    missed_numbers.iter().map(|x| *x as usize).sum::<usize>() * number as usize
}

fn read_input(filename: &str) -> (Vec<u8>, Vec<BingoCard>) {
    let data: Vec<String> = fs::read_to_string(filename)
        .unwrap()
        .split_terminator("\n\n")
        .map(|x| x.to_string())
        .collect::<Vec<String>>();
    let numbers = data[0]
        .split_terminator(",")
        .map(|x| x.parse::<u8>().unwrap())
        .collect();
    let mut cards = vec![];
    for i in 1..data.len() {
        let x: Vec<Vec<u8>> = data[i]
            .split_terminator("\n")
            .map(|x| {
                x.split_whitespace()
                    .map(|y| y.parse::<u8>().unwrap())
                    .collect()
            })
            .collect();
        cards.push(BingoCard::new(&x));
    }
    (numbers, cards)
}

#[cfg(test)]
mod tests {
    use crate::{_solve_part_1, _solve_part_2, read_input};
    #[test]
    fn test_part_1() {
        let (numbers, cards) = read_input("input_test.txt");
        assert_eq!(_solve_part_1(&numbers, &cards), 4512);
    }

    #[test]
    fn test_part_2() {
        let (numbers, cards) = read_input("input_test.txt");
        assert_eq!(_solve_part_2(&numbers, &cards), 1924);
    }
}
