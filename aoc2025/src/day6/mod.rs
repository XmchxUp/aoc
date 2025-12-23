use aoclib::Runner;

#[derive(Default)]
pub struct Aoc2025_6 {
    #[cfg(test)]
    lines: Vec<String>,
    op_nums: Vec<String>,
    row_nums: Vec<Vec<u64>>,
}

impl Aoc2025_6 {
    pub fn new() -> Self {
        Self::default()
    }

    #[cfg(test)]
    pub fn with_input(input: &str) -> Self {
        Self {
            lines: input.lines().map(|s| s.to_string()).collect(),
            op_nums: vec![],
            row_nums: vec![],
        }
    }
}

impl Runner for Aoc2025_6 {
    fn info(&self) -> (usize, usize) {
        (2025, 6)
    }

    fn parse(&mut self) {
        #[cfg(not(test))]
        let inputs = aoclib::utils::read_file("./inputs/2025/06.txt");

        #[cfg(test)]
        let inputs = self.lines.clone();

        self.row_nums = inputs[..inputs.len() - 1]
            .iter()
            .map(|line| {
                line.split_whitespace()
                    .map(|x| x.parse::<u64>().unwrap())
                    .collect::<Vec<u64>>()
            })
            .collect();

        self.op_nums = inputs[inputs.len() - 1]
            .split_whitespace()
            .map(|x| x.to_string())
            .collect();

        // println!("{:?} {:?}", self.op_nums, self.row_nums);
    }

    fn part1(&mut self) -> Vec<String> {
        let mut res = 0;
        let mut idx = 0;
        for op in &self.op_nums {
            match op.as_str() {
                "+" => {
                    let mut tmp = 0;
                    for row in &self.row_nums {
                        tmp += row[idx];
                    }
                    res += tmp;
                }
                "*" => {
                    let mut tmp = 1;
                    for row in &self.row_nums {
                        tmp *= row[idx];
                    }

                    res += tmp;
                }
                _ => panic!("unexpected operation"),
            }
            idx += 1;
        }
        vec![format!("{}", res)]
    }

    fn part2(&mut self) -> Vec<String> {
        let mut res = 0;
        vec![format!("{}", res)]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  
";

    #[test]
    fn test_part1() {
        let mut runner = Aoc2025_6::with_input(TEST_INPUT);
        runner.parse();
        assert_eq!(runner.part1(), vec!["4277556".to_string()]);
    }

    #[test]
    fn test_part2() {
        let mut runner = Aoc2025_6::with_input(TEST_INPUT);
        runner.parse();
        assert_eq!(runner.part2(), vec!["3263827".to_string()]);
    }
}
