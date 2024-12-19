use std::collections::HashMap;

use aoclib::Runner;

#[derive(Default)]
pub struct Aoc2024_11 {
    nums: Vec<u64>,
    cache: HashMap<(u64, u64), u64>,
}

impl Aoc2024_11 {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn count(&mut self, stone: u64, steps: u64) -> u64 {
        if let Some(&cached_result) = self.cache.get(&(stone, steps)) {
            return cached_result;
        }

        if steps == 0 {
            return 1;
        }

        if stone == 0 {
            let result = self.count(1, steps - 1);
            self.cache.insert((stone, steps), result);
            return result;
        }

        let nstr = stone.to_string();

        let result = if nstr.len() % 2 == 0 {
            let mid = nstr.len() / 2;
            let (a, b) = nstr.split_at(mid);
            self.count(a.parse().unwrap(), steps - 1) + self.count(b.parse().unwrap(), steps - 1)
        } else {
            self.count(stone * 2024, steps - 1)
        };

        self.cache.insert((stone, steps), result);

        result
    }
}

impl Runner for Aoc2024_11 {
    fn info(&self) -> (usize, usize) {
        (2024, 11)
    }

    fn parse(&mut self) {
        let inputs =
            aoclib::utils::read_file(format!("./inputs/test{:02}.txt", self.info().1).as_str());
        let inputs =
            aoclib::utils::read_file(format!("./inputs/input{:02}.txt", self.info().1).as_str());
        assert_eq!(inputs.len(), 1);
        for line in inputs {
            self.nums = line.split(' ').map(|v| v.parse::<u64>().unwrap()).collect();
        }
        // println!("{:?}", self.nums);
    }

    fn part1(&mut self) -> Vec<String> {
        let mut nums = self.nums.clone();
        for _ in 0..25 {
            let n = nums.len();
            let mut i = 0;
            while i < n {
                if nums[i] == 0 {
                    nums[i] = 1
                }
                let nstr = nums[i].to_string();
                if nstr.len() % 2 == 0 {
                    let mid = nstr.len() / 2;
                    let (a, b) = nstr.split_at(mid);
                    nums[i] = a.parse().unwrap();
                    nums.push(b.parse().unwrap());
                } else {
                    nums[i] *= 2024;
                }
                i += 1;
            }
            // println!("{:?}", self.nums);
        }
        vec![format!("{}", nums.len())]
    }

    fn part2(&mut self) -> Vec<String> {
        let nums = self.nums.clone();
        vec![format!(
            "{}",
            nums.iter().map(|v| self.count(*v, 75)).sum::<u64>()
        )]
    }
}
