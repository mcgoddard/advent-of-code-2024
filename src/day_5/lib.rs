use std::collections::{HashMap, HashSet};

pub fn check_update(rules_hash: &HashMap<i64, HashSet<i64>>, update: &[i64]) -> bool {
  for (index, v) in update.iter().enumerate() {
    if rules_hash.contains_key(v) {
      for earlier_char in update.iter().take(index) {
        if rules_hash[v].contains(earlier_char) {
          return false;
        }
      }
    }
  }
  true
}

pub fn parse_rules_and_updates(lines: &[String]) -> (HashMap<i64, HashSet<i64>>, Vec<Vec<i64>>) {
  let mut rules = true;
  let mut ordering_rules = vec![];
  let mut updates = vec![];
  for line in lines {
    if line.is_empty() {
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
  (rules_hash, updates)
}
