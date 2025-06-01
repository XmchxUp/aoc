use aoclib::Runner;

const WIDE: usize = 25;
const TALL: usize = 6;

#[derive(Debug)]
struct Layer {
    eles: Vec<Vec<u32>>,
}

#[derive(Default)]
pub struct Aoc2019_8 {
    layers: Vec<Layer>,
}

impl Aoc2019_8 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Aoc2019_8 {
    fn info(&self) -> (usize, usize) {
        (2019, 8)
    }

    fn parse(&mut self) {
        let inputs = aoclib::utils::read_file("./inputs/2019/08.txt");
        for line in inputs {
            let mut chs = line.chars();

            let mut idx = 0;
            let n = line.len();

            while idx < n {
                let mut eles = vec![vec![0; WIDE]; TALL];
                for i in 0..WIDE * TALL {
                    let c = chs.next().unwrap();
                    let row = i / WIDE;
                    let col = i % WIDE;
                    eles[row][col] = c.to_digit(10).unwrap();
                }

                self.layers.push(Layer { eles });
                idx += WIDE * TALL;
            }
        }
    }

    fn part1(&mut self) -> Vec<String> {
        let mut target = None;

        let mut curr_zero_count = usize::MAX;

        for layer in &self.layers {
            let count_zero = layer
                .eles
                .iter()
                .map(|v| v.iter().filter(|e| **e == 0).count())
                .sum();
            if count_zero < curr_zero_count {
                curr_zero_count = count_zero;
                target = Some(layer);
            }
        }

        let target = target.unwrap();

        vec![format!(
            "{}",
            target
                .eles
                .iter()
                .map(|v| v.iter().filter(|e| **e == 1).count())
                .sum::<usize>()
                * target
                    .eles
                    .iter()
                    .map(|v| v.iter().filter(|e| **e == 2).count())
                    .sum::<usize>()
        )]
    }

    fn part2(&mut self) -> Vec<String> {
        let mut final_layer = Layer {
            eles: vec![vec![0; WIDE]; TALL],
        };

        for row in 0..TALL {
            for col in 0..WIDE {
                let mut layer_idx = 0;
                loop {
                    match self.layers[layer_idx].eles[row][col] {
                        0 | 1 => {
                            final_layer.eles[row][col] = self.layers[layer_idx].eles[row][col];
                            break;
                        }
                        2 => {
                            layer_idx += 1;
                            assert!(layer_idx < self.layers.len());
                        }
                        _ => panic!("not expected value"),
                    }
                }
            }
        }

        // println!("{:?}", final_layer);
        for row in &final_layer.eles {
            for &pixel in row {
                print!("{}", if pixel == 1 { "█" } else { " " });
            }
            println!();
        }
        vec![format!("{}", 0)]
    }
}
