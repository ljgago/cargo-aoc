use clap::Parser;
use std::env;

use cargo_aoc::cli::{Cli, CliCommand};

fn main() {
    let cli = Cli::parse();

    match cli.command {
        CliCommand::Aoc(aoc) => {
            let token = env::var("AOC_SESSION");
            if let Err(_) = token {
                println!("ERROR: environment variable AOC_SESSION not found");
                return;
            }

            if let Err(err) = aoc.is_valid_date() {
                println!("{}", err);
                return;
            }

            let puzzle = aoc.fetch_puzzle(&token.unwrap());
            if let Err(err) = puzzle {
                println!("ERROR: {}", err);
                return;
            }

            println!("{}", aoc.write_file(&puzzle.unwrap()).unwrap());
        }
    }
}
