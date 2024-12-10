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
mod day6;
use day6::*;
mod day7;
use day7::*;
mod day8;
use day8::*;
mod day9;
use day9::*;

fn main() {
    let day1 = Aoc2024_1::new();
    let day2 = Aoc2024_2::new();
    let day3 = Aoc2024_3::new();
    let day4 = Aoc2024_4::new();
    let day5 = Aoc2024_5::new();
    let day6 = Aoc2024_6::new();
    let day7 = Aoc2024_7::new();
    let day8 = Aoc2024_8::new();
    let day9 = Aoc2024_9::new();

    let mut days: Vec<Box<dyn Runner>> = Vec::with_capacity(25);
    days.push(Box::new(day1));
    days.push(Box::new(day2));
    days.push(Box::new(day3));
    days.push(Box::new(day4));
    days.push(Box::new(day5));
    days.push(Box::new(day6));
    days.push(Box::new(day7));
    days.push(Box::new(day8));
    days.push(Box::new(day9));

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
