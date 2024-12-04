use advent_of_code_2024::SolverTrait;

mod part1;
mod part2;
mod lib;

pub struct Solver;

impl SolverTrait for Solver {
  fn part_1(&self, lines: Vec<String>) -> i64 {
    part1::part1(lines)
  }

  fn part_2(&self, lines: Vec<String>) -> i64 {
    part2::part2(lines)
  }
}
