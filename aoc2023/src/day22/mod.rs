use std::collections::{HashMap, HashSet, VecDeque};

use aoclib::Runner;

#[derive(Debug)]
struct Brick {
    x1: i32,
    y1: i32,
    z1: i32,
    x2: i32,
    y2: i32,
    z2: i32,
}

impl Brick {
    fn from_coords(c1: &[i32], c2: &[i32]) -> Self {
        assert_eq!(c1.len(), 3);
        assert_eq!(c2.len(), 3);
        Brick {
            x1: c1[0].min(c2[0]),
            y1: c1[1].min(c2[1]),
            z1: c1[2].min(c2[2]),
            x2: c1[0].max(c2[0]),
            y2: c1[1].max(c2[1]),
            z2: c1[2].max(c2[2]),
        }
    }
}

#[derive(Default)]
pub struct Aoc2023_22 {
    bricks: Vec<Brick>,
}

impl Aoc2023_22 {
    pub fn new() -> Self {
        Self::default()
    }

    fn helper(
        &mut self,
    ) -> (
        HashMap<usize, HashSet<usize>>,
        HashMap<usize, HashSet<usize>>,
    ) {
        self.bricks.sort_by(|a, b| a.z1.cmp(&b.z1));

        let mut height_map = HashMap::new(); // (x,y) -> (max_z, brick_id)
        let mut supported_by = HashMap::new(); // brick_id -> supports from
        let mut supports = HashMap::new(); // brick_id -> supports to

        for (brick_id, brick) in self.bricks.iter().enumerate() {
            // Find the highest point below this brick
            let mut max_below = 0;
            let mut supporting_bricks = HashSet::new();

            for x in brick.x1..=brick.x2 {
                for y in brick.y1..=brick.y2 {
                    if let Some(&(curr_z, curr_id)) = height_map.get(&(x, y)) {
                        if curr_z > max_below {
                            max_below = curr_z;
                            supporting_bricks.clear();
                            supporting_bricks.insert(curr_id);
                        } else if curr_z == max_below {
                            supporting_bricks.insert(curr_id);
                        }
                    }
                }
            }

            // The brick will settle at max_below + 1
            let new_z = max_below + 1;
            let brick_height = brick.z2 - brick.z1 + 1;

            for x in brick.x1..=brick.x2 {
                for y in brick.y1..=brick.y2 {
                    height_map.insert((x, y), (new_z + brick_height - 1, brick_id));
                }
            }

            supported_by.insert(brick_id, supporting_bricks.clone());
            for supporting_id in supporting_bricks {
                supports
                    .entry(supporting_id)
                    .or_insert_with(HashSet::new)
                    .insert(brick_id);
            }
        }

        (supported_by, supports)
    }
}

impl Runner for Aoc2023_22 {
    fn info(&self) -> (usize, usize) {
        (2023, 22)
    }

    fn parse(&mut self) {
        let inputs = aoclib::utils::read_file("./inputs/2023/22.txt");
        for line in inputs {
            // 4,4,141~6,4,141
            let parts: Vec<&str> = line.split('~').collect();
            assert_eq!(parts.len(), 2);
            let left_part: Vec<&str> = parts[0].split(',').collect();
            assert_eq!(left_part.len(), 3);
            let right_part: Vec<&str> = parts[1].split(',').collect();
            assert_eq!(right_part.len(), 3);

            let left = left_part
                .iter()
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();
            let right = right_part
                .iter()
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();

            self.bricks.push(Brick::from_coords(&left, &right));
        }
    }

    fn part1(&mut self) -> Vec<String> {
        let (supported_by, supports) = self.helper();
        let mut safe_count = 0;
        for brick_id in 0..self.bricks.len() {
            let mut is_safe = true;

            if let Some(supported_bricks) = supports.get(&brick_id) {
                for &supported_id in supported_bricks {
                    if supported_by[&supported_id].len() <= 1 {
                        is_safe = false;
                        break;
                    }
                }
            }

            if is_safe {
                safe_count += 1;
            }
        }

        vec![format!("{}", safe_count)]
    }

    fn part2(&mut self) -> Vec<String> {
        let (supported_by, supports) = self.helper();

        let mut total = 0;

        for brick_id in 0..self.bricks.len() {
            let mut falling = HashSet::new();
            let mut queue = VecDeque::new();

            if let Some(supported_bricks) = supports.get(&brick_id) {
                for &supported_id in supported_bricks {
                    if supported_by[&supported_id].len() == 1 {
                        queue.push_back(supported_id);
                        falling.insert(supported_id);
                    }
                }
            }

            while let Some(current_id) = queue.pop_front() {
                if let Some(above_bricks) = supports.get(&current_id) {
                    for &above_id in above_bricks {
                        if !falling.contains(&above_id) {
                            let all_supporters_fallen = supported_by[&above_id]
                                .iter()
                                .all(|&id| falling.contains(&id) || id == brick_id);

                            if all_supporters_fallen {
                                falling.insert(above_id);
                                queue.push_back(above_id);
                            }
                        }
                    }
                }
            }

            total += falling.len();
        }

        vec![format!("{}", total)]
    }
}
