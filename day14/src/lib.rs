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

pub fn solve_part_2() {
    let mut test_polymer = Polymer::from_file("day14/input_test.txt");
    test_polymer.polymerize_loop_opt(4);
    test_polymer.print();
    println!("Test result: {}", test_polymer.calc_1());

    let mut polymer = Polymer::from_file("day14/input.txt");
    polymer.polymerize_loop_opt(40);
    println!("Result: {}", polymer.calc_1());
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
            rules: rules,
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

    fn polymerize_loop_opt(&mut self, number_of_cycles: u8) {
        let mut new_chain = self.chain.to_vec();
        let mut p = 0;
        loop {
            for i in 0..number_of_cycles {
                let key = new_chain[p].to_string() + &new_chain[p + 1].to_string();
                let value = *self.rules.get(&key).unwrap();
                new_chain.insert(p + 1, value);
                for _ in 0..i {
                    let ui  = i as usize;
                    let key = new_chain[p + ui].to_string() + &new_chain[p + ui + 1].to_string();
                    let value = *self.rules.get(&key).unwrap();
                    new_chain.insert(p + ui + 1, value);
                }
            }
            p = new_chain.len() - (self.chain.len() - p);
            if p >= new_chain.len() {
                break;
            };
            println!("Pointer: {} {}", p, new_chain.len());
        } /*
          for i in 0..self.chain.len() - 1 {
              new_chain.push(self.chain[i]);
              let second = self.chain[i + 1];
              for j in 0..number_of_cycles {
                  let key = new_chain[j as usize].to_string() + &new_chain[j as usize].to_string();
                  let value = *self.rules.get(&key).unwrap();
                  new_chain.push(value);
              }
              println!("Polymer part: {} {:?}", i, new_chain);
          }
          new_chain.push(*self.chain.last().unwrap());*/
        self.chain = new_chain;
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
