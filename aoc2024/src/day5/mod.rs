use std::collections::{HashMap, HashSet};

use aoclib::Runner;

#[derive(Default)]
pub struct Aoc2024_5 {
    rules: HashMap<u32, HashSet<u32>>,
    seqs: Vec<Vec<u32>>,
}

impl Aoc2024_5 {
    pub fn new() -> Self {
        Self::default()
    }

    fn is_correct_seq(&self, seq: &[u32]) -> bool {
        let mut idx = seq.len() - 1;
        while idx > 0 {
            if !self.rules.contains_key(&seq[idx]) {
                break;
            }
            let rules = self.rules.get(&seq[idx]).unwrap();
            for ele in seq.iter().take(idx) {
                if rules.contains(ele) {
                    return false;
                }
            }
            idx -= 1;
        }
        true
    }
}

impl Runner for Aoc2024_5 {
    fn info(&self) -> (usize, usize) {
        (2024, 5)
    }

    fn parse(&mut self) {
        // let inputs = aoclib::utils::read_file("./inputs/test05.txt");
        let inputs = aoclib::utils::read_file("./inputs/input05.txt");

        let mut parse_seqs = false;
        for line in inputs {
            if line.is_empty() {
                parse_seqs = true;
                continue;
            }

            if !parse_seqs {
                let (a, b) = line.split_once('|').unwrap();
                let a = a.parse::<u32>().unwrap();
                let b = b.parse::<u32>().unwrap();

                self.rules.entry(a).or_default();
                self.rules.get_mut(&a).unwrap().insert(b);
            } else {
                self.seqs
                    .push(line.split(',').map(|v| v.parse::<u32>().unwrap()).collect());
            }
        }
    }

    fn part1(&mut self) -> Vec<String> {
        let mut res = 0;
        for seq in &self.seqs {
            if self.is_correct_seq(seq) {
                res += seq[seq.len() / 2];
            }
        }
        vec![format!("{}", res)]
    }

    fn part2(&mut self) -> Vec<String> {
        let mut res = 0;
        for seq in &self.seqs {
            if !self.is_correct_seq(seq) {
                let mut sorted_seq = seq.clone();
                sorted_seq.sort_by(|&a, &b| {
                    if self.rules.contains_key(&a) && self.rules.get(&a).unwrap().contains(&b) {
                        std::cmp::Ordering::Less
                    } else {
                        std::cmp::Ordering::Greater
                    }
                });
                if self.is_correct_seq(&sorted_seq) {
                    res += sorted_seq[sorted_seq.len() / 2];
                }
            }
        }
        vec![format!("{}", res)]
    }
}
