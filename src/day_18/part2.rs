use std::collections::HashSet;

#[derive(Debug, Clone, Eq, PartialEq)]
enum Space {
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

pub fn part2(lines: &[String]) -> i64 {
  let coordinates = lines.iter().map(|l| l.split(",").map(|s| s.parse().unwrap()).collect()).collect::<Vec<Vec<usize>>>();
  let mut map = vec![];
  let width = 71;
  let height = 71;
  for _ in 0..height {
    let row = vec![Space::Open; width];
    map.push(row);
  }
  let mut last_path = HashSet::new();
  let mut first_unsuccessful_coordinate = (0, 0);
  for coordinate in coordinates.iter() {
    map[coordinate[1]][coordinate[0]] = Space::Corrupted;
    if !last_path.is_empty() && !last_path.contains(&(coordinate[0] as i64, coordinate[1] as i64)) {
      continue;
    }
    if let Some(path) = a_star(&map, (width as i64 - 1, height - 1)) {
      last_path = HashSet::from_iter(path.iter().cloned());
    } else {
      first_unsuccessful_coordinate = (coordinate[0] as i64, coordinate[1] as i64);
      break;
    }
  }
  println!("First unsuccessful coordinate: {:?}", first_unsuccessful_coordinate);
  0
}

fn a_star(map: &[Vec<Space>], target: (i64, i64)) -> Option<Vec<(i64, i64)>> {
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
  return None;
}
