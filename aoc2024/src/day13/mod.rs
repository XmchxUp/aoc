use aoclib::Runner;
use regex::Regex;

#[derive(Default)]
pub struct Aoc2024_13 {
    items: Vec<Item>,
}

impl Aoc2024_13 {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Default, Clone, Copy, Debug)]
struct Item {
    a: Button,
    b: Button,
    prize: Prize,
}

type Prize = Button;

#[derive(Default, Clone, Copy, Debug)]
struct Button {
    x: i64,
    y: i64,
}

impl Runner for Aoc2024_13 {
    fn info(&self) -> (usize, usize) {
        (2024, 13)
    }

    fn parse(&mut self) {
        let _inputs =
            aoclib::utils::read_file(format!("./inputs/test{:02}.txt", self.info().1).as_str());
        let inputs =
            aoclib::utils::read_file(format!("./inputs/input{:02}.txt", self.info().1).as_str());

        let mut items = Vec::default();

        let re1 = Regex::new(r"X\+(\d+),\s*Y\+(\d+)").unwrap();
        let re2 = Regex::new(r"X=(\d+),\s*Y=(\d+)").unwrap();

        let mut item = Item::default();
        let mut is_a = true;

        for line in inputs {
            let line = line.trim();
            if line.is_empty() {
                items.push(item);
                is_a = true;
                continue;
            }
            if let Some(captures) = re1.captures(line) {
                let x = captures.get(1).unwrap().as_str();
                let y = captures.get(2).unwrap().as_str();
                if is_a {
                    is_a = false;
                    item.a = Prize {
                        x: x.parse().unwrap(),
                        y: y.parse().unwrap(),
                    }
                } else {
                    is_a = true;
                    item.b = Prize {
                        x: x.parse().unwrap(),
                        y: y.parse().unwrap(),
                    }
                }
            }

            if let Some(captures) = re2.captures(line) {
                let x = captures.get(1).unwrap().as_str();
                let y = captures.get(2).unwrap().as_str();
                item.prize = Prize {
                    x: x.parse::<i64>().unwrap(),
                    y: y.parse::<i64>().unwrap(),
                }
            }
        }
        items.push(item);

        self.items = items;
        // println!("{:?}", self.items);
    }

    fn part1(&mut self) -> Vec<String> {
        let mut res = 0.0;

        for item in &self.items {
            let a_x = item.a.x;
            let a_y = item.a.y;
            let b_y = item.b.y;
            let b_x = item.b.x;
            let p_y = item.prize.y;
            let p_x = item.prize.x;
            // Solve the system of equations:
            // a_x * a + b_x * b = p_x
            // a_y * a + b_y * b = p_y

            let mut min_score = f64::INFINITY;
            for a in 0..101 {
                for b in 0..101 {
                    if a_x * a + b_x * b == p_x && a_y * a + b_y * b == p_y {
                        min_score = min_score.min((a * 3 + b) as f64);
                    }
                }
            }
            if min_score != f64::INFINITY {
                res += min_score;
            }
        }

        vec![format!("{}", res)]
    }

    fn part2(&mut self) -> Vec<String> {
        let mut res = 0;

        for item in &self.items {
            let a_x = item.a.x;
            let a_y = item.a.y;
            let b_y = item.b.y;
            let b_x = item.b.x;
            let p_y = item.prize.y + 10000000000000;
            let p_x = item.prize.x + 10000000000000;
            // Solve the system of equations:
            // a_x * a + b_x * b = p_x
            // a_y * a + b_y * b = p_y

            // First, find the determinant of the coefficient matrix
            let determinant = a_x * b_y - a_y * b_x;

            if determinant == 0 {
                // The system is either inconsistent or has infinitely many solutions
                continue;
            }

            // Calculate the solution using Cramer's rule
            let a = (p_x * b_y - p_y * b_x) / determinant;
            let b = (a_x * p_y - a_y * p_x) / determinant;

            // Check if the solution satisfies both equations
            if a_x * a + b_x * b == p_x && a_y * a + b_y * b == p_y {
                res += a * 3 + b;
            } else {
                continue;
            }
        }

        vec![format!("{}", res)]
    }
}
