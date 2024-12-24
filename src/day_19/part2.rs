use memoize::memoize;

use super::lib::{parse_input, Colors};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct TrieNode {
  pub children: Vec<Option<TrieNode>>,
  pub word_end: bool,
}

pub fn part2(lines: &[String]) -> String {
  let (available_towels, desired_designs) = parse_input(lines);
  let mut root = TrieNode {
    children: vec![None; 5],
    word_end: false,
  };
  for towel in available_towels {
    add_word(&mut root, towel);
  }
  let mut possible_count = 0;
  for design in desired_designs {
    possible_count += count_combinations(root.clone(), design);
  }
  possible_count.to_string()
}

#[memoize]
fn count_combinations(root: TrieNode, design: Vec<Colors>) -> i64 {
  let mut current = &mut root.clone();
  let n = design.len();
  let mut combinations = 0;

  if n == 0 {
    return 1;
  }

  for i in 0..n {
    let next = color_index(&design[i]);
    if let Some(next_child) = &mut current.children[next] {
      current = next_child;
    } else {
      return combinations;
    }

    if current.word_end {
      combinations += count_combinations(root.clone(), design[i + 1..].to_vec());
    }
  }

  combinations
}

fn color_index(c: &Colors) -> usize {
  match c {
    Colors::White => 0,
    Colors::Blue => 1,
    Colors::Black => 2,
    Colors::Red => 3,
    Colors::Green => 4,
  }
}

fn add_word(root: &mut TrieNode, word: Vec<Colors>) {
  let mut current = root;
  for c in word {
    let index = color_index(&c);
    if current.children[index].is_none() {
      let new_node = TrieNode {
        children: vec![None; 5],
        word_end: false,
      };
      current.children[index] = Some(new_node);
    }
    let next = current.children[index].as_mut().unwrap();
    current = next;
  }
  current.word_end = true;
}
