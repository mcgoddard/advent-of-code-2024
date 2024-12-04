use std::collections::HashSet;

pub fn part2(lines: Vec<String>) -> i64 {
  let mut result = 0;
  let surrounding_chars: HashSet<char> = HashSet::from_iter(vec!['M', 'S']);
  let chars = lines.iter().map(|l| l.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
  for i in 1..chars.len() - 1 {
    for j in 1..chars[i].len() - 1 {
      if chars[i][j] == 'A' &&
        surrounding_chars.contains(&chars[i - 1][j - 1]) && surrounding_chars.contains(&chars[i + 1][j + 1]) && chars[i - 1][j - 1] != chars[i + 1][j + 1] &&
        surrounding_chars.contains(&chars[i - 1][j + 1]) && surrounding_chars.contains(&chars[i + 1][j - 1]) && chars[i - 1][j + 1] != chars[i + 1][j - 1] {
        result += 1;
      }
    }
  }
  result
}
