pub fn part2(lines: Vec<String>) -> i64 {
  let split = lines.iter().map(|line| line.split_whitespace().collect()).collect::<Vec<Vec<&str>>>();
  let left = split.iter().map(|line| line[0].parse::<i64>().unwrap()).collect::<Vec<i64>>();
  let right = split.iter().map(|line| line[1].parse::<i64>().unwrap()).collect::<Vec<i64>>();
  let left_counts = left.iter().fold(std::collections::HashMap::new(), |mut acc, &x| {
    *acc.entry(x).or_insert(0) += 1;
    acc
  });
  let right_counts = right.iter().fold(std::collections::HashMap::new(), |mut acc, &x| {
    *acc.entry(x).or_insert(0) += 1;
    acc
  });
  let mut similarity_score = 0;
  for (k, left_v) in left_counts.iter() {
    if let Some(right_v) = right_counts.get(k) {
      similarity_score += (k * right_v) * left_v;
    }
  }
  similarity_score
}
