#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub enum Direction {
  Left,
  Right,
  Up,
  Down,
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub enum Space {
  Empty,
  Wall,
  BoxLeft,
  BoxRight,
  Robot,
}

pub fn number_of_boxes(map: &Vec<Vec<Space>>, start: (usize, usize), offset: (i64, i64)) -> Option<i64> {
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

pub fn print_map(map: &Vec<Vec<Space>>) {
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
