use anyhow::{Context, Result};
use std::fs;
use std::io::Write;

pub fn generate_aoc_template(year: u32, day: u32) -> Result<()> {
    let dir = format!("aoc{}/src/day{}", year, day);
    let file_path = format!("{}/mod.rs", dir);

    if fs::exists(&file_path).context("check path fail")? {
        println!("skip already exists");
        return Ok(());
    }

    fs::create_dir_all(&dir).context("create dir fail")?;

    let template = format!(
        r#"use aoclib::Runner;

#[derive(Default)]
pub struct Aoc{year}_{day} {{}}

impl Aoc{year}_{day} {{
    pub fn new() -> Self {{
        Self::default()
    }}
}}

impl Runner for Aoc{year}_{day} {{
    fn info(&self) -> (usize, usize) {{
        ({year}, {day})
    }}

    fn parse(&mut self) {{
        let inputs = aoclib::utils::read_file("./inputs/{year}/{day:02}.txt");
    }}

    fn part1(&mut self) -> Vec<String> {{
        let mut res = 0;
        vec![format!("{{}}", res)]
    }}

    fn part2(&mut self) -> Vec<String> {{
        let mut res = 0;
        vec![format!("{{}}", res)]
    }}
}}
"#,
        year = year,
        day = day
    );

    let mut file = fs::File::create(&file_path).context("create file fail")?;
    file.write_all(template.as_bytes())
        .context("write file fail")?;

    println!("Generated template at: {}", file_path);
    Ok(())
}
