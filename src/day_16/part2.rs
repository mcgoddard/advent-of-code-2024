use std::collections::HashSet;

use super::lib::{get_facing_map, get_start_node, parse_input, Node, Space, Step};

pub fn part2(lines: &[String]) -> String {
  let (map, start, end) = parse_input(lines);
  let start_node = get_start_node(start);
  let facing_map = get_facing_map();
  let mut open_list = vec![start_node];
  let mut closed_list = vec![];
  let mut lowest_scoring_path_positions: HashSet<(i64, i64)> = HashSet::new();
  let mut low_score = 0;
  // A star algorithm to find the shortest path to the end
  while !open_list.is_empty() {
    let current_node = open_list.remove(0);
    // Have we reached the end?
    if current_node.step.position == end {
      // Walk back to find score
      let mut path = vec![];
      let mut positions = vec![];
      let mut current = current_node.clone();
      while let Some(parent) = current.parent {
        path.push(current.step);
        current = parent.as_ref().clone();
      }
      path.push(current.step);
      path.reverse();
      let mut current = path.remove(0);
      let mut steps = 0;
      let mut turns = 0;
      while !path.is_empty() {
        let next = path.remove(0);
        positions.push(next.position);
        if current.facing != next.facing {
          turns += 1;
        }
        steps += 1;
        current = next;
      }
      let score = steps + (turns * 1000);
      if lowest_scoring_path_positions.is_empty() {
        low_score = score;
      }
      if score > low_score {
        break;
      } else {
        for position in positions {
          lowest_scoring_path_positions.insert(position);
        }
      }
    }
    closed_list.push(current_node.clone());
    // Generate children
    let mut new_nodes = vec![];
    for offset in [(-1, 0), (1, 0), (0, -1), (0, 1)].iter() {
      let new_position = (current_node.clone().step.position.0 + offset.0, current_node.clone().step.position.1 + offset.1);
      if new_position.0 < 0 || new_position.0 >= map[0].len() as i64 || new_position.1 < 0 || new_position.1 >= map.len() as i64 {
        continue;
      }
      if map[new_position.1 as usize][new_position.0 as usize] == Space::Wall {
        continue;
      }
      let new_facing = facing_map.get(offset).unwrap().clone();
      let new_g = if current_node.clone().step.facing == new_facing { current_node.clone().g + 1 } else { current_node.clone().g + 1001 };
      let new_h = (new_position.0 - end.0).pow(2) + (new_position.1 - end.1).pow(2);
      let new_node = Node {
        step: Step {
          position: new_position,
          facing: new_facing,
        },
        parent: Some(Box::new(current_node.clone())),
        g: new_g,
        h: new_h,
        f: new_g + new_h,
      };
      // Allow children to be added to open_list if they have the same step but an equal or lower g score for reevaluation (i.e. to find other optimal paths)
      if closed_list.iter().any(|n| n.step == new_node.step && new_node.g > n.g) {
        continue;
      }
      new_nodes.push(new_node);
    }
    // Process children
    for new_node in new_nodes {
      if open_list.iter().any(|n| n.step == new_node.step && new_node.g > n.g) {
        continue;
      }
      open_list.push(new_node);
    }
    open_list.sort_by(|a, b| a.f.cmp(&b.f));
  }
  lowest_scoring_path_positions.insert(start);
  lowest_scoring_path_positions.len().to_string()
}
