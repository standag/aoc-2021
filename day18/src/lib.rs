#![allow(dead_code)]

use std::{
    collections::VecDeque,
    fmt::{Display, Formatter, Result},
    fs,
    ops::Add,
};

use itertools::Itertools;

pub fn solve_part_1() {
    println!(
        "Day 18, part one, result: {}",
        solve_part_one(&fs::read_to_string("./day18/input.txt").unwrap())
    );
}

pub fn solve_part_2() {
    println!(
        "Day 18, part two, result: {}",
        solve_part_two(&fs::read_to_string("./day18/input.txt").unwrap())
    );
}

#[derive(Debug)]
struct SnailNumber(BTree);

#[derive(Debug, PartialEq, Clone)]
struct BTree {
    value: u8,
    left: Option<Box<BTree>>,
    right: Option<Box<BTree>>,
}

impl BTree {
    fn new(value: u8) -> Self {
        Self {
            value,
            left: None,
            right: None,
        }
    }
    fn from(snail_number: &String) -> Self {
        let inner = snail_number[1..snail_number.len() - 1].to_string();
        let (left, right) = split(&inner);
        let left_number = left.parse::<u8>();
        let right_number = right.parse::<u8>();
        if left_number.is_ok() && right_number.is_ok() {
            Self::new(0)
                .left(Self::new(left_number.unwrap()))
                .right(Self::new(right_number.unwrap()))
        } else if left_number.is_ok() {
            Self::new(0)
                .left(Self::new(left_number.unwrap()))
                .right(Self::from(&right))
        } else if right_number.is_ok() {
            Self::new(0)
                .left(Self::from(&left))
                .right(Self::new(right_number.unwrap()))
        } else {
            Self::new(0)
                .left(Self::from(&left))
                .right(Self::from(&right))
        }
    }
    fn is_leaf(&self) -> bool {
        self.right.is_none() && self.left.is_none()
    }
    fn depth(&self) -> usize {
        let left = match &self.left {
            Some(node) => 1 + node.depth(),
            _ => 0,
        };
        let right = match &self.right {
            Some(node) => 1 + node.depth(),
            _ => 0,
        };
        left.max(right)
    }
    fn left(mut self, node: BTree) -> Self {
        self.left = Some(Box::new(node));
        self.value = 0;
        self
    }
    fn right(mut self, node: BTree) -> Self {
        self.right = Some(Box::new(node));
        self.value = 0;
        self
    }
    fn flatten(&self) -> Vec<u8> {
        let mut numbers = vec![self.value];
        let mut left = match &self.left {
            Some(node) => (*node).flatten(),
            _ => vec![],
        };
        let mut right = match &self.right {
            Some(node) => (*node).flatten(),
            _ => vec![],
        };
        numbers.append(&mut left);
        numbers.append(&mut right);
        numbers
    }

    fn split(&mut self) {
        let mut queue: VecDeque<&mut BTree> = VecDeque::new();
        queue.push_front(self);
        loop {
            let BTree {
                ref mut left,
                ref mut right,
                ref mut value,
            } = queue.pop_front().unwrap();

            if *value > 9 {
                let new_left = *value / 2;
                let new_right = *value - new_left;
                *left = Some(Box::new(BTree::new(new_left)));
                *right = Some(Box::new(BTree::new(new_right)));
                *value = 0;
                return;
            }

            match right {
                Some(node) => {
                    queue.push_front(node);
                }
                None => (),
            }
            match left {
                Some(node) => {
                    queue.push_front(node);
                }
                None => (),
            }
        }
    }

    fn explode(&mut self) {
        let mut last_left_value: Option<&mut u8> = None;
        let mut queue: VecDeque<(&mut BTree, u8)> = VecDeque::new();
        queue.push_front((self, 0));
        loop {
            let node = queue.pop_front();
            if node.is_none() {
                return;
            }
            let (
                BTree {
                    ref mut left,
                    ref mut right,
                    ref mut value,
                },
                depth,
            ) = node.unwrap();

            if depth >= 4 && left.is_some() {
                let new_left = left.as_ref().unwrap().value;
                let new_right = right.as_ref().unwrap().value;
                // find right leaf element in rest of queue
                loop {
                    let node = queue.pop_front();
                    if node.is_none() {
                        break;
                    }
                    let (
                        BTree {
                            ref mut left,
                            ref mut right,
                            ref mut value,
                        },
                        _depth,
                    ) = node.unwrap();
                    if left.is_none() && right.is_none() {
                        *value += new_right;
                        break;
                    }
                    match right {
                        Some(node) => {
                            queue.push_front((node, depth + 1));
                        }
                        None => (),
                    }
                    match left {
                        Some(node) => {
                            queue.push_front((node, depth + 1));
                        }
                        None => (),
                    }
                }
                // update left element
                if last_left_value.is_some() {
                    *last_left_value.unwrap() += new_left;
                }
                // update currenly exploded subtree
                *left = None;
                *right = None;
                *value = 0;
                break; // skip adding currently exploded subtree
            }
            if left.is_none() && right.is_none() {
                last_left_value = Some(&mut *value);
            }

            match right {
                Some(node) => {
                    queue.push_front((node, depth + 1));
                }
                None => (),
            }
            match left {
                Some(node) => {
                    queue.push_front((node, depth + 1));
                }
                None => (),
            }
        }
    }
    fn reduce(&mut self) {
        loop {
            if self.depth() >= 5 {
                self.explode();
            } else if self.flatten().iter().any(|&n| n >= 10) {
                self.split();
            } else {
                return;
            }
        }
    }
    fn magnitude(&self) -> usize {
        match (self.left.as_ref(), self.right.as_ref()) {
            (Some(lnode), Some(rnode)) => 3 * lnode.magnitude() + 2 * rnode.magnitude(),
            _ => self.value as usize,
        }
    }
}

impl Add for BTree {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        let mut result = BTree::new(0).left(self).right(other);
        result.reduce();
        return result;
    }
}
impl Display for BTree {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            node if node.is_leaf() => write!(f, "{}", node.value),
            _ => write!(
                f,
                "[{},{}]",
                self.left.as_ref().unwrap(),
                self.right.as_ref().unwrap()
            ),
        }
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

fn sum(string: &String) -> BTree {
    string
        .split('\n')
        .filter(|l| l.starts_with("["))
        .map(|l| BTree::from(&l.to_string()))
        .reduce(|acc, n| acc + n)
        .unwrap()
}

fn solve_part_one(string: &String) -> usize {
    sum(string).magnitude()
}

fn solve_part_two(string: &String) -> usize {
    let numbers = string
        .split("\n")
        .filter(|l| l.starts_with("["))
        .map(|l| BTree::from(&l.to_string()));
    numbers
        .permutations(2)
        .map(|combo| combo.first().unwrap().clone() + combo.last().unwrap().clone())
        .map(|bt| bt.magnitude())
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut t = BTree::new(0);
        assert!(t.depth() == 0);
        t = t
            .left(BTree::new(1))
            .right(BTree::new(0).left(BTree::new(2)).right(BTree::new(3)));
        assert!(t.depth() == 2);
        assert!(t == BTree::from(&"[1,[2,3]]".to_string()));
    }

    #[test]
    fn test_sum() {
        let number1 = "[1,2]".to_string();
        let number2 = "[[3,4],5]".to_string();
        let result = BTree::from(&"[[1,2],[[3,4],5]]".to_string());
        assert_eq!(BTree::from(&number1) + BTree::from(&number2), result);
        assert_eq!(result.depth(), 3);
    }

    #[test]
    fn no_reduce() {
        let mut tree = BTree::from(&"[[1,2],[[3,4],5]]".to_string());
        let result = BTree::from(&"[[1,2],[[3,4],5]]".to_string());
        tree.reduce();
        assert_eq!(tree, result);
    }

    #[test]
    fn split() {
        let mut tree = BTree::from(&"[[1,2],[[3,4],11]]".to_string());
        let result = BTree::from(&"[[1,2],[[3,4],[5,6]]]".to_string());
        tree.split();
        assert_eq!(tree, result);

        //[[[[0,7],4],[15,[0,13]]],[1,1]]
        //[[[[0,7],4],[[7,8],[0,13]]],[1,1]]
        //[[[[0,7],4],[[7,8],[0,[6,7]]]],[1,1]]
        tree = BTree::from(&"[[[[0,7],4],[15,[0,13]]],[1,1]]".to_string());
        let result2 = BTree::from(&"[[[[0,7],4],[[7,8],[0,13]]],[1,1]]".to_string());
        let result3 = BTree::from(&"[[[[0,7],4],[[7,8],[0,[6,7]]]],[1,1]]".to_string());
        tree.split();
        assert_eq!(tree, result2);
        tree.split();
        assert_eq!(tree, result3);
    }

    #[test]
    fn explode() {
        // [7,[6,[5,[7,0]]]] -> [[[[0,9],2],3],4]
        let mut tree = BTree::from(&"[[[[[9,8],1],2],3],4]".to_string());
        let result = BTree::from(&"[[[[0,9],2],3],4]".to_string());
        tree.explode();
        assert_eq!(tree, result);
        // [7,[6,[5,[4,[3,2]]]]] -> [7,[6,[5,[7,0]]]]
        let mut tree = BTree::from(&"[7,[6,[5,[4,[3,2]]]]]".to_string());
        let result = BTree::from(&"[7,[6,[5,[7,0]]]]".to_string());
        tree.explode();
        assert_eq!(tree, result);
        // [[6,[5,[4,[3,2]]]],1] -> [[6,[5,[7,0]]],3]
        let mut tree = BTree::from(&"[[6,[5,[4,[3,2]]]],1]".to_string());
        let result = BTree::from(&"[[6,[5,[7,0]]],3]".to_string());
        tree.explode();
        assert_eq!(tree, result);
        // [[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]] -> [[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]
        let mut tree = BTree::from(&"[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]".to_string());
        let result = BTree::from(&"[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]".to_string());
        tree.explode();
        assert_eq!(tree, result);
        // -> [[3,[2,[8,0]]],[9,[5,[7,0]]]]
        let result = BTree::from(&"[[3,[2,[8,0]]],[9,[5,[7,0]]]]".to_string());
        tree.explode();
        assert_eq!(tree, result);
        let mut tree = BTree::from(&"[6,[[[[4,5],3],2],1]]".to_string());
        tree.explode();
        assert_eq!(tree, BTree::from(&"[10,[[[0,8],2],1]]".to_string()));
        let mut tree = BTree::from(&"[[1,[2,3]],[[[[4,5],3],2],1]]".to_string());
        tree.explode();
        assert_eq!(tree, BTree::from(&"[[1,[2,7]],[[[0,8],2],1]]".to_string()));
    }

    #[test]
    fn reduce() {
        // [[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]] -> [[[[0,7],4],[[7,8],[6,0]]],[8,1]]
        let mut tree = BTree::from(&"[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]".to_string());
        let result = BTree::from(&"[[[[0,7],4],[[7,8],[6,0]]],[8,1]]".to_string());
        tree.reduce();
        assert_eq!(tree, result);
    }

    #[test]
    fn magnitude() {
        let tree = BTree::from(&"[[9,1],[1,9]]".to_string());
        assert_eq!(tree.magnitude(), 129);
        let tree =
            BTree::from(&"[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]".to_string());
        assert_eq!(tree.magnitude(), 3488);
    }

    #[test]
    fn test_sum_sum() {
        let input = "[[[[4,3],4],4],[7,[[8,4],9]]]\n[1,1]";
        let result = BTree::from(&"[[[[0,7],4],[[7,8],[6,0]]],[8,1]]".to_string());
        assert_eq!(sum(&input.to_string()), result);
        let input = "[1,1]\n[2,2]\n[3,3]\n[4,4]";
        let result = BTree::from(&"[[[[1,1],[2,2]],[3,3]],[4,4]]".to_string());
        assert_eq!(sum(&input.to_string()), result);
        let input = "[1,1]\n[2,2]\n[3,3]\n[4,4]\n[5,5]\n[6,6]";
        let result = BTree::from(&"[[[[5,0],[7,4]],[5,5]],[6,6]]".to_string());
        assert_eq!(sum(&input.to_string()), result);
        let input = "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]\n[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]";
        let result =
            BTree::from(&"[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]".to_string());
        assert_eq!(sum(&input.to_string()), result);
        let input = "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]\n[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]\n[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]\n[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]\n[7,[5,[[3,8],[1,4]]]]\n[[2,[2,2]],[8,[8,1]]]\n[2,9]\n[1,[[[9,3],9],[[9,0],[0,7]]]]\n[[[5,[7,4]],7],1]\n[[[[4,2],2],6],[8,7]]";
        let result =
            BTree::from(&"[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]".to_string());
        assert_eq!(sum(&input.to_string()), result);

        let input = "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]\n[[[5,[2,8]],4],[5,[[9,9],0]]]\n[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]\n[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]\n[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]\n[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]\n[[[[5,4],[7,7]],8],[[8,3],8]]\n[[9,3],[[9,9],[6,[4,9]]]]\n[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]\n[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]";
        let result = BTree::from(
            &"[[[[6,6],[7,6]],[[7,7],[7,0]]],[[[7,7],[7,7]],[[7,8],[9,9]]]]".to_string(),
        );
        assert_eq!(sum(&input.to_string()), result);
        assert_eq!(solve_part_one(&input.to_string()), 4140);
    }

    #[test]
    fn test_solve_part_two() {
        let input = "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]\n[[[5,[2,8]],4],[5,[[9,9],0]]]\n[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]\n[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]\n[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]\n[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]\n[[[[5,4],[7,7]],8],[[8,3],8]]\n[[9,3],[[9,9],[6,[4,9]]]]\n[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]\n[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]";
        assert_eq!(solve_part_two(&input.to_string()), 3993);
    }
}
