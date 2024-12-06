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
