#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Space {
  Open,
  Corrupted,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Node {
  pub position: (i64, i64),
  pub parent: Option<Box<Node>>,
  pub g: i64,
  pub h: i64,
  pub f: i64,
}

pub fn a_star(map: &[Vec<Space>], target: (i64, i64)) -> Option<Vec<(i64, i64)>> {
  let mut open_list = vec![Node {
    position: (0, 0),
    parent: None,
    g: 0,
    h: 0,
    f: 0,
  }];
  let mut closed_list = vec![];
  // A star algorithm to find the shortest path to the end
  while !open_list.is_empty() {
    let current_node = open_list.remove(0);
    // Have we reached the end?
    if current_node.position == target {
      // Walk back to find score
      let mut path = vec![];
      let mut current = current_node;
      while let Some(parent) = current.parent {
        path.push(current.position);
        current = parent.as_ref().clone();
      }
      path.reverse();
      return Some(path);
    }
    closed_list.push(current_node.clone());
    // Generate children
    let mut new_nodes = vec![];
    for offset in [(-1, 0), (1, 0), (0, -1), (0, 1)].iter() {
      let new_position = (current_node.clone().position.0 + offset.0, current_node.clone().position.1 + offset.1);
      if new_position.0 < 0 || new_position.0 >= map[0].len() as i64 || new_position.1 < 0 || new_position.1 >= map.len() as i64 {
        continue;
      }
      if map[new_position.1 as usize][new_position.0 as usize] == Space::Corrupted {
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
      if closed_list.iter().any(|n| n.position == new_node.position) {
        continue;
      }
      new_nodes.push(new_node);
    }
    // Process children
    for new_node in new_nodes {
      if let Some(i) = open_list.iter().position(|n| n.position == new_node.position) {
        if new_node.g < open_list[i].g {
          open_list.remove(i);
          open_list.push(new_node);
        }
      } else {
        open_list.push(new_node);
      }
    }
    open_list.sort_by(|a, b| a.f.cmp(&b.f));
  }
  None
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Node2 {
  pub position: (i64, i64),
  pub parent: Option<Box<Node2>>,
  pub g: i64,
  pub h: i64,
  pub f: i64,
}

pub fn a_star_2(map: &[Vec<Space>], target: (i64, i64)) -> Option<Vec<(i64, i64)>> {
  let mut open_list = vec![Node2 {
    position: (0, 0),
    parent: None,
    g: 0,
    h: 0,
    f: 0,
  }];
  let mut closed_list = vec![];
  // A star algorithm to find the shortest path to the end
  while !open_list.is_empty() {
    let current_node = open_list.remove(0);
    // Have we reached the end?
    if current_node.position == target {
      // Walk back to find score
      let mut path = vec![];
      let mut current = current_node;
      while let Some(parent) = current.clone().parent {
        path.push(current.position);
        current = *parent;
      }
      path.reverse();
      return Some(path);
    }
    closed_list.push(current_node.position);
    // Generate children
    let mut new_nodes = vec![];
    for offset in [(-1, 0), (1, 0), (0, -1), (0, 1)].iter() {
      let new_position = (current_node.position.0 + offset.0, current_node.position.1 + offset.1);
      if new_position.0 < 0 || new_position.0 >= map[0].len() as i64 || new_position.1 < 0 || new_position.1 >= map.len() as i64 {
        continue;
      }
      if map[new_position.1 as usize][new_position.0 as usize] == Space::Corrupted {
        continue;
      }
      let new_g = current_node.g + 1;
      let new_h = (new_position.0 - target.0).abs() + (new_position.1 - target.1).abs();
      let new_node = Node2 {
        position: new_position,
        parent: Some(Box::new(current_node.clone())),
        g: new_g,
        h: new_h,
        f: new_g + new_h,
      };
      if closed_list.iter().any(|p| *p == new_node.position) {
        continue;
      }
      new_nodes.push(new_node);
    }
    // Process children
    for new_node in new_nodes {
      if let Some(i) = open_list.iter().position(|n| n.position == new_node.position) {
        if new_node.g < open_list[i].g {
          open_list.remove(i);
          open_list.push(new_node);
        }
      } else {
        open_list.push(new_node);
      }
    }
    open_list.sort_by(|a, b| a.f.cmp(&b.f));
  }
  None
}