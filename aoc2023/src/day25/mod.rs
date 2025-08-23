use std::collections::{HashMap, HashSet, VecDeque};

use aoclib::Runner;

#[derive(Default)]
pub struct Aoc2023_25 {
    vertices: HashMap<String, HashSet<String>>,
}

impl Aoc2023_25 {
    pub fn new() -> Self {
        Self::default()
    }

    fn ensure_vertex(&mut self, name: &str) {
        self.vertices.entry(name.to_string()).or_default();
    }

    fn add_edge(&mut self, a: &str, b: &str) {
        self.vertices.get_mut(a).unwrap().insert(b.to_owned());
        self.vertices.get_mut(b).unwrap().insert(a.to_owned());
    }

    // BFS 找最远点和路径
    fn find_farthest(&self, source: &str) -> (String, Vec<String>) {
        let mut q = VecDeque::new();
        q.push_back(source);

        let mut visited = HashSet::new();
        let mut dist = HashMap::new();
        let mut prev = HashMap::new();

        dist.insert(source, 0);
        prev.insert(source, None);

        let mut farthest = source;

        while let Some(cur) = q.pop_front() {
            if visited.contains(cur) {
                continue;
            }
            visited.insert(cur);

            let cur_dist = *dist.get(cur).unwrap();
            if cur_dist > *dist.get(farthest).unwrap() {
                farthest = cur;
            }

            if let Some(neighs) = self.vertices.get(cur) {
                for neigh in neighs.iter() {
                    let neigh = neigh.as_str();
                    if visited.contains(neigh) {
                        continue;
                    }

                    if !dist.contains_key(neigh) {
                        dist.insert(neigh, cur_dist + 1);
                        prev.insert(neigh, Some(cur));
                        q.push_back(neigh);
                    }
                }
            }
        }

        let mut path = Vec::new();
        let mut cur = Some(farthest);
        while let Some(v) = cur {
            path.push(v);
            cur = prev.get(v).cloned().unwrap_or(None);
        }
        path.reverse();

        (
            farthest.to_owned(),
            path.iter().map(|v| v.to_string()).collect::<Vec<String>>(),
        )
    }

    fn remove_edge(&mut self, a: &str, b: &str) {
        if let Some(neigh) = self.vertices.get_mut(a) {
            neigh.remove(b);
        }
        if let Some(neigh) = self.vertices.get_mut(b) {
            neigh.remove(a);
        }
    }

    // BFS 算连通块大小
    fn calculate_component_size(&self, source: &str) -> usize {
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();

        visited.insert(source);
        queue.push_back(source);

        while let Some(cur) = queue.pop_front() {
            if let Some(neighs) = self.vertices.get(cur) {
                for neigh in neighs {
                    let neigh = neigh.as_str();
                    if !visited.contains(neigh) {
                        visited.insert(neigh);
                        queue.push_back(neigh);
                    }
                }
            }
        }

        visited.len()
    }
}

impl Runner for Aoc2023_25 {
    fn info(&self) -> (usize, usize) {
        (2023, 25)
    }

    fn parse(&mut self) {
        let inputs = aoclib::utils::read_file("./inputs/2023/25.txt");
        // let inputs = aoclib::utils::read_file("./inputs/2023/test.txt");

        for line in inputs {
            let parts: Vec<&str> = line.split(": ").collect();
            let name = parts[0].trim();
            let connections: Vec<&str> = parts[1].split(' ').collect();

            self.ensure_vertex(name);

            for conn in connections {
                self.ensure_vertex(conn);
                self.add_edge(name, conn);
            }
        }
    }

    fn part1(&mut self) -> Vec<String> {
        let mut start = self.vertices.keys().next().unwrap().clone();
        for _ in 0..3 {
            let (farthest, path) = self.find_farthest(&start);

            for w in path.windows(2) {
                let (u, v) = (&w[0], &w[1]);
                self.remove_edge(u, v);
            }

            start = farthest;
        }

        let comp_size = self.calculate_component_size(&start);
        let result = comp_size * (self.vertices.len() - comp_size);
        vec![format!("{}", result)]
    }

    fn part2(&mut self) -> Vec<String> {
        let res = 0;
        vec![format!("{}", res)]
    }
}
