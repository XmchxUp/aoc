use std::{collections::HashSet, thread::sleep, time::Duration};

use aoclib::Runner;

#[derive(Debug, Clone)]
struct Robot {
    p: (i32, i32),
    v: (i32, i32),
}

#[derive(Default)]
pub struct Aoc2024_14 {
    robots: Vec<Robot>,
}

const TILES_TALL: i32 = 103;
const TILES_WIDE: i32 = 101;

const CENTER_X: i32 = 50;
const CENTER_Y: i32 = 51;

impl Aoc2024_14 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Aoc2024_14 {
    fn info(&self) -> (usize, usize) {
        (2024, 14)
    }

    fn parse(&mut self) {
        let inputs = aoclib::utils::read_file("./inputs/2024/14.txt");
        for line in inputs {
            let parts: Vec<&str> = line.split(' ').collect();

            let p_coords = parts[0].strip_prefix("p=").unwrap();
            let v_coords = parts[1].strip_prefix("v=").unwrap();

            let p: Vec<i32> = p_coords
                .split(',')
                .map(|s| s.parse::<i32>().unwrap())
                .collect();
            assert_eq!(p.len(), 2);
            let v: Vec<i32> = v_coords
                .split(',')
                .map(|s| s.parse::<i32>().unwrap())
                .collect();
            assert_eq!(v.len(), 2);
            self.robots.push(Robot {
                p: (p[0], p[1]),
                v: (v[0], v[1]),
            });
        }
    }

    fn part1(&mut self) -> Vec<String> {
        let mut robots = self.robots.clone();

        let mut q = (0, 0, 0, 0);
        for robot in &mut robots {
            let mut x = robot.p.0 + robot.v.0 * 100;
            let mut y = robot.p.1 + robot.v.1 * 100;

            x = ((x % TILES_WIDE) + TILES_WIDE) % TILES_WIDE;
            y = ((y % TILES_TALL) + TILES_TALL) % TILES_TALL;

            robot.p.0 = x;
            robot.p.1 = y;

            if x == CENTER_X && y == CENTER_Y {
                continue;
            }

            if x < CENTER_X && y < CENTER_Y {
                q.0 += 1;
            }
            if x > CENTER_X && y < CENTER_Y {
                q.1 += 1;
            }
            if x < CENTER_X && y > CENTER_Y {
                q.2 += 1;
            }
            if x > CENTER_X && y > CENTER_Y {
                q.3 += 1;
            }
        }

        let res = q.0 * q.1 * q.2 * q.3;
        vec![format!("{}", res)]
    }

    fn part2(&mut self) -> Vec<String> {
        let w = TILES_WIDE;
        let h = TILES_TALL;

        for t in 0..100_000 {
            let pos: Vec<(i32, i32)> = self
                .robots
                .iter()
                .map(|r| {
                    let x = ((r.p.0 + r.v.0 * t) % w + w) % w;
                    let y = ((r.p.1 + r.v.1 * t) % h + h) % h;
                    (x, y)
                })
                .collect();

            let pos_set: HashSet<(i32, i32)> = pos.iter().cloned().collect();

            // 打印网格
            for y in 0..h {
                let row: String = (0..w)
                    .map(|x| if pos_set.contains(&(x, y)) { '#' } else { '.' })
                    .collect();
                println!("{}", row);
            }
        }
        unreachable!();
    }
}
