use std::collections::HashSet;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Space {
  Open,
  Blocked,
}

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
pub enum Direction {
  Up,
  Down,
  Left,
  Right,
}

pub fn parse_input(lines: &Vec<String>) -> ((i64, i64), Direction, Vec<Vec<Space>>) {
  let mut position: (i64, i64) = (0, 0);
  let mut direction = Direction::Up;
  let map: Vec<Vec<Space>> = lines.iter().enumerate().map(|(y, line)| line.chars().enumerate().map(|(x, c)| {
    match c {
      '#' => Space::Blocked,
      '^' => {
        direction = Direction::Up;
        position = (x as i64, y as i64);
        return Space::Open;
      },
      'V' => {
        direction = Direction::Down;
        position = (x as i64, y as i64);
        return Space::Open;
      },
      '<' => {
        direction = Direction::Left;
        position = (x as i64, y as i64);
        return Space::Open;
      },
      '>' => {
        direction = Direction::Right;
        position = (x as i64, y as i64);
        return Space::Open;
      },
      _ => Space::Open,
    }
  }).collect()).collect();
  return (position, direction, map);
}

pub fn get_visited(start_position: (i64, i64), start_direction: &Direction, map: &Vec<Vec<Space>>) -> HashSet<(i64, i64)> {
  let mut visited: HashSet::<(i64, i64)> = HashSet::new();
  let mut position = start_position;
  let mut direction = start_direction;
  while position.0 >= 0 && position.0 < map[0].len() as i64 && position.1 >= 0 && position.1 < map[0].len() as i64 {
    visited.insert((position.0, position.1));
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
  visited
}
