use aoclib::Runner;

#[derive(PartialEq, Eq)]
enum Mode {
    Immediate,
    Position,
}

impl From<usize> for Mode {
    fn from(value: usize) -> Self {
        match value {
            1 => Mode::Immediate,
            0 => Mode::Position,
            _ => panic!("error mode value: {}", value),
        }
    }
}

#[derive(Debug)]
enum OpCode {
    Add = 1,
    Mul = 2,
    Input = 3,
    Output = 4,
    JumpIfTrue = 5,
    JumpIfFalse = 6,
    LessThan = 7,
    Equals = 8,
    Halt = 99,
}

impl From<usize> for OpCode {
    fn from(value: usize) -> Self {
        match value {
            1 => OpCode::Add,
            2 => OpCode::Mul,
            3 => OpCode::Input,
            4 => OpCode::Output,
            5 => OpCode::JumpIfTrue,
            6 => OpCode::JumpIfFalse,
            7 => OpCode::LessThan,
            8 => OpCode::Equals,
            99 => OpCode::Halt,
            _ => panic!("error opcode value {}", value),
        }
    }
}

#[derive(Default)]
pub struct Aoc2019_5 {
    nums: Vec<i64>,
}

impl Aoc2019_5 {
    pub fn new() -> Self {
        Self::default()
    }

    fn get_value_from_mode(&self, mode: Mode, v: i64, nums: &[i64]) -> i64 {
        match mode {
            Mode::Immediate => v,
            Mode::Position => nums[v as usize],
        }
    }

    fn get_paramters(&self, pc_v: usize, pc: usize, nums: &[i64]) -> (i64, i64, i64) {
        let first_p_m: Mode = ((pc_v / 100) % 10).into();
        let second_p_m: Mode = ((pc_v / 1000) % 10).into();
        let third_p_m: Mode = ((pc_v / 10_000) % 10).into();
        assert!(third_p_m == Mode::Position);

        let first = self.get_value_from_mode(first_p_m, nums[pc + 1], nums);
        let second = self.get_value_from_mode(second_p_m, nums[pc + 2], nums);
        let third = nums[pc + 3];
        (first, second, third)
    }

    fn get_diagnostic_code(&self, input: i64) -> i64 {
        let mut nums = self.nums.clone();

        let mut pc = 0;
        let mut current_v = input;

        'outer: while pc < nums.len() {
            let pc_v = nums[pc] as usize;
            let opcode: OpCode = (pc_v % 100).into();
            // println!("PC: {} PC_V: {}, Opcode: {:?}", pc, pc_v, opcode);

            match opcode {
                OpCode::Add => {
                    let (first, second, third) = self.get_paramters(pc_v, pc, &nums);
                    let result = first + second;
                    // println!("Add: {} + {} = {} -> [{}]", first, second, result, third);
                    nums[third as usize] = result;
                    pc += 4;
                }
                OpCode::Mul => {
                    let (first, second, third) = self.get_paramters(pc_v, pc, &nums);
                    let result = first * second;
                    // println!("Mul: {} * {} = {} -> [{}]", first, second, result, third);
                    nums[third as usize] = result;
                    pc += 4;
                }
                OpCode::Input => {
                    let addr = nums[pc + 1];
                    // println!("Input: {} -> [{}]", current_v, addr);
                    nums[addr as usize] = current_v;
                    pc += 2;
                }
                OpCode::Output => {
                    let addr = nums[pc + 1];
                    current_v = nums[addr as usize];
                    // println!("Output: [{}] = {}", addr, current_v);
                    pc += 2;
                }
                OpCode::JumpIfTrue => {
                    let (first, second, _) = self.get_paramters(pc_v, pc, &nums);
                    // println!("JumpIfTrue: if {} != 0 jump to {}", first, second);
                    if first != 0 {
                        pc = second as usize;
                    } else {
                        pc += 3;
                    }
                }
                OpCode::JumpIfFalse => {
                    let (first, second, _) = self.get_paramters(pc_v, pc, &nums);
                    // println!("JumpIfFalse: if {} == 0 jump to {}", first, second);
                    if first == 0 {
                        pc = second as usize;
                    } else {
                        pc += 3;
                    }
                }
                OpCode::LessThan => {
                    let (first, second, third) = self.get_paramters(pc_v, pc, &nums);
                    let result = if first < second { 1 } else { 0 };
                    // println!(
                    //     "LessThan: {} < {} => {} -> [{}]",
                    //     first, second, result, third
                    // );
                    nums[third as usize] = result;
                    pc += 4;
                }
                OpCode::Equals => {
                    let (first, second, third) = self.get_paramters(pc_v, pc, &nums);
                    let result = if first == second { 1 } else { 0 };
                    // println!(
                    //     "Equals: {} == {} => {} -> [{}]",
                    //     first, second, result, third
                    // );
                    nums[third as usize] = result;
                    pc += 4;
                }
                OpCode::Halt => {
                    // println!("Halt");
                    break 'outer;
                }
            }
        }
        current_v
    }
}

impl Runner for Aoc2019_5 {
    fn info(&self) -> (usize, usize) {
        (2019, 5)
    }

    fn parse(&mut self) {
        let inputs = aoclib::utils::read_file("./inputs/2019/05.txt");
        for line in inputs {
            let nums_str: Vec<&str> = line.split(",").collect();
            self.nums = nums_str.iter().map(|v| v.parse::<i64>().unwrap()).collect();
        }
    }

    fn part1(&mut self) -> Vec<String> {
        vec![format!("{}", self.get_diagnostic_code(1))]
    }

    fn part2(&mut self) -> Vec<String> {
        vec![format!("{}", self.get_diagnostic_code(5))]
    }
}
