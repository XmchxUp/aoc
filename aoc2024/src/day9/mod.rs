use std::collections::HashMap;

use aoclib::Runner;

#[derive(Default)]
pub struct Aoc2024_9 {
    // index -> id
    index_2_id_map: HashMap<u64, u64>,
    // id -> cnt
    id_2_cnt_map: HashMap<u64, u64>,
    // index->free_space_cnt
    index_2_free_space_map: HashMap<u64, u64>,
    max_idx: u64,
}

impl Aoc2024_9 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Aoc2024_9 {
    fn info(&self) -> (usize, usize) {
        (2024, 9)
    }

    fn parse(&mut self) {
        let inputs =
            aoclib::utils::read_file(format!("./inputs/test{:02}.txt", self.info().1).as_str());
        let inputs =
            aoclib::utils::read_file(format!("./inputs/input{:02}.txt", self.info().1).as_str());

        let mut index = 0;
        let mut id = 0;
        let mut i = 0;
        let mut j = 1;

        for line in inputs {
            let chs = line.chars().collect::<Vec<char>>();

            while i < chs.len() {
                let cnt = chs[i].to_digit(10).unwrap() as u64;

                for _ in 0..cnt {
                    self.index_2_id_map.insert(index, id);
                    index += 1;
                }

                self.id_2_cnt_map.insert(id, cnt);

                if j < chs.len() {
                    let free_cnt = chs[j].to_digit(10).unwrap() as u64;
                    self.index_2_free_space_map.insert(index, free_cnt);
                    index += free_cnt;
                }

                id += 1;
                i += 2;
                j = i + 1;
            }
            self.max_idx = index;
        }
        // println!("{:?}", self.index_2_free_space_map);
    }

    fn part1(&mut self) -> Vec<String> {
        let mut index_2_id_map = self.index_2_id_map.clone();
        let mut i_idx = 0;
        let mut j_idx = self.max_idx - 1;

        while i_idx < j_idx {
            while j_idx > i_idx && !index_2_id_map.contains_key(&j_idx) {
                j_idx -= 1;
            }

            while i_idx < j_idx && index_2_id_map.contains_key(&i_idx) {
                i_idx += 1;
            }

            if i_idx < j_idx {
                let v = index_2_id_map.remove(&j_idx).unwrap();
                index_2_id_map.insert(i_idx, v);
            }
            i_idx += 1;
            j_idx -= 1;
        }

        let res = index_2_id_map.iter().map(|(k, v)| k * v).sum::<u64>();
        vec![format!("{}", res)]
    }

    fn part2(&mut self) -> Vec<String> {
        let mut index_2_id_map = self.index_2_id_map.clone();
        let mut j_idx = self.max_idx - 1;

        while j_idx > 0 {
            while j_idx > 0 && !index_2_id_map.contains_key(&j_idx) {
                j_idx -= 1;
            }

            let id = *index_2_id_map.get(&j_idx).unwrap();
            let cnt = self.id_2_cnt_map.get(&id).unwrap();

            let mut keys: Vec<_> = self.index_2_free_space_map.keys().cloned().collect();
            keys.sort();

            for pos in keys {
                if pos > j_idx {
                    break;
                }
                let free_space_size = self.index_2_free_space_map.get(&pos).cloned().unwrap();
                if free_space_size >= *cnt {
                    for i in 0..*cnt {
                        index_2_id_map.remove(&(j_idx - i)).unwrap();
                        index_2_id_map.insert(pos + i, id);
                    }
                    self.index_2_free_space_map.remove(&pos).unwrap();

                    let new_free_space = free_space_size - *cnt;
                    if new_free_space > 0 {
                        let new_pos = pos + *cnt;
                        self.index_2_free_space_map.insert(new_pos, new_free_space);
                    }
                    break;
                }
            }

            if j_idx <= *cnt {
                break;
            }
            j_idx -= cnt;
        }

        let res = index_2_id_map.iter().map(|(k, v)| k * v).sum::<u64>();
        vec![format!("{}", res)]
    }
}
