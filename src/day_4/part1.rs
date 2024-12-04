use regex::Regex;

pub fn part1(lines: Vec<String>) -> i64 {
  let re = Regex::new(r".*?(XMAS).*?").unwrap();
  let reverse_re = Regex::new(r".*?(SAMX).*?").unwrap();
  let mut result = 0;
  // Count horizontal matches
  for line in &lines {
    result += re.captures_iter(line).count() as i64;
    result += reverse_re.captures_iter(line).count() as i64;
  }
  // Count vertical matches
  let mut reversed = lines.clone();
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
  for line in &transposed {
    result += re.captures_iter(line).count() as i64;
    result += reverse_re.captures_iter(line).count() as i64;
  }
  // Count diagonal matches
  let diagonal  = get_diagonals(lines.iter().map(|l| l.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>());
  for line in &diagonal {
    result += re.captures_iter(line).count() as i64;
    result += reverse_re.captures_iter(line).count() as i64;
  }
  // Count opposite diagonal matches
  let opposite_diagonal  = get_opposite_diagonals(lines.iter().map(|l| l.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>());
  for line in &opposite_diagonal {
    result += re.captures_iter(line).count() as i64;
    result += reverse_re.captures_iter(line).count() as i64;
  }
  result
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
