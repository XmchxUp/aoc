use std::collections::HashMap;

use aoclib::Runner;

#[derive(Default)]
pub struct Aoc2024_7 {
    nums: HashMap<u64, Vec<u64>>,
    part_two: bool,
}

impl Aoc2024_7 {
    pub fn new() -> Self {
        Self::default()
    }

    fn check(&self, target: &u64, cur: u64, nums: &[u64]) -> Option<u64> {
        if cur == *target && nums.is_empty() {
            return Some(cur);
        }

        if nums.is_empty() {
            return None;
        }

        if self.part_two {
            if let Some(v) = self.check(
                target,
                format!("{}{}", cur, nums[0]).parse().unwrap(),
                &nums[1..],
            ) {
                return Some(v);
            }
        }

        if let Some(v) = self.check(target, cur + nums[0], &nums[1..]) {
            return Some(v);
        }

        if let Some(v) = self.check(target, cur * nums[0], &nums[1..]) {
            return Some(v);
        }

        None
    }
}

impl Runner for Aoc2024_7 {
    fn info(&self) -> (usize, usize) {
        (2024, 7)
    }

    fn parse(&mut self) {
        // let inputs = aoclib::utils::read_file("./inputs/test07.txt");
        let inputs = aoclib::utils::read_file("./inputs/input07.txt");
        for line in inputs {
            let (target, nums) = line.split_once(":").unwrap();
            let target = target.parse::<u64>().unwrap();

            let vals = nums
                .split_whitespace()
                .map(|v| v.parse::<u64>().unwrap())
                .collect::<Vec<u64>>();

            self.nums.insert(target, vals);
        }
    }

    fn part1(&mut self) -> Vec<String> {
        let mut res = 0;
        for (target, nums) in &self.nums {
            if let Some(cur) = self.check(target, 0, nums) {
                res += cur;
            }
        }
        vec![format!("{}", res)]
    }

    fn part2(&mut self) -> Vec<String> {
        self.part_two = true;

        self.part1()
    }
}
