use std::collections::HashSet;

use super::lib::{Space, a_star};

pub fn part2(lines: &[String]) -> String {
  let coordinates = lines.iter().map(|l| l.split(",").map(|s| s.parse().unwrap()).collect()).collect::<Vec<Vec<usize>>>();
  let mut map = vec![];
  let width = 71;
  let height = 71;
  for _ in 0..height {
    let row = vec![Space::Open; width];
    map.push(row);
  }
  let mut last_path = HashSet::new();
  let mut first_unsuccessful_coordinate = (0, 0);
  for coordinate in coordinates.iter() {
    map[coordinate[1]][coordinate[0]] = Space::Corrupted;
    if !last_path.is_empty() && !last_path.contains(&(coordinate[0] as i64, coordinate[1] as i64)) {
      continue;
    }
    if let Some(path) = a_star(&map, (width as i64 - 1, height - 1)) {
      last_path = HashSet::from_iter(path.iter().cloned());
    } else {
      first_unsuccessful_coordinate = (coordinate[0] as i64, coordinate[1] as i64);
      break;
    }
  }
  format!("{},{}", first_unsuccessful_coordinate.0, first_unsuccessful_coordinate.1)
}
