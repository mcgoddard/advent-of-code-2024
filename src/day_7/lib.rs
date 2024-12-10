use itertools::Itertools;

#[derive(Debug, Eq, PartialEq, Hash)]
pub struct Equation {
  pub result: i64,
  pub values: Vec<i64>,
}

pub fn variations_up_to_length<T>(items: &[T], length: usize) -> impl Iterator<Item = Vec<&T>> {
  std::iter::repeat(items.iter()).take(length).multi_cartesian_product()
}

pub fn parse_equations(lines: &[String]) -> Vec<Equation> {
  lines.iter().map(|line| {
    let parts = line.split(": ").collect::<Vec<&str>>();
    Equation {
      result: parts[0].parse().unwrap(),
      values: parts[1].split(" ").map(|v| v.parse().unwrap()).collect(),
    }
  }).collect::<Vec<Equation>>()
}
