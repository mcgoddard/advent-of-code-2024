use super::lib::iterate;

pub fn part1(lines: &[String]) -> String {
  let stones = lines[0].split(" ").map(|s| s.parse().unwrap()).collect::<Vec<i64>>();
  iterate(stones, 25).to_string()
}
