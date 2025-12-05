use clap::{Parser, Subcommand};
use common::read_input_u8;
use day05::*;
use miette::Result;
use std::path::PathBuf;

#[derive(Parser, Debug)]
struct Args {
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand, Debug)]
// snake case to match the name of module and thus have the same name between `cargo run` and `cargo test`
#[command(rename_all = "snake_case")]
enum Command {
    /// Part 1
    Part1 { file: Option<PathBuf> },

    /// Part 2
    Part2 { file: Option<PathBuf> },

    /// Part 2 Sort
    Part2Sort { file: Option<PathBuf> },

    /// Part 2 Custom
    Part2Custom { file: Option<PathBuf> },
}

fn main() -> Result<()> {
    let args = Args::parse();

    match args.command {
        Some(Command::Part1 { file }) => {
            println!("Result: {}", part1::run(&read_input_u8!(file)?)?)
        }
        Some(Command::Part2 { file }) => {
            println!("Result: {}", part2::run(&read_input_u8!(file)?)?)
        }
        Some(Command::Part2Sort { file }) => {
            println!("Result: {}", part2_sort::run(&read_input_u8!(file)?)?)
        }
        Some(Command::Part2Custom { file }) => {
            println!("Result: {}", part2_custom::run(&read_input_u8!(file)?)?)
        }
        None => println!("Result: {}", part1::run(&read_input_u8!(None)?)?),
    }

    Ok(())
}
