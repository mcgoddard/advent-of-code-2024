use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum Space {
  Empty,
  Wall,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum Facing {
  North,
  East,
  South,
  West,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Node {
  pub step: Step,
  pub parent: Option<Box<Node>>,
  pub g: i64,
  pub h: i64,
  pub f: i64,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Step {
  pub position: (i64, i64),
  pub facing: Facing,
}

type InputValues = (Vec<Vec<Space>>, (i64, i64), (i64, i64));

pub fn parse_input(lines: &[String]) -> InputValues {
  let mut start = (0, 0);
  let mut end = (0, 0);
  let map = lines.iter().enumerate().map(|(y, l)| {
    l.chars().enumerate().map(|(x, c)| match c {
      '.' => Space::Empty,
      '#' => Space::Wall,
      'S' => {
        start = (x as i64, y as i64);
        Space::Empty
      },
      'E' => {
        end = (x as i64, y as i64);
        Space::Empty
      },
      _ => panic!("Invalid character in map"),
    }).collect::<Vec<Space>>()
  }).collect::<Vec<Vec<Space>>>();
  (map, start, end)
}

pub fn get_facing_map() -> HashMap<(i64, i64), Facing> {
  HashMap::from([
    ((0, -1), Facing::North),
    ((1, 0), Facing::East),
    ((0, 1), Facing::South),
    ((-1, 0), Facing::West),
  ])
}

pub fn get_start_node(start: (i64, i64)) -> Node {
  Node {
    step: Step {
      position: start,
      facing: Facing::East,
    },
    parent: None,
    g: 0,
    h: 0,
    f: 0,
  }
}