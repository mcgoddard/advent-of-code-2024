use std::collections::{HashMap, HashSet};

use super::lib::{a_star, Space};

pub fn part2(lines: &[String]) -> i64 {
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
  for (index, point) in original_path.iter().enumerate() {
    for new_point in within_n(*point, 20) {
      if new_point.0 < 0 || new_point.0 >= map[0].len() as i64 || new_point.1 < 0 || new_point.1 >= map.len() as i64 {
        continue;
      }
      if !path_set.contains(&new_point) {
        continue;
      }
      let start_to_new = original_path.iter().position(|&p| p == new_point).unwrap();
      let cheat_distance = ((new_point.0 - point.0).abs() + (new_point.1 - point.1).abs()) as usize;
      if start_to_new > index + cheat_distance {
        let saved = (start_to_new - index) - cheat_distance;
        *cheats_count.entry(saved).or_insert(0) += 1;
      }
    }
  }
  cheats_count.iter().filter(|(saved, _)| **saved >= 100).fold(0, |acc, (_, count)| acc + count)
}

fn within_n(point: (i64, i64), n: i64) -> Vec<(i64, i64)> {
  let mut points = vec![];
  for x in -n..=n {
    for y in -n..=n {
      if x == 0 && y == 0 {
        continue;
      }
      if x.abs() + y.abs() > n {
        continue;
      }
      points.push((point.0 + x, point.1 + y));
    }
  }
  points
}
