use super::lib::next_secret;

pub fn part1(lines: &[String]) -> String {
  let buyers = lines.iter().map(|l| l.parse().unwrap()).collect::<Vec<i64>>();
  let results = buyers.iter().map(|b| {
    let mut secret = *b;
    for _ in 0..2000 {
      secret = next_secret(secret);
    }
    secret
  }).collect::<Vec<i64>>();
  results.iter().sum::<i64>().to_string()
}
