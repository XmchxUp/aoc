/*
Running Aoc2023_23
---- 2023, Day 23 ----
  0.103 Parsing
  0.583 Part 1: 23xx
 75.107 Part 2: 65xx
 */

use std::collections::{HashMap, HashSet};

use aoclib::Runner;

type Point = (i32, i32);

#[derive(Debug, Eq, PartialEq)]
enum Tile {
    Road,
    Forest,
    SlopeLeft,
    SlopeRight,
    SlopeUp,
    SlopeDown,
}

#[derive(Default)]
pub struct Aoc2023_23 {
    map: HashMap<Point, Tile>,
    start: Point,
    end: Point,
    rows: i32,
    cols: i32,
    ignore_slopes: bool,
    graph: HashMap<Point, Vec<(Point, i32)>>, // node -> [(neighbor, distance)]
}

impl Aoc2023_23 {
    pub fn new() -> Self {
        Self::default()
    }

    fn build_graph(&mut self) {
        let mut graph = HashMap::new();

        let mut junctions = HashSet::new();
        junctions.insert(self.start);
        junctions.insert(self.end);
        for (&pos, _) in self.map.iter().filter(|&(_, tile)| tile != &Tile::Forest) {
            if self.get_neighbors(&pos).len() > 2 {
                junctions.insert(pos);
            }
        }

        for &start in &junctions {
            let mut stack = vec![(start, 0)];
            let mut visited = HashSet::new();

            visited.insert(start);
            while let Some((pos, dist)) = stack.pop() {
                if dist > 0 && junctions.contains(&pos) {
                    graph
                        .entry(start)
                        .or_insert_with(Vec::new)
                        .push((pos, dist));
                    continue;
                }
                for next_pos in self.get_neighbors(&pos) {
                    if !visited.contains(&next_pos) {
                        visited.insert(next_pos);
                        stack.push((next_pos, dist + 1));
                    }
                }
            }
        }

        let mut final_graph = HashMap::new();
        for (&start, neighbors) in &graph {
            final_graph
                .entry(start)
                .or_insert_with(Vec::new)
                .extend(neighbors);
            for &(end, dist) in neighbors {
                final_graph
                    .entry(end)
                    .or_insert_with(Vec::new)
                    .push((start, dist));
            }
        }
        self.graph = final_graph;
    }

    fn get_neighbors(&self, pos: &Point) -> Vec<Point> {
        let dirs: Vec<Point> = if self.ignore_slopes {
            vec![(-1, 0), (0, 1), (1, 0), (0, -1)]
        } else {
            match self.map.get(pos).unwrap() {
                Tile::Road => vec![(-1, 0), (0, 1), (1, 0), (0, -1)],
                Tile::Forest => panic!(),
                Tile::SlopeLeft => vec![(0, -1)],
                Tile::SlopeRight => vec![(0, 1)],
                Tile::SlopeUp => vec![(-1, 0)],
                Tile::SlopeDown => vec![(1, 0)],
            }
        };
        let mut res = vec![];
        for dir in dirs {
            let new_p = (pos.0 + dir.0, pos.1 + dir.1);
            if new_p.0 >= 0
                && new_p.0 < self.rows
                && new_p.1 >= 0
                && new_p.1 < self.cols
                && !matches!(self.map.get(&new_p).unwrap(), Tile::Forest)
            {
                res.push(new_p);
            }
        }
        res
    }

    fn dfs(&self, pos: Point, visited: &mut HashSet<Point>, steps: i32, max_steps: &mut i32) {
        if pos == self.end {
            *max_steps = (*max_steps).max(steps);
            return;
        }
        if self.ignore_slopes {
            if let Some(neighbors) = self.graph.get(&pos) {
                for &(next_pos, dist) in neighbors {
                    if !visited.contains(&next_pos) {
                        visited.insert(next_pos);
                        self.dfs(next_pos, visited, steps + dist, max_steps);
                        visited.remove(&next_pos);
                    }
                }
            }
        } else {
            for next_pos in self.get_neighbors(&pos) {
                if !visited.contains(&next_pos) {
                    visited.insert(next_pos);
                    self.dfs(next_pos, visited, steps + 1, max_steps);
                    visited.remove(&next_pos);
                }
            }
        }
    }
}

impl Runner for Aoc2023_23 {
    fn info(&self) -> (usize, usize) {
        (2023, 23)
    }

    fn parse(&mut self) {
        let inputs = aoclib::utils::read_file("./inputs/2023/23.txt");
        self.rows = inputs.len() as i32;
        for (i, line) in inputs.iter().enumerate() {
            self.cols = line.len() as i32;
            for (j, ch) in line.chars().enumerate() {
                match ch {
                    '.' => {
                        self.map.insert((i as i32, j as i32), Tile::Road);
                    }
                    '#' => {
                        self.map.insert((i as i32, j as i32), Tile::Forest);
                    }
                    '>' => {
                        self.map.insert((i as i32, j as i32), Tile::SlopeRight);
                    }
                    '<' => {
                        self.map.insert((i as i32, j as i32), Tile::SlopeLeft);
                    }
                    'v' => {
                        self.map.insert((i as i32, j as i32), Tile::SlopeDown);
                    }
                    '^' => {
                        self.map.insert((i as i32, j as i32), Tile::SlopeUp);
                    }
                    _ => {
                        panic!("Unexpected char");
                    }
                }
            }
        }
        self.start = self
            .map
            .iter()
            .find(|&(point, tile)| point.0 == 0 && *tile == Tile::Road)
            .map(|e| *e.0)
            .unwrap();
        self.end = self
            .map
            .iter()
            .find(|&(point, tile)| point.0 == (self.rows - 1) && *tile == Tile::Road)
            .map(|e| *e.0)
            .unwrap();
        // println!("{:?} -- {:?}", self.start, self.end);
        self.build_graph();
    }

    fn part1(&mut self) -> Vec<String> {
        let mut max_steps = 0;
        let mut visited = HashSet::new();
        self.ignore_slopes = false;
        self.dfs(self.start, &mut visited, 0, &mut max_steps);
        vec![format!("{}", max_steps)]
    }

    fn part2(&mut self) -> Vec<String> {
        let mut max_steps = 0;
        let mut visited = HashSet::new();
        self.ignore_slopes = true;
        self.dfs(self.start, &mut visited, 0, &mut max_steps);
        vec![format!("{}", max_steps)]
    }
}
