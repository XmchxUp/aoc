use aoclib::Runner;

#[derive(Default)]
pub struct Aoc2025_3 {
    lines: Vec<String>,
}

impl Aoc2025_3 {
    pub fn new() -> Self {
        Self::default()
    }

    #[cfg(test)]
    pub fn with_input(input: &str) -> Self {
        Self {
            lines: input.lines().map(|s| s.to_string()).collect(),
        }
    }

    fn max_joltage(&self, bank: &str, k: usize) -> i64 {
        let s = bank.as_bytes();
        assert_ne!(s.len(), 0);
        let n = s.len();

        let mut pos = 0;
        let mut result = 0;

        for remaining in (1..=k).rev() {
            let end = n - (remaining - 1);

            let mut max_digit = b'0';
            let mut max_idx = pos;

            for i in pos..end {
                if s[i] > max_digit {
                    max_digit = s[i];
                    max_idx = i;
                }
            }
            result = result * 10 + (max_digit - b'0') as i64;
            pos = max_idx + 1;
        }
        result
    }
}

impl Runner for Aoc2025_3 {
    fn info(&self) -> (usize, usize) {
        (2025, 3)
    }

    fn parse(&mut self) {
        #[cfg(not(test))]
        let inputs = aoclib::utils::read_file("./inputs/2025/03.txt");

        #[cfg(test)]
        let inputs = self.lines.clone();
        self.lines = inputs;
    }

    fn part1(&mut self) -> Vec<String> {
        let res = self
            .lines
            .iter()
            .map(|bank| self.max_joltage(&bank, 2))
            .sum::<i64>();
        vec![format!("{}", res)]
    }

    fn part2(&mut self) -> Vec<String> {
        let res = self
            .lines
            .iter()
            .map(|bank| self.max_joltage(&bank, 12))
            .sum::<i64>();
        vec![format!("{}", res)]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "987654321111111
811111111111119
234234234234278
818181911112111
";

    #[test]
    fn test_part1() {
        let mut runner = Aoc2025_3::with_input(TEST_INPUT);
        runner.parse();
        assert_eq!(runner.part1(), vec!["357".to_string()]);
    }

    #[test]
    fn test_part2() {
        let mut runner = Aoc2025_3::with_input(TEST_INPUT);
        runner.parse();
        assert_eq!(runner.part2(), vec!["3121910778619".to_string()]);
    }
}
