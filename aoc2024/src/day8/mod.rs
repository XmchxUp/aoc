use std::collections::{HashMap, HashSet};

use aoclib::Runner;

#[derive(Default)]
pub struct Aoc2024_8 {
    map: Vec<Vec<char>>,
    points: HashMap<char, Vec<(i32, i32)>>,
}

impl Aoc2024_8 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Aoc2024_8 {
    fn info(&self) -> (usize, usize) {
        (2024, 8)
    }

    fn parse(&mut self) {
        let inputs = aoclib::utils::read_file("./inputs/test08.txt");
        // let inputs = aoclib::utils::read_file("./inputs/input08.txt");
        self.map = inputs
            .into_iter()
            .map(|line| line.chars().collect())
            .collect();

        for i in 0..self.map.len() {
            for j in 0..self.map[0].len() {
                if self.map[i][j] == '.' {
                    continue;
                }

                self.points.entry(self.map[i][j]).or_default();
                self.points
                    .get_mut(&self.map[i][j])
                    .unwrap()
                    .push((i as i32, j as i32));
            }
        }
        // println!("{:?}", self.points);
    }

    fn part1(&mut self) -> Vec<String> {
        let m = self.map.len();
        let n = self.map[0].len();

        let mut anti_nodes = HashSet::new();
        for points in self.points.values() {
            for i in 0..points.len() - 1 {
                for j in i + 1..points.len() {
                    let diff_x = points[i].0 - points[j].0;
                    let diff_y = points[i].1 - points[j].1;
                    let new_x = points[i].0 + diff_x;
                    let new_y = points[i].1 + diff_y;
                    if !(new_x < 0 || new_y < 0 || new_x >= m as i32 || new_y >= n as i32) {
                        anti_nodes.insert((new_x, new_y));
                    }

                    let diff_x = points[j].0 - points[i].0;
                    let diff_y = points[j].1 - points[i].1;
                    let new_x = points[j].0 + diff_x;
                    let new_y = points[j].1 + diff_y;
                    if !(new_x < 0 || new_y < 0 || new_x >= m as i32 || new_y >= n as i32) {
                        anti_nodes.insert((new_x, new_y));
                    }
                }
            }
        }

        vec![format!("{}", anti_nodes.len())]
    }

    fn part2(&mut self) -> Vec<String> {
        let m = self.map.len();
        let n = self.map[0].len();

        let mut anti_nodes = HashSet::new();
        for (ch, points) in &self.points {
            for i in 0..points.len() - 1 {
                for j in i + 1..points.len() {
                    let diff_x = points[i].0 - points[j].0;
                    let diff_y = points[i].1 - points[j].1;
                    let mut new_x = points[i].0 + diff_x;
                    let mut new_y = points[i].1 + diff_y;
                    let mut need_include = true;
                    while !(new_x < 0 || new_y < 0 || new_x >= m as i32 || new_y >= n as i32) {
                        if self.map[new_x as usize][new_y as usize] == *ch {
                            need_include = false;
                        }
                        anti_nodes.insert((new_x, new_y));
                        new_x += diff_x;
                        new_y += diff_y;
                    }

                    let diff_x = points[j].0 - points[i].0;
                    let diff_y = points[j].1 - points[i].1;
                    let mut new_x = points[j].0 + diff_x;
                    let mut new_y = points[j].1 + diff_y;
                    while !(new_x < 0 || new_y < 0 || new_x >= m as i32 || new_y >= n as i32) {
                        if self.map[new_x as usize][new_y as usize] == *ch {
                            need_include = false;
                        }
                        anti_nodes.insert((new_x, new_y));
                        new_x += diff_x;
                        new_y += diff_y;
                    }

                    if need_include {
                        anti_nodes.insert(points[i]);
                        anti_nodes.insert(points[j]);
                    }
                }
            }
        }
        vec![format!("{}", anti_nodes.len())]
    }
}
