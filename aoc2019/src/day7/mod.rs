use aoclib::Runner;

#[derive(Default)]
pub struct Aoc2019_7 {}

impl Aoc2019_7 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Aoc2019_7 {
    fn info(&self) -> (usize, usize) {
        (2019, 7)
    }

    fn parse(&mut self) {
        let inputs = aoclib::utils::read_file("./inputs/2019/07.txt");
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
