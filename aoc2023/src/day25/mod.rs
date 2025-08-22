use aoclib::Runner;

#[derive(Default)]
pub struct Aoc2023_25 {}

impl Aoc2023_25 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Aoc2023_25 {
    fn info(&self) -> (usize, usize) {
        (2023, 25)
    }

    fn parse(&mut self) {
        let inputs = aoclib::utils::read_file("./inputs/2023/25.txt");
        let inputs = aoclib::utils::read_file("./inputs/2023/test.txt");
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
