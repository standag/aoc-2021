#![allow(dead_code)]

use std::isize;

struct TargetArea {
    x_min: isize,
    x_max: isize,
    y_min: isize,
    y_max: isize,
}

fn simulate_probe(
    velocity: &(isize, isize),
    target_area: &TargetArea,
) -> Option<(isize, isize, isize)> {
    let mut x = 0;
    let mut y = 0;
    let mut x_velocity = velocity.0;
    let mut y_velocity = velocity.1;
    let mut y_max = isize::MIN;
    loop {
        x += x_velocity;
        y += y_velocity;
        x_velocity -= 1;
        y_velocity -= 1;
        if x_velocity < 0 {
            x_velocity = 0
        };
        if y > y_max {
            y_max = y;
        }
        if x <= target_area.x_max
            && x >= target_area.x_min
            && y <= target_area.y_max
            && y >= target_area.y_min
        {
            break Some((x, y, y_max));
        }
        if x > target_area.x_max || y < target_area.y_min {
            break None;
        }
    }
}

#[test]
fn test_simulate_probe_1() {
    let target_area = TargetArea {
        x_min: 20,
        x_max: 30,
        y_min: -10,
        y_max: -5,
    };
    assert_eq!(simulate_probe(&(7, 2), &target_area), Some((28, -7, 3)));
}

#[test]
fn test_simulate_probe_2() {
    let target_area = TargetArea {
        x_min: 20,
        x_max: 30,
        y_min: -10,
        y_max: -5,
    };
    assert_eq!(simulate_probe(&(6, 9), &target_area), Some((21, -10, 45)));
}

#[test]
fn test_simulate_probe_3() {
    let target_area = TargetArea {
        x_min: 20,
        x_max: 30,
        y_min: -10,
        y_max: -5,
    };
    assert_eq!(simulate_probe(&(17, -4), &target_area), None);
}

#[test]
fn test_simulate_probe_4() {
    let target_area = TargetArea {
        x_min: 20,
        x_max: 30,
        y_min: -10,
        y_max: -5,
    };
    assert_eq!(simulate_probe(&(6, 0), &target_area), Some((20, -10, 0)));
}

fn find_probe_velocities(target_area: &TargetArea) -> Vec<(isize, isize, isize)> {
    let mut results = vec![];
    for i in -127..1000 {
        for j in -127..1000 {
            let i = i as isize;
            let j = j as isize;
            let simulation_result = simulate_probe(&(i.into(), j.into()), target_area);
            if simulation_result.is_some() {
                results.push((
                    i as isize,
                    j as isize,
                    simulation_result.unwrap().2 as isize,
                ));
            }
        }
    }
    results
}

#[test]
fn test_example_part_two() {
    let target_area = TargetArea {
        x_min: 20,
        x_max: 30,
        y_min: -10,
        y_max: -5,
    };
    let result = find_probe_velocities(&target_area);
    assert_eq!(result.len(), 112);
}

#[test]
fn test_part_two() {
    let target_area = TargetArea {
        x_min: 277,
        x_max: 318,
        y_min: -92,
        y_max: -53,
    };
    let result = find_probe_velocities(&target_area);
    assert_eq!(result.len(), 2709);
}

fn find_probe_velocity(target_area: &TargetArea) -> (isize, isize, isize) {
    let mut results = find_probe_velocities(target_area);
    results.sort_by_key(|x| x.2);
    let result = results.last().unwrap();
    (result.0 as isize, result.1 as isize, result.2 as isize)
}

#[test]
fn test_case_1() {
    let target_area = TargetArea {
        x_min: 20,
        x_max: 30,
        y_min: -10,
        y_max: -5,
    };
    assert_eq!(find_probe_velocity(&target_area), (7, 9, 45));
}

#[test]
fn test_part_one() {
    let target_area = TargetArea {
        x_min: 277,
        x_max: 318,
        y_min: -92,
        y_max: -53,
    };
    assert_eq!(find_probe_velocity(&target_area).2, 4186);
}

