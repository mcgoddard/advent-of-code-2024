use super::lib::minimum_length_sequence;

pub fn part1(lines: &[String]) -> String {
  let mut result = 0;
  for line in lines {
    let digit: i64 = line[0..3].parse().unwrap();
    let minimum_length = minimum_length_sequence(2, 0, line.to_string());
    let complexity = digit * minimum_length;
    result += complexity;
  }
  result.to_string()
}
