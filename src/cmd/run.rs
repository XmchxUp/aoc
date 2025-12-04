use aoclib::Runner;

fn create_runner(year: u32, day: u32) -> Option<Box<dyn Runner>> {
    match (year, day) {
        // 2015
        (2015, 4) => Some(Box::new(aoc2015::Aoc2015_4::new())),
        (2015, 5) => Some(Box::new(aoc2015::Aoc2015_5::new())),
        // 2019
        (2019, 1) => Some(Box::new(aoc2019::Aoc2019_1::new())),
        (2019, 2) => Some(Box::new(aoc2019::Aoc2019_2::new())),
        (2019, 3) => Some(Box::new(aoc2019::Aoc2019_3::new())),
        (2019, 4) => Some(Box::new(aoc2019::Aoc2019_4::new())),
        (2019, 5) => Some(Box::new(aoc2019::Aoc2019_5::new())),
        (2019, 6) => Some(Box::new(aoc2019::Aoc2019_6::new())),
        (2019, 7) => Some(Box::new(aoc2019::Aoc2019_7::new())),
        (2019, 8) => Some(Box::new(aoc2019::Aoc2019_8::new())),
        // 2023
        (2023, 12) => Some(Box::new(aoc2023::Aoc2023_12::new())),
        (2023, 13) => Some(Box::new(aoc2023::Aoc2023_13::new())),
        (2023, 14) => Some(Box::new(aoc2023::Aoc2023_14::new())),
        (2023, 15) => Some(Box::new(aoc2023::Aoc2023_15::new())),
        (2023, 16) => Some(Box::new(aoc2023::Aoc2023_16::new())),
        (2023, 17) => Some(Box::new(aoc2023::Aoc2023_17::new())),
        (2023, 18) => Some(Box::new(aoc2023::Aoc2023_18::new())),
        (2023, 19) => Some(Box::new(aoc2023::Aoc2023_19::new())),
        (2023, 20) => Some(Box::new(aoc2023::Aoc2023_20::new())),
        (2023, 21) => Some(Box::new(aoc2023::Aoc2023_21::new())),
        (2023, 22) => Some(Box::new(aoc2023::Aoc2023_22::new())),
        (2023, 23) => Some(Box::new(aoc2023::Aoc2023_23::new())),
        (2023, 24) => Some(Box::new(aoc2023::Aoc2023_24::new())),
        (2023, 25) => Some(Box::new(aoc2023::Aoc2023_25::new())),
        // 2024
        (2024, 1) => Some(Box::new(aoc2024::Aoc2024_1::new())),
        (2024, 2) => Some(Box::new(aoc2024::Aoc2024_2::new())),
        (2024, 3) => Some(Box::new(aoc2024::Aoc2024_3::new())),
        (2024, 4) => Some(Box::new(aoc2024::Aoc2024_4::new())),
        (2024, 5) => Some(Box::new(aoc2024::Aoc2024_5::new())),
        (2024, 6) => Some(Box::new(aoc2024::Aoc2024_6::new())),
        (2024, 7) => Some(Box::new(aoc2024::Aoc2024_7::new())),
        (2024, 8) => Some(Box::new(aoc2024::Aoc2024_8::new())),
        (2024, 9) => Some(Box::new(aoc2024::Aoc2024_9::new())),
        (2024, 10) => Some(Box::new(aoc2024::Aoc2024_10::new())),
        (2024, 11) => Some(Box::new(aoc2024::Aoc2024_11::new())),
        (2024, 12) => Some(Box::new(aoc2024::Aoc2024_12::new())),
        (2024, 13) => Some(Box::new(aoc2024::Aoc2024_13::new())),
        // 2025
        (2025, 1) => Some(Box::new(aoc2025::Aoc2025_1::new())),
        (2025, 2) => Some(Box::new(aoc2025::Aoc2025_2::new())),
        _ => None,
    }
}

pub fn run(year: u32, day: u32) {
    match create_runner(year, day) {
        Some(mut runner) => {
            println!("Running Aoc{}_{}", year, day);
            aoclib::run(runner.as_mut());
        }
        None => {
            eprintln!("No implementation found for Aoc{}_{}", year, day);
        }
    }
}
