#![allow(dead_code)]

use std::{
    fmt::{Display, Formatter, Result},
    ops::Add,
};

#[derive(Debug, PartialEq, Clone)]
enum Number {
    InnerNumber(u8),
    InnerNumbers(Vec<u8>),
    OuterNumbers(Vec<Number>),
}

#[derive(Debug, PartialEq, Clone)]
struct SnailfishNumber(Number);

impl Display for Number {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Number::InnerNumber(number) => write!(f, "{}", number),
            Number::InnerNumbers(numbers) => write!(
                f,
                "[{}]",
                numbers
                    .iter()
                    .fold(String::new(), |acc, n| if acc.is_empty() {
                        format!("{}", n)
                    } else {
                        format!("{},{}", acc, n)
                    })
            ),
            Number::OuterNumbers(numbers) => write!(
                f,
                "[{}]",
                numbers
                    .iter()
                    .fold(String::new(), |acc, n| if acc.is_empty() {
                        format!("{}", n)
                    } else {
                        format!("{},{}", acc, n)
                    })
            ),
        }
    }
}

impl Number {
    fn from_string(string: &String) -> Self {
        let inner = string[1..string.len() - 1].to_string();
        let (left, right) = split(&inner);
        let left_number = left.parse::<u8>();
        let right_number = right.parse::<u8>();
        if left_number.is_ok() && right_number.is_ok() {
            Self::InnerNumbers(vec![left_number.unwrap(), right_number.unwrap()])
        } else if left_number.is_ok() {
            Self::OuterNumbers(vec![
                Self::InnerNumber(left_number.unwrap()),
                Self::from_string(&right),
            ])
        } else if right_number.is_ok() {
            Self::OuterNumbers(vec![
                Self::from_string(&left),
                Self::InnerNumber(right_number.unwrap()),
            ])
        } else {
            Self::OuterNumbers(vec![Self::from_string(&left), Self::from_string(&right)])
        }
    }
    fn reduce(&self) -> Self {
        let mut result = self.clone();
        loop {
            let after_explode = result.explode();
            if after_explode.is_some() {
                result = after_explode.unwrap();
                continue;
            }
            let after_split = result.split();
            if after_split.is_some() {
                result = after_split.unwrap();
                continue;
            }
            break result;
        }
    }
    fn explode(&self) -> Option<Self> {
        let number = self.clone();
        let new = self._explode(0, false, 0, 0).0;
        println!(">> Old: {} New: {}", number, new);
        if number == new {
            None
        } else {
            Some(new)
        }
    }
    fn update_left(&self, increment: u8) -> (Self, u8) {
        let mut increment = increment;
        if increment == 0 {
            return (self.clone(), 0);
        }
        match self {
            Number::OuterNumbers(outer_number) => {
                let left = match outer_number[0] {
                    Number::OuterNumbers(_) | Number::InnerNumbers(_) => {
                        let (new_number, new_increment) = outer_number[0].update_left(increment);
                        increment = new_increment;
                        new_number
                    }
                    Number::InnerNumber(number) => {
                        let tmp = increment;
                        increment = 0;
                        Number::InnerNumber(number + tmp)
                    }
                };
                let right = match outer_number[1] {
                    Number::OuterNumbers(_) | Number::InnerNumbers(_) => {
                        let (new_number, new_increment) = outer_number[1].update_left(increment);
                        increment = new_increment;
                        new_number
                    }
                    Number::InnerNumber(_) => outer_number[1].clone(),
                };
                (Number::OuterNumbers(vec![left, right]), increment)
            }
            Number::InnerNumbers(inner_numbers) => (
                Number::InnerNumbers(vec![inner_numbers[0] + increment, inner_numbers[1]]),
                0,
            ),
            Number::InnerNumber(number) => (Number::InnerNumber(number + increment), 0),
        }
    }
    fn update_right(&self, increment: u8) -> (Self, u8) {
        let mut increment = increment;
        if increment == 0 {
            return (self.clone(), 0);
        }
        match self {
            Number::OuterNumbers(outer_number) => {
                let right = match outer_number[1] {
                    Number::OuterNumbers(_) | Number::InnerNumbers(_) => {
                        let (new_number, new_increment) = outer_number[1].update_right(increment);
                        increment = new_increment;
                        new_number
                    }
                    Number::InnerNumber(number) => {
                        let tmp = increment;
                        increment = 0;
                        Number::InnerNumber(number + tmp)
                    }
                };
                let left = match outer_number[0] {
                    Number::OuterNumbers(_) | Number::InnerNumbers(_) => {
                        let (new_number, new_increment) = outer_number[0].update_right(increment);
                        increment = new_increment;
                        new_number
                    }
                    Number::InnerNumber(_) => outer_number[0].clone(),
                };
                (Number::OuterNumbers(vec![left, right]), increment)
            }
            Number::InnerNumbers(inner_numbers) => (
                Number::InnerNumbers(vec![inner_numbers[0] + increment, inner_numbers[1]]),
                0,
            ),
            Number::InnerNumber(_) => (self.clone(), increment),
        }
    }
    fn _explode(
        &self,
        depth: u8,
        exploded: bool,
        left_inc: u8,
        right_inc: u8,
    ) -> (Self, u8, u8, bool) {
        println!(
            "Processing explosion for: {}, depth: {}, increase: ({},{})",
            self, depth, left_inc, right_inc
        );
        let mut right_inc = right_inc;
        let mut left_inc = left_inc;
        let mut exploded = exploded;
        if let Number::InnerNumbers(numbers) = self {
            if depth >= 4 && !exploded {
                println!("Exploding.. {}", self);
                (Number::InnerNumber(0), numbers[0], numbers[1], true)
            } else {
                (self.clone(), left_inc, right_inc, false)
            }
        } else {
            let mut right = None;
            let mut left = None;
            let mut left_exploded = false;
            let mut right_exploded = false;
            if let Number::OuterNumbers(number) = self {
                println!("OuterNumber: {}", self);
                if let Number::OuterNumbers(_) | Number::InnerNumbers(_) = &number[0] {
                    println!("Left part: {}", number[0]);
                    let (a, b, c, d) =
                        number[0]
                            .clone()
                            ._explode(depth + 1, exploded, left_inc, right_inc);
                    if d {
                        exploded = true;
                        left_exploded = true;
                        println!("Left part exploded");
                    }
                    left_inc += b;
                    right_inc += c;
                    left = Some(a);
                }
                if left_exploded {
                    if right_inc == 0 {
                        right = Some(number[1].clone())
                    } else {
                        println!("Right increment to process: {}", right_inc);
                        let (new_right, new_inc) = number[1].update_left(right_inc);
                        right = Some(new_right);
                        right_inc = new_inc;
                    }
                } else {
                    if let Number::OuterNumbers(_) | Number::InnerNumbers(_) = &number[1] {
                        let (a, b, c, d) =
                            number[1]
                                .clone()
                                ._explode(depth + 1, exploded, left_inc, right_inc);
                        if d {
                            exploded = true;
                            right_exploded = true;
                        }
                        left_inc += b;
                        right_inc += c;
                        right = Some(a);
                    }
                    if right_exploded {
                        if left_inc != 0 {
                            println!("Left increment to process: {}", right_inc);
                            let (new_left, new_inc) = number[1].update_right(left_inc);
                            left = Some(new_left);
                            left_inc = new_inc;
                        }
                    }
                }
                if let Number::InnerNumber(left_number) = number[0] {
                    left = Some(Number::InnerNumber(left_number + left_inc));
                    left_inc = 0;
                };

                if let Number::InnerNumber(right_number) = number[1] {
                    right = Some(Number::InnerNumber(right_number + right_inc));
                    right_inc = 0;
                };
                let new = Number::OuterNumbers(vec![left.unwrap(), right.unwrap()]);
                println!("transform: {} to {}", self, new);
                (new, left_inc, right_inc, exploded)
            } else {
                panic!()
            }
        }
    }
    fn split(&self) -> Option<Self> {
        None
    }
}

impl Add for SnailfishNumber {
    type Output = Self;
    fn add(self, x: Self) -> Self {
        let result = Self(Number::OuterNumbers(vec![self.0, x.0]));
        result.reduce()
    }
}

impl Display for SnailfishNumber {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.0)
    }
}

impl SnailfishNumber {
    fn from_string(string: &String) -> Self {
        Self(Number::from_string(string))
    }

    fn reduce(&self) -> Self {
        Self(self.0.reduce())
    }
}

fn split(string: &String) -> (String, String) {
    let mut index = 0;
    let mut counter = 0;
    for (i, ch) in string.chars().enumerate() {
        if ch == '[' {
            counter += 1;
        }
        if ch == ']' {
            counter -= 1;
        }
        if ch == ',' && counter == 0 {
            index = i
        }
    }
    (string[..index].to_string(), string[index + 1..].to_string())
}

#[test]
fn test_sum() {
    let num1 = SnailfishNumber(Number::InnerNumbers(vec![1, 2]));
    let num2 = SnailfishNumber(Number::OuterNumbers(vec![
        Number::InnerNumbers(vec![1, 2]),
        Number::InnerNumber(3),
    ]));
    let _ = num1 + num2;
}

#[test]
fn test_simple_sum() {
    let num1 = SnailfishNumber(Number::InnerNumbers(vec![1, 2]));
    let num2 = SnailfishNumber(Number::OuterNumbers(vec![
        Number::InnerNumbers(vec![3, 4]),
        Number::InnerNumber(5),
    ]));
    let result = SnailfishNumber(Number::OuterNumbers(vec![
        Number::InnerNumbers(vec![1, 2]),
        Number::OuterNumbers(vec![
            Number::InnerNumbers(vec![3, 4]),
            Number::InnerNumber(5),
        ]),
    ]));
    println!("{}", result);
    assert_eq!(num1 + num2, result)
}

#[test]
fn test_split() {
    assert_eq!(
        split(&"[1,3],5".to_string()),
        ("[1,3]".to_string(), "5".to_string())
    );
    assert_eq!(
        split(&"[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]".to_string()),
        ("[3,[2,[8,0]]]".to_string(), "[9,[5,[4,[3,2]]]]".to_string())
    );
}

#[test]
fn test_from_string() {
    //let num = SnailfishNumber::from_string(&"[[[[[9,8],1],2],3],4]".to_string());
    let num = SnailfishNumber::from_string(&"[[1,2],[[3,4],5]]".to_string());
    let result = SnailfishNumber(Number::OuterNumbers(vec![
        Number::InnerNumbers(vec![1, 2]),
        Number::OuterNumbers(vec![
            Number::InnerNumbers(vec![3, 4]),
            Number::InnerNumber(5),
        ]),
    ]));
    assert_eq!(num, result);
}

#[test]
fn test_sum_simple_nums() {
    let nums = vec![
        SnailfishNumber::from_string(&"[1,1]".to_string()),
        SnailfishNumber::from_string(&"[2,2]".to_string()),
        SnailfishNumber::from_string(&"[3,3]".to_string()),
        SnailfishNumber::from_string(&"[4,4]".to_string()),
    ];
    let result = nums[1..]
        .iter()
        .fold(nums[0].clone(), |acc, n| acc + n.clone());
    assert_eq!(format!("{}", result), "[[[[1,1],[2,2]],[3,3]],[4,4]]");
}

#[test]
fn test_reduce_with_explode() {
    let mut num = SnailfishNumber::from_string(&"[[[[[9,8],1],2],3],4]".to_string());
    let mut result = num.reduce();
    assert_eq!(format!("{}", result), "[[[[0,9],2],3],4]".to_string());
    num = SnailfishNumber::from_string(&"[7,[6,[5,[4,[3,2]]]]]".to_string());
    result = num.reduce();
    assert_eq!(format!("{}", result), "[7,[6,[5,[7,0]]]]".to_string());
    num = SnailfishNumber::from_string(&"[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]".to_string());
    result = num.reduce();
    assert_eq!(
        format!("{}", result),
        "[[3,[2,[8,0]]],[9,[5,[7,0]]]]".to_string()
    );
}

#[test]
fn test_sum_with_explode() {
    let nums = vec![
        SnailfishNumber::from_string(&"[1,1]".to_string()),
        SnailfishNumber::from_string(&"[2,2]".to_string()),
        SnailfishNumber::from_string(&"[3,3]".to_string()),
        SnailfishNumber::from_string(&"[4,4]".to_string()),
        SnailfishNumber::from_string(&"[5,5]".to_string()),
    ];
    let result = nums[1..]
        .iter()
        .fold(nums[0].clone(), |acc, n| acc + n.clone());
    assert_eq!(format!("{}", result), "[[[[3,0],[5,3]],[4,4]],[5,5]]");
}
