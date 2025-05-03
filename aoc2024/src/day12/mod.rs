use std::collections::{HashSet, VecDeque};

use aoclib::Runner;

#[derive(Default)]
pub struct Aoc2024_12 {
    map: Vec<Vec<char>>,
    dirs: Vec<(i32, i32)>,
    regions: Vec<HashSet<(i32, i32)>>,
}

impl Aoc2024_12 {
    pub fn new() -> Self {
        Self {
            map: Vec::new(),
            dirs: vec![(0, 1), (0, -1), (1, 0), (-1, 0)],
            regions: Vec::new(),
        }
    }

    fn parse_region(&mut self) {
        let mut regions = Vec::new();

        let m = self.map.len() as i32;
        let n = self.map[0].len() as i32;

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

                    for dir in &self.dirs {
                        let ni = ii + dir.0;
                        let nj = jj + dir.1;
                        if ni < 0 || nj < 0 || ni >= m || nj >= n {
                            continue;
                        }
                        if self.map[ni as usize][nj as usize] != self.map[i as usize][j as usize] {
                            continue;
                        }
                        if region.contains(&(ni, nj)) {
                            continue;
                        }
                        region.insert((ni, nj));
                        q.push_back((ni, nj));
                    }
                }
                seen.extend(region.iter());
                regions.push(region);
            }
        }
        self.regions = regions;
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
        self.parse_region();
    }

    fn part1(&mut self) -> Vec<String> {
        let m = self.map.len() as i32;
        let n = self.map[0].len() as i32;
        let price = |region: &HashSet<(i32, i32)>| -> usize {
            let mut res = 0;
            for (i, j) in region {
                res += 4;
                for dir in &self.dirs {
                    let ni = *i + dir.0;
                    let nj = *j + dir.1;
                    if ni < 0 || nj < 0 || ni >= m || nj >= n {
                        continue;
                    }
                    if region.contains(&(ni, nj)) {
                        res -= 1;
                    }
                }
            }
            res * region.len()
        };

        vec![format!("{}", self.regions.iter().map(price).sum::<usize>())]
    }

    fn part2(&mut self) -> Vec<String> {
        let price = |region: &HashSet<(i32, i32)>| -> usize {
            let mut corner_candidates = HashSet::new();

            for (i, j) in region {
                let fi = *i as f32;
                let fj = *j as f32;
                for (ii, jj) in [
                    (fi - 0.5, fj - 0.5),
                    (fi + 0.5, fj - 0.5),
                    (fi + 0.5, fj + 0.5),
                    (fi - 0.5, fj + 0.5),
                ] {
                    let scaled_ii = (ii * 100.0).round() as i32;
                    let scaled_jj = (jj * 100.0).round() as i32;
                    corner_candidates.insert((scaled_ii, scaled_jj));
                }
            }

            let mut corners = 0;

            for (i, j) in corner_candidates {
                let mut flags = vec![];
                for (ii, jj) in [
                    (i - 50, j - 50),
                    (i + 50, j - 50),
                    (i + 50, j + 50),
                    (i - 50, j + 50),
                ] {
                    flags.push(region.contains(&(ii / 100, jj / 100)));
                }

                let number: i32 = flags.iter().map(|f| if *f { 1 } else { 0 }).sum();
                match number {
                    1 => corners += 1,
                    3 => corners += 1,
                    2 => {
                        if flags == [true, false, true, false]
                            || flags == [false, true, false, true]
                        {
                            corners += 2;
                        }
                    }
                    4 => corners += 0,
                    _ => {
                        panic!("error number:{}", number)
                    }
                }
            }

            // println!("{} * {}", corners, region.len());
            corners * region.len()
        };

        // println!("{:?}", self.regions);
        vec![format!("{}", self.regions.iter().map(price).sum::<usize>())]
    }
}
