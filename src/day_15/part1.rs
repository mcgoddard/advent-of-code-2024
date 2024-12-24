use std::collections::HashMap;

use super::lib::{number_of_boxes, Direction, Space};

pub fn part1(lines: &[String]) -> String {
  let mut map_complete = false;
  let mut map = vec![];
  let mut instructions = vec![];
  let mut robot_position = (0, 0);
  for (y, line) in lines.iter().enumerate() {
    if line.is_empty() {
      map_complete = true;
      continue;
    }
    if !map_complete {
      let map_row = line.chars().enumerate().map(|(x, c)| match c {
        '.' => Space::Empty,
        '#' => Space::Wall,
        'O' => Space::BoxLeft,
        '@' => {
          robot_position = (x, y);
          Space::Robot
        },
        _ => panic!("Invalid character in map"),
      }).collect::<Vec<Space>>();
      map.push(map_row);
    } else {
      let instruction = line.chars().map(|c| match c {
        '<' => Direction::Left,
        '>' => Direction::Right,
        '^' => Direction::Up,
        'v' => Direction::Down,
        _ => panic!("Invalid character in instruction"),
      }).collect::<Vec<Direction>>();
      instructions.extend(instruction);
    }
  }
  let offset_map: HashMap<Direction, (i64, i64)> = HashMap::from([
    (Direction::Left, (-1, 0)),
    (Direction::Right, (1, 0)),
    (Direction::Up, (0, -1)),
    (Direction::Down, (0, 1)),
  ]);
  for instruction in instructions {
    let offset = offset_map.get(&instruction).unwrap();
    if (robot_position.0 < 1 && instruction == Direction::Left) || (robot_position.0 >= map[0].len() - 1 && instruction == Direction::Right) || (robot_position.1 < 1 && instruction == Direction::Up) || (robot_position.1 >= map.len() - 1 && instruction == Direction::Down) {
      continue;
    }
    let new_position = (robot_position.0 + (offset.0 as usize), robot_position.1 + (offset.1 as usize));
    match map[new_position.1][new_position.0] {
      Space::Wall => continue,
      Space::Empty => {
        map[robot_position.1][robot_position.0] = Space::Empty;
        map[new_position.1][new_position.0] = Space::Robot;
        robot_position = new_position;
      },
      Space::BoxLeft => {
        let number_of_boxes = number_of_boxes(&map, new_position, *offset);
        if let Some(number_of_boxes) = number_of_boxes {
          map[robot_position.1][robot_position.0] = Space::Empty;
          map[new_position.1][new_position.0] = Space::Robot;
          robot_position = new_position;
          map[(new_position.1 as i64 + (offset.1 * number_of_boxes)) as usize][(new_position.0 as i64 + (offset.0 * number_of_boxes)) as usize] = Space::BoxLeft;
        }
      }
      _ => panic!("Invalid space in map {:?}", map[new_position.1][new_position.0]),
    }
  }
  let mut gps = 0;
  for (y, line) in map.iter().enumerate() {
    for (x, space) in line.iter().enumerate() {
      if *space == Space::BoxLeft {
        gps += y as i64 * 100 + x as i64;
      }
    }
  }
  gps.to_string()
}
