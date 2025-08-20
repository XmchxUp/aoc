use aoclib::Runner;

#[derive(Debug)]
struct Hailstone {
    x: f64,
    y: f64,
    z: f64,
    vx: f64,
    vy: f64,
    vz: f64,
}

#[derive(Default)]
pub struct Aoc2023_24 {
    hailstones: Vec<Hailstone>,
}

impl Aoc2023_24 {
    pub fn new() -> Self {
        Self::default()
    }

    fn intersect(&self, a: &Hailstone, b: &Hailstone) -> Option<(f64, f64, f64, f64)> {
        let dx = b.x - a.x;
        let dy = b.y - a.y;

        let d = b.vx * a.vy - a.vx * b.vy;
        if d == 0.0 {
            return None;
        }

        let t = (b.vx * dy - b.vy * dx) / d;
        let u = (a.vx * dy - a.vy * dx) / d;

        if t < 0.0 || u < 0.0 {
            return None;
        }

        let x = a.x + a.vx * t;
        let y = a.y + a.vy * t;

        Some((x, y, t, u))
    }
}

impl Runner for Aoc2023_24 {
    fn info(&self) -> (usize, usize) {
        (2023, 24)
    }

    fn parse(&mut self) {
        let inputs = aoclib::utils::read_file("./inputs/2023/24.txt");
        // let inputs = aoclib::utils::read_file("./inputs/2023/test.txt");
        for line in inputs {
            let parts: Vec<&str> = line.split("@").collect();
            assert_eq!(parts.len(), 2);
            let position: Vec<f64> = parts[0]
                .split(",")
                .map(|v| v.trim().parse::<f64>().unwrap())
                .collect();
            let velocity: Vec<f64> = parts[1]
                .split(",")
                .map(|v| v.trim().parse::<f64>().unwrap())
                .collect();

            let hailstone = Hailstone {
                x: position[0],
                y: position[1],
                z: position[2],
                vx: velocity[0],
                vy: velocity[1],
                vz: velocity[2],
            };
            self.hailstones.push(hailstone);
        }
        // println!("{:?}", self.hailstones);
    }

    fn part1(&mut self) -> Vec<String> {
        let mut res = 0;
        let xmin = 200000000000000f64;
        let xmax = 400000000000000f64;
        let ymin = 200000000000000f64;
        let ymax = 400000000000000f64;
        // let xmin = 7f64;
        // let xmax = 27f64;
        // let ymin = 7f64;
        // let ymax = 27f64;

        let n = self.hailstones.len();
        for i in 0..n {
            for j in i + 1..n {
                match self.intersect(&self.hailstones[i], &self.hailstones[j]) {
                    None => continue,
                    Some((x, y, t, u)) => {
                        // println!(
                        //     "{:?} {:?} {} {} {} {}",
                        //     self.hailstones[i], self.hailstones[j], x, y, t, u
                        // );
                        if x >= xmin && x <= xmax && y >= ymin && y <= ymax {
                            res += 1;
                        }
                    }
                }
            }
        }
        vec![format!("{}", res)]
    }

    fn part2(&mut self) -> Vec<String> {
        let mut res = 0;
        vec![format!("{}", res)]
    }
}
