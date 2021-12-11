use std::collections::HashMap;
use std::fs;

pub fn solve_part_1() {
    let mut fish = fs::read_to_string("day06/input.txt")
        .unwrap()
        .split_terminator(",")
        .map(|x| x.trim().parse::<u8>().unwrap())
        .collect::<Vec<_>>();

    let mut to_append = vec![];
    for i in 0..80 {
        // println!("{:?}", fish);
        println!("{}", i);

        fish = fish
            .iter()
            .map(|x| match x {
                0 => 6,
                _ => x - 1,
            })
            .collect();
        fish.append(&mut to_append);
        to_append = fish
            .iter()
            .filter(|f| **f == 0)
            .map(|_| 8)
            .collect::<Vec<_>>();
        //(0..news).as_vec().iter().for_each(|x| fish.push(8));
        //news = fish.iter().filter(|x| **x == 0).count();
    }

    println!(
        "{:?} {} {}",
        fish,
        fish.iter().map(|x| *x as usize).sum::<usize>(),
        fish.iter().count()
    );
}
pub fn solve_part_2() {
    let fish = fs::read_to_string("day06/input.txt")
        .unwrap()
        .split_terminator(",")
        .map(|x| x.trim().parse::<u8>().unwrap())
        .collect::<Vec<_>>();

    let mut mem = HashMap::new();

    for i in 0..=8 {
        mem.insert(i, fish.iter().filter(|x| **x == i).count());
    }

    for _ in 0..256 {
        let zeros = mem[&0];
        for i in 1..=8 {
            mem.insert(i - 1, mem[&i]);
        }
        mem.insert(6, mem[&6] + zeros);
        mem.insert(8, zeros);
    }
    println!("{:?} {}", mem, mem.iter().map(|(_, j)| j).sum::<usize>(),);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
