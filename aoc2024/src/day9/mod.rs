use std::collections::HashMap;

use aoclib::Runner;

#[derive(Default)]
pub struct Aoc2024_9 {
    eles: HashMap<u64, u64>,
    max_idx: u64,
}

impl Aoc2024_9 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Aoc2024_9 {
    fn info(&self) -> (usize, usize) {
        (2024, 9)
    }

    fn parse(&mut self) {
        let inputs = aoclib::utils::read_file("./inputs/test09.txt");
        let inputs = aoclib::utils::read_file("./inputs/input09.txt");

        let mut index = 0;
        let mut id = 0;
        let mut i = 0;
        let mut j = 1;

        for line in inputs {
            let chs = line.chars().collect::<Vec<char>>();

            while i < chs.len() {
                let cnt = chs[i].to_digit(10).unwrap() as u64;

                for _ in 0..cnt {
                    self.eles.insert(index, id);
                    index += 1;
                }

                if j < chs.len() {
                    let free_cnt = chs[j].to_digit(10).unwrap() as u64;
                    index += free_cnt;
                }

                id += 1;
                i += 2;
                j = i + 1;
            }
            self.max_idx = index;
        }
        // println!("{:?}", self.eles);
    }

    fn part1(&mut self) -> Vec<String> {
        let mut res = 0;

        let mut i_idx = 0;
        let mut j_idx = self.max_idx - 1;

        while i_idx < j_idx {
            while j_idx > i_idx && !self.eles.contains_key(&j_idx) {
                j_idx -= 1;
            }

            while i_idx < j_idx && self.eles.contains_key(&i_idx) {
                i_idx += 1;
            }

            if i_idx < j_idx {
                let v = self.eles.remove(&j_idx).unwrap();
                self.eles.insert(i_idx, v);
            }
            i_idx += 1;
            j_idx -= 1;
        }

        res = self
            .eles
            .iter()
            .map(|(k, v)| {
                // println!("{}*{}", k, v);
                k * v
            })
            .sum();

        vec![format!("{}", res)]
    }

    fn part2(&mut self) -> Vec<String> {
        let mut res = 0;
        vec![format!("{}", res)]
    }
}
