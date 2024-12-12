use std::collections::HashSet;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
enum Direction {
  Up,
  Down,
  Left,
  Right,
}

pub fn part2(lines: &[String]) -> i64 {
  let chars = lines.iter().map(|l| l.chars().collect()).collect::<Vec<Vec<char>>>();
  let mut visited: HashSet<(usize, usize)> = HashSet::new();
  let mut result = 0;
  for y in 0..chars.len() {
    for x in 0..chars[0].len() {
      let mut sides_set: HashSet<(usize, usize, Direction)> = HashSet::new();
      let area = flood_fill(&chars, (x, y), &mut visited, &mut sides_set);
      result += area * sides_set.len() as i64;
    }
  }
  result
}

fn flood_fill(map: &[Vec<char>], start_position: (usize, usize), visited: &mut HashSet<(usize, usize)>, sides: &mut HashSet<(usize, usize, Direction)>) -> i64 {
  if visited.contains(&start_position) {
    return 0;
  }
  let mut area = 0;
  visited.insert(start_position);
  area += 1;
  if start_position.0 >= 1 && map[start_position.1][start_position.0 - 1] == map[start_position.1][start_position.0] {
    let new_area = flood_fill(map, (start_position.0 - 1, start_position.1), visited, sides);
    area += new_area;
  } else if !check_side(map, sides, start_position, Direction::Left) {
    sides.insert((start_position.0, start_position.1, Direction::Left));
  }
  if start_position.1 >= 1 && map[start_position.1 - 1][start_position.0] == map[start_position.1][start_position.0] {
    let new_area = flood_fill(map, (start_position.0, start_position.1 - 1), visited, sides);
    area += new_area;
  } else if !check_side(map, sides, start_position, Direction::Up) {
    sides.insert((start_position.0, start_position.1, Direction::Up));
  }
  if start_position.0 < map[0].len() - 1 && map[start_position.1][start_position.0 + 1] == map[start_position.1][start_position.0] {
    let new_area = flood_fill(map, (start_position.0 + 1, start_position.1), visited, sides);
    area += new_area;
  } else if !check_side(map, sides, start_position, Direction::Right) {
    sides.insert((start_position.0, start_position.1, Direction::Right));
  }
  if start_position.1 < map.len() - 1 && map[start_position.1 + 1][start_position.0] == map[start_position.1][start_position.0] {
    let new_area = flood_fill(map, (start_position.0, start_position.1 + 1), visited, sides);
    area += new_area;
  } else if !check_side(map, sides, start_position, Direction::Down) {
    sides.insert((start_position.0, start_position.1, Direction::Down));
  }
  area
}

fn check_side(map: &[Vec<char>], sides: &HashSet<(usize, usize, Direction)>, position: (usize, usize), direction: Direction) -> bool {
  if direction == Direction::Up {
    for i in position.0 + 1..map[0].len() {
      if map[position.1][i] != map[position.1][position.0] || (position.1 > 0 && map[position.1 - 1][i] == map[position.1][position.0]) {
        break;
      }
      if sides.contains(&(i, position.1, direction.clone())) {
        return true;
      }
    }
    if position.0 > 0 {
      for i in (0..position.0).rev() {
        if map[position.1][i] != map[position.1][position.0] || (position.1 > 0 && map[position.1 - 1][i] == map[position.1][position.0]) {
          break;
        }
        if sides.contains(&(i, position.1, direction.clone())) {
          return true;
        }
      }
    }
  }
  if direction == Direction::Down {
    for i in position.0 + 1..map[0].len() {
      if map[position.1][i] != map[position.1][position.0] || (position.1 < map.len() - 1 && map[position.1 + 1][i] == map[position.1][position.0]) {
        break;
      }
      if sides.contains(&(i, position.1, direction.clone())) {
        return true;
      }
    }
    if position.0 > 0 {
      for i in (0..position.0).rev() {
        if map[position.1][i] != map[position.1][position.0] || (position.1 < map.len() - 1 && map[position.1 + 1][i] == map[position.1][position.0]) {
          break;
        }
        if sides.contains(&(i, position.1, direction.clone())) {
          return true;
        }
      }
    }
  }
  if direction == Direction::Left {
    for i in position.1 + 1..map.len() {
      if map[i][position.0] != map[position.1][position.0] || (position.0 > 0 && map[i][position.0 - 1] == map[position.1][position.0]) {
        break;
      }
      if sides.contains(&(position.0, i, direction.clone())) {
        return true;
      }
    }
    if position.1 > 0 {
      for i in (0..position.1).rev() {
        if map[i][position.0] != map[position.1][position.0] || (position.0 > 0 && map[i][position.0 - 1] == map[position.1][position.0]) {
          break;
        }
        if sides.contains(&(position.0, i, direction.clone())) {
          return true;
        }
      }
    }
  }
  if direction == Direction::Right {
    for i in position.1 + 1..map.len() {
      if map[i][position.0] != map[position.1][position.0] || (position.0 < map.len() - 1 && map[i][position.0 + 1] == map[position.1][position.0]) {
        break;
      }
      if sides.contains(&(position.0, i, direction.clone())) {
        return true;
      }
    }
    if position.1 > 0 {
      for i in (0..position.1).rev() {
        if map[i][position.0] != map[position.1][position.0] || (position.0 < map.len() - 1 && map[i][position.0 + 1] == map[position.1][position.0]) {
          break;
        }
        if sides.contains(&(position.0, i, direction.clone())) {
          return true;
        }
      }
    }
  }
  false
}
