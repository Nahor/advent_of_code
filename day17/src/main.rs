use clap::{Parser, Subcommand};
use common::read_input_u8;
use day17::{part1, part1_hardcoded, part2, part2_brute_force};
use miette::Result;
use std::path::PathBuf;

#[derive(Parser, Debug)]
struct Args {
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Part 1
    Part1 { file: Option<PathBuf> },

    /// Part 1
    Part1Hardcoded,

    /// Part 2
    Part2 { file: Option<PathBuf> },

    /// Part 2
    Part2BruteForce,
}

fn main() -> Result<()> {
    let args = Args::parse();

    match args.command {
        Some(Command::Part1 { file }) => {
            println!("Result: {}", part1::run(&read_input_u8!(file)?)?);
        }
        Some(Command::Part1Hardcoded) => {
            part1_hardcoded::run();
        }
        Some(Command::Part2 { file }) => {
            println!("Result: {}", part2::run(&read_input_u8!(file)?)?);
        }
        Some(Command::Part2BruteForce) => {
            part2_brute_force::run();
        }
        None => println!("Result: {}", part1::run(&read_input_u8!(None)?)?),
    }

    Ok(())
}
