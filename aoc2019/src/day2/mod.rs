use aoclib::Runner;

#[derive(Default)]
pub struct Aoc2019_2 {
    nums: Vec<usize>,
}

impl Aoc2019_2 {
    pub fn new() -> Self {
        Self::default()
    }

    fn get_output(&self, nums: &mut Vec<usize>) -> usize {
        let mut pc = 0;

        while pc < nums.len() && nums[pc] != 99 {
            let n1_pos = nums[pc + 1];
            let n2_pos = nums[pc + 2];
            let target_pos = nums[pc + 3];
            match nums[pc] {
                1 => {
                    nums[target_pos] = nums[n1_pos] + nums[n2_pos];
                }
                2 => {
                    nums[target_pos] = nums[n1_pos] * nums[n2_pos];
                }
                _ => panic!(),
            }
            pc += 4;
        }
        nums[0]
    }
}

impl Runner for Aoc2019_2 {
    fn info(&self) -> (usize, usize) {
        (2019, 2)
    }

    fn parse(&mut self) {
        let inputs = aoclib::utils::read_file("./inputs/2019/02.txt");
        for line in inputs {
            let mut x: Vec<usize> = line
                .split(',')
                .map(|x| x.parse::<usize>().unwrap())
                .collect();
            self.nums.append(&mut x);
        }
    }

    fn part1(&mut self) -> Vec<String> {
        let mut nums = self.nums.clone();
        nums[1] = 12;
        nums[2] = 2;
        vec![format!("{}", self.get_output(&mut nums))]
    }

    fn part2(&mut self) -> Vec<String> {
        let mut noun = 0;
        let mut verb = 0;

        'outer: for i in 0..99 {
            for j in 0..99 {
                let mut nums = self.nums.clone();
                nums[1] = i;
                nums[2] = j;
                if self.get_output(&mut nums) == 19690720 {
                    noun = i;
                    verb = j;
                    break 'outer;
                }
            }
        }

        vec![format!("{}", 100 * noun + verb)]
    }
}
