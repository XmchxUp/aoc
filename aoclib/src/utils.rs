use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn read_file(filename: &str) -> Vec<String> {
    let file = File::open(filename).expect("cannot open file");

    BufReader::new(file)
        .lines()
        .map(|line| line.unwrap())
        .collect()
}

pub fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

/// ax + by = gcd(a,b)
pub fn extended_gcd(a: i64, b: i64) -> (i64, i64, i64) {
    if a == 0 {
        // Base case: gcd(0, b) = b, and coefficients are x = 0, y = 1
        (b, 0, 1)
    } else {
        // Recursive case: apply the Extended Euclidean Algorithm
        let (gcd, x1, y1) = extended_gcd(b % a, a);
        // Update coefficients using the results of the recursive call
        let x = y1 - (b / a) * x1;
        let y = x1;
        (gcd, x, y)
    }
}

pub fn lcm(a: u64, b: u64) -> u64 {
    (a * b) / gcd(a, b)
}
