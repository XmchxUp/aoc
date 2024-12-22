use aoclib::Runner;

#[derive(Default)]
pub struct Aoc2024_13 {}

impl Aoc2024_13 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Aoc2024_13 {
    fn info(&self) -> (usize, usize) {
        (2024, 13)
    }

    fn parse(&mut self) {
        let _ = aoclib::utils::read_file("./inputs/test05.txt");
    }

    fn part1(&mut self) -> Vec<String> {
        let res = 0;
        vec![format!("{}", res)]
    }

    fn part2(&mut self) -> Vec<String> {
        let res = 0;
        vec![format!("{}", res)]
    }
}
