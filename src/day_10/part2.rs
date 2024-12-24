use super::lib::{get_ratings, get_trailheads, parse_map};

pub fn part2(lines: &[String]) -> String {
  let map = parse_map(lines);
  let trailheads = get_trailheads(&map);
  let ratings = get_ratings(&map, trailheads);
  ratings.values().sum::<i64>().to_string()
}
