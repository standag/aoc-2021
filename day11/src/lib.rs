use std::{fs, ops::DerefMut};

pub fn solve_part_1() {
    let mut grid = read_grid_from_file("day11/input_test.txt");
    for i in 0..3 {
        grid.print();
        grid.iterate();
        println!();
    }
}
pub fn solve_part_2() {}

#[derive(Debug)]
struct Point {
    x: usize,
    y: usize,
    value: u8,
    exploded: bool,
}

#[derive(Debug)]
struct Grid {
    rows: Vec<Vec<Point>>,
}

impl Grid {
    /*
    fn new(size: usize) -> Self {
        let row: Vec<Point> = (0..size).map(|_| 0).collect();
        Grid {
            rows: (0..size).map(|_| row.to_vec()).collect(),
        }
    }
    */

    fn from_rows(rows: Vec<Vec<Point>>) -> Self {
        Grid { rows }
    }

    fn iterate(&mut self) {
        let mut to_explode: Vec<&mut Point> = vec![];
        self.rows.iter_mut().for_each(|r| {
            r.iter_mut().for_each(|p| match p.value {
                9 => to_explode.push(p),
                _ => p.value += 1,
            })
        });
        //to_explode.iter_mut().for_each(|p| self.explode(p));
    }

    fn explode(&mut self, p: &mut Point) {
        p.value = 0;
    }

    fn print(&self) {
        for row in &self.rows {
            row.iter().for_each(|x| match x.value {
                0 => print!("."),
                _ => print!("{}", x.value),
            });
            println!();
        }
    }
}

fn read_grid_from_file(filename: &str) -> Grid {
    Grid::from_rows(
        fs::read_to_string(filename)
            .unwrap()
            .split_whitespace()
            .enumerate()
            .map(|(y, r)| {
                r.chars()
                    .enumerate()
                    .map(|(x, c)| Point {
                        x: x,
                        y: y,
                        value: c.to_string().parse::<u8>().unwrap(),
                        exploded: false,
                    })
                    .collect()
            })
            .collect(),
    )
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
