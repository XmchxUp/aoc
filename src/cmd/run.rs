use aoclib::Runner;

fn create_runner(year: u32, day: u32) -> Option<Box<dyn Runner>> {
    match (year, day) {
        (2019, 1) => Some(Box::new(aoc2019::Aoc2019_1::new())),
        (2019, 2) => Some(Box::new(aoc2019::Aoc2019_2::new())),
        (2024, 1) => Some(Box::new(aoc2024::Aoc2024_1::new())),
        (2024, 2) => Some(Box::new(aoc2024::Aoc2024_2::new())),
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
