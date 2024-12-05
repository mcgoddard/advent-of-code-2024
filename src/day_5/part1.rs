use std::collections::{HashMap, HashSet};

pub fn part1(lines: Vec<String>) -> i64 {
  let mut rules = true;
  let mut ordering_rules = vec![];
  let mut updates = vec![];
  for line in lines {
    if line == "" {
      rules = false;
      continue;
    }
    if rules {
      ordering_rules.push(line.split("|").map(|x| x.parse().unwrap()).collect::<Vec<i64>>());
    } else {
      updates.push(line.split(",").map(|x| x.parse().unwrap()).collect::<Vec<i64>>());
    }
  }
  let mut rules_hash: HashMap<i64, HashSet<i64>> = HashMap::new();
  for rule in ordering_rules {
    match rules_hash.get_mut(&rule[0]) {
      Some(x) => {
        x.insert(rule[1]);
      },
      None => {
        let mut rule_set = HashSet::new();
        rule_set.insert(rule[1]);
        rules_hash.insert(rule[0], rule_set);
      },
    };
  }
  let valid_updates = updates.iter().filter(|u| {
    for (index, v) in u.iter().enumerate() {
      if rules_hash.contains_key(v) {
        for i in 0..index {
          if rules_hash[v].contains(&u[i]) {
            return false;
          }
        }
      }
    }
    true
  });
  let middle_values = valid_updates.map(|u| u[u.len() / 2]).collect::<Vec<i64>>();
  middle_values.iter().sum()
}
