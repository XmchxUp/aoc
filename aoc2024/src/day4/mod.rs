use aoclib::Runner;

#[derive(Default)]
pub struct Aoc2024_4 {
    map: Vec<Vec<char>>,
}

impl Aoc2024_4 {
    pub fn new() -> Self {
        Self::default()
    }

    fn search(&self, target: &str, i: i32, j: i32, di: i32, dj: i32) -> usize {
        let m = self.map.len() as i32;
        let n = self.map[0].len() as i32;

        if i < 0 || j < 0 || i >= m || j >= n {
            return 0;
        }

        let ch = target.chars().next().unwrap();
        if self.map[i as usize][j as usize] != ch {
            return 0;
        }

        if target.len() == 1 {
            return 1;
        }

        self.search(&target[1..], i + di, j + dj, di, dj)
    }
}

impl Runner for Aoc2024_4 {
    fn info(&self) -> (usize, usize) {
        (2024, 4)
    }

    fn parse(&mut self) {
        // let inputs = aoclib::utils::read_file("./inputs/test04.txt");
        let inputs = aoclib::utils::read_file("./inputs/input04.txt");
        self.map = inputs
            .into_iter()
            .map(|line| line.chars().collect())
            .collect();
    }

    fn part1(&mut self) -> Vec<String> {
        let mut res = 0;

        let m = self.map.len() as i32;
        let n = self.map[0].len() as i32;
        let target = "XMAS";

        let directions = [
            (1, 0),
            (-1, 0),
            (0, 1),
            (0, -1),
            (-1, -1),
            (-1, 1),
            (1, 1),
            (1, -1),
        ];

        for i in 0..m {
            for j in 0..n {
                if self.map[i as usize][j as usize] != 'X' {
                    continue;
                }
                for (di, dj) in directions.iter() {
                    res += self.search(target, i, j, *di, *dj);
                }
            }
        }

        vec![format!("{}", res)]
    }

    fn part2(&mut self) -> Vec<String> {
        let mut res = 0;

        let m = self.map.len() as i32;
        let n = self.map[0].len() as i32;

        for i in 1..m - 1 {
            for j in 1..n - 1 {
                if self.map[i as usize][j as usize] == 'A' {
                    if self.map[(i - 1) as usize][(j - 1) as usize] == 'M'
                        && self.map[(i + 1) as usize][(j + 1) as usize] == 'S'
                        && self.map[(i + 1) as usize][(j - 1) as usize] == 'M'
                        && self.map[(i - 1) as usize][(j + 1) as usize] == 'S'
                    {
                        res += 1;
                    }
                    if self.map[(i - 1) as usize][(j - 1) as usize] == 'S'
                        && self.map[(i + 1) as usize][(j + 1) as usize] == 'M'
                        && self.map[(i + 1) as usize][(j - 1) as usize] == 'M'
                        && self.map[(i - 1) as usize][(j + 1) as usize] == 'S'
                    {
                        res += 1;
                    }
                    if self.map[(i - 1) as usize][(j - 1) as usize] == 'M'
                        && self.map[(i + 1) as usize][(j + 1) as usize] == 'S'
                        && self.map[(i + 1) as usize][(j - 1) as usize] == 'S'
                        && self.map[(i - 1) as usize][(j + 1) as usize] == 'M'
                    {
                        res += 1;
                    }
                    if self.map[(i - 1) as usize][(j - 1) as usize] == 'S'
                        && self.map[(i + 1) as usize][(j + 1) as usize] == 'M'
                        && self.map[(i + 1) as usize][(j - 1) as usize] == 'S'
                        && self.map[(i - 1) as usize][(j + 1) as usize] == 'M'
                    {
                        res += 1;
                    }
                }
            }
        }

        vec![format!("{}", res)]
    }
}
