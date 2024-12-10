use std::collections::HashSet;

use super::lib::{get_visited, parse_input, Direction, Space};

pub fn part2(lines: &[String]) -> i64 {
  let (start_position, start_direction, map) = parse_input(lines);
  let mut visited = get_visited(start_position, &start_direction, &map);
  visited.remove(&start_position);
  let valid_positions = visited.iter().map(|(x, y)| {
    if (*x == start_position.0 && *y == start_position.1) || map[*y as usize][*x as usize] == Space::Blocked {
      return false;
    }
    let mut new_map = map.iter().map(|line| line.to_vec()).collect::<Vec<Vec<Space>>>();
    new_map[*y as usize][*x as usize] = Space::Blocked;
    is_cycle(start_position, &start_direction, &new_map)
  }).filter(|v| *v).collect::<Vec<bool>>();
  valid_positions.len() as i64
}

fn is_cycle(start_position: (i64, i64), start_direction: &Direction, map: &[Vec<Space>]) -> bool {
  let mut visited: HashSet::<(i64, i64, Direction)> = HashSet::new();
  let mut position = start_position;
  let mut direction = start_direction;
  while position.0 >= 0 && position.0 < map[0].len() as i64 && position.1 >= 0 && position.1 < map[0].len() as i64 {
    if !visited.contains(&(position.0, position.1, direction.clone())) {
      visited.insert((position.0, position.1, direction.clone()));
    } else {
      return true;
    }
    let new_position = match direction {
      Direction::Up => (position.0, position.1 - 1),
      Direction::Down => (position.0, position.1 + 1),
      Direction::Left => (position.0 - 1, position.1),
      Direction::Right => (position.0 + 1, position.1),
    };
    if new_position.0 >= 0 && new_position.0 < map[0].len() as i64 && new_position.1 >= 0 && new_position.1 < map[0].len() as i64 {
      match map[new_position.1 as usize][new_position.0 as usize] {
        Space::Blocked => {
          direction = match direction {
            Direction::Up => &Direction::Right,
            Direction::Right => &Direction::Down,
            Direction::Down => &Direction::Left,
            Direction::Left => &Direction::Up,
          };
        },
        Space::Open => {
          position = new_position;
        }
      }
    } else {
      position = new_position
    }
  }
  false
}
