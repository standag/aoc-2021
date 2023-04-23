#[allow(dead_code)]
use anyhow::{bail, Result};
use itertools::Itertools;
use std::collections::HashSet;
use std::fs::read_to_string;
use std::iter::FromIterator;

pub fn solve_part_1() {}
pub fn solve_part_2() {}

#[derive(Debug, Clone, Copy)]
struct Transformation {
    movement: (isize, isize, isize),
    signs: (isize, isize, isize),
    order: (usize, usize, usize), // mimic rotation
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Point {
    x: isize,
    y: isize,
    z: isize,
}

impl Point {
    fn new(x: isize, y: isize, z: isize) -> Self {
        Self { x, y, z }
    }
    fn from_vec(coord: &Vec<isize>) -> Self {
        Point {
            x: coord[0],
            y: coord[1],
            z: coord[2],
        }
    }
    fn distance(&self, other: &Self) -> f64 {
        let x = (self.x - other.x) as f64;
        let y = (self.y - other.y) as f64;
        let z = (self.z - other.z) as f64;
        (x.powi(2) + y.powi(2) + z.powi(2)).sqrt()
    }
    fn coord(&self) -> Vec<isize> {
        vec![self.x, self.y, self.z]
    }
    fn transform(&self, t: &Transformation) -> Self {
        let coords = self.coord();
        Self {
            x: t.movement.0 - t.signs.0 * coords[t.order.0],
            y: t.movement.1 - t.signs.1 * coords[t.order.1],
            z: t.movement.2 - t.signs.2 * coords[t.order.2],
        }
    }
    fn extrapolation(&self, other: &Point) -> Vec<Transformation> {
        /*
            Axis can be rotated but in 90, 180, 270 degree, there are just switch in order and sign.
            Lets create combinations of order of coordinates [[1,2,3], [1,3,2], ...] to simulate limited rotation.
            Lets create combinations of signs [[1,1,1], [1,1,-1], ...]
        */

        let orders = (0..3).permutations(3).collect::<Vec<_>>();
        let signs = [-1, -1, -1, 1, 1, 1]
            .iter()
            .permutations(3)
            .unique()
            .collect::<Vec<_>>();

        orders
            .into_iter()
            .flat_map(|order| {
                signs.iter().map(move |sign| Transformation {
                    movement: (
                        self.x + sign[0] * other.coord()[order[0]],
                        self.y + sign[1] * other.coord()[order[1]],
                        self.z + sign[2] * other.coord()[order[2]],
                    ),
                    order: (order[0], order[1], order[2]),
                    signs: (*sign[0], *sign[1], *sign[2]),
                })
            })
            .collect::<Vec<_>>()
    }
}

#[derive(Debug)]
struct Map {
    scanners: Vec<Point>,
    beacons: Vec<Point>,
}

impl Map {
    fn new(beacons: &Vec<Vec<isize>>) -> Self {
        Self {
            scanners: vec![Point::new(0, 0, 0)],
            beacons: beacons
                .iter()
                .map(|p| Point::new(p[0], p[1], p[2]))
                .collect(),
        }
    }
    fn add(&mut self, beacons: &Vec<Vec<isize>>) -> Result<()> {
        let beacons: Vec<Point> = beacons
            .iter()
            .map(|p| Point::new(p[0], p[1], p[2]))
            .collect();

        // 1. calc distance matrix between current and adding points -> idea: there should be same
        //    distance between common beacons
        // 2. compare distances
        //    a/ find all pairs that have same distance
        //    b/ foreach pair until find transformation (x',y',z',rotation)
        //       - transformation that apply for all point and found at least same 12 overlapped
        //       points
        // 3. apply tranformation to all incomming uncommon points and add them to beacons list
        // 4. add position of a new beacon (x=x',y=y',z=z')

        let transformation = self.get_transformation(&beacons)?;
        let ref mut missing = self.get_missing(&beacons, &transformation);
        self.beacons.append(missing);
        self.scanners.push(Point {
            x: transformation.movement.0,
            y: transformation.movement.1,
            z: transformation.movement.2,
        });
        Ok(())
    }

    fn get_potential_pairs(&self, new_beacons: &Vec<Point>) -> Vec<(usize, usize)> {
        let current = calc_distance_matrix(&self.beacons);
        let new = calc_distance_matrix(new_beacons);
        let potential_combos = current.iter().fold(vec![], |acc, i| {
            let mut acc = acc.clone();
            acc.append(
                &mut new
                    .iter()
                    .filter(|x| x.2 == i.2)
                    .map(|(i1, i2, _)| ((i.0, *i1), (i.1, *i2)))
                    .collect::<Vec<((usize, usize), (usize, usize))>>(),
            );
            acc
        });
        let potential_pairs_with_repeat: Vec<(usize, usize)> =
            potential_combos.iter().fold(vec![], |acc, i| {
                let mut acc = acc;
                acc.push(i.0);
                acc.push(i.1);
                acc
            });
        let potential_pairs: HashSet<(usize, usize)> =
            HashSet::from_iter(potential_pairs_with_repeat.iter().cloned());
        let mut potential_pairs_with_count: Vec<((usize, usize), usize)> = potential_pairs
            .iter()
            .map(|pair| {
                (
                    (pair.0, pair.1),
                    potential_pairs_with_repeat
                        .iter()
                        .filter(|i| i == &pair)
                        .count(),
                )
            })
            .collect();

        potential_pairs_with_count.sort_by(|a, b| b.1.cmp(&a.1));
        println!("Potential pairs {:?}", potential_pairs_with_count);

        potential_pairs_with_count
            .iter()
            .filter(|(_, count)| count > &2)
            .map(|(pair, _)| *pair)
            .collect()
    }

    fn get_transformation(&self, new_beacons: &Vec<Point>) -> Result<Transformation> {
        let potential_pairs = self.get_potential_pairs(new_beacons);
        println!(
            "combs: {:?} from {:?}",
            potential_pairs
                .iter()
                .combinations_with_replacement(2)
                .collect_vec(),
            potential_pairs
        );
        for comb in potential_pairs.iter().combinations(2) {
            let (point_c1, point_c2) = (self.beacons[comb[0].0], self.beacons[comb[1].0]);
            let (point_n1, point_n2) = (new_beacons[comb[0].1], new_beacons[comb[1].1]);
            let potential_transformations = point_c1.extrapolation(&point_n1);
            for t in potential_transformations {
                println!(
                    "trans: {:?} excepted: {:?} get: {:?} from: {:?}",
                    t,
                    point_c2,
                    point_n2.transform(&t),
                    point_n2,
                );
                if point_n2.transform(&t) == point_c2 {
                    println!("trans: {:?}", t);
                    return Ok(t);
                }
            }
        }
        bail!("cannot find transformation")
    }

    fn get_missing(&self, new_beacons: &Vec<Point>, t: &Transformation) -> Vec<Point> {
        new_beacons
            .iter()
            .map(|p| p.transform(&t))
            .filter(|p| !self.beacons.contains(p))
            .collect()
    }
}

fn calc_distance_matrix(points: &Vec<Point>) -> Vec<(usize, usize, f64)> {
    let ids: Vec<usize> = (0..points.len()).collect();
    ids.iter()
        .combinations(2)
        .map(|ps| (*ps[0], *ps[1], points[*ps[0]].distance(&points[*ps[1]])))
        .collect()
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn test_transform() {
        let p1 = Point {
            x: 571,
            y: -461,
            z: -707,
        };
        let expected = Point {
            x: 534,
            y: -1912,
            z: 768,
        };
        let trans = Transformation {
            movement: (1105, -1205, 1229),
            signs: (1, -1, -1),
            order: (0, 2, 1),
        };
        assert_eq!(p1.transform(&trans), expected);
    }

    #[test]
    fn test_add() {
        let beacons0 = vec![
            vec![404, -588, -901],
            vec![528, -643, 409],
            vec![-838, 591, 734],
            vec![390, -675, -793],
            vec![-537, -823, -458],
            vec![-485, -357, 347],
            vec![-345, -311, 381],
            vec![-661, -816, -575],
            vec![-876, 649, 763],
            vec![-618, -824, -621],
            vec![553, 345, -567],
            vec![474, 580, 667],
            vec![-447, -329, 318],
            vec![-584, 868, -557],
            vec![544, -627, -890],
            vec![564, 392, -477],
            vec![455, 729, 728],
            vec![-892, 524, 684],
            vec![-689, 845, -530],
            vec![423, -701, 434],
            vec![7, -33, -71],
            vec![630, 319, -379],
            vec![443, 580, 662],
            vec![-789, 900, -551],
            vec![459, -707, 401],
        ];
        let mut map = Map::new(&beacons0);
        let beacons1 = vec![
            vec![686, 422, 578],
            vec![605, 423, 415],
            vec![515, 917, -361],
            vec![-336, 658, 858],
            vec![95, 138, 22],
            vec![-476, 619, 847],
            vec![-340, -569, -846],
            vec![567, -361, 727],
            vec![-460, 603, -452],
            vec![669, -402, 600],
            vec![729, 430, 532],
            vec![-500, -761, 534],
            vec![-322, 571, 750],
            vec![-466, -666, -811],
            vec![-429, -592, 574],
            vec![-355, 545, -477],
            vec![703, -491, -529],
            vec![-328, -685, 520],
            vec![413, 935, -424],
            vec![-391, 539, -444],
            vec![586, -435, 557],
            vec![-364, -763, -893],
            vec![807, -499, -711],
            vec![755, -354, -619],
            vec![553, 889, -390],
        ];
        let beacons2 = vec![
            vec![649, 640, 665],
            vec![682, -795, 504],
            vec![-784, 533, -524],
            vec![-644, 584, -595],
            vec![-588, -843, 648],
            vec![-30, 6, 44],
            vec![-674, 560, 763],
            vec![500, 723, -460],
            vec![609, 671, -379],
            vec![-555, -800, 653],
            vec![-675, -892, -343],
            vec![697, -426, -610],
            vec![578, 704, 681],
            vec![493, 664, -388],
            vec![-671, -858, 530],
            vec![-667, 343, 800],
            vec![571, -461, -707],
            vec![-138, -166, 112],
            vec![-889, 563, -600],
            vec![646, -828, 498],
            vec![640, 759, 510],
            vec![-630, 509, 768],
            vec![-681, -892, -333],
            vec![673, -379, -804],
            vec![-742, -814, -386],
            vec![577, -820, 562],
        ];
        let beacons3 = vec![
            vec![-589, 542, 597],
            vec![605, -692, 669],
            vec![-500, 565, -823],
            vec![-660, 373, 557],
            vec![-458, -679, -417],
            vec![-488, 449, 543],
            vec![-626, 468, -788],
            vec![338, -750, -386],
            vec![528, -832, -391],
            vec![562, -778, 733],
            vec![-938, -730, 414],
            vec![543, 643, -506],
            vec![-524, 371, -870],
            vec![407, 773, 750],
            vec![-104, 29, 83],
            vec![378, -903, -323],
            vec![-778, -728, 485],
            vec![426, 699, 580],
            vec![-438, -605, -362],
            vec![-469, -447, -387],
            vec![509, 732, 623],
            vec![647, 635, -688],
            vec![-868, -804, 481],
            vec![614, -800, 639],
            vec![595, 780, -596],
        ];
        let beacons4 = vec![
            vec![727, 592, 562],
            vec![-293, -554, 779],
            vec![441, 611, -461],
            vec![-714, 465, -776],
            vec![-743, 427, -804],
            vec![-660, -479, -426],
            vec![832, -632, 460],
            vec![927, -485, -438],
            vec![408, 393, -506],
            vec![466, 436, -512],
            vec![110, 16, 151],
            vec![-258, -428, 682],
            vec![-393, 719, 612],
            vec![-211, -452, 876],
            vec![808, -476, -593],
            vec![-575, 615, 604],
            vec![-485, 667, 467],
            vec![-680, 325, -822],
            vec![-627, -443, -432],
            vec![872, -547, -609],
            vec![833, 512, 582],
            vec![807, 604, 487],
            vec![839, -516, 451],
            vec![891, -625, 532],
            vec![-652, -548, -490],
            vec![30, -46, -14],
        ];
        map.add(&beacons1);
        assert_eq!(map.beacons.len(), beacons0.len() + beacons1.len() - 12);
        assert_eq!(map.scanners.last().unwrap().coord(), vec![68, -1246, -43]);
        map.add(&beacons2).unwrap();
        map.add(&beacons3).unwrap();
        assert_eq!(map.scanners.last().unwrap().coord(), vec![-92, -2380, -20]);
        map.add(&beacons4).unwrap();
        assert_eq!(map.scanners.last().unwrap().coord(), vec![-20, -1133, 1061]);
    }

    #[test]
    fn part_one() {
        // let map = Map::new(beacons)
        let mut beacons = vec![];
        let mut buffer = vec![];

        let input_file = fs::read_to_string("input.txt").unwrap();
        let data = input_file.split("\n").for_each(|line| {
            let numbers = line.split(",").collect::<Vec<_>>();
            if numbers.len() == 3 {
                buffer.push(
                    numbers
                        .iter()
                        .map(|n| n.parse::<isize>().unwrap())
                        .collect::<Vec<_>>(),
                );
            }
            if line.is_empty() {
                beacons.push(buffer.clone());
                buffer = vec![];
            }
        });
        let mut map = Map::new(beacons.first().unwrap());
        let mut skipped = vec![];
        println!("{:?}", beacons.first().unwrap());
        beacons.iter().skip(1).enumerate().for_each(|(i, b)| {
            println!("{} beacons: {:?}", i, b);
            match map.add(b) {
                Ok(_) => (),
                Err(_) => skipped.push(b),
            }
            // map.add(b).expect("cannot add beacons to map");
        });

        println!("Skipped: {}", skipped.len());
        let mut skipped2 = vec![];
        skipped.iter().enumerate().for_each(|(i, s)| {
            println!("{} skipped beacons: {:?}", i, s);
            match map.add(s) {
                Ok(_) => (),
                Err(_) => skipped2.push(s),
            }
        });
        println!("Skipped in wave 2: {}", skipped2.len());
        let mut skipped3 = vec![];
        skipped2.iter().enumerate().for_each(|(i, s)| {
            println!("{} skipped beacons: {:?}", i, s);
            match map.add(s) {
                Ok(_) => (),
                Err(_) => skipped3.push(s),
            }
        });
        println!("Skipped in wave 3: {}", skipped3.len());
        assert_eq!(map.beacons.len(), 1);
    }
}
