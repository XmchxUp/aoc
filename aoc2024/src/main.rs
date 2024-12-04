use std::collections::HashMap;

use aoclib::{Runner, Selector};

mod day1;
use day1::*;
mod day2;
use day2::*;
mod day3;
use day3::*;
mod day4;
use day4::*;

fn main() {
    let day1 = Aoc2024_1::new();
    let day2 = Aoc2024_2::new();
    let day3 = Aoc2024_3::new();
    let day4 = Aoc2024_4::new();

    let mut days: HashMap<usize, Box<dyn Runner>> = HashMap::new();
    days.insert(1, Box::new(day1));
    days.insert(2, Box::new(day2));
    days.insert(3, Box::new(day3));
    days.insert(4, Box::new(day4));

    let which = Selector::One(4);

    match which {
        Selector::All => todo!(),
        Selector::One(d) => {
            let day = days.get_mut(&d).unwrap();
            aoclib::run(day.as_mut());
        }
        Selector::Last => todo!(),
    }
}
