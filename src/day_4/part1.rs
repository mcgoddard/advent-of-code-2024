use regex::Regex;

pub fn part1(lines: &[String]) -> String {
  let re = Regex::new(r".*?(XMAS).*?").unwrap();
  let reverse_re = Regex::new(r".*?(SAMX).*?").unwrap();
  let mut result = 0;
  // Add horizonal lines
  let mut all_lines = lines.to_vec();
  // Add vertical lines
  let vertical = get_verticals(lines);
  all_lines.extend(vertical);
  // Add diagonal lines
  let diagonal  = get_diagonals(lines.iter().map(|l| l.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>());
  all_lines.extend(diagonal);
  // Add opposite diagonal lines
  let opposite_diagonal  = get_opposite_diagonals(lines.iter().map(|l| l.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>());
  all_lines.extend(opposite_diagonal);
  // Count matches
  for line in &all_lines {
    result += re.captures_iter(line).count() as i64;
    result += reverse_re.captures_iter(line).count() as i64;
  }
  result.to_string()
}

fn get_verticals(vector: &[String]) -> Vec<String> {
  let mut reversed = vector.to_owned();
  reversed.reverse();
  let reversed_chars = reversed.iter().map(|l| l.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
  let mut transposed = vec![];
  for i in 0..reversed_chars[0].len() {
    let mut new_chars = vec![];
    for chars in &reversed_chars {
      new_chars.push(chars[i]);
    }
    transposed.push(new_chars.into_iter().collect::<String>());
  }
  transposed
}

fn get_diagonals(vector: Vec<Vec<char>>) -> Vec<String> {
  let mut rotated = vec![];
  for (index, i) in (0..vector.len()).rev().enumerate() {
    let mut new_chars = vec![];
    for j in 0..(index + 1) {
      new_chars.push(vector[i + j][j]);
    }
    rotated.push(new_chars.into_iter().collect::<String>());
  }
  for (index, i) in (1..vector[0].len()).rev().enumerate() {
    let mut new_chars = vec![];
    for (j, v) in vector.iter().enumerate().take(index + 1) {
      new_chars.push(v[i + j]);
    }
    rotated.push(new_chars.into_iter().collect::<String>());
  }
  rotated
}

fn get_opposite_diagonals(vector: Vec<Vec<char>>) -> Vec<String> {
  let mut rotated = vec![];
  for (index, i) in (0..vector.len()).enumerate() {
    let mut new_chars = vec![];
    for j in 0..(index + 1) {
      new_chars.push(vector[i - j][j]);
    }
    rotated.push(new_chars.into_iter().collect::<String>());
  }
  for (index, i) in (1..vector[0].len()).rev().enumerate() {
    let mut new_chars = vec![];
    for j in (0..index + 1).rev() {
      new_chars.push(vector[i + j][vector[0].len() - j - 1]);
    }
    rotated.push(new_chars.into_iter().collect::<String>());
  }
  rotated
}
