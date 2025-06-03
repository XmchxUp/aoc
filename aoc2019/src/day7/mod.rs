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
            let mut signal = 0;
            for phase in phase_settings {
                let mut pc = 0;
                let mut input_idx = 0;
                signal = self
                    .computer
                    .get_diagnostic_code(
                        &mut program,
                        &mut pc,
                        &mut input_idx,
                        &vec![phase, signal],
                    )
                    .0
                    .unwrap();
            }
            max_signal = max_signal.max(signal);
        }

        vec![format!("{}", max_signal)]
    }

    fn part2(&mut self) -> Vec<String> {
        let mut res = 0;

        struct Amplifier {
            memory: Vec<i64>,
            pc: usize,
            input_idx: usize,
            inputs: Vec<i64>,
            halted: bool,
        }

        for phase_settings in (5..10).permutations(5) {
            let mut amplifiers = phase_settings
                .iter()
                .map(|&phase| Amplifier {
                    memory: self.program.to_vec(),
                    pc: 0,
                    input_idx: 0,
                    inputs: vec![phase],
                    halted: false,
                })
                .collect::<Vec<Amplifier>>();

            amplifiers[0].inputs.push(0);

            let mut last_output = 0;
            let mut current_amp_idx = 0;

            while !amplifiers.iter().all(|a| a.halted) {
                let amp = &mut amplifiers[current_amp_idx];
                if amp.halted {
                    current_amp_idx = (current_amp_idx + 1) % 5;
                    continue;
                }

                let (output, halted) = self.computer.get_diagnostic_code(
                    &mut amp.memory,
                    &mut amp.pc,
                    &mut amp.input_idx,
                    &amp.inputs,
                );

                amp.halted = halted;

                if let Some(output_val) = output {
                    let next_amp_idx = (current_amp_idx + 1) % 5;
                    amplifiers[next_amp_idx].inputs.push(output_val);
                    last_output = output_val;
                }

                current_amp_idx = (current_amp_idx + 1) % 5;
            }

            res = res.max(last_output);
        }

        vec![format!("{}", res)]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_one() {
        let mut solver = Aoc2019_7::new();
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
    }

    #[test]
    fn test_part_two() {
        let mut solver = Aoc2019_7::default();
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
