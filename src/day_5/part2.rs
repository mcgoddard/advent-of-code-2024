use std::collections::{HashMap, HashSet};
use super::lib::{check_update, parse_rules_and_updates};

pub fn part2(lines: Vec<String>) -> i64 {
  let (rules_hash, updates) = parse_rules_and_updates(lines);
  let invalid_updates = updates.iter().filter(|u| !check_update(&rules_hash, u)).cloned().collect::<Vec<Vec<i64>>>();
  let fixed_updates = invalid_updates.iter().map(|u| {
    let mut new_update = u.clone();
    while !check_update(&rules_hash, &new_update) {
      new_update = fix_update(&rules_hash, &new_update);
    }
    new_update
  }).collect::<Vec<Vec<i64>>>();
  let middle_values = fixed_updates.iter().map(|u| u[u.len() / 2]).collect::<Vec<i64>>();
  middle_values.iter().sum()
}

fn fix_update(rules_hash: &HashMap<i64, HashSet<i64>>, update: &[i64]) -> Vec<i64> {
  let mut new_update = update.to_owned();
  for (index, v) in update.iter().enumerate() {
    if rules_hash.contains_key(v) {
      for i in 0..index {
        if rules_hash[v].contains(&new_update[i]) {
          new_update.remove(index);
          new_update.insert(i, *v);
          return new_update;
        }
      }
    }
  }
  new_update
}
