use std::collections::HashMap;

pub fn parse_antennas(lines: &[String]) -> HashMap<char, Vec<(i64, i64)>> {
  let mut antenna_map: HashMap<char, Vec<(i64, i64)>> = HashMap::new();
  for (y, line) in lines.iter().enumerate() {
    for (x, char) in line.chars().enumerate() {
      if char != '.' {
        antenna_map.entry(char).or_default().push((x as i64, y as i64));
      }
    }
  }
  antenna_map
}

pub fn get_antinode(position1: &(i64, i64), position2: &(i64, i64), diff: &(i64, i64), multiplier: i64) -> (i64, i64) {
  let mut antinode = (position1.0 + (diff.0 * multiplier), position1.1 + (diff.1 * multiplier));
  if position1.0 < position2.0 {
    antinode.0 = position1.0 - (diff.0 * multiplier);
  }
  if position1.1 < position2.1 {
    antinode.1 = position1.1 - (diff.1 * multiplier);
  }
  antinode
}
