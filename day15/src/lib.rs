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
    points: Vec<Point>,
    alternative_paths: Vec<Path>,
}
impl Path {
    fn len(&self) -> usize {
        self.points.iter().map(|p| p.value).sum()
    }
}

struct Graph {
    start: Rc<LinkedPoint>,
    end: Rc<LinkedPoint>,
    points: Vec<Rc<LinkedPoint>>,
}

impl Graph {
    fn find_path(&self) -> Vec<Rc<LinkedPoint>> {
        let mut paths = vec![vec![Rc::clone(&self.start)]];
        loop {
            // I can get with extension of any path or creating a new path from already existing
            // one
            let mut potential_extended_paths: HashSet<Vec<Rc<LinkedPoint>>> = paths
                .iter()
                .map(|p| {
                    let mut np = p.to_vec();
                    np.push(Rc::clone(
                        p.last()
                            .unwrap()
                            .neighbors
                            .iter()
                            .filter(|n| !np.contains(n))
                            .reduce(|acc, p| if acc.value < p.value { acc } else { p })
                            .unwrap(),
                    ));
                    np
                })
                .collect();
            for path in &paths {
                for i in 0..path.len() - 1 {
                    let mut np = path[0..=i].to_vec();
                    let new_point = np
                        .last()
                        .unwrap()
                        .neighbors
                        .iter()
                        .filter(|n| !np.contains(n))
                        .filter(|n| !paths.iter().flatten().any(|p| &p == n))
                        .reduce(|acc, p| if acc.value < p.value { acc } else { p });
                    if let Some(p) = new_point {
                        np.push(Rc::clone(p));
                        potential_extended_paths.insert(np);
                    }
                }
            }
            let mut pep = potential_extended_paths.iter().collect::<Vec<_>>();
            pep.sort_by_key(|path| path.iter().map(|p| p.value).sum::<usize>());
            println!(
                "{:?}",
                pep.iter()
                    .map(|path| path.iter().map(|p| p.value).sum::<usize>())
                    .collect::<Vec<_>>()
            );
            let finalist = pep.first().unwrap().to_vec();
            paths.retain(|p| p != &finalist[0..finalist.len() - 1]);
            paths.push(finalist);
            println!(
                ">>graph>pahts {:?}",
                paths
                    .iter()
                    .map(|p| p
                        .iter()
                        .map(|p| (p.x, p.y, p.value))
                        .collect::<Vec<(usize, usize, usize)>>())
                    .collect::<Vec<Vec<(usize, usize, usize)>>>()
            );
            let final_path: Vec<Vec<Rc<LinkedPoint>>> = paths
                .iter()
                .filter(|p| p.contains(&self.end))
                .map(|p| p.to_vec())
                .collect();
            if final_path.len() > 0 {
                break final_path.first().unwrap().to_vec();
            }
            //break paths.first().unwrap().to_vec();
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
            //children.extend(parents);
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

fn find_path(finish: &Point, grid: &Grid, path: &Path) -> Option<Path> {
    let mut path = path.clone();
    let mut i = 0;
    let final_path = loop {
        let mut paths = vec![path.clone()];
        path.alternative_paths
            .iter()
            .for_each(|p| paths.push(p.clone()));
        for path in &mut paths {
            let last_point = path.points.last().unwrap().clone();
            let mut neighbors = grid.get_neighbors(&last_point);
            neighbors.sort_by_key(|x| x.value);
            let unvisited_neighbors: Vec<Point> = neighbors
                .iter()
                .filter(|x| !path.points.contains(x))
                .map(|r| *r)
                .collect();
            if unvisited_neighbors.len() != 0 {
                unvisited_neighbors[1..].iter().for_each(|point| {
                    let mut p = path.clone();
                    p.points.push(*point);
                    path.alternative_paths.push(p);
                });
                path.points.push(unvisited_neighbors[0]);
            }
        }
        let final_path: Vec<Path> = paths
            .to_vec()
            .iter()
            .map(|path| {
                let last_point = path.points.last().unwrap().clone();
                if last_point.x == finish.x && last_point.y == finish.y {
                    Some(path)
                } else {
                    None
                }
            })
            .filter(|maybe_path| maybe_path.is_some())
            .map(|some_path| some_path.unwrap().clone())
            .collect();
        if final_path.len() != 0 {
            break final_path.first().unwrap().clone();
        }
        paths.sort_by_key(|path| path.len());
        path = paths.first().unwrap().clone();
        paths[1..]
            .iter_mut()
            .filter(|p| !path.points.contains(p.points.last().unwrap()))
            .for_each(|p| {
                p.points.pop();
                p.alternative_paths = vec![];
                if !path.alternative_paths.contains(p) {
                    path.alternative_paths.push(p.clone());
                }
            });
        if i >= 500 {
            break path;
        }
        i += 1;
    };
    Some(final_path)
}
#[test]
fn test_simple() {
    let finish = Point {
        x: 2,
        y: 2,
        value: 0,
    };
    let grid = Grid::from_str(&"199\n123\n964".to_string());
    let path = Path {
        points: vec![grid.get(0, 0)],
        alternative_paths: vec![],
    };
    let finded_path = find_path(&finish, &grid, &path).unwrap();
    assert_eq!(finded_path.len(), 11);
}

#[test]
fn test_simple2() {
    let grid = Grid::from_str(&"1911\n1991\n1991\n1991".to_string());
    let path = Path {
        points: vec![grid.get(0, 0)],
        alternative_paths: vec![],
    };
    let finded_path = find_path(&grid.get(3, 3), &grid, &path).unwrap();
    assert_eq!(finded_path.len(), 15);
}

#[test]
fn test_test_case() {
    let grid = Grid::from_file("input_test.txt");
    let graph = grid.to_graph();
    assert_eq!(graph.start.value, 1);
    assert_eq!(graph.find_path().iter().map(|p| p.value).sum::<usize>(), 40);
    //    let path = Path {
    //        points: vec![grid.get(0, 0)],
    //        alternative_paths: vec![],
    //    };
    //    println!("{:?}", grid.get_right_down_corner());
    //    let finded_path = find_path(&grid.get_right_down_corner(), &grid, &path).unwrap();
    //    println!("{:?}", finded_path.points);
    //    assert_eq!(finded_path.len(), 40);
}

#[test]
fn test_graph() {
    let grid = Grid::from_str(&"12\n34".to_string());
    let graph = grid.to_graph();
    assert_eq!(graph.start.value, 1);
}
#[test]
fn test_simple_with_graph() {
    let grid = Grid::from_str(&"199\n123\n964".to_string());
    let graph = grid.to_graph();
    let path = graph.find_path();
    assert_eq!(path.iter().map(|p| p.value).sum::<usize>(), 11);
}
#[test]
fn test_simple2_with_graph() {
    let grid = Grid::from_str(&"1911\n1991\n1991\n1991".to_string());
    let graph = grid.to_graph();
    let path = graph.find_path();
    println!(
        "{:?}",
        path.iter()
            .map(|p| (p.x, p.y, p.value))
            .collect::<Vec<(usize, usize, usize)>>()
    );
    assert_eq!(graph.find_path().iter().map(|p| p.value).sum::<usize>(), 15);
}
