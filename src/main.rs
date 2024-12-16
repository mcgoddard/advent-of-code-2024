mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod day_7;
mod day_8;
mod day_9;
mod day_10;
mod day_11;
mod day_12;
mod day_13;
mod day_15;
mod solverwrapper;

use advent_of_code_2024::read_lines;
use clap::Parser;

use solverwrapper::SolverWrapper;

/// Advent of Code 2024
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
  /// Day of AoC
  #[arg(short, long)]
  day: u8,

  /// Part of the day
  #[arg(short, long)]
  part: u8,

  /// Use sample input file
  #[arg(short, long, default_value_t=false)]
  sample: bool,
}

fn main() {
  let args = Args::parse();

  let solver = SolverWrapper::new(args.day);

  let mut filename = format!("./src/day_{}/input.txt", args.day);
  if args.sample {
    filename = format!("./src/day_{}/sample.txt", args.day);
  }
  let lines = read_lines(&filename);

  let result = solver.solve(args.part, &lines);

  println!("Result: {}", result);
}
