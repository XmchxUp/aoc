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

    fn reachable_steps(&self, steps: usize) -> usize {
        let mut current = HashSet::new();
        current.insert(self.start);

        let mut next = HashSet::new();
        let dirs = [(0, 1), (0, -1), (1, 0), (-1, 0)];

        let h = self.map.len() as i32;
        let w = self.map[0].len() as i32;
        for _ in 1..=steps {
            next.clear();
            for (x, y) in &current {
                for (dx, dy) in dirs {
                    let nx = x + dx;
                    let ny = y + dy;

                    let cx = ((nx % w) + w) % w;
                    let cy = ((ny % h) + h) % h;

                    if self.map[cx as usize][cy as usize] != '#' {
                        next.insert((nx, ny));
                    }
                }
            }
            std::mem::swap(&mut current, &mut next);
        }
        current.len()
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
        // let steps = 26501365;
        // start = (65, 65)
        // println!("{} {}", self.start.0, self.start.1);
        // grid_length*grid_height = 131*131

        // 26501365 = 202300 * 131 + 65
        // f(n) = a*n^2 + b*n + c
        let base_steps = 65;
        let v0 = self.reachable_steps(base_steps) as i128;
        let v1 = self.reachable_steps(base_steps + 131) as i128;
        let v2 = self.reachable_steps(base_steps + 131 * 2) as i128;

        let n = 202300;
        vec![format!("{}", interpolate(n, v0, v1, v2))]
    }
}

fn interpolate(n: i128, v0: i128, v1: i128, v2: i128) -> i128 {
    // Lagrange interpolation for n-th value
    let a = v0;
    let b = v1 - v0;
    let c = v2 - 2 * v1 + v0;
    a + b * n + c * n * (n - 1) / 2
}
