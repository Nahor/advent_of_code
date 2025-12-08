use clap::{Parser, Subcommand};
use common::read_input_u8;
use day08::*;
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

    /// Part 2 Indexed
    Part2Indexed { file: Option<PathBuf> },

    /// Part 2 Vec Sort
    Part2VecSort { file: Option<PathBuf> },

    /// Part 2 Vec Indexed
    Part2VecIndexed { file: Option<PathBuf> },

    /// Part 2 Parallel Sort
    Part2ParSort { file: Option<PathBuf> },
}

fn main() -> Result<()> {
    let args = Args::parse();

    match args.command {
        Some(Command::Part1 { file }) => {
            println!("Result: {}", part1::run(&read_input_u8!(file)?, 1000)?)
        }
        Some(Command::Part2 { file }) => {
            println!("Result: {}", part2::run(&read_input_u8!(file)?)?)
        }
        Some(Command::Part2Indexed { file }) => {
            println!("Result: {}", part2_indexed::run(&read_input_u8!(file)?)?)
        }
        Some(Command::Part2VecSort { file }) => {
            println!("Result: {}", part2_vec_sort::run(&read_input_u8!(file)?)?)
        }
        Some(Command::Part2VecIndexed { file }) => {
            println!(
                "Result: {}",
                part2_vec_indexed::run(&read_input_u8!(file)?)?
            )
        }
        Some(Command::Part2ParSort { file }) => {
            println!("Result: {}", part2_par_sort::run(&read_input_u8!(file)?)?)
        }
        None => println!("Result: {}", part1::run(&read_input_u8!(None)?, 1000)?),
    }

    Ok(())
}
