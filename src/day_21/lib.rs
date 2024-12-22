use std::collections::HashMap;

use itertools::Itertools;
use memoize::memoize;

#[memoize]
pub fn minimum_length_sequence(number_of_bots: i64, level: i64, text: String) -> i64 {
  if level == number_of_bots + 1 {
    return text.len() as i64;
  }
  let keypad = if level == 0 { generate_final_mappings() } else { generate_intermediate_mappings() };
  let mut k_total = 0;
  let mut new_text = "A".to_string();
  new_text.push_str(&text);
  for (start, end) in new_text.chars().zip(text.chars()) {
    let mut minks = vec![];
    for path in keypad.get(&(start.to_string(), end.to_string())).unwrap_or(&vec![]) {
      let mut new_path = path.clone();
      new_path.push('A');
      minks.push(minimum_length_sequence(number_of_bots, level + 1, new_path));
    }
    k_total += minks.iter().min().unwrap_or(&1);
  }
  k_total
}

#[memoize]
fn generate_final_mappings() -> HashMap<(String, String), Vec<String>> {
  let keypad = vec![
    vec!["7", "8", "9"],
    vec!["4", "5", "6"],
    vec!["1", "2", "3"],
    vec!["", "0", "A"],
  ];
  generate_mappings(keypad)
}

#[memoize]
fn generate_intermediate_mappings() -> HashMap<(String, String), Vec<String>> {
  let keypad = vec![
    vec!["", "^", "A"],
    vec!["<", "v", ">"],
  ];
  generate_mappings(keypad)
}

fn generate_mappings(keypad: Vec<Vec<&str>>) -> HashMap<(String, String), Vec<String>> {
  let mut mappings = HashMap::new();
  for (y1, row_1) in keypad.iter().enumerate() {
    for (x1, key_1) in row_1.iter().enumerate() {
      if key_1.is_empty() {
        continue;
      }
      for (y2, row_2) in keypad.iter().enumerate() {
        for (x2, key_2) in row_2.iter().enumerate() {
          if key_2.is_empty() || key_1 == key_2 {
            continue;
          }
          let mut keys = vec![];
          if x1 > x2 { keys.extend(vec!["<".to_string(); x1 - x2]) } else { keys.extend(vec![">".to_string(); x2 - x1]) };
          if y1 > y2 { keys.extend(vec!["^".to_string(); y1 - y2]) } else { keys.extend(vec!["v".to_string(); y2 - y1]) };
          let permutations = keys.iter().permutations(keys.len()).unique().collect::<Vec<Vec<&String>>>();
          let permutations_strings = permutations.into_iter().filter(|p| {
            let mut x_offset = 0;
            let mut y_offset = 0;
            for key in p.iter() {
              match key.as_str() {
                "<" => x_offset -= 1,
                ">" => x_offset += 1,
                "^" => y_offset -= 1,
                "v" => y_offset += 1,
                _ => panic!("Invalid key"),
              }
              if keypad[y1 + y_offset][x1 + x_offset].is_empty() {
                return false;
              }
            }
            true
          }).map(|p| p.into_iter().join("")).collect::<Vec<String>>();

          mappings.insert((key_1.to_string(), key_2.to_string()), permutations_strings);
        }
      }
    }
  }
  mappings
}
