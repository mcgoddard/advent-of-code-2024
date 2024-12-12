use std::collections::HashSet;

pub fn part1(lines: &[String]) -> i64 {
  let chars = lines.iter().map(|l| l.chars().collect()).collect::<Vec<Vec<char>>>();
  let mut visited: HashSet<(usize, usize)> = HashSet::new();
  let mut result = 0;
  for y in 0..chars.len() {
    for x in 0..chars[0].len() {
      let (area, perimeter) = flood_fill(&chars, (x, y), &mut visited);
      result += area * perimeter;
    }
  }
  result
}

fn flood_fill(map: &[Vec<char>], start_position: (usize, usize), visited: &mut HashSet<(usize, usize)>) -> (i64, i64) {
  if visited.contains(&start_position) {
    return (0, 0);
  }
  let mut area = 0;
  let mut perimeter = 0;
  visited.insert(start_position);
  area += 1;
  if start_position.0 >= 1 && map[start_position.1][start_position.0 - 1] == map[start_position.1][start_position.0] {
    let (new_area, new_perimeter) = flood_fill(map, (start_position.0 - 1, start_position.1), visited);
    area += new_area;
    perimeter += new_perimeter;
  } else {
    perimeter += 1;
  }
  if start_position.1 >= 1 && map[start_position.1 - 1][start_position.0] == map[start_position.1][start_position.0] {
    let (new_area, new_perimeter) = flood_fill(map, (start_position.0, start_position.1 - 1), visited);
    area += new_area;
    perimeter += new_perimeter;
  } else {
    perimeter += 1;
  }
  if start_position.0 < map[0].len() - 1 && map[start_position.1][start_position.0 + 1] == map[start_position.1][start_position.0] {
    let (new_area, new_perimeter) = flood_fill(map, (start_position.0 + 1, start_position.1), visited);
    area += new_area;
    perimeter += new_perimeter;
  } else {
    perimeter += 1;
  }
  if start_position.1 < map.len() - 1 && map[start_position.1 + 1][start_position.0] == map[start_position.1][start_position.0] {
    let (new_area, new_perimeter) = flood_fill(map, (start_position.0, start_position.1 + 1), visited);
    area += new_area;
    perimeter += new_perimeter;
  } else {
    perimeter += 1;
  }
  (area, perimeter)
}
