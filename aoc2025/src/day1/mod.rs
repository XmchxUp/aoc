use aoclib::Runner;

#[derive(Clone, Copy)]
enum Turn {
    Left(i32),
    Right(i32),
}

#[derive(Default)]
pub struct Aoc2025_1 {
    instructions: Vec<Turn>,
}

impl Aoc2025_1 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Aoc2025_1 {
    fn info(&self) -> (usize, usize) {
        (2025, 1)
    }

    fn parse(&mut self) {
        let inputs = aoclib::utils::read_file("./inputs/2025/01.txt");
        for line in inputs {
            if line.trim().is_empty() {
                continue;
            }
            let steps: i32 = line[1..].parse().expect("invalid number");
            let turn = match &line[0..1] {
                "L" => Turn::Left(steps),
                "R" => Turn::Right(steps),
                _ => panic!("Invalid turn: {}", line),
            };
            self.instructions.push(turn);
        }
        assert_eq!(self.instructions.len(), 4570);
    }

    fn part1(&mut self) -> Vec<String> {
        let mut res = 0;

        let mut dial = 50;

        for turn in &self.instructions {
            dial = match turn {
                Turn::Left(v) => (dial - v).rem_euclid(100),
                Turn::Right(v) => (dial + v).rem_euclid(100),
            };
            if dial == 0 {
                res += 1;
            }
        }

        vec![format!("{}", res)]
    }

    fn part2(&mut self) -> Vec<String> {
        let mut res = 0;
        let mut dial = 50i32;

        for turn in &self.instructions {
            match turn {
                Turn::Left(v) => {
                    let div = v / 100;
                    res += div;

                    let m = -(v % 100);
                    if dial != 0 && dial + m <= 0 {
                        res += 1;
                    }

                    dial = (dial - v).rem_euclid(100);
                }
                Turn::Right(v) => {
                    let div = v / 100;
                    res += div;

                    let m = v % 100;
                    if dial + m >= 100 {
                        res += 1;
                    }

                    dial = (dial + v).rem_euclid(100);
                }
            };
        }

        vec![format!("{}", res)]
    }
}
