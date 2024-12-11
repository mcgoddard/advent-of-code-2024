use memoize::memoize;

#[memoize]
pub fn iterate(stones: Vec<i64>, iterations: i64) -> i64 {
  if iterations == 0 {
    return stones.len() as i64;
  }
  let new_stones = stones.into_iter().map(apply_rules).fold(Vec::new(), |mut acc, s| {
    acc.extend(s.iter());
    acc
  });
  new_stones.iter().map(|s| iterate(vec![*s], iterations - 1)).sum()
}

#[memoize]
pub fn apply_rules(stone: i64) -> Vec<i64> {
  if stone == 0 {
    vec![1]
  } else if stone.to_string().len() % 2 == 0 {
    let stone_string = stone.to_string();
    vec![stone_string[0..stone_string.len() / 2].parse().unwrap(), stone_string[stone_string.len() / 2..stone_string.len()].parse().unwrap()]
  } else {
    vec![stone * 2024]
  }
}
