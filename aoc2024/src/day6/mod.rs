use std::collections::HashSet;

use aoclib::Runner;

#[derive(Default)]
pub struct Aoc2024_6 {
    map: Vec<Vec<char>>,
}

impl Aoc2024_6 {
    pub fn new() -> Self {
        Self::default()
    }

    fn found_start(&self) -> (i32, i32) {
        let m = self.map.len();
        let n = self.map[0].len();
        for i in 0..m {
            for j in 0..n {
                if self.map[i][j] == '^' {
                    return (i as i32, j as i32);
                }
            }
        }
        panic!("error")
    }
}

impl Runner for Aoc2024_6 {
    fn info(&self) -> (usize, usize) {
        (2024, 6)
    }

    fn parse(&mut self) {
        // let inputs = aoclib::utils::read_file("./inputs/test06.txt");
        let inputs = aoclib::utils::read_file("./inputs/input06.txt");
        self.map = inputs.iter().map(|line| line.chars().collect()).collect();
    }

    fn part1(&mut self) -> Vec<String> {
        let mut visited = HashSet::new();

        let m = self.map.len();
        let n = self.map[0].len();
        let dirs = [(-1, 0), (0, 1), (1, 0), (0, -1)];

        let (mut cur_i, mut cur_j) = self.found_start();

        visited.insert((cur_i, cur_j));
        let mut cur_dir = 0;
        loop {
            let next_i = cur_i + dirs[cur_dir].0;
            let next_j = cur_j + dirs[cur_dir].1;

            if next_i < 0 || next_j < 0 || next_i >= m as i32 || next_j >= n as i32 {
                break;
            }

            if self.map[next_i as usize][next_j as usize] == '#' {
                cur_dir = (cur_dir + 1) % 4;
                continue;
            }
            cur_i = next_i;
            cur_j = next_j;
            visited.insert((cur_i, cur_j));
        }

        vec![format!("{}", visited.len())]
    }

    fn part2(&mut self) -> Vec<String> {
        let mut res = 0;
        let m = self.map.len();
        let n = self.map[0].len();
        let dirs = [(-1, 0), (0, 1), (1, 0), (0, -1)];

        let (start_i, start_j) = self.found_start();
        let simulate = |map: &Vec<Vec<char>>| -> bool {
            let mut visited_states = HashSet::new();
            let mut cur_i = start_i;
            let mut cur_j = start_j;
            let mut cur_dir = 0;

            visited_states.insert((cur_i, cur_j, cur_dir));

            loop {
                let next_i = cur_i + dirs[cur_dir].0;
                let next_j = cur_j + dirs[cur_dir].1;

                if next_i < 0 || next_j < 0 || next_i >= m as i32 || next_j >= n as i32 {
                    break;
                }

                if map[next_i as usize][next_j as usize] == '#' {
                    cur_dir = (cur_dir + 1) % 4;
                    continue;
                }

                cur_i = next_i;
                cur_j = next_j;

                if visited_states.contains(&(cur_i, cur_j, cur_dir)) {
                    return true;
                }

                visited_states.insert((cur_i, cur_j, cur_dir));
            }
            false
        };

        for i in 0..m {
            for j in 0..n {
                if self.map[i][j] == '.' {
                    let mut tmp_map = self.map.clone();
                    tmp_map[i][j] = '#';
                    if simulate(&tmp_map) {
                        res += 1;
                    }
                }
            }
        }

        vec![format!("{}", res)]
    }
}
