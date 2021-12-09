pub fn solve_part_1() {}
pub fn solve_part_2() {}

struct Point {
    x: u16,
    y: u16
}

struct Grid {
    rows: Vec<Vec<Vec<Point>>>
}

impl Grid {
    fn new() -> Self {
        Grid {rows : vec![]}
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
