use std::collections::{HashMap, HashSet};

use super::lib::NetworkSet;

pub fn part1(lines: &[String]) -> String {
  let mut connection_maps = HashMap::new();
  lines.iter().for_each(|l| {
    let parts = l.split("-").collect::<Vec<&str>>();
    connection_maps.entry(parts[0]).or_insert(HashSet::new()).insert(parts[1]);
    connection_maps.entry(parts[1]).or_insert(HashSet::new()).insert(parts[0]);
  });
  let mut three_sets = HashSet::new();
  connection_maps.iter().for_each(|(k, first_connections)| {
    first_connections.iter().for_each(|v_2| {
      let second_connections = connection_maps.get(v_2).unwrap();
      second_connections.iter().for_each(|v_3| {
        if v_3 != k && first_connections.contains(v_3) {
          three_sets.insert(NetworkSet(HashSet::from([k.to_string(), v_2.to_string(), v_3.to_string()])));
        }
      });
    });
  });
  three_sets.iter().filter(|s| s.0.iter().any(|c| c.starts_with("t"))).count().to_string()
}
