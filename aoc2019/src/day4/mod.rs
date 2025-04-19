use aoclib::Runner;

#[derive(Default)]
pub struct Aoc2019_4 {
    start_num: i32,
    end_num: i32,
}

impl Aoc2019_4 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Aoc2019_4 {
    fn info(&self) -> (usize, usize) {
        (2019, 4)
    }

    fn parse(&mut self) {
        let inputs = aoclib::utils::read_file("./inputs/2019/04.txt");
        for line in inputs {
            let (start, end) = line.split_once("-").unwrap();
            self.start_num = start.parse::<i32>().unwrap();
            self.end_num = end.parse::<i32>().unwrap();
        }
    }

    fn part1(&mut self) -> Vec<String> {
        let mut res = 0;
        'outer: for num in self.start_num..=self.end_num {
            let num_vec = [num]
                .iter()
                .flat_map(|v| {
                    let mut res = vec![];
                    let mut v = *v;
                    while v > 0 {
                        res.insert(0, v % 10);
                        v /= 10;
                    }
                    res
                })
                .collect::<Vec<i32>>();
            let mut prev_idx = 0;
            let mut valid = false;
            for idx in 1..num_vec.len() {
                if num_vec[idx] < num_vec[prev_idx] {
                    continue 'outer;
                }
                if num_vec[idx] == num_vec[prev_idx] {
                    valid = true;
                }
                prev_idx = idx;
            }
            if valid {
                res += 1;
            }
        }

        vec![format!("{}", res)]
    }

    fn part2(&mut self) -> Vec<String> {
        let mut res = 0;
        'outer: for num in self.start_num..=self.end_num {
            let num_vec = [num]
                .iter()
                .flat_map(|v| {
                    let mut res = vec![];
                    let mut v = *v;
                    while v > 0 {
                        res.insert(0, v % 10);
                        v /= 10;
                    }
                    res
                })
                .collect::<Vec<i32>>();
            let mut valid = false;
            for i in 1..num_vec.len() {
                if num_vec[i] < num_vec[i - 1] {
                    continue 'outer;
                }
                if num_vec[i] == num_vec[i - 1] {
                    if i >= 2 && num_vec[i - 1] == num_vec[i - 2] {
                        continue;
                    }
                    if i < num_vec.len() - 1 && num_vec[i] == num_vec[i + 1] {
                        continue;
                    }
                    valid = true;
                }
            }
            if valid {
                println!("{:?}", num_vec);
                res += 1;
            }
        }
        vec![format!("{}", res)]
    }
}
