use clap::{Parser, Subcommand};
use common::read_input_u8;
use day14::{part1, part2};
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
}

fn main() -> Result<()> {
    let args = Args::parse();

    match args.command {
        Some(Command::Part1 { file }) => {
            println!(
                "Result: {}",
                part1::run(&read_input_u8!(file)?, (101, 103), 100)?
            )
        }
        Some(Command::Part2 { file }) => {
            println!(
                "Result: {}",
                part2::run(&read_input_u8!(file)?, (101, 103), 100)?
            )
        }
        None => println!(
            "Result: {}",
            part1::run(&read_input_u8!(None)?, (101, 103), 100)?
        ),
    }

    Ok(())
}
