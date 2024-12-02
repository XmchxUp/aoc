use std::collections::HashMap;

use aoclib::{Runner, Selector};

mod day1;
use day1::*;
mod day2;
use day2::*;
fn main() {
    let day1 = Aoc2023_1::new();
    let day2 = Aoc2023_2::new();

    let mut days: HashMap<usize, Box<dyn Runner>> = HashMap::new();
    days.insert(1, Box::new(day1));
    days.insert(2, Box::new(day2));

    let which = Selector::One(2);

    match which {
        Selector::All => todo!(),
        Selector::One(d) => {
            let day = days.get_mut(&d).unwrap();
            aoclib::run(day.as_mut());
        }
        Selector::Last => todo!(),
    }
}
