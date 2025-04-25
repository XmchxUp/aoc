use aoclib::Runner;

#[derive(Default)]
pub struct Aoc2023_21 {}

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
