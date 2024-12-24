use super::lib::{check_update, parse_rules_and_updates};

pub fn part1(lines: &[String]) -> String {
  let (rules_hash, updates) = parse_rules_and_updates(lines);
  let valid_updates = updates.iter().filter(|u| check_update(&rules_hash, u));
  let middle_values = valid_updates.map(|u| u[u.len() / 2]).collect::<Vec<i64>>();
  middle_values.iter().sum::<i64>().to_string()
}
