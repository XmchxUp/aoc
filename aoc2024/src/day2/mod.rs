use aoclib::Runner;

#[derive(Default)]
pub struct Aoc2024_2 {
    levels: Vec<Vec<i32>>,
}

impl Aoc2024_2 {
    pub fn new() -> Self {
        Self::default()
    }
    fn is_safe_report(&self, nums: &[i32]) -> bool {
        let asc = nums[1] > nums[0];
        for i in 1..nums.len() {
            let diff = nums[i] - nums[i - 1];
            if diff == 0 || diff.abs() > 3 || (diff > 0) != asc {
                return false;
            }
        }
        true
    }
    fn is_safe_report_by_remove_one(&self, nums: &[i32]) -> bool {
        for i in 0..nums.len() {
            let mut modified = nums.to_vec();
            modified.remove(i);
            if self.is_safe_report(&modified) {
                return true;
            }
        }
        false
    }
}

impl Runner for Aoc2024_2 {
    fn info(&self) -> (usize, usize) {
        (2024, 2)
    }

    fn parse(&mut self) {
        let inputs = aoclib::utils::read_file("./inputs/input02.txt");
        inputs.iter().for_each(|line| {
            self.levels
                .push(line.split(' ').map(|v| v.parse::<i32>().unwrap()).collect());
        });
    }

    fn part1(&mut self) -> Vec<String> {
        vec![format!(
            "{}",
            self.levels
                .iter()
                .map(|nums| if self.is_safe_report(nums) { 1 } else { 0 })
                .sum::<i32>()
        )]
    }

    fn part2(&mut self) -> Vec<String> {
        vec![format!(
            "{}",
            self.levels
                .iter()
                .map(|nums| {
                    if self.is_safe_report(nums) || self.is_safe_report_by_remove_one(nums) {
                        1
                    } else {
                        0
                    }
                })
                .sum::<i32>()
        )]
    }
}
