use std::collections::HashSet;

use super::lib::{parse_input, Colors};

pub fn part1(lines: &[String]) -> String {
  let (available_towels, desired_designs) = parse_input(lines);
  let mut possible_designed = 0;
  for design in desired_designs {
    let mut open: HashSet<Vec<Colors>> = HashSet::from([vec![]]);
    let mut found = false;
    while !found && !open.is_empty() {
      let mut next: HashSet<Vec<Colors>> = HashSet::new();
      for o in open.iter() {
        for available in available_towels.iter() {
          let new_design = [o.to_vec(), available.to_vec()].concat();
          if new_design == *design {
            possible_designed += 1;
            found = true;
            break;
          } else if new_design.len() > design.len() {
            continue;
          } else if new_design == design[..new_design.len()].to_vec() {
            next.insert(new_design);
          }
        }
        if found {
          break;
        }
      }
      open = next;
    }
  }
  possible_designed.to_string()
}
