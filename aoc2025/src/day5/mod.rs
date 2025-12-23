use aoclib::Runner;

#[derive(Default)]
pub struct Aoc2025_5 {
    #[cfg(test)]
    lines: Vec<String>,
    available: Vec<u64>,
    ranges: Vec<(u64, u64)>,
}

impl Aoc2025_5 {
    pub fn new() -> Self {
        Self::default()
    }

    #[cfg(test)]
    pub fn with_input(input: &str) -> Self {
        Self {
            lines: input.lines().map(|s| s.to_string()).collect(),
            available: vec![],
            ranges: vec![],
        }
    }
}

fn is_in_ranges(ranges: &[(u64, u64)], x: u64) -> bool {
    for range in ranges {
        if range.0 <= x && x <= range.1 {
            return true;
        }
    }
    false
}

fn merge_ranges(ranges: &mut [(u64, u64)]) -> Vec<(u64, u64)> {
    ranges.sort();
    let mut res = vec![ranges[0]];
    for i in 1..ranges.len() {
        let current = ranges[i];
        let last = res.last_mut().unwrap();
        if current.0 <= last.1 {
            last.1 = last.1.max(current.1);
        } else {
            res.push(current);
        }
    }
    res
}

impl Runner for Aoc2025_5 {
    fn info(&self) -> (usize, usize) {
        (2025, 5)
    }

    fn parse(&mut self) {
        #[cfg(not(test))]
        let inputs = aoclib::utils::read_file("./inputs/2025/05.txt");

        #[cfg(test)]
        let inputs = self.lines.clone();

        let mut reading_ranges = true;
        for line in inputs {
            if line.is_empty() {
                reading_ranges = false;
                continue;
            }

            if reading_ranges {
                if let Some((l, r)) = line.split_once('-') {
                    self.ranges.push((l.parse().unwrap(), r.parse().unwrap()));
                } else {
                    panic!("unexpected range")
                }
            } else {
                self.available.push(line.parse().unwrap());
            }
        }
        // println!("{:?} {:?}", self.available, self.ranges);
    }

    fn part1(&mut self) -> Vec<String> {
        let res = self
            .available
            .iter()
            .filter(|x| is_in_ranges(&self.ranges, **x))
            .count();
        vec![format!("{}", res)]
    }

    fn part2(&mut self) -> Vec<String> {
        let res = merge_ranges(&mut self.ranges)
            .iter()
            .map(|range| range.1 - range.0 + 1)
            .sum::<u64>();
        vec![format!("{}", res)]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";

    #[test]
    fn test_part1() {
        let mut runner = Aoc2025_5::with_input(TEST_INPUT);
        runner.parse();
        assert_eq!(runner.part1(), vec!["3".to_string()]);
    }

    #[test]
    fn test_part2() {
        let mut runner = Aoc2025_5::with_input(TEST_INPUT);
        runner.parse();
        assert_eq!(runner.part2(), vec!["14".to_string()]);
    }
}
