use std::collections::{HashMap, HashSet};

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
enum Direction {
  Left,
  Right,
  Up,
  Down,
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
enum Space {
  Empty,
  Wall,
  BoxLeft,
  BoxRight,
  Robot,
}

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
          if can_move_vertical(&map, new_position, *offset, &mut seen) {
            println!("Can move vertical {:?}", instruction);
          } else {
            println!("Cannot move vertical {:?}", instruction);
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

fn print_map(map: &Vec<Vec<Space>>) {
  for line in map {
    for char in line {
      print!("{}", match char {
        Space::Empty => '.',
        Space::Wall => '#',
        Space::BoxLeft => '[',
        Space::BoxRight => ']',
        Space::Robot => '@',
      });
    }
    println!();
  }
}

fn number_of_boxes(map: &Vec<Vec<Space>>, start: (usize, usize), offset: (i64, i64)) -> Option<i64> {
  let mut count = 1;
  let mut still_box = true;
  let mut position = start;
  while still_box {
    if position.0 < 1 && offset.0 < 0 || position.0 >= map[0].len() - 1 && offset.0 > 0 || position.1 < 1 && offset.1 < 0 || position.1 >= map.len() - 1 && offset.1 > 0 {
      still_box = false;
      continue;
    }
    let new_position = (((position.0 as i64) + offset.0) as usize, ((position.1 as i64) + offset.1) as usize);
    match map[new_position.1][new_position.0] {
      Space::Wall => return None,
      Space::Empty => still_box = false,
      Space::BoxLeft | Space::BoxRight => {
        count += 1;
        position = new_position;
      },
      Space::Robot => panic!("Found robot in box path"),
    }
  }
  Some(count)
}

fn can_move_vertical(map: &Vec<Vec<Space>>, position: (usize, usize), offset: (i64, i64), seen: &mut HashSet<(usize, usize)>) -> bool {
  if seen.contains(&position) {
    panic!("Already checked position {:?} {:?}", position, seen);
  }
  seen.insert(position);
  if position.0 < 1 && offset.0 < 0 || position.0 >= map[0].len() - 1 && offset.0 > 0 || position.1 < 1 && offset.1 < 0 || position.1 >= map.len() - 1 && offset.1 > 0 {
    return false;
  }
  let new_position = (((position.0 as i64) + offset.0) as usize, ((position.1 as i64) + offset.1) as usize);
  match map[position.1][position.0] {
    Space::Wall => false,
    Space::Empty => true,
    Space::BoxLeft => {
      let mut seen = seen.clone();
      let above = can_move_vertical(map, new_position, offset, &mut seen);
      let right = if seen.contains(&(position.0 + 1, position.1)) {
        true
      } else {
        can_move_vertical(map, (position.0 + 1, position.1), offset, &mut seen)
      };
      above && right
    },
    Space::BoxRight => {
      let mut seen = seen.clone();
      let above = can_move_vertical(map, new_position, offset, &mut seen);
      let left = if seen.contains(&(position.0 - 1, position.1)) {
        true
      } else {
        can_move_vertical(map, (position.0 - 1, position.1), offset, &mut seen)
      };
      above && left
    },
    Space::Robot => panic!("Found robot in box path"),
  }
}
