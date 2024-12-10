use std::fs::read_to_string;

pub fn read_lines(filename: &str) -> Vec<String> {
  let mut result = Vec::new();

  for line in read_to_string(filename).unwrap().lines() {
      result.push(line.to_string())
  }

  result
}

pub trait SolverTrait {
  fn part_1(&self, lines: &[String]) -> i64;
  fn part_2(&self, lines: &[String]) -> i64;
}
