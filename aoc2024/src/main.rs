use aoclib::{Runner, Selector};

mod day1;
use day1::*;
mod day2;
use day2::*;
mod day3;
use day3::*;
mod day4;
use day4::*;
mod day5;
use day5::*;

fn main() {
    let day1 = Aoc2024_1::new();
    let day2 = Aoc2024_2::new();
    let day3 = Aoc2024_3::new();
    let day4 = Aoc2024_4::new();
    let day5 = Aoc2024_5::new();

    let mut days: Vec<Box<dyn Runner>> = Vec::with_capacity(25);
    days.insert(0, Box::new(day1));
    days.insert(1, Box::new(day2));
    days.insert(2, Box::new(day3));
    days.insert(3, Box::new(day4));
    days.insert(4, Box::new(day5));

    let which = Selector::Last;

    match which {
        Selector::All => todo!(),
        Selector::One(d) => {
            aoclib::run(days[d].as_mut());
        }
        Selector::Last => {
            let last = days.len() - 1;
            aoclib::run(days[last].as_mut());
        }
    }
}
