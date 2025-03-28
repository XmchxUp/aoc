use std::env;
use std::fs;

use reqwest::blocking::Client;

pub fn download_input(year: u32, day: u32) -> Result<(), Box<dyn std::error::Error>> {
    let input_dir = format!("./inputs/{}", year);
    fs::create_dir_all(&input_dir)?;

    let url = format!("https://adventofcode.com/{}/day/{}/input", year, day);
    let input_path = format!("{}/{:02}.txt", input_dir, day);

    let session_cookie =
        env::var("AOC_SESSION_COOKIE").expect("Please set AOC_SESSION_COOKIE environment variable");

    let client = Client::new();
    let response = client
        .get(&url)
        .header("Cookie", format!("session={}", session_cookie))
        .send()?;

    if response.status().is_success() {
        let content = response.text()?;
        fs::write(&input_path, content)?;
        println!("Downloaded input to: {}", input_path);
    } else {
        println!("Failed to download input: HTTP {}", response.status());
    }

    Ok(())
}
