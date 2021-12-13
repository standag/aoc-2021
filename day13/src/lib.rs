use std::fs;

pub fn solve_part_1() {
    let (points, fold_instruction) = read_instruction("day13/input.txt");
    println!("Result: {}", _solve_part_1(&points, &fold_instruction));
}
pub fn solve_part_2() {
    let (points, fold_instruction) = read_instruction("day13/input.txt");
    _solve_part_2(&points, &fold_instruction);
}

#[derive(Debug)]
enum Direction {
    X,
    Y,
}

#[derive(Debug)]
struct Fold {
    direction: Direction,
    value: usize,
}

#[derive(Debug)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct Grid {
    rows: Vec<Vec<u8>>,
}

impl Grid {
    fn new(size: usize) -> Self {
        let row: Vec<u8> = (0..size).map(|_| 0).collect();
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
                1 => print!("#"),
                _ => print!("?"),
            });
            println!();
        }
    }

    fn print_part(&self, size: usize) {
        (0..size).for_each(|i| {
            (0..size).for_each(|j| match self.rows[i][j] {
                0 => print!("."),
                1 => print!("#"),
                _ => print!("?"),
            });
            println!();
        })
    }

    fn get_column(&self, col: usize) -> Vec<u8> {
        (0..self.rows.len())
            .map(|i| self.rows[i][col])
            .collect::<Vec<u8>>()
    }

    fn fold(&mut self, fold: &Fold) {
        match fold.direction {
            Direction::X => (0..fold.value).for_each(|i| {
                println!(
                    "{:?}",
                    merge_points(&self.get_column(i), &self.get_column(2 * fold.value - i))
                );
                merge_points(&self.get_column(i), &self.get_column(2 * fold.value - i))
                    .iter()
                    .enumerate()
                    .for_each(|(j, v)| {
                        self.rows[j][i] = *v;
                        self.rows[j][2 * fold.value - i] = 0;
                    });
            }),
            Direction::Y => {
                (0..=fold.value).for_each(|i| {
                    self.rows[i] = merge_points(&self.rows[i], &self.rows[2 * fold.value - i]);
                    self.rows[2 * fold.value - i] = (0..self.rows.len()).map(|_| 0).collect();
                });
            }
        }
    }
}

fn merge_points(points1: &Vec<u8>, points2: &Vec<u8>) -> Vec<u8> {
    points1
        .iter()
        .enumerate()
        .map(|(i, p1)| match (p1, points2[i]) {
            (0, 0) => 0,
            (1, 0) => 1,
            (0, 1) => 1,
            (1, 1) => 1,
            _ => panic!(),
        })
        .collect()
}

fn _solve_part_1(points: &Vec<Point>, fold_instruction: &Vec<Fold>) -> usize {
    let mut grid = Grid::new(1500);
    grid.register_points(points);
    println!(
        "Dots before fold: {}",
        grid.rows
            .iter()
            .flatten()
            .map(|x| *x as usize)
            .sum::<usize>()
    );
    grid.print();

    grid.fold(&fold_instruction[0]);
    println!();
    grid.print();

    //grid.fold(&fold_instruction[1]);
    //println!();
    //grid.print();
    grid.rows.iter().flatten().map(|x| *x as usize).sum()
}

fn _solve_part_2(points: &Vec<Point>, fold_instruction: &Vec<Fold>) -> usize {
    let mut grid = Grid::new(1500);
    grid.register_points(points);
    fold_instruction.iter().for_each(|i| grid.fold(i));
    println!();
    grid.print_part(50);
    grid.rows.iter().flatten().map(|x| *x as usize).sum()
}

fn read_instruction(filename: &str) -> (Vec<Point>, Vec<Fold>) {
    let raw = fs::read_to_string(filename)
        .unwrap()
        .split_terminator("\n\n")
        .map(|x| x.to_string())
        .collect::<Vec<String>>();
    let points = raw[0]
        .split_terminator("\n")
        .map(|p| Point {
            x: p.split_terminator(",").collect::<Vec<&str>>()[0]
                .parse::<usize>()
                .unwrap(),
            y: p.split_terminator(",").collect::<Vec<&str>>()[1]
                .parse::<usize>()
                .unwrap(),
        })
        .collect();
    let fold_instruction = raw[1]
        .split_terminator("\n")
        .map(|x| {
            x.split_whitespace()
                .nth(2)
                .map(|i| Fold {
                    direction: match i.chars().nth(0).unwrap() {
                        'x' => Direction::X,
                        'y' => Direction::Y,
                        _ => panic!(),
                    },
                    value: i[2..].parse::<usize>().unwrap(),
                })
                .unwrap()
        })
        .collect::<Vec<Fold>>();
    (points, fold_instruction)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
