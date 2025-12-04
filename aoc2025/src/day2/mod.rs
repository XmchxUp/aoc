use std::collections::HashSet;

use aoclib::Runner;

type Range = (u64, u64);

#[derive(Default)]
pub struct Aoc2025_2 {
    ranges: Vec<Range>,
}

impl Aoc2025_2 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Aoc2025_2 {
    fn info(&self) -> (usize, usize) {
        (2025, 2)
    }

    fn parse(&mut self) {
        let inputs = aoclib::utils::read_file("./inputs/2025/02.txt");
        assert!(inputs.len() >= 1);

        for line in inputs[0].split(',') {
            if line.trim().is_empty() {
                continue;
            }

            let parts = line.split_once('-').expect("Invalid format");
            let left: u64 = parts.0.parse().expect("Invalid integer");
            let right: u64 = parts.1.parse().expect("Invalid integer");

            assert!(left <= right);
            self.ranges.push((left, right));
        }
    }

    fn part1(&mut self) -> Vec<String> {
        let mut count = 0;
        // ID=S*10^L+S
        // L = len(s)
        for range in &self.ranges {
            let mut s = 0;
            let mut tmp = range.0;
            while tmp != 0 {
                tmp = tmp / 10;
                s = tmp;
            }

            loop {
                let l = s.to_string().len();
                let id = s * 10_u64.pow(l as u32) + s;
                if id > range.1 {
                    break;
                }
                if id >= range.0 {
                    count += id;
                }
                s += 1;
            }
        }
        vec![format!("{}", count)]
    }

    fn part2(&mut self) -> Vec<String> {
        let mut res = 0;

        for range in &self.ranges {
            let mut i = range.0;
            while i <= range.1 {
                let s = i.to_string();
                let n = s.len();
                for k in 2..=n {
                    if n % k != 0 {
                        continue;
                    }
                    let chunk_size = n / k;
                    let chunk = &s[..chunk_size];

                    if chunk.repeat(k) == s {
                        res += i;
                        break;
                    }
                }
                i += 1;
            }
        }
        vec![format!("{}", res)]
    }
}
