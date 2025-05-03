use std::collections::HashMap;

use aoclib::Runner;

type Wire = HashMap<(i32, i32), i32>;

#[derive(Default)]
pub struct Aoc2019_3 {
    wires: Vec<Wire>,
}

impl Aoc2019_3 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Aoc2019_3 {
    fn info(&self) -> (usize, usize) {
        (2019, 3)
    }

    fn parse(&mut self) {
        let inputs = aoclib::utils::read_file("./inputs/2019/03.txt");
        for line in inputs {
            let mut wire = Wire::new();

            let mut x = 0;
            let mut y = 0;
            let mut cur_steps = 0;

            for ele in line.split(',').collect::<Vec<&str>>() {
                let (dir, steps) = ele.split_at(1);
                let steps = steps.parse::<i32>().unwrap();
                for i in 0..steps {
                    match dir {
                        "U" => {
                            y -= 1;
                        }
                        "D" => {
                            y += 1;
                        }
                        "L" => {
                            x -= 1;
                        }
                        "R" => {
                            x += 1;
                        }
                        _ => panic!("invalid direction"),
                    }
                    wire.entry((x, y)).or_insert(cur_steps + i + 1);
                }
                cur_steps += steps;
            }
            self.wires.push(wire);
        }
    }

    fn part1(&mut self) -> Vec<String> {
        let mut res = i32::MAX;
        let w1 = self.wires.first().unwrap();
        let w2 = self.wires.get(1).unwrap();

        for p in w1.keys() {
            if p == &(0, 0) {
                continue;
            }
            if let Some(_steps2) = w2.get(p) {
                res = res.min(p.0.abs() + p.1.abs());
            }
        }

        vec![format!("{}", res)]
    }

    fn part2(&mut self) -> Vec<String> {
        let mut res = i32::MAX;
        let w1 = self.wires.first().unwrap();
        let w2 = self.wires.get(1).unwrap();

        for (p, steps1) in w1 {
            if p == &(0, 0) {
                continue;
            }
            if let Some(steps2) = w2.get(p) {
                res = res.min(steps1 + steps2);
            }
        }

        vec![format!("{}", res)]
    }
}
