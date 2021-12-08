use day01;
use day02;
use day03;
use day04;
use day07;
use day08;

fn main() {
    let mut args = std::env::args();
    args.next();

    let day = args.next().expect("no day given").parse::<u8>().unwrap();
    let part = args.next().expect("no part given").parse::<u8>().unwrap();

    match (day, part) {
        (1, 1) => day01::solve_part_1(),
        (1, 2) => day01::solve_part_2(),
        (2, 1) => day02::solve_part_1(),
        (2, 2) => day02::solve_part_2(),
        (3, 1) => day03::solve_part_1(),
        (3, 2) => day03::solve_part_2(),
        (4, 1) => day04::solve_part_1(),
        (4, 2) => day04::solve_part_2(),
        (7, 1) => day07::solve_part_1(),
        (7, 2) => day07::solve_part_2(),
        (8, 1) => day08::solve_part_1(),
        (8, 2) => day08::solve_part_2(),
        _ => (),
    }
}
