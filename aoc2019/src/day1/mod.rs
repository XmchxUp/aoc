use aoclib::Runner;

#[derive(Default)]
pub struct Aoc2019_1 {
    eles: Vec<i32>,
}

impl Aoc2019_1 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Aoc2019_1 {
    fn info(&self) -> (usize, usize) {
        (2019, 1)
    }

    fn parse(&mut self) {
        let inputs = aoclib::utils::read_file("./inputs/2019/01.txt");
        for line in inputs {
            self.eles.push(line.parse::<i32>().unwrap());
        }
    }

    fn part1(&mut self) -> Vec<String> {
        vec![format!(
            "{}",
            self.eles
                .iter()
                .map(|v| {
                    let v = v / 3;
                    v - 2
                })
                .sum::<i32>()
        )]
    }

    fn part2(&mut self) -> Vec<String> {
        let cal = |v: i32| -> i32 {
            let v = v / 3;
            v - 2
        };

        let mut res = 0;
        for ele in &self.eles {
            let mut cur = *ele;
            loop {
                let next_v = cal(cur);
                if next_v > 0 {
                    res += next_v;
                    cur = next_v;
                } else {
                    break;
                }
            }
        }

        vec![format!("{}", res)]
    }
}
