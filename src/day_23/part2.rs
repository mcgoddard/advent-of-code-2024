use std::collections::{HashMap, HashSet};

use itertools::Itertools;

pub fn part2(lines: &[String]) -> String {
  let mut connection_maps = HashMap::new();
  lines.iter().for_each(|l| {
    let parts = l.split("-").collect::<Vec<&str>>();
    connection_maps.entry(parts[0]).or_insert(HashSet::new()).insert(parts[1]);
    connection_maps.entry(parts[1]).or_insert(HashSet::new()).insert(parts[0]);
  });
  let mut connected: Vec<HashSet<String>> = vec![];
  connection_maps.iter().for_each(|(k, first_connections)| {
    first_connections.iter().for_each(|v_2| {
      for c in connected.iter_mut() {
        if c.contains(*v_2) && c.iter().all(|m| first_connections.contains(m.as_str())) {
          c.insert(k.to_string());
        }
      };
      connected.push(HashSet::from([k.to_string(), v_2.to_string()]));
    });
  });
  let sorted = connected.iter().sorted_by(|a, b| b.len().cmp(&a.len())).collect::<Vec<&HashSet<String>>>();
  let sorted_result = sorted[0].iter().sorted().collect::<Vec<&String>>();
  sorted_result.iter().join(",")
}
