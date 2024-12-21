use std::collections::{HashMap, HashSet};

use super::lib::{a_star, Space};

pub fn part1(lines: &[String]) -> i64 {
  let mut start = (0, 0);
  let mut end = (0, 0);
  let map = lines.iter().enumerate().map(|(y, line)| {
    line.chars().enumerate().map(|(x, c)| match c {
      '#' => Space::Wall,
      '.' => Space::Empty,
      'S' => {
        start = (x as i64, y as i64);
        Space::Empty
      },
      'E' => {
        end = (x as i64, y as i64);
        Space::Empty
      },
      _ => panic!("Invalid space"),
    }).collect::<Vec<Space>>()
  }).collect::<Vec<Vec<Space>>>();
  let original_path = [vec![start], a_star(&map, start, end).unwrap()].concat();
  let path_set: HashSet<(i64, i64)> = HashSet::from_iter(original_path.iter().cloned());
  let mut cheats_count: HashMap<usize, i64> = HashMap::new();
  let offsets = [(-2, 0), (2, 0), (0, -2), (0, 2), (-1, -1), (-1, 1), (1, -1), (1, 1)];
  for (index, point) in original_path.iter().enumerate() {
    for offset in offsets.iter() {
      let new_point = (point.0 + offset.0, point.1 + offset.1);
      if new_point.0 < 0 || new_point.0 >= map[0].len() as i64 || new_point.1 < 0 || new_point.1 >= map.len() as i64 {
        continue;
      }
      if !path_set.contains(&new_point) {
        continue;
      }
      let start_to_new = original_path.iter().position(|&p| p == new_point).unwrap();
      if start_to_new > index + 2 {
        let saved = (start_to_new - index) - 2;
        *cheats_count.entry(saved).or_insert(0) += 1;
      }
    }
  }
  cheats_count.iter().filter(|(saved, _)| **saved >= 100).fold(0, |acc, (_, count)| acc + count)
}
