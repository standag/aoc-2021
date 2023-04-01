#![allow(dead_code)]
use std::collections::VecDeque;
use std::fs::read_to_string;

struct Grid {
    data: Vec<usize>,
    size: usize,
}

impl Grid {
    fn from_file(filename: &str) -> Self {
        let mut size: usize = 0;
        let data = read_to_string(filename)
            .unwrap()
            .lines()
            .fold(vec![], |acc, line| {
                acc.into_iter()
                    .chain(line.chars().enumerate().map(|(i, c)| {
                        size = size.max(i);
                        c.to_string().parse::<usize>().unwrap()
                    }))
                    .collect::<Vec<_>>()
            });
        Self {
            data,
            size: size + 1,
        }
    }
    fn from_file_and_multiple_it(filename: &str) -> Self {
        let grid = Self::from_file(filename);
        let first_line = grid
            .data
            .chunks(grid.size)
            .flat_map(|line| {
                (0..5)
                    .flat_map(move |i| line.iter().map(move |n| (*n - 1 + i) % 9 + 1))
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        let data = (0..5)
            .flat_map(move |i| {
                first_line
                    .iter()
                    .map(move |n| (*n - 1 + i) % 9 + 1)
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        Self {
            data,
            size: grid.size * 5,
        }
    }
    fn get(&self, x: usize, y: usize) -> usize {
        let index = y * self.size + x;
        self.data[index]
    }
    fn neighbors(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        vec![
            (x.checked_sub(1), Some(y)),
            (Some(x), y.checked_sub(1)),
            (Some(x + 1), Some(y)),
            (Some(x), Some(y + 1)),
        ]
        .into_iter()
        .filter_map(|point| {
            if let (Some(x), Some(y)) = point {
                Some((x, y))
            } else {
                None
            }
        })
        .filter(|(x, y)| y * self.size + x < self.data.len())
        .collect::<Vec<_>>()
    }
    fn find_path(&self) -> usize {
        let mut path = vec![usize::MAX; self.data.len()];
        path[0] = 0_usize; //self.get(0, 0);
        let mut to_be_visited = self
            .neighbors(0, 0)
            .into_iter()
            .zip([0_usize].into_iter().cycle()) // make infinite
            .collect::<VecDeque<_>>();
        loop {
            let ((x, y), from) = to_be_visited.pop_front().unwrap();
            let index = y * self.size + x;
            let new_value = self.get(x, y) + path[from];
            let current_value = path[index];
            if new_value < current_value {
                path[index] = new_value;
                let mut new_to_be_visited = self
                    .neighbors(x, y)
                    .into_iter()
                    .zip([index].into_iter().cycle())
                    .filter(|((x, y), _)| y * self.size + x != from)
                    .collect::<VecDeque<_>>();
                to_be_visited.append(&mut new_to_be_visited);
            }
            if to_be_visited.is_empty() {
                break;
            }
        }
        *path.last().unwrap()
    }
}

#[test]
fn part_one_example() {
    let grid = Grid::from_file("input_test.txt");
    assert_eq!(grid.find_path(), 40);
}
#[test]
fn part_one() {
    let grid = Grid::from_file("input.txt");
    assert_eq!(grid.find_path(), 398);
}
#[test]
fn part_two_example() {
    let grid = Grid::from_file_and_multiple_it("input_test.txt");
    assert_eq!(grid.find_path(), 315);
}
#[test]
fn part_two() {
    let grid = Grid::from_file_and_multiple_it("input.txt");
    assert_eq!(grid.find_path(), 2817);
}
