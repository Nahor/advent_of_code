use clap::{Parser, Subcommand};
use common::read_input_u8;
use day23::{part1, part2, part2_brute_force, part2_optimize};
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
    Part2 { file: Option<PathBuf> },

    /// Part 2
    Part2Optimize { file: Option<PathBuf> },

    /// Part 2
    Part2BruteForce { file: Option<PathBuf> },
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
        Some(Command::Part2Optimize { file }) => {
            println!("Result: {}", part2_optimize::run(&read_input_u8!(file)?)?)
        }
        Some(Command::Part2BruteForce { file }) => {
            println!(
                "Result: {}",
                part2_brute_force::run(&read_input_u8!(file)?)?
            )
        }
        None => println!("Result: {}", part1::run(&read_input_u8!(None)?)?),
    }

    Ok(())
}
