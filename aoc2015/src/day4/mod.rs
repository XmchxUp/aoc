use aoclib::Runner;

#[derive(Default)]
pub struct Aoc2015_4 {
    secret: String,
}

impl Aoc2015_4 {
    pub fn new() -> Self {
        Self::default()
    }

    fn find_lowest_number_with_prefix(&self, prefix: &str) -> u64 {
        for i in 1.. {
            let x = format!("{}{}", self.secret, i);
            let digest = md5::compute(x);
            let hash = format!("{:x}", digest);
            if hash.starts_with(prefix) {
                return i;
            }
        }
        unreachable!("should find a number")
    }
}

impl Runner for Aoc2015_4 {
    fn info(&self) -> (usize, usize) {
        (2015, 4)
    }

    fn parse(&mut self) {
        let inputs = aoclib::utils::read_file("./inputs/2015/04.txt");
        self.secret = inputs[0].clone();
    }

    fn part1(&mut self) -> Vec<String> {
        vec![format!("{}", self.find_lowest_number_with_prefix("00000"))]
    }

    fn part2(&mut self) -> Vec<String> {
        vec![format!("{}", self.find_lowest_number_with_prefix("000000"))]
    }
}
