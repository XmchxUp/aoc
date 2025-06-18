use std::collections::{HashSet, VecDeque};

use aoclib::Runner;

type Point = (i32, i32);

#[derive(Default)]
pub struct Aoc2023_21 {
    map: Vec<Vec<char>>,
    start: Point,
}

impl Aoc2023_21 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Aoc2023_21 {
    fn info(&self) -> (usize, usize) {
        (2023, 21)
    }

    fn parse(&mut self) {
        let inputs = aoclib::utils::read_file("./inputs/2023/21.txt");
        for line in inputs {
            self.map.push(line.chars().collect());
        }
        for (i, row) in self.map.iter().enumerate() {
            for (j, ch) in row.iter().enumerate() {
                if *ch == 'S' {
                    self.start = (i as i32, j as i32);
                }
            }
        }
    }

    fn part1(&mut self) -> Vec<String> {
        let mut visisted = HashSet::new();
        let mut queue = VecDeque::new();

        queue.push_back((self.start, 0));

        let dirs = [(0, 1), (0, -1), (1, 0), (-1, 0)];

        let steps = 64;

        let mut reachable_steps = HashSet::new();

        while let Some((pos, step)) = queue.pop_front() {
            if step == steps {
                reachable_steps.insert(pos);
                continue;
            }

            for (dx, dy) in dirs {
                let next = (pos.0 + dx, pos.1 + dy);
                if self.map[next.0 as usize][next.1 as usize] != '#'
                    && !visisted.contains(&(next, step + 1))
                {
                    visisted.insert((next, step + 1));
                    queue.push_back((next, step + 1));
                }
            }
        }

        vec![format!("{}", reachable_steps.len())]
    }

    fn part2(&mut self) -> Vec<String> {
        let res = 0;
        vec![format!("{}", res)]
    }
}
