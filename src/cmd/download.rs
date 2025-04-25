use anyhow::bail;
use anyhow::Context;
use anyhow::Result;
use std::env;
use std::fs;

use reqwest::blocking::Client;

pub fn download_input(year: u32, day: u32) -> Result<()> {
    let input_dir = format!("./inputs/{}", year);
    fs::create_dir_all(&input_dir).context("create file fail")?;

    let url = format!("https://adventofcode.com/{}/day/{}/input", year, day);
    let input_path = format!("{}/{:02}.txt", input_dir, day);

    let session_cookie = env::var("AOC_SESSION_COOKIE")
        .context("Please set AOC_SESSION_COOKIE environment variable")?;

    let client = Client::new();
    let response = client
        .get(&url)
        .header("Cookie", format!("session={}", session_cookie))
        .send()
        .context("http request fail")?;

    if response.status().is_success() {
        let content = response.text().context("read http response fail")?;
        fs::write(&input_path, content).context("write file fail")?;
        println!("Downloaded input to: {}", input_path);
    } else {
        bail!("http response {:?}", response);
    }

    Ok(())
}
