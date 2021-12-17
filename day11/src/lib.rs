use std::cell::RefCell;
use std::{fs, ops::DerefMut};

pub fn solve_part_1() {
    let mut grid = read_grid_from_file("day11/input.txt");
    println!("Result: {}", _solve_part_1(&mut grid));
}
pub fn solve_part_2() {
    let mut grid = read_grid_from_file("day11/input.txt");
    println!("Result: {}", _solve_part_2(&mut grid));
}

fn _solve_part_1(grid: &mut Grid) -> usize {
    let mut grid = grid.clone();
    let mut total = 0;
    for _ in 0..100 {
        print(&grid);
        let result = iterate(&grid);
        grid = result.0;
        println!();
        total += result.1;
    }
    total
}

fn _solve_part_2(grid: &mut Grid) -> usize {

    let mut grid = grid.clone();
    let mut i = 0;
    loop {
        let (new_grid, _) = iterate(&grid);
        grid = new_grid;
        i += 1;
        if grid.iter().flatten().all(|p| p.value == 0){break;}
    }
    i
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Point {
    x: usize,
    y: usize,
    value: u8,
    exploded: bool,
}

type Grid = Vec<Vec<Point>>;

fn iterate(grid: &Grid) -> (Grid, usize) {
    let mut new_grid = grid.clone();
    new_grid
        .iter_mut()
        .for_each(|r| r.iter_mut().for_each(|p| p.value += 1));
    loop {
        new_grid = explode(&new_grid);
        if !new_grid.iter().flatten().any(|p| p.value >= 10) {
            break;
        }
    }
    new_grid = new_grid.clone();
    let exploded = new_grid.iter().flatten().filter(|p| p.exploded).count();
    new_grid
        .iter_mut()
        .flatten()
        .for_each(|p| p.exploded = false);
    (new_grid, exploded)
}

fn explode(grid: &Grid) -> Grid {
    let mut grid = grid.clone();
    let mut exploded = vec![];
    grid.iter_mut().for_each(|r| {
        r.iter_mut().filter(|p| p.value >= 10).for_each(|p| {
            p.value = 0;
            p.exploded = true;
            exploded.push((p.x, p.y));
        })
    });
    grid = grid.clone();
    exploded.iter().for_each(|(x, y)| {
        get_neighbors(&grid, *x, *y).iter().for_each(|(x, y)| {
            let mut p = &mut grid[*y][*x];
            if !p.exploded {
                p.value += 1
            }
        })
    });
    grid
}

fn get_neighbors(grid: &Grid, x: usize, y: usize) -> Vec<(usize, usize)> {
    let directions: [(isize, isize); 8] = [
        (1, 0),
        (-1, 0),
        (0, 1),
        (0, -1),
        (1, 1),
        (1, -1),
        (-1, 1),
        (-1, -1),
    ];
    directions
        .iter()
        .map(|d| (x as isize + d.0, y as isize + d.1))
        .filter(|(x, y)| {
            *x >= 0 && *y >= 0 && *x < grid[0].len() as isize && *y < grid.len() as isize
        })
        .map(|(x, y)| (x as usize, y as usize))
        .collect()
}

fn print(grid: &Grid) {
    for row in grid {
        row.iter().for_each(|x| match x.value {
            0 => print!("."),
            _ => print!("{:1}", x.value),
        });
        println!();
    }
}

fn read_grid_from_file(filename: &str) -> Grid {
    read_grid_from_str(&fs::read_to_string(filename).unwrap())
}

fn read_grid_from_str(input: &str) -> Grid {
    input
        .split_whitespace()
        .enumerate()
        .map(|(y, r)| {
            r.chars()
                .enumerate()
                .map(|(x, c)| Point {
                    x,
                    y,
                    value: c.to_string().parse::<u8>().unwrap(),
                    exploded: false,
                })
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_solution1() {
        let mut grid = read_grid_from_file("input_test.txt");
        assert_eq!(_solve_part_1(&mut grid), 26397);
    }

    #[test]
    fn test_step() {
        let input = "11111
19991
19191
19991
11111";
        let expected = "34543
40004
50005
40004
34543";
        let mut parsed_input = read_grid_from_str(input);
        let parsed_expected = read_grid_from_str(expected);
        iterate(&mut parsed_input);
        assert_eq!(parsed_input, parsed_expected);
    }

    #[test]
    fn test_step_2() {
        let input = "6594254334
3856965822
6375667284
7252447257
7468496589
5278635756
3287952832
7993992245
5957959665
6394862637";
        let expected = "8807476555
5089087054
8597889608
8485769600
8700908800
6600088989
6800005943
0000007456
9000000876
8700006848";
        let mut parsed_input = read_grid_from_str(input);
        let parsed_expected = read_grid_from_str(expected);
        iterate(&mut parsed_input);
        assert_eq!(parsed_input, parsed_expected);
    }
}
