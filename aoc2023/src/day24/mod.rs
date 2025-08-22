use aoclib::Runner;
use nalgebra::Vector3;

#[derive(Debug)]
struct Hailstone {
    x: f64,
    y: f64,
    z: f64,
    vx: f64,
    vy: f64,
    vz: f64,
}

impl Hailstone {
    fn position(&self) -> Vector3<f64> {
        Vector3::new(self.x, self.y, self.z)
    }

    fn velocity(&self) -> Vector3<f64> {
        Vector3::new(self.vx, self.vy, self.vz)
    }
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
                    Some((x, y, _t, _u)) => {
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

    // https://www.reddit.com/r/adventofcode/comments/18pnycy/comment/kxqjg33/?utm_source=share&utm_medium=web3x&utm_name=web3xcss&utm_term=1&utm_content=share_button
    fn part2(&mut self) -> Vec<String> {
        let h0 = &self.hailstones[0];
        let h1 = &self.hailstones[1];
        let h2 = &self.hailstones[2];

        let p1 = Vector3::new(h1.x - h0.x, h1.y - h0.y, h1.z - h0.z);
        let v1 = Vector3::new(h1.vx - h0.vx, h1.vy - h0.vy, h1.vz - h0.vz);

        let p2 = Vector3::new(h2.x - h0.x, h2.y - h0.y, h2.z - h0.z);
        let v2 = Vector3::new(h2.vx - h0.vx, h2.vy - h0.vy, h2.vz - h0.vz);

        let cross_p1p2 = p1.cross(&p2);
        let cross_v1p2 = v1.cross(&p2);
        let cross_p1v2 = p1.cross(&v2);

        // 求 t1
        let numerator_t1 = cross_p1p2.dot(&v2);
        let denominator_t1 = cross_v1p2.dot(&v2);
        let t1 = -numerator_t1 / denominator_t1;

        // 求 t2
        let numerator_t2 = cross_p1p2.dot(&v1);
        let denominator_t2 = cross_p1v2.dot(&v1);
        let t2 = -numerator_t2 / denominator_t2;

        // 两个碰撞点
        let c1 = h1.position() + h1.velocity() * t1;
        let c2 = h2.position() + h2.velocity() * t2;

        let v = (c2 - c1) / (t2 - t1);
        let p = c1 - v * t1;

        println!("p = {:?}, v = {:?}", p, v);

        vec![format!("{}", p.sum())]
    }
}
