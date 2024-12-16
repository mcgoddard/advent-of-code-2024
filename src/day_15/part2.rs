use std::collections::{HashMap, HashSet};

use itertools::Itertools;

use super::lib::{number_of_boxes, print_map, Direction, Space};

pub fn part2(lines: &[String]) -> i64 {
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
      let mut map_row = vec![];
      for (x, c) in line.chars().enumerate() {
        match c {
          '.' => {
            map_row.push(Space::Empty);
            map_row.push(Space::Empty);
          },
          '#' => {
            map_row.push(Space::Wall);
            map_row.push(Space::Wall);
          },
          'O' => {
            map_row.push(Space::BoxLeft);
            map_row.push(Space::BoxRight);
          },
          '@' => {
            robot_position = (x * 2, y);
            map_row.push(Space::Robot);
            map_row.push(Space::Empty);
          },
          _ => panic!("Invalid character in map"),
        }
      }
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
      Space::BoxLeft | Space::BoxRight => {
        if instruction == Direction::Left || instruction == Direction::Right {
          let number_of_boxes = number_of_boxes(&map, new_position, *offset);
          if let Some(number_of_boxes) = number_of_boxes {
            let number_of_boxes = number_of_boxes;
            map[robot_position.1][robot_position.0] = Space::Empty;
            map[new_position.1][new_position.0] = Space::Robot;
            for i in 1..=number_of_boxes {
              if instruction == Direction::Right {
                map[new_position.1][(new_position.0 as i64 + (offset.0 * i)) as usize] = if i % 2 == 0 { Space::BoxRight } else { Space::BoxLeft };
              } else {
                map[new_position.1][(new_position.0 as i64 + (offset.0 * i)) as usize] = if i % 2 == 0 { Space::BoxLeft } else { Space::BoxRight };
              }
            }
            robot_position = new_position;
          }
        } else {
          let mut seen: HashSet<(usize, usize)> = HashSet::new();
          if let Some(move_set) = can_move_vertical(&map, new_position, *offset, &mut seen) {
            let mut ordered_move = move_set.iter().sorted_by(|a, b| a.1.cmp(&b.1)).collect::<Vec<&(usize, usize)>>();
            if instruction == Direction::Down {
              ordered_move.reverse();
            }
            for position in ordered_move {
              map[position.1 + offset.1 as usize][position.0] = map[position.1][position.0].clone();
              map[position.1][position.0] = Space::Empty;
            }
            map[robot_position.1][robot_position.0] = Space::Empty;
            map[new_position.1][new_position.0] = Space::Robot;
            robot_position = new_position;
          }
          print_map(&map);
        }
      }
      Space::Robot => panic!("Invalid robot position"),
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
  gps
}

fn can_move_vertical(map: &Vec<Vec<Space>>, position: (usize, usize), offset: (i64, i64), seen: &mut HashSet<(usize, usize)>) -> Option<HashSet<(usize, usize)>> {
  if seen.contains(&position) {
    panic!("Already checked position {:?} {:?}", position, seen);
  }
  if position.0 < 1 && offset.0 < 0 || position.0 >= map[0].len() - 1 && offset.0 > 0 || position.1 < 1 && offset.1 < 0 || position.1 >= map.len() - 1 && offset.1 > 0 {
    return None;
  }
  let new_position = (((position.0 as i64) + offset.0) as usize, ((position.1 as i64) + offset.1) as usize);
  match map[position.1][position.0] {
    Space::Wall => None,
    Space::Empty => Some(seen.clone()),
    Space::BoxLeft => {
      seen.insert(position);
      let mut seen = seen.clone();
      let above = can_move_vertical(map, new_position, offset, &mut seen);
      let right = if seen.contains(&(position.0 + 1, position.1)) {
        Some(seen.clone())
      } else {
        can_move_vertical(map, (position.0 + 1, position.1), offset, &mut seen)
      };
      if let (Some(above), Some(right)) = (above, right) {
        return Some(above.union(&right).cloned().collect());
      }
      return None;
    },
    Space::BoxRight => {
      seen.insert(position);
      let mut seen = seen.clone();
      let above = can_move_vertical(map, new_position, offset, &mut seen);
      let left = if seen.contains(&(position.0 - 1, position.1)) {
        Some(seen.clone())
      } else {
        can_move_vertical(map, (position.0 - 1, position.1), offset, &mut seen)
      };
      if let (Some(above), Some(left)) = (above, left) {
        return Some(above.union(&left).cloned().collect());
      }
      return None;
    },
    Space::Robot => panic!("Found robot in box path"),
  }
}
