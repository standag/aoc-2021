use std::collections::HashSet;
use std::hash::Hash;

use std::fs;

pub fn solve_part_1() {
    let start = Cave {
        name: "start".to_string(),
        size: CaveSize::Small,
    };
    let cave_A = Cave {
        name: "A".to_string(),
        size: CaveSize::Big,
    };
    let cave_b = Cave {
        name: "b".to_string(),
        size: CaveSize::Small,
    };
    let cave_c = Cave {
        name: "c".to_string(),
        size: CaveSize::Small,
    };
    let cave_d = Cave {
        name: "d".to_string(),
        size: CaveSize::Small,
    };
    let end = Cave {
        name: "end".to_string(),
        size: CaveSize::Small,
    };
    let mut ct = ConnectionTable {
        connections: vec![],
    };
    ct.connections.push((start.clone(), cave_A.clone()));
    ct.connections.push((start.clone(), cave_b.clone()));
    ct.connections.push((cave_A.clone(), cave_c.clone()));
    ct.connections.push((cave_c.clone(), cave_A.clone()));
    ct.connections.push((cave_A.clone(), cave_b.clone()));
    ct.connections.push((cave_b.clone(), cave_A.clone()));
    ct.connections.push((cave_A.clone(), cave_b.clone()));
    ct.connections.push((cave_b.clone(), end.clone()));
    ct.connections.push((cave_A.clone(), end.clone()));

    let path = ct.find_path(&vec![start.clone()], &end);
    println!("{:?} {}", path, path.len());

    let ct2 = ConnectionTable::from_file("day12/input_test.txt");
    let path2 = ct2.find_path(&vec![start.clone()], &end);
    println!("{:?} {}", path2, path2.len());

    let ct3 = ConnectionTable::from_file("day12/input.txt");
    let path3 = ct3.find_path(&vec![start.clone()], &end);
    println!("{:?} {}", path3, path3.len());
}
pub fn solve_part_2() {
    let start = Cave {
        name: "start".to_string(),
        size: CaveSize::Small,
    };

    let end = Cave {
        name: "end".to_string(),
        size: CaveSize::Small,
    };

    let ct_test = ConnectionTable::from_file("day12/input_test.txt");
    let path_test = ct_test.find_path_v2(&vec![start.clone()], &end);
    println!("{:?} {}", path_test, path_test.len());
    
    let ct = ConnectionTable::from_file("day12/input.txt");
    let path = ct.find_path_v2(&vec![start.clone()], &end);
    println!("{:?} {}", path, path.len());
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
enum CaveSize {
    Small,
    Big,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
struct Cave {
    name: String,
    size: CaveSize,
}

struct ConnectionTable {
    connections: Vec<(Cave, Cave)>,
}

impl ConnectionTable {
    fn find_path(&self, path: &Vec<Cave>, end: &Cave) -> Vec<Vec<Cave>> {
        let mut result: Vec<Vec<Cave>> = vec![];
        let mut x: Vec<Vec<Cave>> = self
            .connections
            .iter()
            .filter(|(s, e)| {
                s == path.last().unwrap() && !(e.size == CaveSize::Small && path.contains(e))
            })
            .map(|(s, e)| {
                let mut np: Vec<Cave> = path.iter().map(|c| (*c).clone()).collect();
                np.push((*e).clone());
                np
            })
            .collect();
        //println!("{:?}", x);
        //        x.iter().for_each(|p| print_path(p));
        x.iter_mut().for_each(|p| match p.last().unwrap() {
            e if e == end => {
                print_path(p);
                result.append(&mut vec![p.to_vec()])
            }
            _ => result.append(
                &mut self.find_path(&p.iter().map(|c| c.clone()).collect(), &(*end).clone()),
            ),
        });
        result
            .iter()
            .map(|x| x.clone())
            .collect::<HashSet<Vec<Cave>>>()
            .iter()
            .map(|x| x.clone())
            .collect::<Vec<Vec<Cave>>>()
    }

    fn find_path_v2(&self, path: &Vec<Cave>, end: &Cave) -> Vec<Vec<Cave>> {
        let mut result: Vec<Vec<Cave>> = vec![];
        let can_small = !path
            .iter()
            .filter(|c| c.size == CaveSize::Small)
            .any(|c| path.iter().filter(|x| *x == c).count() == 2);
        println!("Can small: {}", can_small);
        let mut x: Vec<Vec<Cave>> = self
            .connections
            .iter()
            .filter(|(s, e)| {
                s == path.last().unwrap() && (e.size != CaveSize::Small || !path.contains(e) || can_small)
            })
            .map(|(s, e)| {
                let mut np: Vec<Cave> = path.iter().map(|c| (*c).clone()).collect();
                np.push((*e).clone());
                np
            })
            .collect();
        println!("{:?}", x);
        //        x.iter().for_each(|p| print_path(p));
        x.iter_mut().for_each(|p| match p.last().unwrap() {
            e if e == end => {
                print_path(p);
                result.append(&mut vec![p.to_vec()])
            }
            _ => result.append(
                &mut self.find_path_v2(&p.iter().map(|c| c.clone()).collect(), &(*end).clone()),
            ),
        });
        result
            .iter()
            .map(|x| x.clone())
            .collect::<HashSet<Vec<Cave>>>()
            .iter()
            .map(|x| x.clone())
            .collect::<Vec<Vec<Cave>>>()
    }

    fn from_file(filename: &str) -> Self {
        let START: String = "start".to_string();
        let END: String = "end".to_string();
        let mut connections: Vec<(Cave, Cave)> = vec![];
        fs::read_to_string(filename)
            .unwrap()
            .split_whitespace()
            .for_each(|line| {
                let con: Vec<&str> = line.split_terminator("-").collect();
                let s1: CaveSize;
                if con[0].to_string().to_uppercase().eq(&con[0].to_string()) {
                    s1 = CaveSize::Big
                } else {
                    s1 = CaveSize::Small
                };
                let s2: CaveSize;
                if con[1].to_string().to_uppercase().eq(&con[1].to_string()) {
                    s2 = CaveSize::Big
                } else {
                    s2 = CaveSize::Small
                };
                let c1 = Cave {
                    name: con[0].to_string(),
                    size: s1,
                };
                let c2 = Cave {
                    name: con[1].to_string(),
                    size: s2,
                };
                let mut v1 = vec![(c1.clone(), c2.clone())];
                let mut v2 = vec![(c2.clone(), c1.clone())];
                match (c1.name, c2.name) {
                    (i, _) if i == START => connections.append(&mut v1),
                    (_, j) if j == START => connections.append(&mut v2),
                    (_, j) if j == END => connections.append(&mut v1),
                    (i, _) if i == END => connections.append(&mut v2),
                    _ => {
                        connections.append(&mut v1);
                        connections.append(&mut v2)
                    }
                }
            });
        ConnectionTable { connections }
    }
}

fn print_path(path: &Vec<Cave>) {
    print!("Path: ");
    path.iter().for_each(|c| print!("{}->", c.name));
    println!();
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
