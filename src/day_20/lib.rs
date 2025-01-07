use std::{cmp::Ordering, collections::{BinaryHeap, HashSet}};

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Space {
  Empty,
  Wall,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Node {
  pub position: (i64, i64),
  pub parent: Option<Box<Node>>,
  pub g: i64,
  pub h: i64,
  pub f: i64,
}

impl Ord for Node {
  fn cmp(&self, other: &Self) -> Ordering {
    other.f.cmp(&self.f)
  }
}

impl PartialOrd for Node {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

type Inputs = (Vec<Vec<Space>>, (i64, i64), (i64, i64));

pub fn parse_input(lines: &[String]) -> Inputs {
  let mut start = (0, 0);
  let mut end = (0, 0);
  let map = lines.iter().enumerate().map(|(y, line)| {
    line.chars().enumerate().map(|(x, c)| match c {
      '#' => Space::Wall,
      '.' => Space::Empty,
      'S' => {
        start = (x as i64, y as i64);
        Space::Empty
      },
      'E' => {
        end = (x as i64, y as i64);
        Space::Empty
      },
      _ => panic!("Invalid space"),
    }).collect::<Vec<Space>>()
  }).collect::<Vec<Vec<Space>>>();
  (map, start, end)
}

pub fn a_star(map: &[Vec<Space>], start: (i64, i64), target: (i64, i64)) -> Option<Vec<(i64, i64)>> {
  let mut open_list = BinaryHeap::from([Node {
    position: start,
    parent: None,
    g: 0,
    h: 0,
    f: 0,
  }]);
  let mut closed_list = HashSet::new();
  // A star algorithm to find the shortest path to the end
  while !open_list.is_empty() {
    let current_node = open_list.pop().unwrap();
    // Have we reached the end?
    if current_node.position == target {
      // Walk back to find score
      let mut path = vec![];
      let mut current = current_node;
      while let Some(parent) = current.parent {
        path.push(current.position);
        current = *parent;
      }
      path.reverse();
      return Some(path);
    }
    closed_list.insert(current_node.position);
    // Generate children
    let mut new_nodes = vec![];
    for offset in [(-1, 0), (1, 0), (0, -1), (0, 1)].iter() {
      let new_position = (current_node.position.0 + offset.0, current_node.position.1 + offset.1);
      if closed_list.contains(&new_position) {
        continue;
      }
      if new_position.0 < 0 || new_position.0 >= map[0].len() as i64 || new_position.1 < 0 || new_position.1 >= map.len() as i64 {
        continue;
      }
      if map[new_position.1 as usize][new_position.0 as usize] == Space::Wall {
        continue;
      }
      let new_g = current_node.g + 1;
      let new_h = (new_position.0 - target.0).abs() + (new_position.1 - target.1).abs();
      let new_node = Node {
        position: new_position,
        parent: Some(Box::new(current_node.clone())),
        g: new_g,
        h: new_h,
        f: new_g + new_h,
      };
      new_nodes.push(new_node);
    }
    // Process children
    for new_node in new_nodes {
      open_list.retain(|i| i.position != new_node.position || i.g < new_node.g);
      open_list.push(new_node);
    }
  }
  None
}
