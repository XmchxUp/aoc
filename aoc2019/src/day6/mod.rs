use std::collections::{HashMap, VecDeque};

use aoclib::Runner;

#[derive(Debug)]
struct Orbit {
    conns: Vec<String>,
}

#[derive(Default)]
pub struct Aoc2019_6 {
    orbits: HashMap<String, Orbit>,
}

impl Aoc2019_6 {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn help(&self, start: &str) -> HashMap<String, u32> {
        let mut cache: HashMap<String, u32> = HashMap::new();
        let mut queue = VecDeque::new();
        queue.push_back(start);

        cache.insert(start.to_string(), 0);

        let mut cur_v = 0;

        while !queue.is_empty() {
            let size = queue.len();
            for _ in 0..size {
                let orbit_name = queue.pop_front().unwrap();
                let orbit = self.orbits.get(orbit_name).unwrap();

                for conn in &orbit.conns {
                    if cache.contains_key(conn) {
                        continue;
                    }
                    cache.insert(conn.clone(), cur_v + 1);
                    queue.push_back(conn);
                }
            }
            cur_v += 1;
        }

        cache
    }
}

impl Runner for Aoc2019_6 {
    fn info(&self) -> (usize, usize) {
        (2019, 6)
    }

    fn parse(&mut self) {
        let inputs = aoclib::utils::read_file("./inputs/2019/06.txt");
        for line in inputs {
            let (right, left) = line.split_once(")").unwrap();

            self.orbits
                .entry(right.to_string())
                .and_modify(|e| {
                    e.conns.push(left.to_string());
                })
                .or_insert(Orbit {
                    conns: vec![left.to_string()],
                });

            self.orbits
                .entry(left.to_string())
                .and_modify(|e| {
                    e.conns.push(right.to_string());
                })
                .or_insert(Orbit {
                    conns: vec![right.to_string()],
                });
        }
        // println!("{:?}", self.orbits);
    }

    fn part1(&mut self) -> Vec<String> {
        let cache = self.help("COM");
        vec![format!("{}", cache.iter().map(|t| t.1).sum::<u32>())]
    }

    fn part2(&mut self) -> Vec<String> {
        let cache = self.help("SAN");

        // for ele in &cache {
        //     println!("{}{}", ele.0, ele.1);
        // }

        let mut res = u32::MAX;

        let start = "YOU";

        let you_orbit = self.orbits.get(start).unwrap();

        for conn in &you_orbit.conns {
            res = res.min(*cache.get(conn).unwrap() - 1);
        }

        vec![format!("{}", res)]
    }
}
