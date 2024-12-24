use std::collections::HashSet;

use itertools::Itertools;

use super::lib::{get_antinode, parse_antennas};

pub fn part2(lines: &[String]) -> String {
  let antenna_map = parse_antennas(lines);
  let mut antinodes: HashSet<(i64, i64)> = HashSet::new();
  for (_, positions) in antenna_map.iter() {
    let combinations = positions.iter().combinations(2).collect::<Vec<Vec<&(i64, i64)>>>();
    for combination in combinations {
      antinodes.insert(*combination[0]);
      antinodes.insert(*combination[1]);
      let diff = ((combination[0].0 - combination[1].0).abs(), (combination[0].1 - combination[1].1).abs());
      let mut multiplier = 1;
      let mut placed = true;
      while placed {
        placed = false;
        let antinode1 = get_antinode(combination[0], combination[1], &diff, multiplier);
        if antinode1.0 >= 0 && antinode1.0 < lines[0].len() as i64 && antinode1.1 >= 0 && antinode1.1 < lines.len() as i64 {
          antinodes.insert(antinode1);
          placed = true;
        }
        let antinode2 = get_antinode(combination[1], combination[0], &diff, multiplier);
        if antinode2.0 >= 0 && antinode2.0 < lines[0].len() as i64 && antinode2.1 >= 0 && antinode2.1 < lines.len() as i64 {
          antinodes.insert(antinode2);
          placed = true;
        }
        multiplier += 1;
      }
    }
  }
  antinodes.len().to_string()
}
