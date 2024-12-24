use std::collections::HashMap;

use super::lib::next_secret;

pub fn part2(lines: &[String]) -> String {
  let buyers = lines.iter().map(|l| l.parse().unwrap()).collect::<Vec<i64>>();
  let results = buyers.iter().map(|b| {
    let mut secrets = vec![];
    secrets.push(*b);
    for i in 1..2000 {
      secrets.push(next_secret(secrets[i - 1]));
    }
    let prices = secrets.iter().map(|s| s % 10).collect::<Vec<i64>>();
    let price_diffs = prices[0..prices.len() - 1].iter().zip(prices[1..prices.len()].iter()).map(|(a, b)| b - a).collect::<Vec<i64>>();
    let mut sequence_map = HashMap::new();
    price_diffs.windows(4).enumerate().for_each(|(i, w)| {
      let key = (w[0], w[1], w[2], w[3]);
      sequence_map.entry(key).or_insert(prices[i + 4]);
    });
    sequence_map
  }).collect::<Vec<HashMap<(i64, i64, i64, i64), i64>>>();
  let mut merged = HashMap::new();
  results.iter().for_each(|r| {
    r.iter().for_each(|(k, v)| {
      *merged.entry(k).or_insert(0) += v;
    });
  });
  let mut max = 0;
  for (_, v) in merged.iter() {
    if *v > max {
      max = *v;
    }
  }
  max.to_string()
}

