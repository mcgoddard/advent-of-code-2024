use std::collections::HashSet;

use super::lib::OFFSETS;

pub fn part1(lines: &[String]) -> String {
  let chars = lines.iter().map(|l| l.chars().collect()).collect::<Vec<Vec<char>>>();
  let mut visited: HashSet<(usize, usize)> = HashSet::new();
  let mut result = 0;
  for y in 0..chars.len() {
    for x in 0..chars[0].len() {
      let (area, perimeter) = flood_fill(&chars, (x, y), &mut visited);
      result += area * perimeter;
    }
  }
  result.to_string()
}

fn flood_fill(map: &[Vec<char>], start_position: (usize, usize), visited: &mut HashSet<(usize, usize)>) -> (i64, i64) {
  if visited.contains(&start_position) {
    return (0, 0);
  }
  let mut area = 0;
  let mut perimeter = 0;
  visited.insert(start_position);
  area += 1;
  for offset in OFFSETS {
    let new_x = start_position.0 as i64 + offset.0;
    let new_y = start_position.1 as i64 + offset.1;
    if new_x >= 0 && new_x < map[0].len() as i64 && new_y >= 0 && new_y < map.len() as i64 && map[new_y as usize][new_x as usize] == map[start_position.1][start_position.0] {
      let (new_area, new_perimeter) = flood_fill(map, (new_x as usize, new_y as usize), visited);
      area += new_area;
      perimeter += new_perimeter;
    } else {
      perimeter += 1;
    }
  }
  (area, perimeter)
}
