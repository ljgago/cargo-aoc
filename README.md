# cargo-aoc

A simple Advent of Code helper for the Cargo subcommand to download your puzzles into a file.

## Install

```bash
git clone https://github.com/ljgago/cargo-aoc
cd cargo-aoc
cargo install --path .
```

## Usage

You need set the environment variable `AOC_SESSION` using your Advent of Code session cookie.

```
$ cargo aoc -h
Advent of Code helper to download your puzzles into a file.

Usage: cargo aoc [OPTIONS] --day <DAY>

Options:
  -y, --year <YEAR>  Year of event (2015 to [default]) [default: 2024]
  -d, --day <DAY>    Day of challenger (1 to 25)
  -p, --path <PATH>  Path where the puzzle will be saved [default: .]
  -h, --help         Print help (see more with '--help')
  -V, --version      Print version
```

```
$ cargo aoc --help
Advent of Code helper to download your puzzles into a file.

You must set the AOC_SESSION environment variable using your Advent of Code session cookie.
You can use direnv and configure your .envrc file like this:

    export AOC_SESSION="YOUR_SESSION_COOKIE"

Usage: cargo aoc [OPTIONS] --day <DAY>

Options:
  -y, --year <YEAR>
          Year of event (2015 to [default])

          [default: 2024]

  -d, --day <DAY>
          Day of challenger (1 to 25)

  -p, --path <PATH>
          Path where the puzzle will be saved

          [default: .]

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```

## Licence

[MIT License](./LICENSE)
