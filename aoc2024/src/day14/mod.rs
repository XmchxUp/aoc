use std::collections::HashMap;

use aoclib::Runner;

#[derive(Default)]
pub struct Aoc2024_8 {}

impl Aoc2024_8 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Aoc2024_8 {
    fn info(&self) -> (usize, usize) {
        (2024, 5)
    }

    fn parse(&mut self) {
        let inputs = aoclib::utils::read_file("./inputs/test05.txt");
    }

    fn part1(&mut self) -> Vec<String> {
        let mut res = 0;
        vec![format!("{}", res)]
    }

    fn part2(&mut self) -> Vec<String> {
        let mut res = 0;
        vec![format!("{}", res)]
    }
}
