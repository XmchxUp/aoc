use std::collections::{HashSet, VecDeque};

use aoclib::Runner;

#[derive(Default)]
pub struct Aoc2024_12 {
    map: Vec<Vec<char>>,
}

impl Aoc2024_12 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Aoc2024_12 {
    fn info(&self) -> (usize, usize) {
        (2024, 12)
    }

    fn parse(&mut self) {
        // let inputs =
        // aoclib::utils::read_file(format!("./inputs/test{:02}.txt", self.info().1).as_str());
        let inputs =
            aoclib::utils::read_file(format!("./inputs/input{:02}.txt", self.info().1).as_str());

        for line in inputs {
            self.map.push(line.chars().collect());
        }
    }

    fn part1(&mut self) -> Vec<String> {
        let mut regions = Vec::new();
        let dirs = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];

        let m = self.map.len();
        let n = self.map[0].len();

        let mut seen = HashSet::new();
        for i in 0..m {
            for j in 0..n {
                if seen.contains(&(i, j)) {
                    continue;
                }
                seen.insert((i, j));

                let mut q = VecDeque::new();
                q.push_back((i, j));

                let mut region = HashSet::new();
                region.insert((i, j));

                while !q.is_empty() {
                    let (ii, jj) = q.pop_front().unwrap();

                    for dir in &dirs {
                        let ni = ii as i32 + dir.0;
                        let nj = jj as i32 + dir.1;
                        if ni < 0 || nj < 0 || ni >= m as i32 || nj >= n as i32 {
                            continue;
                        }
                        if self.map[ni as usize][nj as usize] != self.map[i][j] {
                            continue;
                        }
                        if region.contains(&(ni as usize, nj as usize)) {
                            continue;
                        }
                        region.insert((ni as usize, nj as usize));
                        q.push_back((ni as usize, nj as usize));
                    }
                }
                let _ = seen.union(&region);
                regions.push(region);
            }
        }

        let perimeter = |region: &HashSet<(usize, usize)>| -> usize {
            let mut res = 0;
            for (i, j) in region {
                res += 4;
                for dir in &dirs {
                    let ni = *i as i32 + dir.0;
                    let nj = *j as i32 + dir.1;
                    if ni < 0 || nj < 0 || ni >= m as i32 || nj >= n as i32 {
                        continue;
                    }
                    if region.contains(&(ni as usize, nj as usize)) {
                        res -= 1;
                    }
                }
            }
            res
        };

        vec![format!("{}", regions.iter().map(perimeter).sum::<usize>())]
    }

    fn part2(&mut self) -> Vec<String> {
        let res = 0;
        vec![format!("{}", res)]
    }
}
