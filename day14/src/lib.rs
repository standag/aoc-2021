use std::{collections::HashMap, fs};

pub fn solve_part_1() {
    let mut test_polymer = Polymer::from_file("day14/input_test.txt");
    test_polymer.print();
    test_polymer.polymerize();
    test_polymer.print();
    test_polymer.polymerize_loop(9);
    test_polymer.print();
    println!("Test result: {}", test_polymer.calc_1());

    let mut polymer = Polymer::from_file("day14/input.txt");
    polymer.polymerize_loop(10);
    println!("Result: {}", polymer.calc_1());
}

fn calc(chain: &HashMap<String, u128>, first: &char) -> u128 {
    let mut counter = HashMap::new();
    chain.iter().for_each(|s| {
        let c = s.0.chars().nth(1).unwrap();
        let value = counter.entry(c).or_insert(0);
        *value += s.1;
    });
    let value = counter.entry(*first).or_insert(0);
    *value += 1;
    counter.values().max().unwrap() - counter.values().min().unwrap()
}

pub fn solve_part_2() {
    let mut test_polymer = Polymer::from_file("day14/input_test.txt");
    let test_chain = test_polymer.polymerize_loop_opt(40);
    let test_result = calc(&test_chain, &test_polymer.chain.first().unwrap());
    println!("Test result: {:?}", test_result);

    let mut polymer = Polymer::from_file("day14/input.txt");
    let final_chain = polymer.polymerize_loop_opt(40);
    let result = calc(&final_chain, &polymer.chain[0]);
    println!("Result: {:?}", result);
}

#[derive(Debug)]
struct Polymer {
    chain: Vec<char>,
    rules: HashMap<String, char>,
}

impl Polymer {
    fn from_file(filename: &str) -> Self {
        let mut rules = HashMap::new();
        let raws: Vec<String> = fs::read_to_string(filename)
            .unwrap()
            .split_terminator("\n\n")
            .take(2)
            .map(|x| x.to_string())
            .collect();
        raws[1].split_terminator("\n").for_each(|r| {
            let raw_rule: Vec<String> = r.split_terminator(" -> ").map(|x| x.to_string()).collect();
            rules.insert(raw_rule[0].to_string(), raw_rule[1].chars().nth(0).unwrap());
        });
        Polymer {
            chain: raws[0].chars().collect::<Vec<char>>().to_vec(),
            rules,
        }
    }

    fn polymerize(&mut self) {
        let mut new_chain = vec![];
        for i in 0..self.chain.len() - 1 {
            new_chain.push(self.chain[i]);
            let key = self.chain[i].to_string() + &self.chain[i + 1].to_string();
            new_chain.push(*self.rules.get(&key).unwrap());
            //new_chain.push(self.chain[i+1]);
        }
        new_chain.push(*self.chain.last().unwrap());
        self.chain = new_chain;
    }

    fn polymerize_loop(&mut self, number_of_cycles: u8) {
        (0..number_of_cycles).for_each(|i| {
            println!("Step {} ...", i);
            self.polymerize();
        })
    }

    fn polymerize_loop_opt(&mut self, number_of_cycles: u8) -> HashMap<String, u128> {
        let mut chain = HashMap::new();
        let mut new_chain = HashMap::new();
        (0..(self.chain.len() - 1)).for_each(|i| {
            let key = self.chain[i].to_string() + &self.chain[i + 1].to_string();
            if let Some(value) = chain.get_mut(&key) {
                *value += 1;
            } else {
                chain.insert(key, 1);
            }
        });
        for _ in 0..number_of_cycles {
            chain.iter().for_each(|x| {
                let c = self.rules.get(x.0).unwrap();
                let key1 = x.0.chars().nth(0).unwrap().to_string() + &c.to_string();
                let key2 = c.to_string() + &x.0.chars().nth(1).unwrap().to_string();
                let value1 = new_chain.entry(key1).or_insert(0);
                *value1 += x.1;
                let value2 = new_chain.entry(key2).or_insert(0);
                *value2 += x.1;
            });
            chain = new_chain.clone();
            new_chain = HashMap::new();
        }
        chain
    }

    fn print(&self) {
        println!(
            "{}",
            self.chain
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join("")
        );
    }

    fn calc_1(&self) -> usize {
        let mut elements = self.chain.clone();
        elements.dedup();
        let element_counts = elements
            .iter()
            .map(|x| self.chain.iter().filter(|y| *y == x).count())
            .collect::<Vec<usize>>();
        let max = element_counts.iter().fold(0, |e, acc| e.max(*acc));
        let min = element_counts.iter().fold(usize::MAX, |e, acc| e.min(*acc));
        max - min
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
