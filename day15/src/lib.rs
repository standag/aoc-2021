use std::collections::HashSet;
use std::fs;
use std::rc::Rc;

#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
struct Point {
    x: usize,
    y: usize,
    value: usize,
}

impl Point {
    fn to_linked(&self, neighbors: Vec<Rc<LinkedPoint>>) -> LinkedPoint {
        LinkedPoint {
            x: self.x,
            y: self.y,
            value: self.value,
            neighbors,
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
struct LinkedPoint {
    x: usize,
    y: usize,
    value: usize,
    neighbors: Vec<Rc<LinkedPoint>>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Path {
    points: Vec<Rc<LinkedPoint>>,
    lenght: usize,
}

impl Path {
    fn from_points(points: &Vec<Rc<LinkedPoint>>) -> Self {
        Path {
            points: points.to_vec(),
            lenght: points[1..].iter().map(|p| p.value).sum(),
        }
    }
    fn push(&mut self, point: &Rc<LinkedPoint>) {
        self.points.push(Rc::clone(point));
        self.lenght += point.value;
    }
}

struct Graph {
    start: Rc<LinkedPoint>,
    end: Rc<LinkedPoint>,
    points: Vec<Rc<LinkedPoint>>,
}

impl Graph {
    fn find_path(&self) -> Path {
        let mut paths = vec![Path::from_points(&vec![Rc::clone(&self.start)])];
        let mut explored_paths: Vec<Path> = vec![];
        loop {
            let last_path = paths.last().unwrap().clone();
            let last_point = last_path.points.last().unwrap();
            let mut neighbors: Vec<Rc<LinkedPoint>> = last_point
                .neighbors
                .iter()
                .filter(|p| !last_path.points.contains(p))
                .map(|p| Rc::clone(p))
                .collect();
            neighbors.sort_by_key(|p| p.value);
            let mut extended_path = last_path.clone();
            extended_path.push(&neighbors[0]);
            // if last path extend about new point is shortest
            if explored_paths.len() == 0
                || extended_path.lenght < explored_paths.last().unwrap().lenght
            {
                paths.pop();
                paths.push(extended_path);
                neighbors = neighbors[1..].to_vec();
            }
            // there is exists already explored path with shortest lenght
            else {
                paths.push(explored_paths.pop().unwrap());
            }
            neighbors.iter().for_each(|p| {
                let mut new_explored_path = last_path.clone();
                new_explored_path.push(&p);
                if !explored_paths
                    .iter()
                    .filter(|explored_path| explored_path.points.last().unwrap() == p)
                    .any(|explored_path| explored_path.lenght < new_explored_path.lenght)
                {
                    explored_paths
                        .retain(|explored_path| explored_path.points.last().unwrap() != p);
                    explored_paths.push(new_explored_path)
                };
            });
            explored_paths.sort_by_key(|path| path.lenght);
            explored_paths.reverse();
            if paths.last().unwrap().points.last().unwrap() == &self.end {
                paths.iter().for_each(|path| {
                    println!(
                        "Lenght: {}, path: {:?}",
                        path.lenght,
                        path.points
                            .iter()
                            .map(|point| (point.x, point.y, point.value))
                            .collect::<Vec<(usize, usize, usize)>>()
                    )
                });
                break paths.last().unwrap().clone();
            }
            // println!("{}", paths.last().unwrap().lenght);
        }
    }
}

struct Grid {
    rows: Vec<Vec<Point>>,
}
impl Grid {
    fn from_str(rows: &String) -> Self {
        let rows = rows
            .split_whitespace()
            .enumerate()
            .map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(|(x, c)| Point {
                        x,
                        y,
                        value: c.to_digit(10).unwrap() as usize,
                    })
                    .collect()
            })
            .collect();
        Grid { rows }
    }
    fn from_file(filename: &str) -> Self {
        Self::from_str(&fs::read_to_string(filename).unwrap())
    }
    fn get(&self, x: usize, y: usize) -> Point {
        self.rows[y][x]
    }
    fn get_right_down_corner(&self) -> Point {
        self.get(self.rows[0].len() - 1, self.rows.len() - 1)
    }
    fn get_neighbors(&self, point: &Point) -> Vec<Point> {
        let directions: Vec<(isize, isize)> = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];
        directions
            .iter()
            .map(|(x, y)| (point.x as isize + x, point.y as isize + y))
            .filter(|(x, y)| {
                *x >= 0
                    && *y >= 0
                    && *x < (self.rows[0].len() as isize)
                    && *y < (self.rows.len() as isize)
            })
            .map(|(x, y)| self.get(x as usize, y as usize))
            .collect()
    }
    fn get_neighbors_by_coordinates(&self, x: usize, y: usize) -> Vec<Point> {
        self.get_neighbors(&Point { x, y, value: 0 })
    }
    fn to_graph(&self) -> Graph {
        let point = self.get_right_down_corner();
        let end = Rc::new(point.to_linked(vec![]));
        let mut children = vec![Rc::clone(&end)];
        let mut parents = self
            .get_neighbors(&point)
            .iter()
            .map(|p| Rc::new(p.to_linked(vec![Rc::clone(&end)])))
            .collect();
        loop {
            parents = get_parents(&parents, self, &children);
            parents.iter().for_each(|p| children.push(Rc::clone(p)));
            if parents.len() == 1 {
                break;
            }
        }
        let start = Rc::clone(&parents.first().unwrap());
        Graph {
            start,
            end,
            points: children,
        }
    }
}

fn get_parents(
    currents: &Vec<Rc<LinkedPoint>>,
    grid: &Grid,
    children: &Vec<Rc<LinkedPoint>>,
) -> Vec<Rc<LinkedPoint>> {
    let parents = currents
        .iter()
        .map(|lp| grid.get_neighbors_by_coordinates(lp.x, lp.y))
        .flatten()
        .collect::<HashSet<Point>>();
    parents
        .iter()
        .filter(|p| {
            !children
                .iter()
                .any(|child| p.x == child.x && p.y == child.y)
        })
        .map(|p| {
            Rc::clone(&Rc::new(
                p.to_linked(
                    currents
                        .iter()
                        .filter(|lp| {
                            let neighbors = grid.get_neighbors_by_coordinates(lp.x, lp.y);
                            neighbors.contains(&p)
                        })
                        .map(|lp| Rc::clone(lp))
                        .collect(),
                ),
            ))
        })
        .collect()
}

#[test]
fn test_test_case() {
    let grid = Grid::from_file("input_test.txt");
    let graph = grid.to_graph();
    assert_eq!(graph.start.value, 1);
    assert_eq!(graph.find_path().lenght, 40);
}

#[test]
fn test_first_part() {
    let grid = Grid::from_file("input.txt");
    let graph = grid.to_graph();
    assert_eq!(graph.find_path().lenght, 40);
}

#[test]
fn test_graph() {
    let grid = Grid::from_str(&"12\n34".to_string());
    let graph = grid.to_graph();
    assert_eq!(graph.start.value, 1);
}
#[test]
fn test_simple() {
    let grid = Grid::from_str(&"199\n123\n964".to_string());
    let graph = grid.to_graph();
    let path = graph.find_path();
    assert_eq!(path.lenght, 10);
}
#[test]
fn test_simple2() {
    let grid = Grid::from_str(&"1911\n1991\n1991\n1991".to_string());
    let graph = grid.to_graph();
    assert_eq!(graph.find_path().lenght, 14);
}
