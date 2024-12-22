use std::collections::{HashSet, VecDeque};

use aoclib::Runner;

#[derive(Default)]
pub struct Aoc2024_10 {
    map: Vec<Vec<u32>>,
}

impl Aoc2024_10 {
    pub fn new() -> Self {
        Self::default()
    }

    fn get_score(&self, start_i: usize, start_j: usize, patr2: bool) -> usize {
        let m = self.map.len() as i32;
        let n = self.map[0].len() as i32;
        let mut score = 0;
        let dirs = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];

        let mut q = VecDeque::new();
        q.push_back((start_i as i32, start_j as i32, 0));
        let mut seen = HashSet::new();
        seen.insert((start_i as i32, start_j as i32));

        while !q.is_empty() {
            let (i, j, depth) = q.pop_front().unwrap();
            if depth == 9 {
                score += 1;
                continue;
            }

            for dir in &dirs {
                let new_i = i + dir.0;
                let new_j = j + dir.1;

                if new_i < 0 || new_j < 0 || new_i >= m || new_j >= n {
                    continue;
                }

                if seen.contains(&(new_i, new_j))
                    || self.map[new_i as usize][new_j as usize] != depth + 1
                {
                    continue;
                }
                if !patr2 {
                    seen.insert((new_i, new_j));
                }
                q.push_back((new_i, new_j, depth + 1));
            }
        }

        score
    }
}

impl Runner for Aoc2024_10 {
    fn info(&self) -> (usize, usize) {
        (2024, 10)
    }

    fn parse(&mut self) {
        // let inputs =
        // aoclib::utils::read_file(format!("./inputs/test{:02}.txt", self.info().1).as_str());
        let inputs =
            aoclib::utils::read_file(format!("./inputs/input{:02}.txt", self.info().1).as_str());
        for line in inputs {
            self.map
                .push(line.chars().map(|ch| ch.to_digit(10).unwrap()).collect());
        }
        // println!("{:?}", self.map);
    }

    fn part1(&mut self) -> Vec<String> {
        let mut res = 0;

        let m = self.map.len();
        let n = self.map[0].len();

        for i in 0..m {
            for j in 0..n {
                if self.map[i][j] == 0 {
                    res += self.get_score(i, j, false);
                }
            }
        }
        vec![format!("{}", res)]
    }

    fn part2(&mut self) -> Vec<String> {
        let mut res = 0;
        let m = self.map.len();
        let n = self.map[0].len();

        for i in 0..m {
            for j in 0..n {
                if self.map[i][j] == 0 {
                    res += self.get_score(i, j, true);
                }
            }
        }
        vec![format!("{}", res)]
    }
}
