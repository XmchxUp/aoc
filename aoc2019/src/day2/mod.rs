use aoclib::Runner;

#[derive(Default)]
pub struct Aoc2019_2 {}

impl Aoc2019_2 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Aoc2019_2 {
    fn info(&self) -> (usize, usize) {
        (2019, 2)
    }

    fn parse(&mut self) {
        let inputs = aoclib::utils::read_file("./inputs/2019/02.txt");
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
