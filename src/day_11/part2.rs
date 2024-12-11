use super::lib::iterate;

pub fn part2(lines: &[String]) -> i64 {
  let stones = lines[0].split(" ").map(|s| s.parse().unwrap()).collect::<Vec<i64>>();
  iterate(stones, 75)
}
