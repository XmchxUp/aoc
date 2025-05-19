use aoclib::Runner;

use itertools::Itertools;

use crate::Aoc2019_5;

#[derive(Default)]
pub struct Aoc2019_7 {
    program: Vec<i64>,
    computer: Aoc2019_5,
}

impl Aoc2019_7 {
    pub fn new() -> Self {
        Self::default()
    }

    fn get_thruster_signal(&self, program: &mut [i64], phase_settings: &[i64]) -> i64 {
        let mut signal = 0;
        for phase in phase_settings {
            signal = self
                .computer
                .get_diagnostic_code(program, &vec![*phase, signal]);
        }
        signal
    }
}

impl Runner for Aoc2019_7 {
    fn info(&self) -> (usize, usize) {
        (2019, 7)
    }

    fn parse(&mut self) {
        let inputs = aoclib::utils::read_file("./inputs/2019/07.txt");
        for line in inputs {
            let nums_str: Vec<&str> = line.split(",").collect();
            self.program = nums_str.iter().map(|v| v.parse::<i64>().unwrap()).collect();
        }
    }

    fn part1(&mut self) -> Vec<String> {
        let mut max_signal = 0;

        for phase_settings in (0..5).permutations(5) {
            let mut program = self.program.clone();
            let signal = self.get_thruster_signal(&mut program, &phase_settings);
            max_signal = max_signal.max(signal);
        }

        vec![format!("{}", max_signal)]
    }

    fn part2(&mut self) -> Vec<String> {
        let mut max_signal = 0;

        for phase_settings in (5..10).permutations(5) {
            let mut program = self.program.clone();
            let signal = self.get_thruster_signal(&mut program, &phase_settings);
            max_signal = max_signal.max(signal);
        }

        vec![format!("{}", max_signal)]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_one() {
        let mut solver = Aoc2019_7::default();
        solver.program = vec![
            3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0,
        ];
        assert_eq!(vec!["43210".to_string()], solver.part1());

        solver.program = vec![
            3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4, 23,
            99, 0, 0,
        ];
        assert_eq!(vec!["54321".to_string()], solver.part1());

        solver.program = vec![
            3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7, 33, 1,
            33, 31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0,
        ];
        assert_eq!(vec!["65210".to_string()], solver.part1());

        solver.program = vec![
            3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26, 27, 4, 27, 1001, 28, -1,
            28, 1005, 28, 6, 99, 0, 0, 5,
        ];
        assert_eq!(vec!["139629729".to_string()], solver.part2());

        solver.program = vec![
            3, 52, 1001, 52, -5, 52, 3, 53, 1, 52, 56, 54, 1007, 54, 5, 55, 1005, 55, 26, 1001, 54,
            -5, 54, 1105, 1, 12, 1, 53, 54, 53, 1008, 54, 0, 55, 1001, 55, 1, 55, 2, 53, 55, 53, 4,
            53, 1001, 56, -1, 56, 1005, 56, 6, 99, 0, 0, 0, 0, 10,
        ];
        assert_eq!(vec!["18216".to_string()], solver.part2());
    }
}
