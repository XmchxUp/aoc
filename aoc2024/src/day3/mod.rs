use aoclib::Runner;
use regex::Regex;

#[derive(Default)]
pub struct Aoc2024_3 {
    inputs: Vec<String>,
}

impl Aoc2024_3 {
    pub fn new() -> Self {
        Self::default()
    }

    fn helper(&self, input: &str) -> u32 {
        let pattern = r"mul\((\d{1,3}),(\d{1,3})\)";
        let re = Regex::new(pattern).unwrap();
        let mut res = 0;

        for cap in re.captures_iter(input) {
            let a = cap.get(1).map_or(0, |m| m.as_str().parse::<u32>().unwrap());
            let b = cap.get(2).map_or(0, |m| m.as_str().parse::<u32>().unwrap());
            res += a * b;
        }
        res
    }
}

impl Runner for Aoc2024_3 {
    fn info(&self) -> (usize, usize) {
        (2024, 3)
    }

    fn parse(&mut self) {
        let inputs = aoclib::utils::read_file("./inputs/input03.txt");
        // let inputs = aoclib::utils::read_file("./inputs/test03.txt");
        self.inputs = inputs;
    }

    fn part1(&mut self) -> Vec<String> {
        vec![format!("{}", self.helper(&self.inputs.join("")))]
    }

    fn part2(&mut self) -> Vec<String> {
        let mut res = 0;

        let input = self.inputs.join("");
        let mut tmp = input.as_str();

        while !tmp.is_empty() {
            let end = tmp.find("don't()").unwrap_or(tmp.len());
            res += self.helper(&tmp[..end]);
            if let Some(p) = tmp[end..].find("do()") {
                tmp = &tmp[end + p..];
            } else {
                break;
            }
        }
        vec![format!("{}", res)]
    }
}
