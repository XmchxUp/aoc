use std::collections::HashMap;

use aoclib::Runner;

#[derive(Default)]
pub struct Aoc2015_5 {
    inputs: Vec<String>,
}

impl Aoc2015_5 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Aoc2015_5 {
    fn info(&self) -> (usize, usize) {
        (2015, 5)
    }

    fn parse(&mut self) {
        let inputs = aoclib::utils::read_file("./inputs/2015/05.txt");
        self.inputs = inputs;
    }

    fn part1(&mut self) -> Vec<String> {
        fn is_nice(v: &str) -> bool {
            let vowels = ['a', 'e', 'i', 'o', 'u'];
            let banned = ["ab", "cd", "pq", "xy"];

            if banned.iter().any(|&b| v.contains(b)) {
                return false;
            }

            if v.chars().filter(|c| vowels.contains(c)).count() < 3 {
                return false;
            }

            v.chars()
                .collect::<Vec<_>>()
                .windows(2)
                .any(|pair| pair[0] == pair[1])
        }

        vec![format!(
            "{}",
            self.inputs.iter().filter(|v| is_nice(&v)).count()
        )]
    }

    fn part2(&mut self) -> Vec<String> {
        fn is_nice(v: &str) -> bool {
            let chs: Vec<char> = v.chars().collect();
            let len = chs.len();

            let mut pairs = HashMap::new();

            let mut has_pair = false;
            for i in 0..len - 1 {
                let pair = (chs[i], chs[i + 1]);
                if let Some(pair_i) = pairs.get(&pair) {
                    if i - pair_i > 1 {
                        has_pair = true;
                    }
                } else {
                    pairs.insert(pair, i);
                }
            }

            has_pair && (0..len - 2).any(|i| chs[i] == chs[i + 2])
        }

        vec![format!(
            "{}",
            self.inputs.iter().filter(|v| is_nice(&v)).count()
        )]
    }
}
