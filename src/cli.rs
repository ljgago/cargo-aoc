use clap::{Args, Parser, Subcommand};

use crate::utils;

#[derive(Parser)]
#[command(
    about = "Advent of Code helper to download your puzzles into a file.",
    display_name = "cargo",
    bin_name = "cargo",
    propagate_version = true,
    version
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: CliCommand,
}

// 1st command level
#[derive(Subcommand)]
pub enum CliCommand {
    Aoc(Aoc),
}

#[derive(Args)]
#[command(
    about = "Advent of Code helper to download your puzzles into a file.",
    long_about = r#"
Advent of Code helper to download your puzzles into a file.

You must set the AOC_SESSION environment variable using your Advent of Code session cookie.
You can use direnv and configure your .envrc file like this:

    export AOC_SESSION="YOUR_SESSION_COOKIE""#
)]
pub struct Aoc {
    /// Year of event (2015 to [default])
    #[arg(short, long, default_value_t = utils::last_year())]
    pub year: u32,

    /// Day of challenger (1 to 25)
    #[arg(short, long)]
    pub day: u32,

    /// Path where the puzzle will be saved
    #[arg(short, long, default_value_t = String::from("."))]
    pub path: String,
}
