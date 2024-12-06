use clap::{Parser, Subcommand};
use common::input::read_input_u8;
use miette::Result;
use std::path::PathBuf;
use template::{part1, part2};

#[derive(Parser, Debug)]
struct Args {
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Part 1
    Part1 { file: Option<PathBuf> },

    /// Part 2
    Part2 { file: Option<PathBuf> },
}

fn main() -> Result<()> {
    let args = Args::parse();

    match args.command {
        Some(Command::Part1 { file }) => {
            println!("Result: {}", part1::run(&read_input_u8(file)?)?)
        }
        Some(Command::Part2 { file }) => {
            println!("Result: {}", part2::run(&read_input_u8(file)?)?)
        }
        None => println!("Result: {}", part1::run(&read_input_u8(None)?)?),
    }

    Ok(())
}
