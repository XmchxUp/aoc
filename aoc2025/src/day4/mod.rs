use std::collections::HashSet;

use aoclib::Runner;

type Point = (i32, i32);

#[derive(Default)]
pub struct Aoc2025_4 {
    #[cfg(test)]
    lines: Vec<String>,
    papers: HashSet<Point>,
}

impl Aoc2025_4 {
    pub fn new() -> Self {
        Self::default()
    }

    #[cfg(test)]
    pub fn with_input(input: &str) -> Self {
        Self {
            lines: input.lines().map(|s| s.to_string()).collect(),
            papers: HashSet::default(),
        }
    }
}

fn count_adjacent_paper(papers: &HashSet<Point>, x: &Point) -> u32 {
    let mut count = 0;
    let dirs = vec![
        (-1, 0),
        (1, 0),
        (0, -1),
        (0, 1),
        (-1, -1),
        (-1, 1),
        (1, 1),
        (1, -1),
    ];
    dirs.iter().for_each(|dir| {
        if papers.contains(&(x.0 + dir.0, x.1 + dir.1)) {
            count += 1;
        }
    });
    count
}

impl Runner for Aoc2025_4 {
    fn info(&self) -> (usize, usize) {
        (2025, 4)
    }

    fn parse(&mut self) {
        #[cfg(not(test))]
        let inputs = aoclib::utils::read_file("./inputs/2025/04.txt");

        #[cfg(test)]
        let inputs = self.lines.clone();
        let mut y = 0;
        for line in inputs {
            let mut x = 0;
            for ch in line.chars() {
                match ch {
                    '@' => {
                        self.papers.insert((x, y));
                    }
                    '.' => {}
                    _ => panic!("unexpect character"),
                };
                x += 1;
            }
            y += 1;
        }
        // println!("{:?}", self.papers);
    }

    fn part1(&mut self) -> Vec<String> {
        let res = self
            .papers
            .iter()
            .filter(|x| count_adjacent_paper(&self.papers, x) < 4)
            .count();
        vec![format!("{}", res)]
    }

    fn part2(&mut self) -> Vec<String> {
        let mut res = 0;
        let mut cur_papers = self.papers.clone();
        let mut new_papers = HashSet::new();

        while !cur_papers.is_empty() {
            let mut removed = false;
            for x in cur_papers.iter() {
                if count_adjacent_paper(&cur_papers, &x) < 4 {
                    res += 1;
                    removed = true;
                } else {
                    new_papers.insert(*x);
                }
            }
            if !removed {
                break;
            }
            cur_papers = new_papers.clone();
            new_papers = HashSet::new();
        }

        vec![format!("{}", res)]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
";

    #[test]
    fn test_part1() {
        let mut runner = Aoc2025_4::with_input(TEST_INPUT);
        runner.parse();
        assert_eq!(runner.part1(), vec!["13".to_string()]);
    }

    #[test]
    fn test_part2() {
        let mut runner = Aoc2025_4::with_input(TEST_INPUT);
        runner.parse();
        assert_eq!(runner.part2(), vec!["43".to_string()]);
    }
}
