use clap::{Parser, Subcommand};
use common::read_input_u8;
use day12::*;
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
}

fn main() -> Result<()> {
    let args = Args::parse();

    match args.command {
        Some(Command::Part1 { file }) => {
            println!("Result: {}", part1::run(&read_input_u8!(file)?)?)
        }
        None => println!("Result: {}", part1::run(&read_input_u8!(None)?)?),
    }

    Ok(())
}
