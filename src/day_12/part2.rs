use std::collections::{HashMap, HashSet};

use super::lib::OFFSETS;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
enum Direction {
  Up,
  Down,
  Left,
  Right,
}

pub fn part2(lines: &[String]) -> String {
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
  result.to_string()
}

fn flood_fill(map: &[Vec<char>], start_position: (usize, usize), visited: &mut HashSet<(usize, usize)>, sides: &mut HashSet<(usize, usize, Direction)>) -> i64 {
  if visited.contains(&start_position) {
    return 0;
  }
  let mut area = 0;
  visited.insert(start_position);
  area += 1;
  let offset_direction = HashMap::from([
    ((0, -1), Direction::Up),
    ((-1, 0), Direction::Left),
    ((0, 1), Direction::Down),
    ((1, 0), Direction::Right),
  ]);
  for offset in OFFSETS {
    let new_x = start_position.0 as i64 + offset.0;
    let new_y = start_position.1 as i64 + offset.1;
    if new_x >= 0 && new_x < map[0].len() as i64 && new_y >= 0 && new_y < map.len() as i64 && map[new_y as usize][new_x as usize] == map[start_position.1][start_position.0]{
      let new_area = flood_fill(map, (new_x as usize, new_y as usize), visited, sides);
      area += new_area;
    } else {
      let direction = offset_direction.get(offset).unwrap();
      if !check_side(map, sides, start_position, direction) {
        sides.insert((start_position.0, start_position.1, direction.clone()));
      }
    }
  }
  area
}

fn check_side(map: &[Vec<char>], sides: &HashSet<(usize, usize, Direction)>, position: (usize, usize), direction: &Direction) -> bool {
  let direction_offset_map: HashMap<Direction, (i64, i64)> = HashMap::from([
    (Direction::Up, (0, -1)),
    (Direction::Left, (-1, 0)),
    (Direction::Down, (0, 1)),
    (Direction::Right, (1, 0)),
  ]);
  let offset = direction_offset_map.get(direction).unwrap();
  let (position_index, max_position, max_other) = if direction == &Direction::Up || direction == &Direction::Down {
    (position.0, map[0].len(), map.len())
  } else {
    (position.1, map.len(), map[0].len())
  };
  let offset_position = (position.0 as i64 + offset.0, position.1 as i64 + offset.1);
  for i in position_index + 1..max_position {
    if direction == &Direction::Up || direction == &Direction::Down {
      if map[position.1][i] != map[position.1][position.0] || (offset_position.1 >= 0 && (offset_position.1 as usize) < max_other && map[offset_position.1 as usize][i] == map[position.1][position.0]) {
        break;
      }
      if sides.contains(&(i, position.1, direction.clone())) {
        return true;
      }
    } else {
      if map[i][position.0] != map[position.1][position.0] || (offset_position.0 >= 0 && (offset_position.0 as usize) < max_other && map[i][offset_position.0 as usize] == map[position.1][position.0]) {
        break;
      }
      if sides.contains(&(position.0, i, direction.clone())) {
        return true;
      }
    }
  }
  if position_index > 0 {
    for i in (0..position_index).rev() {
      if direction == &Direction::Up || direction == &Direction::Down {
        if map[position.1][i] != map[position.1][position.0] || (offset_position.1 >= 0 && (offset_position.1 as usize) < max_other && map[offset_position.1 as usize][i] == map[position.1][position.0]) {
          break;
        }
        if sides.contains(&(i, position.1, direction.clone())) {
          return true;
        }
      } else {
        if map[i][position.0] != map[position.1][position.0] || (offset_position.0 >= 0 && (offset_position.0 as usize) < max_other && map[i][offset_position.0 as usize] == map[position.1][position.0]) {
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
