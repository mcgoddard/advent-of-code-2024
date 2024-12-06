use std::collections::HashSet;

use super::lib::{parse_input, Direction, Space};

pub fn part1(lines: Vec<String>) -> i64 {
  let (mut position, mut direction, map) = parse_input(&lines);
  let mut visited: HashSet::<(i64, i64)> = HashSet::new();
  while position.0 >= 0 && position.0 < lines[0].len() as i64 && position.1 >= 0 && position.1 < lines[0].len() as i64 {
    visited.insert((position.0, position.1));
    let new_position = match direction {
      Direction::Up => (position.0, position.1 - 1),
      Direction::Down => (position.0, position.1 + 1),
      Direction::Left => (position.0 - 1, position.1),
      Direction::Right => (position.0 + 1, position.1),
    };
    if new_position.0 >= 0 && new_position.0 < lines[0].len() as i64 && new_position.1 >= 0 && new_position.1 < lines[0].len() as i64 {
      match map[new_position.1 as usize][new_position.0 as usize] {
        Space::Blocked => {
          direction = match direction {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
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
  visited.len() as i64
}
