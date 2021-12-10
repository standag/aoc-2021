use std::fs;

pub fn solve_part_1() {
    let heightmap = read_heightmap("day09/input.txt");
    println!("Result: {}", _solve_part_1(&heightmap));
}
pub fn solve_part_2() {
    let heightmap = read_heightmap("day09/input.txt");
    println!("Result: {}", _solve_part_2(&heightmap));
}

#[derive(Debug, Clone, Copy)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn in_(&self, others: &Vec<Point>) -> bool {
        others.iter().any(|p| p.x == self.x && p.y == self.y)
    }
}

struct Grid {
    points: Vec<Vec<u8>>,
}

impl Grid {
    fn get(&self, point: &Point) -> u8 {
        self.points[point.y][point.x]
    }
    fn find_next_unvisited(&self, visited: &Vec<Point>) -> Option<Point> {
        for j in 0..self.points.len() {
            for i in 0..self.points[0].len() {
                let point = Point { x: i, y: j };
                if !visited.iter().any(|p| p.x == point.x && p.y == point.y) {
                    return Some(point);
                }
            }
        }
        None
    }
    fn find_local_minima(&self) -> Vec<Point> {
        let mut minima: Vec<Point> = vec![];
        let mut visited: Vec<Point> = vec![];
        loop {
            let point = self.find_next_unvisited(&visited);
            if point.is_none() {
                break;
            }
            let p = point.as_ref().unwrap();
            let value = self.get(p);
            let mut neighbors = self.get_neighbors(p);
            if value < neighbors.iter().map(|x| self.get(x)).min().unwrap() {
                println!(
                    "Local min: {:?} with {} ({:?})",
                    p,
                    value,
                    neighbors.iter().map(|x| self.get(x)).collect::<Vec<u8>>()
                );
                minima.push(*p);
                visited.append(&mut neighbors);
            }
            visited.push(point.unwrap());
        }
        minima
    }

    fn find_local_minima_values(&self) -> Vec<u8> {
        self.find_local_minima()
            .iter()
            .map(|p| self.get(p))
            .collect()
    }

    fn is_in_grid(&self, point: &Point) -> bool {
        point.x < self.points[0].len() && point.y < self.points.len()
    }

    fn get_neighbors(&self, point: &Point) -> Vec<Point> {
        let directions: [(isize, isize); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];
        directions
            .iter()
            .map(|d| (point.x as isize + d.0, point.y as isize + d.1))
            .filter(|x| x.0 >= 0 && x.1 >= 0)
            .map(|c| Point {
                x: c.0 as usize,
                y: c.1 as usize,
            })
            .filter(|p| self.is_in_grid(p))
            .collect()
    }

    fn find_basin(&self, point: &Point) -> Vec<Point> {
        let mut basin: Vec<Point> = vec![*point];
        let mut neighbors: Vec<Point> = vec![*point];

        loop {
            println!("1> {:?}", neighbors);
            neighbors = neighbors
                .iter()
                .map(|start| {
                    self.get_neighbors(start)
                        .iter()
                        .filter(|p| self.get(p) != 9 && !p.in_(&basin))
                        .map(|p| *p)
                        .collect::<Vec<Point>>()
                })
                .flatten()
                .collect();

            println!("2> {:?}", neighbors);
            if neighbors.is_empty() {
                break;
            }
            neighbors.iter().for_each(|x| {
                if !x.in_(&basin) {
                    basin.push(*x)
                }
            });
        }
        basin
    }
}

fn _solve_part_1(heightmap: &Grid) -> usize {
    let minima: Vec<u8> = heightmap.find_local_minima_values();
    println!(
        "{} {}",
        minima.iter().map(|x| *x as usize).sum::<usize>(),
        minima.len()
    );
    minima.iter().map(|x| *x as usize).sum::<usize>() + minima.len()
}
fn _solve_part_2(heightmap: &Grid) -> usize {
    let mut x: Vec<usize> = heightmap
        .find_local_minima()
        .iter()
        .map(|min| heightmap.find_basin(min).len())
        .collect();
    x.sort_by(|a, b| b.cmp(a));
    x.iter().take(3).product()
}

fn read_heightmap(filename: &str) -> Grid {
    let points = fs::read_to_string(filename)
        .unwrap()
        .split_whitespace()
        .map(|x| {
            x.chars()
                .map(|y| y.to_string().parse::<u8>().unwrap())
                .collect()
        })
        .collect();
    Grid { points }
}

#[cfg(test)]
mod tests {
    use crate::{_solve_part_1, _solve_part_2, read_heightmap};
    #[test]
    fn test_part_1() {
        assert_eq!(_solve_part_1(&read_heightmap("input_test.txt")), 15);
    }
    #[test]
    fn test_part_2() {
        assert_eq!(_solve_part_2(&read_heightmap("input_test.txt")), 1134);
    }
}
