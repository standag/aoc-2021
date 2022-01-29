#![allow(dead_code)]

use std::{
    fmt::{Display, Formatter, Result},
    ops::Add,
};

#[derive(Debug, PartialEq, Clone)]
enum Tree {
    Number(u8),
    Node(Vec<Tree>),
}

#[derive(Debug, PartialEq, Clone)]
struct SnailfishNumber(Tree);

impl Display for Tree {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Tree::Number(number) => write!(f, "{}", number),
            Tree::Node(numbers) => write!(
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

impl Tree {
    fn from_string(string: &String) -> Self {
        let inner = string[1..string.len() - 1].to_string();
        let (left, right) = split(&inner);
        let left_number = left.parse::<u8>();
        let right_number = right.parse::<u8>();
        if left_number.is_ok() && right_number.is_ok() {
            Self::Node(vec![
                Self::Number(left_number.unwrap()),
                Self::Number(right_number.unwrap()),
            ])
        } else if left_number.is_ok() {
            Self::Node(vec![
                Self::Number(left_number.unwrap()),
                Self::from_string(&right),
            ])
        } else if right_number.is_ok() {
            Self::Node(vec![
                Self::from_string(&left),
                Self::Number(right_number.unwrap()),
            ])
        } else {
            Self::Node(vec![Self::from_string(&left), Self::from_string(&right)])
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
        if number == new {
            None
        } else {
            println!("Explode >> Old: {}\n           New: {}", number, new);
            Some(new)
        }
    }

    fn update_left(&self, increment: u8) -> (Self, u8) {
        let mut increment = increment;
        if increment == 0 {
            return (self.clone(), 0);
        }
        match self {
            Tree::Node(outer_node) => {
                let left = match outer_node[0] {
                    Tree::Node(_) => {
                        let (new_number, new_increment) = outer_node[0].update_left(increment);
                        increment = new_increment;
                        new_number
                    }
                    Tree::Number(number) => {
                        let tmp = increment;
                        increment = 0;
                        Tree::Number(number + tmp)
                    }
                };
                let right = match outer_node[1] {
                    Tree::Node(_) => {
                        let (new_number, new_increment) = outer_node[1].update_left(increment);
                        increment = new_increment;
                        new_number
                    }
                    Tree::Number(_) => outer_node[1].clone(),
                };
                (Tree::Node(vec![left, right]), increment)
            }
            Tree::Number(number) => (Tree::Number(number + increment), 0),
        }
    }
    fn update_right(&self, increment: u8) -> (Self, u8) {
        let mut increment = increment;
        if increment == 0 {
            return (self.clone(), 0);
        }
        match self {
            Tree::Node(outer_node) => {
                let right = match outer_node[1] {
                    Tree::Node(_) => {
                        let (new_number, new_increment) = outer_node[1].update_right(increment);
                        increment = new_increment;
                        new_number
                    }
                    Tree::Number(number) => {
                        let tmp = increment;
                        increment = 0;
                        Tree::Number(number + tmp)
                    }
                };
                let left = match outer_node[0] {
                    Tree::Node(_) => {
                        let (new_number, new_increment) = outer_node[0].update_right(increment);
                        increment = new_increment;
                        new_number
                    }
                    Tree::Number(_) => outer_node[0].clone(),
                };
                (Tree::Node(vec![left, right]), increment)
            }
            Tree::Number(_) => (self.clone(), increment),
        }
    }
    fn _explode(
        &self,
        depth: u8,
        exploded: bool,
        left_inc: u8,
        right_inc: u8,
    ) -> (Self, u8, u8, bool) {
        let mut right_inc = right_inc;
        let mut left_inc = left_inc;
        let mut exploded = exploded;
        match self {
            Tree::Node(outer_node) => {
                let left = &outer_node[0];
                let right = &outer_node[1];
                match (left, right) {
                    (Tree::Number(left_number), Tree::Number(right_number)) => {
                        if depth >= 4 && !exploded {
                            (Tree::Number(0), *left_number, *right_number, true)
                        } else {
                            (self.clone(), left_inc, right_inc, false)
                        }
                    }
                    _ => {
                        let mut new_left = None;
                        let mut new_right = None;
                        if exploded {
                            if right_inc > 0 {
                                let (_right, _inc) = right.update_right(right_inc);
                                new_right = Some(_right);
                                right_inc = _inc;
                            }
                            if left_inc > 0 {
                                let (_left, _inc) = left.update_left(left_inc);
                                new_left = Some(_left);
                                left_inc = _inc;
                            }
                        } else {
                            let mut left_exploded = false;
                            let mut right_exploded = false;
                            if let Tree::Node(_) = &left {
                                let (a, b, c, d) =
                                    left.clone()
                                        ._explode(depth + 1, exploded, left_inc, right_inc);
                                if d {
                                    exploded = true;
                                    left_exploded = true;
                                }
                                left_inc += b;
                                right_inc += c; // -> 8
                                new_left = Some(a);
                            }
                            if left_exploded {
                                if right_inc == 0 {
                                    new_right = Some(right.clone());
                                } else {
                                    // right->left
                                    let (_right, new_inc) = right.update_left(right_inc);
                                    new_right = Some(_right);
                                    right_inc = new_inc;
                                }
                            } else {
                                if let Tree::Node(_) = right {
                                    let (a, b, c, d) = right.clone()._explode(
                                        depth + 1,
                                        exploded,
                                        left_inc,
                                        right_inc,
                                    );
                                    if d {
                                        exploded = true;
                                        right_exploded = true;
                                    }
                                    left_inc += b; // -> 3
                                    right_inc += c; // -> 2
                                    new_right = Some(a); // --> InnerNumber(0)
                                }
                                if right_exploded {
                                    // right increment isn't affected
                                    if left_inc != 0 {
                                        let (_left, new_inc) = left.update_left(left_inc);
                                        new_left = Some(_left);
                                        left_inc = new_inc;
                                        if left_inc != 0 {
                                            println!("** left --> right");
                                            let (_left, new_inc) = left.update_right(left_inc);
                                            new_left = Some(_left);
                                            left_inc = new_inc;
                                        }
                                    }
                                }
                            }
                        }
                        let new = Tree::Node(vec![
                            new_left.unwrap_or(left.clone()),
                            new_right.unwrap_or(right.clone()),
                        ]);
                        (new, left_inc, right_inc, exploded)
                    }
                }
            }
            _ => panic!(),
        }
    }
    fn split(&self) -> Option<Self> {
        let number = self.clone();
        let new = self._split();
        if number == new {
            None
        } else {
            println!("Split >> Old: {} New: {}", number, new);
            Some(new)
        }
    }
    fn _split(&self) -> Self {
        match self {
            Tree::Node(outer_node) => {
                let left = outer_node[0]._split();
                let right = if left != outer_node[0] {
                    outer_node[1].clone()
                } else {
                    outer_node[1]._split()
                };
                Tree::Node(vec![left, right])
            }
            Tree::Number(number) => {
                if number > &9 {
                    let left = number / 2;
                    let right = number - left;
                    let num_left = Tree::Number(left);
                    let num_right = Tree::Number(right);
                    Tree::Node(vec![num_left, num_right])
                } else {
                    self.clone()
                }
            }
        }
    }
}

impl Add for SnailfishNumber {
    type Output = Self;
    fn add(self, x: Self) -> Self {
        let result = Self(Tree::Node(vec![self.0, x.0]));
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
        Self(Tree::from_string(string))
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

fn sum(numbers: &Vec<SnailfishNumber>) -> SnailfishNumber {
    numbers[1..]
        .iter()
        .fold(numbers[0].clone(), |acc, n| acc + n.clone())
}

fn partial_sum(numbers: &Vec<SnailfishNumber>, take: usize) -> SnailfishNumber {
    let numbers = numbers.clone()[..take].to_vec();
    sum(&numbers)
}

#[test]
fn test_sum() {
    let num1 = SnailfishNumber(Tree::Node(vec![Tree::Number(1), Tree::Number(2)]));
    let num2 = SnailfishNumber(Tree::Node(vec![
        Tree::Node(vec![Tree::Number(1), Tree::Number(2)]),
        Tree::Number(3),
    ]));
    let _ = num1 + num2;
}
#[test]
fn test_simple_sum() {
    let num1 = SnailfishNumber::from_string(&"[1,2]".to_string());
    let num2 = SnailfishNumber::from_string(&"[[3,4],5]".to_string());
    let result = SnailfishNumber::from_string(&"[[1,2],[[3,4],5]]".to_string());
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
    let result = SnailfishNumber(Tree::Node(vec![
        Tree::Node(vec![Tree::Number(1), Tree::Number(2)]),
        Tree::Node(vec![
            Tree::Node(vec![Tree::Number(3), Tree::Number(4)]),
            Tree::Number(5),
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
fn test_explode_left_to_right() {
    let num =
        SnailfishNumber::from_string(&"[[[[4,0],[5,0]],[[[4,5],2],[9,5]]],[7,[3,7]]]".to_string());
    assert_eq!(
        format!("{}", num.reduce()),
        "[[[[4,0],[5,4]],[[0,7],[9,5]]],[7,[3,7]]]"
    );
}

#[test]
fn test_update_left() {
    assert_eq!(Tree::Number(1).update_left(8), (Tree::Number(9), 0));
    assert_eq!(
        Tree::from_string(&"[[1,2],3]".to_string()).update_left(5),
        (Tree::from_string(&"[[6,2],3]".to_string()), 0)
    )
}

#[test]
fn test_sum_with_explode() {
    let nums = vec![
        SnailfishNumber::from_string(&"[1,1]".to_string()),
        SnailfishNumber::from_string(&"[2,2]".to_string()),
        SnailfishNumber::from_string(&"[3,3]".to_string()),
        SnailfishNumber::from_string(&"[4,4]".to_string()),
        SnailfishNumber::from_string(&"[5,5]".to_string()),
        SnailfishNumber::from_string(&"[6,6]".to_string()),
    ];
    let mut result = nums[1..=4]
        .iter()
        .fold(nums[0].clone(), |acc, n| acc + n.clone());
    assert_eq!(
        format!("{}", partial_sum(&nums, 5)),
        "[[[[3,0],[5,3]],[4,4]],[5,5]]"
    );
    result = nums[1..]
        .iter()
        .fold(nums[0].clone(), |acc, n| acc + n.clone());
    assert_eq!(format!("{}", result), "[[[[5,0],[7,4]],[5,5]],[6,6]]");
}

#[test]
fn test_number_split() {
    let num = SnailfishNumber::from_string(&"[11,0]".to_string());
    assert_eq!(
        num.reduce(),
        SnailfishNumber::from_string(&"[[5,6],0]".to_string())
    )
}

#[test]
fn test_reduce() {
    let num1 = SnailfishNumber::from_string(&"[[[[4,3],4],4],[7,[[8,4],9]]]".to_string());
    let num2 = SnailfishNumber::from_string(&"[1,1]".to_string());
    assert_eq!(
        num1 + num2,
        SnailfishNumber::from_string(&"[[[[0,7],4],[[7,8],[6,0]]],[8,1]]".to_string())
    );
}

#[test]
fn test_complex_sum() {
    let raw_numbers = "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]
[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]
[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]
[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]
[7,[5,[[3,8],[1,4]]]]
[[2,[2,2]],[8,[8,1]]]
[2,9]
[1,[[[9,3],9],[[9,0],[0,7]]]]
[[[5,[7,4]],7],1]
[[[[4,2],2],6],[8,7]]";
    let numbers = raw_numbers
        .split_whitespace()
        .map(|n| SnailfishNumber::from_string(&n.to_string()).reduce())
        .collect::<Vec<SnailfishNumber>>();
    assert_eq!(
        format!("{}", partial_sum(&numbers, 2)),
        "[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]"
    );
    assert_eq!(
        sum(&numbers[..3].to_vec()),
        SnailfishNumber::from_string(
            &"[[[[6,7],[6,7]],[[7,7],[0,7]]],[[[8,7],[7,7]],[[8,8],[8,0]]]]".to_string()
        )
    );

    assert_eq!(
        sum(&numbers),
        SnailfishNumber::from_string(
            &"[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]".to_string()
        )
    );
}
