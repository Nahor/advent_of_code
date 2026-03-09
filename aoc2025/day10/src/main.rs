use clap::{Parser, Subcommand};
use common::read_input_u8;
use day10::*;
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

    /// Part 2 Z3
    Part2Z3 { file: Option<PathBuf> },

    /// Part 2 Z3 Optimize
    Part2Z3Optimize { file: Option<PathBuf> },

    /// Part 2 Good-Lp
    Part2GoodLp { file: Option<PathBuf> },

    /// Part 2 Smart
    Part2Smart { file: Option<PathBuf> },

    /// Part 2 Gaussian
    Part2Gaussian { file: Option<PathBuf> },
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
        Some(Command::Part2Z3 { file }) => {
            println!("Result: {}", part2_z3::run(&read_input_u8!(file)?)?)
        }
        Some(Command::Part2Z3Optimize { file }) => {
            println!(
                "Result: {}",
                part2_z3_optimize::run(&read_input_u8!(file)?)?
            )
        }
        Some(Command::Part2GoodLp { file }) => {
            println!("Result: {}", part2_good_lp::run(&read_input_u8!(file)?)?)
        }
        Some(Command::Part2Smart { file }) => {
            println!("Result: {}", part2_smart::run(&read_input_u8!(file)?)?)
        }
        Some(Command::Part2Gaussian { file }) => {
            println!("Result: {}", part2_gaussian::run(&read_input_u8!(file)?)?)
        }
        None => println!("Result: {}", part1::run(&read_input_u8!(None)?)?),
    }

    Ok(())
}
