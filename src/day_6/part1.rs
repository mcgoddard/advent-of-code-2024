use super::lib::{get_visited, parse_input};

pub fn part1(lines: &[String]) -> String {
  let (position, direction, map) = parse_input(lines);
  get_visited(position, &direction, &map).len().to_string()
}
