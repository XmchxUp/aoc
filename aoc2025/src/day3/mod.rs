use aoclib::Runner;

#[derive(Default)]
pub struct Aoc2025_3 {}

impl Aoc2025_3 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Aoc2025_3 {
    fn info(&self) -> (usize, usize) {
        (2025, 3)
    }

    fn parse(&mut self) {
        let inputs = aoclib::utils::read_file("./inputs/2025/03.txt");
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
