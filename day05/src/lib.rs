use std::{fs, ops::RangeInclusive};

pub fn solve_part_1() {
    let input = read_input("day05/input.txt");
    println!("Result: {}", _solve_part_1(&input))
}
pub fn solve_part_2() {
    let input = read_input("day05/input.txt");
    println!("Result: {}", _solve_part_2(&input))
}

#[derive(Debug, Clone, Copy)]
struct Point {
    x: u16,
    y: u16,
}

impl Point {
    fn from_str(s: &str) -> Self {
        let sp = s
            .split_terminator(",")
            .map(|x| x.parse::<u16>().unwrap())
            .collect::<Vec<u16>>();
        Point { x: sp[0], y: sp[1] }
    }
}

#[derive(Debug)]
struct Grid {
    rows: Vec<Vec<usize>>,
}

impl Grid {
    fn new(size: usize) -> Self {
        let row: Vec<usize> = (0..size).map(|_| 0).collect();
        Grid {
            rows: (0..size).map(|_| row.to_vec()).collect(),
        }
    }
    fn register_points(&mut self, points: &Vec<Point>) {
        //println!("Points: {:?}", points);
        points
            .iter()
            .for_each(|p| self.rows[p.y as usize][p.x as usize] += 1);
    }
    fn print(&self) {
        for row in &self.rows {
            row.iter().for_each(|x| match x {
                0 => print!("."),
                _ => print!("{}", x),
            });
            println!();
        }
    }
}

fn read_input(filename: &str) -> Vec<(Point, Point)> {
    fs::read_to_string(filename)
        .unwrap()
        .split_terminator("\n")
        .map(|x| {
            x.split_terminator(" -> ")
                .map(|y| Point::from_str(y))
                .collect::<Vec<Point>>()
        })
        .collect::<Vec<Vec<Point>>>()
        .iter()
        .map(|x| (x[0], x[1]))
        .collect()
}

fn range(i: u16, j: u16) -> Vec<u16> {
    if i <= j {
        (0..=j - i).map(|x| i + x).collect()
    } else {
        (0..=i - j).map(|x| i - x).collect()
    }
}

fn _solve_part_1(input: &Vec<(Point, Point)>) -> usize {
    let mut grid = Grid::new(1000);
    input.iter().for_each(|c| match c {
        (Point { x: x1, y: _ }, Point { x: x2, y: _ }) if x1 == x2 => grid.register_points(
            &(range(c.0.y, c.1.y)
                .iter()
                .map(|i| Point { x: c.0.x, y: *i })
                .collect::<Vec<Point>>()),
        ),
        (Point { x: _, y: y1 }, Point { x: _, y: y2 }) if y1 == y2 => grid.register_points(
            &(range(c.0.x, c.1.x)
                .iter()
                .map(|i| Point { x: *i, y: c.0.y })
                .collect::<Vec<Point>>()),
        ),
        _ => (), //println!("Skip connection: {:?}", c),
    });
    //grid.print();
    grid.rows
        .iter()
        .flatten()
        .filter(|v| **v >= 2_usize)
        .count()
}

fn connection(points: &(Point, Point)) -> Vec<Point> {
    let mut x_range = range(points.0.x, points.1.x);
    let mut y_range = range(points.0.y, points.1.y);
    match (x_range.len(), y_range.len()) {
        (1, 2..) => y_range
            .iter()
            .map(|i| Point {
                x: points.0.x,
                y: *i,
            })
            .collect::<Vec<Point>>(),
        (2.., 1) => x_range
            .iter()
            .map(|i| Point {
                x: *i,
                y: points.0.y,
            })
            .collect::<Vec<Point>>(),
        _ => x_range
            .iter()
            .enumerate()
            .map(|(i, _)| Point {
                x: x_range[i],
                y: y_range[i],
            })
            .collect::<Vec<Point>>(),
    }
}

fn _solve_part_2(input: &Vec<(Point, Point)>) -> usize {
    let mut grid = Grid::new(1000);
    input
        .iter()
        .for_each(|c| grid.register_points(&connection(&c)));
    //grid.print();
    grid.rows
        .iter()
        .flatten()
        .filter(|v| **v >= 2_usize)
        .count()
}

#[cfg(test)]
mod tests {
    use crate::{_solve_part_1, _solve_part_2, read_input};

    #[test]
    fn test_part_1() {
        assert_eq!(_solve_part_1(&read_input("input_test.txt")), 5);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(_solve_part_2(&read_input("input_test.txt")), 12);
    }
}
