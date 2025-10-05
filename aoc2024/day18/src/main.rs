use clap::{Parser, Subcommand};
use common::read_input_u8;
use day18::{part1, part2, part2_brute_force};
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
    Part2BF { file: Option<PathBuf> },
}

fn main() -> Result<()> {
    let args = Args::parse();

    match args.command {
        Some(Command::Part1 { file }) => {
            println!("Result: {}", part1::run(&read_input_u8!(file)?, 70, 1024)?)
        }
        Some(Command::Part2 { file }) => {
            println!(
                "Result: {:?}",
                part2::run(&read_input_u8!(file)?, 70, 1024)?
            )
        }
        Some(Command::Part2BF { file }) => {
            println!(
                "Result: {:?}",
                part2_brute_force::run(&read_input_u8!(file)?, 70, 1024)?
            )
        }
        None => println!("Result: {}", part1::run(&read_input_u8!(None)?, 70, 1024)?),
    }

    Ok(())
}
