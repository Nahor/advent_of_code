use clap::{Parser, Subcommand};
use common::read_input_str;
use day02::{part1, part2_brute_force, part2_optimized};
use miette::Result;
use std::path::PathBuf;

#[derive(Parser, Debug)]
struct Args {
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand, Debug)]
#[command(rename_all = "lower")]
enum Command {
    /// Part 1
    Part1 { file: Option<PathBuf> },

    /// Part 2
    Part2Brute { file: Option<PathBuf> },

    /// Part 2
    Part2Optimized { file: Option<PathBuf> },
}

fn main() -> Result<()> {
    let args = Args::parse();

    match args.command {
        Some(Command::Part1 { file }) => {
            println!("Result: {}", part1::run(&read_input_str!(file)?)?)
        }
        Some(Command::Part2Brute { file }) => {
            println!(
                "Result: {}",
                part2_brute_force::run(&read_input_str!(file)?)?
            )
        }
        Some(Command::Part2Optimized { file }) => {
            println!("Result: {}", part2_optimized::run(&read_input_str!(file)?)?)
        }
        None => println!("Result: {}", part1::run(&read_input_str!(None)?)?),
    }

    Ok(())
}
