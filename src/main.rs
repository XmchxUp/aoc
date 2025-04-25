use anyhow::{Context, Result};
use clap::{Parser, Subcommand};

mod cmd;
use cmd::*;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate template code
    Gen {
        #[arg(short, long)]
        year: u32,
        #[arg(short, long)]
        day: u32,
    },
    /// Download inputs
    Download {
        #[arg(short, long)]
        year: u32,
        #[arg(short, long)]
        day: u32,
    },
    /// Execute code
    Run {
        #[arg(short, long)]
        year: u32,
        #[arg(short, long)]
        day: u32,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Gen { year, day } => {
            generate_aoc_template(year, day).context("generate aoc template fail")?;
        }
        Commands::Run { year, day } => {
            run(year, day);
        }
        Commands::Download { year, day } => {
            download_input(year, day).context("download input fail:")?;
        }
    }
    Ok(())
}
