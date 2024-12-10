pub fn part1(lines: &[String]) -> i64 {
  let split = lines.iter().map(|line| line.split_whitespace().collect()).collect::<Vec<Vec<&str>>>();
  let mut left = split.iter().map(|line| line[0].parse::<i64>().unwrap()).collect::<Vec<i64>>();
  let mut right = split.iter().map(|line| line[1].parse::<i64>().unwrap()).collect::<Vec<i64>>();
  left.sort();
  right.sort();
  let zipped = left.iter().zip(right.iter());
  let diffs = zipped.map(|(l, r)| (r - l).abs()).collect::<Vec<i64>>();
  diffs.iter().sum()
}
